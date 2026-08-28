use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::mpsc;
use std::time::Instant;
use sun3::machine::{pace_realtime, parse_ram_size};
use sun3::{MachineConfig, Sun3Machine, TraceFlags};

const HELP: &str = r#"Sun 3/60 emulator

Usage:
  sun3 --rom FILE [options]

Required:
  --rom FILE                 Assembled 64 KiB Sun 3/60 boot PROM

Machine:
  --ram SIZE                 4M, 8M, 12M, 16M, 20M, or 24M (default 24M)
  --mac XX:XX:XX:XX:XX:XX   IDPROM Ethernet address
  --hostid HEX               Low 24 bits of the IDPROM serial field
  --diagnostic-switch        Assert the physical diagnostic switch
  --disk FILE                Reserved for the NCR5380 disk milestone

Execution:
  --max-instructions N       Deterministic instruction bound (default 100000000)
  --max-cycles N             Optional guest-cycle bound
  --unlimited                Remove the default instruction bound
  --speed realtime           Pace approximately to 20 MHz
  --break HEX                Stop before executing this PC (repeatable)
  --require-monitor          Return failure unless a real PROM prompt appears
  --script-cr                Send carriage return after first prompt and require another
  --script TEXT              Send escaped text after first prompt (supports \\r, \\n, \\xNN)
  --interactive              Forward host input lines to serial port A

Console/output:
  --console stdio|capture    Stream authentic SCC output or only capture it
  --console-log FILE         Write the exact captured serial transcript
  --trace-file FILE          Write the bounded trace ring
  --headless                 Accepted for scripting; the core is headless by design

Tracing:
  --trace-cpu --trace-mmu --trace-bus --trace-io
  --trace-irq --trace-scc --trace-prom --trace-all

  -h, --help                 Show this help
"#;

#[derive(Debug)]
struct Options {
    rom: PathBuf,
    config: MachineConfig,
    max_instructions: Option<u64>,
    max_cycles: Option<u64>,
    stdout_console: bool,
    console_log: Option<PathBuf>,
    trace_file: Option<PathBuf>,
    require_monitor: bool,
    script: Option<Vec<u8>>,
    interactive: bool,
    realtime: bool,
    breakpoints: BTreeSet<u32>,
    disk: Option<PathBuf>,
}

fn main() -> ExitCode {
    match run_main() {
        Ok(success) => {
            if success {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
        Err(error) => {
            eprintln!("error: {error}\n\n{HELP}");
            ExitCode::from(2)
        }
    }
}

fn run_main() -> Result<bool, String> {
    let options = parse_options(env::args().skip(1))?;
    let rom = fs::read(&options.rom)
        .map_err(|error| format!("failed to read {}: {error}", options.rom.display()))?;
    let mut machine = Sun3Machine::new(rom, &options.config)?;

    if let Some(disk) = &options.disk {
        eprintln!(
            "note: {} was supplied, but disk command execution remains behind the PROM-monitor baseline",
            disk.display()
        );
    }

    let (sender, receiver) = mpsc::channel();
    if options.interactive {
        std::thread::spawn(move || {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                let Ok(line) = line else { break };
                let mut bytes = line.into_bytes();
                bytes.push(b'\r');
                if sender.send(bytes).is_err() {
                    break;
                }
            }
        });
    }

    let start = Instant::now();
    let outcome = machine.run(
        options.max_instructions,
        options.max_cycles,
        options.script.as_deref(),
        options.script.is_some(),
        options.stdout_console,
        options.interactive.then_some(&receiver),
        &options.breakpoints,
    );
    if options.realtime {
        pace_realtime(start, outcome.cycles);
    }

    if let Some(path) = &options.console_log {
        fs::write(path, &outcome.console)
            .map_err(|error| format!("failed to write {}: {error}", path.display()))?;
    }
    if let Some(path) = &options.trace_file {
        fs::write(path, machine.trace_dump())
            .map_err(|error| format!("failed to write {}: {error}", path.display()))?;
    }

    eprintln!();
    eprintln!("Sun 3/60 execution summary");
    eprintln!("  instructions: {}", outcome.instructions);
    eprintln!("  guest cycles: {}", outcome.cycles);
    eprintln!("  final PC: 0x{:08x}", outcome.final_pc);
    eprintln!("  console bytes: {}", outcome.console.len());
    eprintln!("  monitor prompts: {}", outcome.prompt_count);
    eprintln!("  scripted input injected: {}", outcome.script_injected);
    eprintln!("  termination: {}", outcome.reason);
    eprintln!(
        "ACCEPTANCE monitor_reached={} interactive_round_trip={}",
        outcome.monitor_reached,
        outcome.script_injected && outcome.prompt_count >= 2
    );

    Ok(!options.require_monitor || outcome.monitor_reached && (options.script.is_none() || outcome.prompt_count >= 2))
}

fn parse_options(arguments: impl Iterator<Item = String>) -> Result<Options, String> {
    let mut arguments = arguments.peekable();
    if arguments.peek().is_none() {
        return Err("--rom is required".to_owned());
    }

    let mut rom = None;
    let mut config = MachineConfig::default();
    let mut max_instructions = Some(100_000_000);
    let mut max_cycles = None;
    let mut stdout_console = true;
    let mut console_log = None;
    let mut trace_file = None;
    let mut require_monitor = false;
    let mut script = None;
    let mut interactive = false;
    let mut realtime = false;
    let mut breakpoints = BTreeSet::new();
    let mut disk = None;

    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "-h" | "--help" => {
                print!("{HELP}");
                std::process::exit(0);
            }
            "--rom" => rom = Some(PathBuf::from(next_value(&mut arguments, "--rom")?)),
            "--ram" => config.ram_bytes = parse_ram_size(&next_value(&mut arguments, "--ram")?)?,
            "--mac" => config.mac = parse_mac(&next_value(&mut arguments, "--mac")?)?,
            "--hostid" => config.host_id = parse_u32(&next_value(&mut arguments, "--hostid")?)? & 0x00ff_ffff,
            "--diagnostic-switch" => config.diagnostic_switch = true,
            "--disk" => disk = Some(PathBuf::from(next_value(&mut arguments, "--disk")?)),
            "--max-instructions" => {
                max_instructions = Some(parse_u64(&next_value(&mut arguments, "--max-instructions")?)?)
            }
            "--max-cycles" => max_cycles = Some(parse_u64(&next_value(&mut arguments, "--max-cycles")?)?),
            "--unlimited" => max_instructions = None,
            "--speed" => {
                let value = next_value(&mut arguments, "--speed")?;
                realtime = match value.as_str() {
                    "realtime" | "20MHz" | "20mhz" => true,
                    "unlimited" => false,
                    _ => return Err(format!("unknown speed {value:?}")),
                };
            }
            "--break" => {
                breakpoints.insert(parse_u32(&next_value(&mut arguments, "--break")?)?);
            }
            "--require-monitor" => require_monitor = true,
            "--script-cr" => script = Some(vec![b'\r']),
            "--script" => script = Some(parse_escaped(&next_value(&mut arguments, "--script")?)?),
            "--interactive" => interactive = true,
            "--console" => {
                stdout_console = match next_value(&mut arguments, "--console")?.as_str() {
                    "stdio" => true,
                    "capture" | "none" => false,
                    value => return Err(format!("unknown console mode {value:?}")),
                }
            }
            "--console-log" => console_log = Some(PathBuf::from(next_value(&mut arguments, "--console-log")?)),
            "--trace-file" => trace_file = Some(PathBuf::from(next_value(&mut arguments, "--trace-file")?)),
            "--trace-cpu" => config.traces.cpu = true,
            "--trace-mmu" => config.traces.mmu = true,
            "--trace-bus" => config.traces.bus = true,
            "--trace-io" => config.traces.io = true,
            "--trace-irq" => config.traces.irq = true,
            "--trace-scc" => config.traces.scc = true,
            "--trace-prom" => config.traces.prom = true,
            "--trace-all" => {
                config.traces = TraceFlags {
                    cpu: true,
                    mmu: true,
                    bus: true,
                    io: true,
                    irq: true,
                    scc: true,
                    prom: true,
                };
            }
            "--headless" => {}
            other => return Err(format!("unknown option {other:?}")),
        }
    }

    Ok(Options {
        rom: rom.ok_or_else(|| "--rom is required".to_owned())?,
        config,
        max_instructions,
        max_cycles,
        stdout_console,
        console_log,
        trace_file,
        require_monitor,
        script,
        interactive,
        realtime,
        breakpoints,
        disk,
    })
}

fn next_value(arguments: &mut impl Iterator<Item = String>, option: &str) -> Result<String, String> {
    arguments
        .next()
        .ok_or_else(|| format!("{option} requires a value"))
}

fn parse_u64(text: &str) -> Result<u64, String> {
    let normalized = text.replace('_', "");
    if let Some(hex) = normalized.strip_prefix("0x").or_else(|| normalized.strip_prefix("0X")) {
        u64::from_str_radix(hex, 16).map_err(|error| format!("invalid number {text:?}: {error}"))
    } else {
        normalized.parse().map_err(|error| format!("invalid number {text:?}: {error}"))
    }
}

fn parse_u32(text: &str) -> Result<u32, String> {
    u32::try_from(parse_u64(text)?).map_err(|_| format!("number does not fit u32: {text:?}"))
}

fn parse_mac(text: &str) -> Result<[u8; 6], String> {
    let parts = text.split([':', '-']).collect::<Vec<_>>();
    if parts.len() != 6 {
        return Err(format!("invalid MAC address {text:?}"));
    }
    let mut mac = [0_u8; 6];
    for (destination, source) in mac.iter_mut().zip(parts) {
        *destination = u8::from_str_radix(source, 16)
            .map_err(|error| format!("invalid MAC address {text:?}: {error}"))?;
    }
    Ok(mac)
}

fn parse_escaped(text: &str) -> Result<Vec<u8>, String> {
    let bytes = text.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] != b'\\' {
            output.push(bytes[index]);
            index += 1;
            continue;
        }
        index += 1;
        let escape = *bytes.get(index).ok_or_else(|| "trailing backslash in --script".to_owned())?;
        index += 1;
        match escape {
            b'r' => output.push(b'\r'),
            b'n' => output.push(b'\n'),
            b't' => output.push(b'\t'),
            b'\\' => output.push(b'\\'),
            b'x' => {
                let hi = *bytes.get(index).ok_or_else(|| "short \\x escape".to_owned())?;
                let lo = *bytes.get(index + 1).ok_or_else(|| "short \\x escape".to_owned())?;
                index += 2;
                let digits = [hi, lo];
                let digits = std::str::from_utf8(&digits).map_err(|error| error.to_string())?;
                output.push(u8::from_str_radix(digits, 16).map_err(|error| error.to_string())?);
            }
            _ => return Err(format!("unsupported script escape \\{}", char::from(escape))),
        }
    }
    Ok(output)
}
