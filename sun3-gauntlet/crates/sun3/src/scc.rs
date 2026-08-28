//! Functional Zilog Z8530 SCC model used by the keyboard/mouse and serial ports.

use std::collections::VecDeque;

const RR0_RX_AVAILABLE: u8 = 0x01;
const RR0_TX_EMPTY: u8 = 0x04;
const RR0_DCD: u8 = 0x08;
const RR0_CTS: u8 = 0x20;
const RR1_ALL_SENT: u8 = 0x01;
const WR1_TX_INT_ENABLE: u8 = 0x02;
const WR1_RX_INT_MASK: u8 = 0x18;
const WR3_RX_ENABLE: u8 = 0x01;
const WR5_TX_ENABLE: u8 = 0x08;
const WR9_MASTER_INT_ENABLE: u8 = 0x08;

/// One channel of a Z8530.
#[derive(Debug, Clone)]
pub struct SccChannel {
    write_registers: [u8; 16],
    register_pointer: u8,
    pointer_pending: bool,
    receive: VecDeque<u8>,
    transmit_holding: Option<u8>,
}

impl Default for SccChannel {
    fn default() -> Self {
        Self {
            write_registers: [0; 16],
            register_pointer: 0,
            pointer_pending: false,
            receive: VecDeque::new(),
            transmit_holding: None,
        }
    }
}

impl SccChannel {
    fn reset(&mut self) {
        *self = Self::default();
    }

    fn rr0(&self) -> u8 {
        let mut status = RR0_TX_EMPTY | RR0_DCD | RR0_CTS;
        if !self.receive.is_empty() {
            status |= RR0_RX_AVAILABLE;
        }
        status
    }

    fn read_register(&self, register: u8) -> u8 {
        match register & 0x0f {
            0 => self.rr0(),
            1 => RR1_ALL_SENT,
            2 => self.write_registers[2],
            3 => 0,
            4..=7 | 9..=15 => self.write_registers[usize::from(register & 0x0f)],
            8 => self.receive.front().copied().unwrap_or(0),
            _ => 0,
        }
    }

    /// Read the control/status port.
    pub fn control_read(&mut self) -> u8 {
        let register = if self.pointer_pending {
            self.pointer_pending = false;
            self.register_pointer
        } else {
            0
        };
        self.read_register(register)
    }

    /// Write the control port, including indirect register selection.
    pub fn control_write(&mut self, value: u8) {
        if self.pointer_pending {
            let register = usize::from(self.register_pointer & 0x0f);
            self.pointer_pending = false;
            self.write_registers[register] = value;
            return;
        }

        let register = (value & 0x07) | if value & 0x08 != 0 { 0x08 } else { 0 };
        self.register_pointer = register;
        self.pointer_pending = register != 0;

        // WR0 commands that affect state needed by firmware diagnostics.
        match value & 0x38 {
            0x28 => {
                // Reset transmitter interrupt pending. TX is permanently
                // ready in this functional model, so no extra latch is needed.
            }
            0x30 => {
                // Error reset: receive errors are not latched by this model.
            }
            _ => {}
        }
    }

    /// Read received data.
    pub fn data_read(&mut self) -> u8 {
        if self.write_registers[3] & WR3_RX_ENABLE == 0 {
            return 0;
        }
        self.receive.pop_front().unwrap_or(0)
    }

    /// Write transmit data. Returns a byte when it reached the serial line.
    pub fn data_write(&mut self, value: u8) -> Option<u8> {
        self.transmit_holding = Some(value);
        if self.write_registers[5] & WR5_TX_ENABLE != 0 {
            self.transmit_holding.take()
        } else {
            None
        }
    }

    /// Queue bytes received from a host terminal.
    pub fn inject(&mut self, bytes: &[u8]) {
        self.receive.extend(bytes.iter().copied());
    }

    fn interrupt_pending(&self, master_enabled: bool) -> bool {
        if !master_enabled {
            return false;
        }
        let wr1 = self.write_registers[1];
        let rx = !self.receive.is_empty() && wr1 & WR1_RX_INT_MASK != 0;
        let tx = wr1 & WR1_TX_INT_ENABLE != 0;
        rx || tx
    }
}

/// Dual-channel Z8530 device. Universal-bus port ordering is B-control,
/// B-data, A-control, A-data.
#[derive(Debug, Clone, Default)]
pub struct Z8530 {
    /// Channel A.
    pub channel_a: SccChannel,
    /// Channel B.
    pub channel_b: SccChannel,
    write_register_9: u8,
}

impl Z8530 {
    /// Reset both channels.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Read one Universal Bus port (0=B ctl, 1=B data, 2=A ctl, 3=A data).
    pub fn read(&mut self, port: u8) -> u8 {
        match port & 3 {
            0 => self.channel_b.control_read(),
            1 => self.channel_b.data_read(),
            2 => self.channel_a.control_read(),
            3 => self.channel_a.data_read(),
            _ => unreachable!(),
        }
    }

    /// Write one Universal Bus port. Returns a transmitted byte plus channel.
    pub fn write(&mut self, port: u8, value: u8) -> Option<(u8, u8)> {
        let result = match port & 3 {
            0 => {
                self.channel_b.control_write(value);
                None
            }
            1 => self.channel_b.data_write(value).map(|byte| (1, byte)),
            2 => {
                self.channel_a.control_write(value);
                None
            }
            3 => self.channel_a.data_write(value).map(|byte| (0, byte)),
            _ => unreachable!(),
        };

        // WR9 is shared and is selected through either channel. Mirror writes
        // after an indirect register-9 access.
        let a9 = self.channel_a.write_registers[9];
        let b9 = self.channel_b.write_registers[9];
        let new_wr9 = if a9 != self.write_register_9 { a9 } else { b9 };
        if new_wr9 != self.write_register_9 {
            self.write_register_9 = new_wr9;
            match new_wr9 & 0xc0 {
                0x40 => self.channel_b.reset(),
                0x80 => self.channel_a.reset(),
                0xc0 => self.reset(),
                _ => {}
            }
        }
        result
    }

    /// Queue host input on channel A (the Sun serial-A console).
    pub fn inject_channel_a(&mut self, bytes: &[u8]) {
        self.channel_a.inject(bytes);
    }

    /// Whether the SCC is asserting its interrupt output.
    #[must_use]
    pub fn interrupt_pending(&self) -> bool {
        let master = self.write_register_9 & WR9_MASTER_INT_ENABLE != 0;
        self.channel_a.interrupt_pending(master) || self.channel_b.interrupt_pending(master)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_register(channel_control_port: u8, scc: &mut Z8530, reg: u8, value: u8) {
        let select = (reg & 7) | if reg >= 8 { 8 } else { 0 };
        assert!(scc.write(channel_control_port, select).is_none());
        assert!(scc.write(channel_control_port, value).is_none());
    }

    #[test]
    fn universal_port_order_and_serial_io() {
        let mut scc = Z8530::default();
        write_register(2, &mut scc, 3, WR3_RX_ENABLE);
        write_register(2, &mut scc, 5, WR5_TX_ENABLE);
        scc.inject_channel_a(b"x");
        assert_ne!(scc.read(2) & RR0_RX_AVAILABLE, 0);
        assert_eq!(scc.read(3), b'x');
        assert_eq!(scc.write(3, b'y'), Some((0, b'y')));
    }
}
