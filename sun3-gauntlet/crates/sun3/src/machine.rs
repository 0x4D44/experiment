//! Sun 3/60 machine, Sun-3 MMU, physical bus, interrupts, and PROM runner.

use crate::idprom::{default_idprom, is_valid};
use crate::scc::Z8530;
use m68k::core::memory::{BusFault, BusFaultKind};
use m68k::{AddressBus, CpuCore, CpuType, NoOpHleHandler, StepResult};
use std::collections::{BTreeSet, VecDeque};
use std::fmt::Write as _;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

const PROM_SIZE: usize = 64 * 1024;
const MAX_RAM: usize = 24 * 1024 * 1024;
const FRAMEBUFFER_BASE: u32 = 0xff00_0000;
const FRAMEBUFFER_SIZE: usize = 256 * 1024;
const TYPE1_PROM_BASE: u32 = 0x0010_0000;
const TYPE1_PROM_END: u32 = TYPE1_PROM_BASE + PROM_SIZE as u32 - 1;
const TIMER_PERIOD_CYCLES: u64 = 200_000;

const PTE_VALID: u32 = 0x8000_0000;
const PTE_WRITABLE: u32 = 0x4000_0000;
const PTE_SYSTEM: u32 = 0x2000_0000;
const PTE_TYPE_MASK: u32 = 0x0c00_0000;
const PTE_ACCESSED: u32 = 0x0200_0000;
const PTE_MODIFIED: u32 = 0x0100_0000;
const PTE_PAGE_MASK: u32 = 0x0007_ffff;
const PTE_IMPLEMENTED_MASK: u32 = 0xff07_ffff;

const BUS_ERROR_TIMEOUT: u8 = 0x20;
const BUS_ERROR_PROTECTION: u8 = 0x40;
const BUS_ERROR_INVALID: u8 = 0x80;

const POST_MARKERS: &[(u32, &str)] = &[
    (0x0fef_b104, "bus-error diagnostic"),
    (0x0fef_b18e, "interrupt diagnostic"),
    (0x0fef_b1da, "clock interrupt diagnostic"),
    (0x0fef_b344, "MMU valid-bit diagnostic"),
    (0x0fef_b3c4, "MMU write-protection diagnostic"),
    (0x0fef_b45e, "parity non-NMI diagnostic"),
    (0x0fef_b50c, "parity NMI diagnostic"),
    (0x0fef_b5c8, "RAM sizing"),
    (0x0fef_581c, "EPROM remapping check"),
    (0x0fef_02b2, "framebuffer enable"),
];

/// Selective tracing controls.
#[derive(Debug, Clone, Copy, Default)]
pub struct TraceFlags {
    /// Record one line per instruction.
    pub cpu: bool,
    /// Record MMU translations and faults.
    pub mmu: bool,
    /// Record physical bus transactions.
    pub bus: bool,
    /// Record device register accesses.
    pub io: bool,
    /// Record interrupt changes and acknowledgements.
    pub irq: bool,
    /// Record SCC register accesses and characters.
    pub scc: bool,
    /// Record PROM POST marker PCs.
    pub prom: bool,
}

impl TraceFlags {
    /// Whether any trace class is active.
    #[must_use]
    pub fn any(self) -> bool {
        self.cpu || self.mmu || self.bus || self.io || self.irq || self.scc || self.prom
    }
}

/// Runtime configuration for one deterministic machine instance.
#[derive(Debug, Clone)]
pub struct MachineConfig {
    /// Installed RAM in bytes.
    pub ram_bytes: usize,
    /// Deterministic Ethernet address.
    pub mac: [u8; 6],
    /// Deterministic 24-bit host ID serial field.
    pub host_id: u32,
    /// Diagnostic switch state.
    pub diagnostic_switch: bool,
    /// Trace classes.
    pub traces: TraceFlags,
    /// Maximum retained trace lines.
    pub trace_capacity: usize,
}

impl Default for MachineConfig {
    fn default() -> Self {
        Self {
            ram_bytes: MAX_RAM,
            mac: [0x08, 0x00, 0x20, 0x12, 0x34, 0x56],
            host_id: 0x12_34_56,
            diagnostic_switch: false,
            traces: TraceFlags::default(),
            trace_capacity: 8192,
        }
    }
}

/// Result of a bounded emulator run.
#[derive(Debug, Clone)]
pub struct RunOutcome {
    /// Whether the PROM monitor prompt was seen.
    pub monitor_reached: bool,
    /// Number of monitor prompts seen.
    pub prompt_count: usize,
    /// Whether scripted input was injected after the first prompt.
    pub script_injected: bool,
    /// Retired instruction boundaries.
    pub instructions: u64,
    /// Approximate guest CPU cycles.
    pub cycles: u64,
    /// Last program counter.
    pub final_pc: u32,
    /// Captured serial console bytes.
    pub console: Vec<u8>,
    /// Human-readable termination reason.
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AccessSize {
    Byte,
    Word,
    Three,
    Long,
}

impl AccessSize {
    const fn bytes(self) -> usize {
        match self {
            Self::Byte => 1,
            Self::Word => 2,
            Self::Three => 3,
            Self::Long => 4,
        }
    }

    const fn mask(self) -> u32 {
        match self {
            Self::Byte => 0xff,
            Self::Word => 0xffff,
            Self::Three => 0x00ff_ffff,
            Self::Long => u32::MAX,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Translation {
    page_type: u8,
    physical: u32,
    pte: u32,
}

#[derive(Debug, Clone, Copy)]
struct LastFault {
    virtual_address: u32,
    physical_address: Option<u32>,
    function_code: u8,
    write: bool,
    size: AccessSize,
    reason: &'static str,
    pte: Option<u32>,
}

/// Complete functional Sun 3/60 machine around an MC68020 core.
pub struct Sun3Machine {
    rom: Vec<u8>,
    ram: Vec<u8>,
    framebuffer: Vec<u8>,
    nvram: Vec<u8>,
    idprom: [u8; 32],

    context: u8,
    segment_map: Vec<u8>,
    page_map: Vec<u32>,
    enable: u8,
    dvma_enable: u8,
    bus_error: u8,
    diagnostic: u8,
    diagnostic_switch: bool,
    cache_tags: Vec<u32>,
    cache_data: Vec<u32>,

    irq_control: u8,
    rtc_irq_pending: bool,
    timer_irq_pending: bool,
    parity_control: u8,
    parity_address: u32,

    keyboard_scc: Z8530,
    serial_scc: Z8530,
    ncr5380: [u8; 8],
    scsi_control: u16,
    udc_address: u16,
    udc_registers: [u16; 32],
    lance_rap: u16,
    lance_csrs: [u16; 4],

    current_function_code: u8,
    current_pc: u32,
    guest_cycles: u64,
    next_timer_cycle: u64,
    console: Vec<u8>,
    console_flush_cursor: usize,
    trace_flags: TraceFlags,
    trace_capacity: usize,
    trace: VecDeque<String>,
    last_fault: Option<LastFault>,
    last_irq_level: u8,
}

impl Sun3Machine {
    /// Construct a reset machine around a validated 64 KiB logical PROM.
    pub fn new(rom: Vec<u8>, config: &MachineConfig) -> Result<Self, String> {
        if rom.len() != PROM_SIZE {
            return Err(format!(
                "Sun 3/60 PROM must be exactly {PROM_SIZE} bytes; got {}",
                rom.len()
            ));
        }
        if !(4 * 1024 * 1024..=MAX_RAM).contains(&config.ram_bytes)
            || config.ram_bytes % (4 * 1024 * 1024) != 0
        {
            return Err("RAM must be 4, 8, 12, 16, 20, or 24 MiB".to_owned());
        }
        let idprom = default_idprom(config.mac, config.host_id);
        debug_assert!(is_valid(&idprom));

        let mut machine = Self {
            rom,
            ram: vec![0; config.ram_bytes],
            framebuffer: vec![0; FRAMEBUFFER_SIZE],
            nvram: vec![0; 2048],
            idprom,
            context: 0,
            segment_map: vec![0; 8 * 2048],
            page_map: vec![0; 4096],
            enable: 0,
            dvma_enable: 0,
            bus_error: 0,
            diagnostic: 1,
            diagnostic_switch: config.diagnostic_switch,
            cache_tags: vec![0; 4096],
            cache_data: vec![0; 16384],
            irq_control: 0,
            rtc_irq_pending: false,
            timer_irq_pending: false,
            parity_control: 0,
            parity_address: 0,
            keyboard_scc: Z8530::default(),
            serial_scc: Z8530::default(),
            ncr5380: [0; 8],
            scsi_control: 0,
            udc_address: 0,
            udc_registers: [0; 32],
            lance_rap: 0,
            lance_csrs: [0; 4],
            current_function_code: 6,
            current_pc: 0,
            guest_cycles: 0,
            next_timer_cycle: TIMER_PERIOD_CYCLES,
            console: Vec::new(),
            console_flush_cursor: 0,
            trace_flags: config.traces,
            trace_capacity: config.trace_capacity.max(128),
            trace: VecDeque::new(),
            last_fault: None,
            last_irq_level: 0,
        };
        machine.reset_devices();
        Ok(machine)
    }

    /// Access the exact serial transcript accumulated so far.
    #[must_use]
    pub fn console(&self) -> &[u8] {
        &self.console
    }

    /// Drain trace lines to a string without altering machine state.
    #[must_use]
    pub fn trace_dump(&self) -> String {
        let mut dump = String::new();
        for line in &self.trace {
            let _ = writeln!(dump, "{line}");
        }
        if let Some(fault) = self.last_fault {
            let _ = writeln!(
                dump,
                "LAST-FAULT pc={:08x} va={:08x} pa={} fc={} {} {:?} reason={} pte={}",
                self.current_pc,
                fault.virtual_address,
                fault
                    .physical_address
                    .map_or_else(|| "-".to_owned(), |value| format!("{value:08x}")),
                fault.function_code,
                if fault.write { "write" } else { "read" },
                fault.size,
                fault.reason,
                fault
                    .pte
                    .map_or_else(|| "-".to_owned(), |value| format!("{value:08x}"))
            );
        }
        dump
    }

    /// Whether the FPP enable bit exposes the attached MC68881.
    #[must_use]
    pub fn fpp_enabled(&self) -> bool {
        self.enable & 0x40 != 0
    }

    /// Set the PC associated with subsequent bus trace entries.
    pub fn set_current_pc(&mut self, pc: u32) {
        self.current_pc = pc;
    }

    /// Add bytes to serial port A's receive FIFO.
    pub fn inject_serial_a(&mut self, bytes: &[u8]) {
        self.serial_scc.inject_channel_a(bytes);
        if self.trace_flags.scc {
            self.record_trace(format!(
                "SCC pc={:08x} inject ttya {:?}",
                self.current_pc,
                String::from_utf8_lossy(bytes)
            ));
        }
    }

    /// Write newly produced console bytes to a host stream.
    pub fn flush_console<W: std::io::Write>(&mut self, mut output: W) -> std::io::Result<()> {
        if self.console_flush_cursor < self.console.len() {
            output.write_all(&self.console[self.console_flush_cursor..])?;
            output.flush()?;
            self.console_flush_cursor = self.console.len();
        }
        Ok(())
    }

    /// Current highest asserted interrupt priority level.
    #[must_use]
    pub fn irq_level(&self) -> u8 {
        if self.irq_control & 0x01 == 0 {
            return 0;
        }
        if self.timer_irq_pending && self.irq_control & 0x80 != 0 {
            return 7;
        }
        if self.parity_control & 0x80 != 0 && self.parity_control & 0x40 != 0 {
            return 7;
        }
        if self.serial_scc.interrupt_pending() || self.keyboard_scc.interrupt_pending() {
            return 6;
        }
        if self.rtc_irq_pending && self.irq_control & 0x20 != 0 {
            return 5;
        }
        if self.irq_control & 0x08 != 0 {
            return 3;
        }
        if self.irq_control & 0x04 != 0 {
            return 2;
        }
        if self.irq_control & 0x02 != 0 {
            return 1;
        }
        0
    }

    /// Advance deterministic devices by guest CPU cycles.
    pub fn advance_cycles(&mut self, cycles: u64) {
        self.guest_cycles = self.guest_cycles.saturating_add(cycles);
        while self.guest_cycles >= self.next_timer_cycle {
            self.next_timer_cycle = self.next_timer_cycle.saturating_add(TIMER_PERIOD_CYCLES);
            if self.irq_control & 0x81 == 0x81 {
                self.timer_irq_pending = true;
                if self.trace_flags.irq {
                    self.record_trace(format!(
                        "IRQ pc={:08x} timer asserted at cycle {}",
                        self.current_pc, self.guest_cycles
                    ));
                }
            }
        }
    }

    fn record_trace(&mut self, line: String) {
        if !self.trace_flags.any() {
            return;
        }
        if self.trace.len() == self.trace_capacity {
            self.trace.pop_front();
        }
        self.trace.push_back(line);
    }

    fn reset_devices(&mut self) {
        self.enable = 0;
        self.dvma_enable = 0;
        self.bus_error = 0;
        self.diagnostic = 1;
        self.irq_control = 0;
        self.rtc_irq_pending = false;
        self.timer_irq_pending = false;
        self.parity_control = 0;
        self.parity_address = 0;
        self.keyboard_scc.reset();
        self.serial_scc.reset();
        self.ncr5380 = [0; 8];
        self.scsi_control = 0;
        self.udc_address = 0;
        self.udc_registers = [0; 32];
        self.lance_rap = 0;
        self.lance_csrs = [0; 4];
        self.current_function_code = 6;
        self.guest_cycles = 0;
        self.next_timer_cycle = TIMER_PERIOD_CYCLES;
    }

    fn fault(
        &mut self,
        address: u32,
        physical: Option<u32>,
        write: bool,
        size: AccessSize,
        reason: &'static str,
        register_bits: u8,
        pte: Option<u32>,
    ) -> BusFault {
        self.bus_error = register_bits;
        self.last_fault = Some(LastFault {
            virtual_address: address,
            physical_address: physical,
            function_code: self.current_function_code,
            write,
            size,
            reason,
            pte,
        });
        if self.trace_flags.mmu || self.trace_flags.bus {
            self.record_trace(format!(
                "FAULT pc={:08x} va={address:08x} pa={} fc={} {} {:?} {reason} berr={register_bits:02x} pte={}",
                self.current_pc,
                physical.map_or_else(|| "-".to_owned(), |value| format!("{value:08x}")),
                self.current_function_code,
                if write { "W" } else { "R" },
                size,
                pte.map_or_else(|| "-".to_owned(), |value| format!("{value:08x}"))
            ));
        }
        BusFault {
            kind: BusFaultKind::BusError,
            address,
        }
    }

    fn segment_index(&self, virtual_address: u32) -> usize {
        usize::from(self.context & 7) * 2048 + ((virtual_address >> 17) & 0x7ff) as usize
    }

    fn pte_index(&self, virtual_address: u32) -> usize {
        let pmeg = usize::from(self.segment_map[self.segment_index(virtual_address)]);
        pmeg * 16 + ((virtual_address >> 13) & 0x0f) as usize
    }

    fn translate(
        &mut self,
        virtual_address: u32,
        write: bool,
        size: AccessSize,
    ) -> Result<Translation, BusFault> {
        let pte_index = self.pte_index(virtual_address);
        let pte = self.page_map[pte_index] & PTE_IMPLEMENTED_MASK;
        if pte & PTE_VALID == 0 {
            return Err(self.fault(
                virtual_address,
                None,
                write,
                size,
                "invalid PTE",
                BUS_ERROR_INVALID,
                Some(pte),
            ));
        }
        let supervisor = self.current_function_code >= 4;
        if (write && pte & PTE_WRITABLE == 0) || (!supervisor && pte & PTE_SYSTEM != 0) {
            return Err(self.fault(
                virtual_address,
                None,
                write,
                size,
                "MMU protection",
                BUS_ERROR_PROTECTION,
                Some(pte),
            ));
        }

        let mut updated = pte | PTE_ACCESSED;
        if write {
            updated |= PTE_MODIFIED;
        }
        self.page_map[pte_index] = updated;
        let physical = ((pte & PTE_PAGE_MASK) << 13) | (virtual_address & 0x1fff);
        let translation = Translation {
            page_type: ((pte & PTE_TYPE_MASK) >> 26) as u8,
            physical,
            pte: updated,
        };
        if self.trace_flags.mmu {
            self.record_trace(format!(
                "MMU pc={:08x} fc={} cx={} va={virtual_address:08x} seg={:03x} pmeg={:02x} pte#{pte_index:03x}={updated:08x} -> type{}:{physical:08x} {} {:?}",
                self.current_pc,
                self.current_function_code,
                self.context & 7,
                (virtual_address >> 17) & 0x7ff,
                self.segment_map[self.segment_index(virtual_address)],
                translation.page_type,
                if write { "W" } else { "R" },
                size
            ));
        }
        Ok(translation)
    }

    fn read_access(&mut self, address: u32, size: AccessSize) -> Result<u32, BusFault> {
        if self.current_function_code == 3 {
            return self.control_read(address, size);
        }
        if self.current_function_code == 6 && self.enable & 0x80 == 0 {
            let value = read_wrapped_be(&self.rom, (address as usize) & (PROM_SIZE - 1), size);
            if self.trace_flags.bus {
                self.record_trace(format!(
                    "BUS pc={:08x} boot-ROM R{:?} va={address:08x} -> {value:08x}",
                    self.current_pc, size
                ));
            }
            return Ok(value);
        }
        let translation = self.translate(address, false, size)?;
        self.physical_read(address, translation, size)
    }

    fn write_access(&mut self, address: u32, size: AccessSize, value: u32) -> Result<(), BusFault> {
        if self.current_function_code == 3 {
            return self.control_write(address, size, value);
        }
        let translation = self.translate(address, true, size)?;
        self.physical_write(address, translation, size, value)
    }

    fn control_read(&mut self, address: u32, size: AccessSize) -> Result<u32, BusFault> {
        let space = (address >> 28) & 0x0f;
        let value = match space {
            0x0 => read_wrapped_be(&self.idprom, (address as usize) & 0x1f, size),
            0x1 => {
                let index = self.pte_index(address);
                extract_register(self.page_map[index] & PTE_IMPLEMENTED_MASK, address, size)
            }
            0x2 => byte_register_read(self.segment_map[self.segment_index(address)], address, size),
            0x3 => byte_register_read(self.context & 7, address, size),
            0x4 => {
                let value = (self.enable & !1) | u8::from(self.diagnostic_switch);
                byte_register_read(value, address, size)
            }
            0x5 => byte_register_read(self.dvma_enable, address, size),
            0x6 => {
                let value = byte_register_read(self.bus_error, address, size);
                self.last_fault = None;
                value
            }
            0x7 => byte_register_read(self.diagnostic, address, size),
            0x8 => {
                let index = ((address as usize) & 0x3fff) >> 2;
                extract_register(self.cache_tags[index], address, size)
            }
            0x9 => {
                let index = (address as usize) & 0x3fff;
                extract_register(self.cache_data[index], address, size)
            }
            0xa => size.mask(),
            0xb => size.mask(),
            0xf => size.mask(),
            _ => {
                return Err(self.fault(
                    address,
                    None,
                    false,
                    size,
                    "undefined control-space device",
                    BUS_ERROR_TIMEOUT,
                    None,
                ));
            }
        };
        if self.trace_flags.io || self.trace_flags.mmu {
            self.record_trace(format!(
                "CTRL pc={:08x} R{:?} {address:08x} space={space:x} -> {value:08x}",
                self.current_pc, size
            ));
        }
        Ok(value)
    }

    fn control_write(
        &mut self,
        address: u32,
        size: AccessSize,
        value: u32,
    ) -> Result<(), BusFault> {
        let space = (address >> 28) & 0x0f;
        if self.trace_flags.io || self.trace_flags.mmu {
            self.record_trace(format!(
                "CTRL pc={:08x} W{:?} {address:08x} space={space:x} <- {value:08x}",
                self.current_pc, size
            ));
        }
        match space {
            0x0 => {}
            0x1 => {
                let index = self.pte_index(address);
                let current = self.page_map[index];
                self.page_map[index] =
                    merge_register(current, value, address, size) & PTE_IMPLEMENTED_MASK;
            }
            0x2 => {
                let map_index = self.segment_index(address);
                self.segment_map[map_index] = byte_register_write(value, address, size);
            }
            0x3 => self.context = byte_register_write(value, address, size) & 7,
            0x4 => self.enable = byte_register_write(value, address, size),
            0x5 => self.dvma_enable = byte_register_write(value, address, size),
            0x6 => {}
            0x7 => self.diagnostic = byte_register_write(value, address, size),
            0x8 => {
                let index = ((address as usize) & 0x3fff) >> 2;
                self.cache_tags[index] =
                    merge_register(self.cache_tags[index], value, address, size);
            }
            0x9 => {
                let index = (address as usize) & 0x3fff;
                self.cache_data[index] =
                    merge_register(self.cache_data[index], value, address, size);
            }
            0xa | 0xb | 0xf => {}
            _ => {
                return Err(self.fault(
                    address,
                    None,
                    true,
                    size,
                    "undefined control-space device",
                    BUS_ERROR_TIMEOUT,
                    None,
                ));
            }
        }
        Ok(())
    }

    fn physical_read(
        &mut self,
        virtual_address: u32,
        translation: Translation,
        size: AccessSize,
    ) -> Result<u32, BusFault> {
        let result = match translation.page_type {
            0 => self.type0_read(virtual_address, translation, size),
            1 => self.type1_read(virtual_address, translation, size),
            2 | 3 => Err(self.fault(
                virtual_address,
                Some(translation.physical),
                false,
                size,
                "unimplemented VME address space",
                BUS_ERROR_TIMEOUT,
                Some(translation.pte),
            )),
            _ => unreachable!(),
        };
        if let Ok(value) = result
            && self.trace_flags.bus
        {
            self.record_trace(format!(
                "BUS pc={:08x} R{:?} type{}:{:08x} -> {value:08x}",
                self.current_pc, size, translation.page_type, translation.physical
            ));
        }
        result
    }

    fn physical_write(
        &mut self,
        virtual_address: u32,
        translation: Translation,
        size: AccessSize,
        value: u32,
    ) -> Result<(), BusFault> {
        if self.trace_flags.bus {
            self.record_trace(format!(
                "BUS pc={:08x} W{:?} type{}:{:08x} <- {value:08x}",
                self.current_pc, size, translation.page_type, translation.physical
            ));
        }
        match translation.page_type {
            0 => self.type0_write(virtual_address, translation, size, value),
            1 => self.type1_write(virtual_address, translation, size, value),
            2 | 3 => Err(self.fault(
                virtual_address,
                Some(translation.physical),
                true,
                size,
                "unimplemented VME address space",
                BUS_ERROR_TIMEOUT,
                Some(translation.pte),
            )),
            _ => unreachable!(),
        }
    }

    fn type0_read(
        &mut self,
        virtual_address: u32,
        translation: Translation,
        size: AccessSize,
    ) -> Result<u32, BusFault> {
        let address = translation.physical as usize;
        if address
            .checked_add(size.bytes())
            .is_some_and(|end| end <= self.ram.len())
        {
            return Ok(read_be(&self.ram, address, size));
        }
        if (FRAMEBUFFER_BASE..FRAMEBUFFER_BASE + FRAMEBUFFER_SIZE as u32)
            .contains(&translation.physical)
        {
            let offset = (translation.physical - FRAMEBUFFER_BASE) as usize;
            if offset + size.bytes() <= self.framebuffer.len() {
                return Ok(read_be(&self.framebuffer, offset, size));
            }
        }
        Err(self.fault(
            virtual_address,
            Some(translation.physical),
            false,
            size,
            "main-memory timeout",
            BUS_ERROR_TIMEOUT,
            Some(translation.pte),
        ))
    }

    fn type0_write(
        &mut self,
        virtual_address: u32,
        translation: Translation,
        size: AccessSize,
        value: u32,
    ) -> Result<(), BusFault> {
        let address = translation.physical as usize;
        if address
            .checked_add(size.bytes())
            .is_some_and(|end| end <= self.ram.len())
        {
            write_be(&mut self.ram, address, size, value);
            self.apply_parity_write(translation.physical, size);
            return Ok(());
        }
        if (FRAMEBUFFER_BASE..FRAMEBUFFER_BASE + FRAMEBUFFER_SIZE as u32)
            .contains(&translation.physical)
        {
            let offset = (translation.physical - FRAMEBUFFER_BASE) as usize;
            if offset + size.bytes() <= self.framebuffer.len() {
                write_be(&mut self.framebuffer, offset, size, value);
                return Ok(());
            }
        }
        Err(self.fault(
            virtual_address,
            Some(translation.physical),
            true,
            size,
            "main-memory timeout",
            BUS_ERROR_TIMEOUT,
            Some(translation.pte),
        ))
    }

    fn type1_read(
        &mut self,
        virtual_address: u32,
        translation: Translation,
        size: AccessSize,
    ) -> Result<u32, BusFault> {
        let address = translation.physical;
        if (TYPE1_PROM_BASE..=TYPE1_PROM_END).contains(&address) {
            return Ok(read_wrapped_be(
                &self.rom,
                (address - TYPE1_PROM_BASE) as usize,
                size,
            ));
        }
        if (0x0004_0000..=0x0004_07ff).contains(&address) {
            return Ok(read_be(&self.nvram, (address - 0x0004_0000) as usize, size));
        }
        if address <= 0x0000_000f {
            return self.scc_read(false, address, size);
        }
        if (0x0002_0000..=0x0002_000f).contains(&address) {
            return self.scc_read(true, address - 0x0002_0000, size);
        }
        if (0x0006_0000..=0x0006_ffff).contains(&address) {
            return Ok(size.mask());
        }
        if (0x0008_0000..=0x0008_000f).contains(&address) {
            return Ok(self.parity_read(address - 0x0008_0000, size));
        }
        if (0x000a_0000..=0x000a_0003).contains(&address) {
            return Ok(byte_register_read(self.irq_control, address, size));
        }
        if (0x0012_0000..=0x0012_0003).contains(&address) {
            let register = ((address - 0x0012_0000) >> 1) as usize;
            let value = if register == 0 {
                self.lance_csrs[usize::from(self.lance_rap & 3)]
            } else {
                self.lance_rap
            };
            return Ok(word_register_read(value, address, size));
        }
        if (0x0014_0000..=0x0014_0007).contains(&address) {
            return Ok(byte_register_read(
                self.ncr5380[(address - 0x0014_0000) as usize],
                address,
                size,
            ));
        }
        if (0x0014_0010..=0x0014_0013).contains(&address) {
            let value = if address & 2 == 0 {
                self.udc_registers[usize::from(self.udc_address & 0x1f)]
            } else {
                self.udc_address
            };
            return Ok(word_register_read(value, address, size));
        }
        if (0x0014_0018..=0x0014_0019).contains(&address) {
            return Ok(word_register_read(self.scsi_control, address, size));
        }
        if (0x001e_0000..=0x001e_00ff).contains(&address) {
            return Err(self.fault(
                virtual_address,
                Some(address),
                false,
                size,
                "ECC register absent on parity-memory 3/60",
                BUS_ERROR_TIMEOUT,
                Some(translation.pte),
            ));
        }
        Err(self.fault(
            virtual_address,
            Some(address),
            false,
            size,
            "unmapped type-1 device",
            BUS_ERROR_TIMEOUT,
            Some(translation.pte),
        ))
    }

    fn type1_write(
        &mut self,
        virtual_address: u32,
        translation: Translation,
        size: AccessSize,
        value: u32,
    ) -> Result<(), BusFault> {
        let address = translation.physical;
        if (TYPE1_PROM_BASE..=TYPE1_PROM_END).contains(&address) {
            return Ok(());
        }
        if (0x0004_0000..=0x0004_07ff).contains(&address) {
            write_be(
                &mut self.nvram,
                (address - 0x0004_0000) as usize,
                size,
                value,
            );
            return Ok(());
        }
        if address <= 0x0000_000f {
            self.scc_write(false, address, size, value);
            return Ok(());
        }
        if (0x0002_0000..=0x0002_000f).contains(&address) {
            self.scc_write(true, address - 0x0002_0000, size, value);
            return Ok(());
        }
        if (0x0006_0000..=0x0006_ffff).contains(&address) {
            let byte = byte_register_write(value, address, size);
            if (address - 0x0006_0000) & 0xff == 0x11 && byte == 0x1c {
                self.rtc_irq_pending = true;
            }
            return Ok(());
        }
        if (0x0008_0000..=0x0008_000f).contains(&address) {
            self.parity_write(address - 0x0008_0000, size, value);
            return Ok(());
        }
        if (0x000a_0000..=0x000a_0003).contains(&address) {
            let old_level = self.irq_level();
            self.irq_control = byte_register_write(value, address, size);
            if self.irq_control & 0x01 == 0 {
                self.rtc_irq_pending = false;
                self.timer_irq_pending = false;
            }
            let new_level = self.irq_level();
            if self.trace_flags.irq && old_level != new_level {
                self.record_trace(format!(
                    "IRQ pc={:08x} control={:02x} level {old_level}->{new_level}",
                    self.current_pc, self.irq_control
                ));
            }
            return Ok(());
        }
        if (0x0012_0000..=0x0012_0003).contains(&address) {
            let register = ((address - 0x0012_0000) >> 1) as usize;
            let word = word_register_write(value, address, size);
            if register == 0 {
                self.lance_csrs[usize::from(self.lance_rap & 3)] = word;
            } else {
                self.lance_rap = word & 3;
            }
            return Ok(());
        }
        if (0x0014_0000..=0x0014_0007).contains(&address) {
            let index = (address - 0x0014_0000) as usize;
            self.ncr5380[index] = byte_register_write(value, address, size);
            return Ok(());
        }
        if (0x0014_0010..=0x0014_0013).contains(&address) {
            let word = word_register_write(value, address, size);
            if address & 2 == 0 {
                self.udc_registers[usize::from(self.udc_address & 0x1f)] = word;
            } else {
                self.udc_address = word;
            }
            return Ok(());
        }
        if (0x0014_0018..=0x0014_0019).contains(&address) {
            self.scsi_control = word_register_write(value, address, size) & 0x000f;
            return Ok(());
        }
        if (0x001e_0000..=0x001e_00ff).contains(&address) {
            return Err(self.fault(
                virtual_address,
                Some(address),
                true,
                size,
                "ECC register absent on parity-memory 3/60",
                BUS_ERROR_TIMEOUT,
                Some(translation.pte),
            ));
        }
        Err(self.fault(
            virtual_address,
            Some(address),
            true,
            size,
            "unmapped type-1 device",
            BUS_ERROR_TIMEOUT,
            Some(translation.pte),
        ))
    }

    fn scc_read(&mut self, serial: bool, offset: u32, size: AccessSize) -> Result<u32, BusFault> {
        let mut value = 0_u32;
        for index in 0..size.bytes() {
            let byte_address = offset + index as u32;
            let byte = if byte_address & 1 == 0 {
                let port = ((byte_address >> 1) & 3) as u8;
                if serial {
                    self.serial_scc.read(port)
                } else {
                    self.keyboard_scc.read(port)
                }
            } else {
                0xff
            };
            value = (value << 8) | u32::from(byte);
        }
        if self.trace_flags.scc {
            self.record_trace(format!(
                "SCC pc={:08x} {} R{:?} off={offset:x} -> {value:08x}",
                self.current_pc,
                if serial { "serial" } else { "kbd/mouse" },
                size
            ));
        }
        Ok(value)
    }

    fn scc_write(&mut self, serial: bool, offset: u32, size: AccessSize, value: u32) {
        for index in 0..size.bytes() {
            let shift = 8 * (size.bytes() - 1 - index);
            let byte = (value >> shift) as u8;
            let byte_address = offset + index as u32;
            if byte_address & 1 != 0 {
                continue;
            }
            let port = ((byte_address >> 1) & 3) as u8;
            let transmitted = if serial {
                self.serial_scc.write(port, byte)
            } else {
                self.keyboard_scc.write(port, byte)
            };
            if let Some((channel, character)) = transmitted
                && serial
                && channel == 0
            {
                self.console.push(character);
            }
        }
        if self.trace_flags.scc {
            self.record_trace(format!(
                "SCC pc={:08x} {} W{:?} off={offset:x} <- {value:08x}",
                self.current_pc,
                if serial { "serial" } else { "kbd/mouse" },
                size
            ));
        }
    }

    fn parity_read(&mut self, offset: u32, size: AccessSize) -> u32 {
        let register = if offset < 4 {
            let value = u32::from(self.parity_control) << 24;
            self.parity_control &= 0x70;
            value
        } else {
            self.parity_address
        };
        extract_register(register, offset, size)
    }

    fn parity_write(&mut self, offset: u32, size: AccessSize, value: u32) {
        if offset < 4 {
            let data = merge_register(0, value, offset, size).to_be_bytes()[0];
            let errors = self.parity_control & 0x8f;
            self.parity_control = errors | (data & 0x70);
        } else {
            self.parity_address = merge_register(self.parity_address, value, offset, size);
        }
    }

    fn apply_parity_write(&mut self, physical: u32, size: AccessSize) {
        if self.parity_control & 0x20 == 0 || self.irq_control & 1 == 0 {
            return;
        }
        self.parity_address = physical;
        let first_lane = (physical & 3) as usize;
        let lanes = size.bytes().min(4 - first_lane);
        for lane in first_lane..first_lane + lanes {
            self.parity_control |= 1 << (3 - lane);
        }
        self.parity_control |= 0x80;
        if self.trace_flags.irq {
            self.record_trace(format!(
                "IRQ pc={:08x} injected parity error control={:02x} address={physical:08x}",
                self.current_pc, self.parity_control
            ));
        }
    }

    fn note_post_marker(&mut self, pc: u32, seen: &mut BTreeSet<u32>) {
        if !self.trace_flags.prom || seen.contains(&pc) {
            return;
        }
        if let Some((_, label)) = POST_MARKERS.iter().find(|(address, _)| *address == pc) {
            seen.insert(pc);
            self.record_trace(format!("PROM pc={pc:08x} reached {label}"));
        }
    }

    /// Execute the genuine PROM until a monitor prompt, bound, or fatal stop.
    #[allow(clippy::too_many_arguments)]
    pub fn run(
        &mut self,
        max_instructions: Option<u64>,
        max_cycles: Option<u64>,
        scripted_input: Option<&[u8]>,
        require_script_round_trip: bool,
        stdout_console: bool,
        input: Option<&Receiver<Vec<u8>>>,
        breakpoints: &BTreeSet<u32>,
    ) -> RunOutcome {
        let mut cpu = CpuCore::new();
        cpu.set_cpu_type(CpuType::M68020);
        cpu.fpu_present = false;
        self.set_function_code(6);
        cpu.reset(self);
        let mut handler = NoOpHleHandler;
        let mut instructions = 0_u64;
        let mut cycles = 0_u64;
        let mut prompts = 0_usize;
        let mut scan_cursor = 0_usize;
        let mut script_injected = false;
        let mut seen_post = BTreeSet::new();
        let mut reason = "instruction limit reached".to_owned();
        let mut last_pc = u32::MAX;
        let mut same_pc_count = 0_u64;

        loop {
            if max_instructions.is_some_and(|limit| instructions >= limit) {
                break;
            }
            if max_cycles.is_some_and(|limit| cycles >= limit) {
                reason = "cycle limit reached".to_owned();
                break;
            }
            if breakpoints.contains(&cpu.pc) {
                reason = format!("breakpoint at 0x{:08x}", cpu.pc);
                break;
            }

            self.set_current_pc(cpu.pc);
            self.note_post_marker(cpu.pc, &mut seen_post);
            if self.trace_flags.cpu {
                self.record_trace(format!(
                    "CPU pc={:08x} sr={:04x} d0={:08x} d1={:08x} a0={:08x} a1={:08x} sp={:08x}",
                    cpu.pc,
                    cpu.get_sr(),
                    cpu.d(0),
                    cpu.d(1),
                    cpu.a(0),
                    cpu.a(1),
                    cpu.a(7)
                ));
            }

            cpu.fpu_present = self.fpp_enabled();
            let irq = self.irq_level();
            if irq != self.last_irq_level {
                if self.trace_flags.irq {
                    self.record_trace(format!(
                        "IRQ pc={:08x} level {}->{}",
                        cpu.pc, self.last_irq_level, irq
                    ));
                }
                self.last_irq_level = irq;
            }
            cpu.set_irq(irq);

            let step = cpu.step_with_hle_handler(self, &mut handler);
            instructions = instructions.saturating_add(1);
            let step_cycles = match step {
                StepResult::Ok { cycles } => cycles.max(0) as u64,
                StepResult::Stopped => {
                    if self.irq_level() == 0 {
                        reason = "CPU entered STOP with no pending interrupt".to_owned();
                        break;
                    }
                    0
                }
                other => {
                    reason = format!("unexpected surfaced CPU result: {other:?}");
                    break;
                }
            };
            cycles = cycles.saturating_add(step_cycles);
            self.advance_cycles(step_cycles);

            if let Some(receiver) = input {
                while let Ok(bytes) = receiver.try_recv() {
                    self.inject_serial_a(&bytes);
                }
            }
            if stdout_console {
                let _ = self.flush_console(std::io::stdout().lock());
            }

            while scan_cursor < self.console.len() {
                scan_cursor += 1;
                let prefix = &self.console[..scan_cursor];
                if looks_like_prompt(prefix) {
                    prompts += 1;
                    if !script_injected && let Some(script) = scripted_input {
                        self.inject_serial_a(script);
                        script_injected = true;
                    }
                }
            }

            let success = prompts
                >= if require_script_round_trip && scripted_input.is_some() {
                    2
                } else {
                    1
                };
            if success {
                reason = if script_injected {
                    "authentic PROM monitor accepted scripted input and returned to its prompt"
                        .to_owned()
                } else {
                    "authentic PROM monitor prompt reached".to_owned()
                };
                break;
            }

            if cpu.pc == last_pc {
                same_pc_count = same_pc_count.saturating_add(1);
            } else {
                last_pc = cpu.pc;
                same_pc_count = 0;
            }
            if same_pc_count > 20_000_000 {
                reason = format!("execution remained at PC 0x{:08x}", cpu.pc);
                break;
            }
        }

        RunOutcome {
            monitor_reached: prompts > 0,
            prompt_count: prompts,
            script_injected,
            instructions,
            cycles,
            final_pc: cpu.pc,
            console: self.console.clone(),
            reason,
        }
    }
}

impl AddressBus for Sun3Machine {
    fn read_byte(&mut self, address: u32) -> u8 {
        self.read_access(address, AccessSize::Byte).unwrap_or(0xff) as u8
    }

    fn read_word(&mut self, address: u32) -> u16 {
        self.read_access(address, AccessSize::Word)
            .unwrap_or(0xffff) as u16
    }

    fn read_long(&mut self, address: u32) -> u32 {
        self.read_access(address, AccessSize::Long)
            .unwrap_or(u32::MAX)
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        let _ = self.write_access(address, AccessSize::Byte, u32::from(value));
    }

    fn write_word(&mut self, address: u32, value: u16) {
        let _ = self.write_access(address, AccessSize::Word, u32::from(value));
    }

    fn write_long(&mut self, address: u32, value: u32) {
        let _ = self.write_access(address, AccessSize::Long, value);
    }

    fn try_read_byte(&mut self, address: u32) -> Result<u8, BusFault> {
        self.read_access(address, AccessSize::Byte)
            .map(|value| value as u8)
    }

    fn try_read_word(&mut self, address: u32) -> Result<u16, BusFault> {
        self.read_access(address, AccessSize::Word)
            .map(|value| value as u16)
    }

    fn try_read_long(&mut self, address: u32) -> Result<u32, BusFault> {
        self.read_access(address, AccessSize::Long)
    }

    fn try_write_byte(&mut self, address: u32, value: u8) -> Result<(), BusFault> {
        self.write_access(address, AccessSize::Byte, u32::from(value))
    }

    fn try_write_word(&mut self, address: u32, value: u16) -> Result<(), BusFault> {
        self.write_access(address, AccessSize::Word, u32::from(value))
    }

    fn try_write_long(&mut self, address: u32, value: u32) -> Result<(), BusFault> {
        self.write_access(address, AccessSize::Long, value)
    }

    fn try_read_three_bytes(&mut self, address: u32) -> Result<u32, BusFault> {
        self.read_access(address, AccessSize::Three)
    }

    fn try_write_three_bytes(&mut self, address: u32, value: u32) -> Result<(), BusFault> {
        self.write_access(address, AccessSize::Three, value)
    }

    fn try_read_immediate_word(&mut self, address: u32) -> Result<u16, BusFault> {
        self.try_read_word(address)
    }

    fn try_read_immediate_long(&mut self, address: u32) -> Result<u32, BusFault> {
        self.try_read_long(address)
    }

    fn set_function_code(&mut self, function_code: u8) {
        self.current_function_code = function_code & 7;
    }

    fn interrupt_acknowledge(&mut self, level: u8) -> u32 {
        if self.trace_flags.irq {
            self.record_trace(format!(
                "IRQ pc={:08x} acknowledge level {level} (autovector)",
                self.current_pc
            ));
        }
        if level == 7 {
            self.timer_irq_pending = false;
        }
        u32::MAX
    }

    fn reset_devices(&mut self) {
        Sun3Machine::reset_devices(self);
    }
}

fn read_be(bytes: &[u8], offset: usize, size: AccessSize) -> u32 {
    bytes[offset..offset + size.bytes()]
        .iter()
        .fold(0_u32, |value, byte| (value << 8) | u32::from(*byte))
}

fn read_wrapped_be(bytes: &[u8], offset: usize, size: AccessSize) -> u32 {
    (0..size.bytes()).fold(0_u32, |value, index| {
        (value << 8) | u32::from(bytes[(offset + index) % bytes.len()])
    })
}

fn write_be(bytes: &mut [u8], offset: usize, size: AccessSize, value: u32) {
    for index in 0..size.bytes() {
        let shift = 8 * (size.bytes() - 1 - index);
        bytes[offset + index] = (value >> shift) as u8;
    }
}

fn extract_register(register: u32, address: u32, size: AccessSize) -> u32 {
    let bytes = register.to_be_bytes();
    let offset = (address & 3) as usize;
    (0..size.bytes()).fold(0_u32, |value, index| {
        let byte = bytes.get(offset + index).copied().unwrap_or(0xff);
        (value << 8) | u32::from(byte)
    })
}

fn merge_register(current: u32, value: u32, address: u32, size: AccessSize) -> u32 {
    let mut bytes = current.to_be_bytes();
    let offset = (address & 3) as usize;
    for index in 0..size.bytes() {
        if let Some(destination) = bytes.get_mut(offset + index) {
            let shift = 8 * (size.bytes() - 1 - index);
            *destination = (value >> shift) as u8;
        }
    }
    u32::from_be_bytes(bytes)
}

fn byte_register_read(register: u8, address: u32, size: AccessSize) -> u32 {
    extract_register(u32::from(register) << 24, address, size)
}

fn byte_register_write(value: u32, address: u32, size: AccessSize) -> u8 {
    merge_register(0, value, address, size).to_be_bytes()[0]
}

fn word_register_read(register: u16, address: u32, size: AccessSize) -> u32 {
    extract_register(u32::from(register) << 16, address, size)
}

fn word_register_write(value: u32, address: u32, size: AccessSize) -> u16 {
    let bytes = merge_register(0, value, address, size).to_be_bytes();
    u16::from_be_bytes([bytes[0], bytes[1]])
}

fn looks_like_prompt(console: &[u8]) -> bool {
    if !console.ends_with(b">") || console.len() < 2 {
        return false;
    }
    let text = String::from_utf8_lossy(console);
    let lower = text.to_ascii_lowercase();
    (lower.contains("sun") || lower.contains("rom rev") || lower.contains("selftest"))
        && (console[console.len() - 2] == b'\n'
            || console[console.len() - 2] == b'\r'
            || console[console.len() - 2] == b' ')
}

/// Parse RAM sizes accepted by the command line.
pub fn parse_ram_size(text: &str) -> Result<usize, String> {
    let normalized = text.trim().to_ascii_uppercase();
    let number = normalized
        .strip_suffix("MIB")
        .or_else(|| normalized.strip_suffix('M'))
        .unwrap_or(&normalized)
        .parse::<usize>()
        .map_err(|error| format!("invalid RAM size {text:?}: {error}"))?;
    let bytes = number
        .checked_mul(1024 * 1024)
        .ok_or_else(|| "RAM size overflow".to_owned())?;
    if matches!(number, 4 | 8 | 12 | 16 | 20 | 24) {
        Ok(bytes)
    } else {
        Err("RAM must be one of 4M, 8M, 12M, 16M, 20M, or 24M".to_owned())
    }
}

/// Pace approximately to 20 MHz when requested.
pub fn pace_realtime(start: Instant, cycles: u64) {
    let target = Duration::from_secs_f64(cycles as f64 / 20_000_000.0);
    if let Some(delay) = target.checked_sub(start.elapsed()) {
        std::thread::sleep(delay.min(Duration::from_millis(10)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn machine() -> Sun3Machine {
        Sun3Machine::new(vec![0; PROM_SIZE], &MachineConfig::default()).unwrap()
    }

    #[test]
    fn mmu_context_segment_pmeg_translation_and_statistics() {
        let mut machine = machine();
        machine.current_function_code = 5;
        machine.context = 3;
        let virtual_address = 0x0123_4567;
        let segment_index = machine.segment_index(virtual_address);
        machine.segment_map[segment_index] = 0x42;
        let pte_index = 0x42 * 16 + ((virtual_address >> 13) & 15) as usize;
        machine.page_map[pte_index] = PTE_VALID | PTE_WRITABLE | 7;
        machine.ram[7 * 8192 + (virtual_address as usize & 0x1fff)] = 0xa5;
        assert_eq!(machine.read_byte(virtual_address), 0xa5);
        assert_ne!(machine.page_map[pte_index] & PTE_ACCESSED, 0);
        machine.write_byte(virtual_address, 0x5a);
        assert_ne!(machine.page_map[pte_index] & PTE_MODIFIED, 0);
        assert_eq!(machine.read_byte(virtual_address), 0x5a);
    }

    #[test]
    fn invalid_and_write_protected_pages_fault_honestly() {
        let mut machine = machine();
        machine.current_function_code = 5;
        let address = 0x0001_0000;
        assert!(machine.try_read_byte(address).is_err());
        assert_eq!(machine.bus_error, BUS_ERROR_INVALID);
        let pte_index = machine.pte_index(address);
        machine.page_map[pte_index] = PTE_VALID;
        assert!(machine.try_write_byte(address, 1).is_err());
        assert_eq!(machine.bus_error, BUS_ERROR_PROTECTION);
    }

    #[test]
    fn control_space_programs_context_segment_and_page_maps() {
        let mut machine = machine();
        machine.current_function_code = 3;
        machine.write_byte(0x3000_0000, 5);
        assert_eq!(machine.context, 5);
        let virtual_address = 0x0123_4000;
        machine.write_byte(0x2000_0000 | virtual_address, 0x33);
        assert_eq!(
            machine.segment_map[machine.segment_index(virtual_address)],
            0x33
        );
        machine.write_long(0x1000_0000 | virtual_address, PTE_VALID | PTE_WRITABLE | 9);
        assert_eq!(
            machine.page_map[machine.pte_index(virtual_address)],
            PTE_VALID | PTE_WRITABLE | 9
        );
    }

    #[test]
    fn boot_state_forces_supervisor_program_fetches_to_prom() {
        let mut rom = vec![0; PROM_SIZE];
        rom[0x1234] = 0x5a;
        let mut machine = Sun3Machine::new(rom, &MachineConfig::default()).unwrap();
        machine.current_function_code = 6;
        assert_eq!(machine.read_byte(0x0fed_1234), 0x5a);
        machine.current_function_code = 5;
        assert!(machine.try_read_byte(0x0fed_1234).is_err());
    }

    #[test]
    fn idprom_and_ram_parser_are_valid() {
        let machine = machine();
        assert!(is_valid(&machine.idprom));
        assert_eq!(parse_ram_size("24M").unwrap(), MAX_RAM);
        assert!(parse_ram_size("6M").is_err());
    }
}
