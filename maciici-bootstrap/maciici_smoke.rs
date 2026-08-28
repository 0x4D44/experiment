use std::collections::HashSet;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};
use clap::Parser;
use log::{info, warn};
use serde::Serialize;

use snow_core::emulator::comm::{EmulatorCommand, EmulatorEvent, EmulatorSpeed, UserMessageType};
use snow_core::emulator::{Emulator, MouseMode};
use snow_core::mac::{MacModel, MacMonitor};
use snow_core::renderer::DisplayBuffer;
use snow_core::tickable::{Tickable, Ticks};

#[derive(Parser, Debug)]
#[command(about = "Headless Macintosh IIci ROM and framebuffer smoke test")]
struct Args {
    /// Macintosh IIci ROM image.
    #[arg(long)]
    rom: PathBuf,

    /// Guest cycles to execute before stopping.
    #[arg(long, default_value_t = 250_000_000)]
    cycles: Ticks,

    /// Stop the smoke run after this many host seconds.
    #[arg(long, default_value_t = 60)]
    host_timeout: u64,

    /// RAM size in MiB.
    #[arg(long, default_value_t = 8)]
    ram_mib: usize,

    /// Directory for PNG, raw frame and JSON evidence.
    #[arg(long, default_value = ".")]
    out_dir: PathBuf,

    /// Prefix for generated evidence files.
    #[arg(long, default_value = "maciici-smoke")]
    prefix: String,

    /// Permit a run with no non-uniform framebuffer to exit successfully.
    #[arg(long, default_value_t = false)]
    allow_blank: bool,
}

#[derive(Debug, Serialize)]
struct SmokeReport {
    rom_path: String,
    rom_size: usize,
    detected_model: String,
    target_cycles: Ticks,
    completed_cycles: Ticks,
    elapsed_host_seconds: f64,
    timed_out: bool,
    stopped_early: bool,
    final_pc: Option<u32>,
    frames_received: usize,
    best_width: Option<u16>,
    best_height: Option<u16>,
    best_unique_colors: usize,
    best_nonblack_pixels: usize,
    best_frame_hash_fnv1a64: Option<String>,
    user_messages: Vec<String>,
    success: bool,
}

#[derive(Debug)]
struct FrameEvidence {
    width: u16,
    height: u16,
    rgba: Vec<u8>,
    unique_colors: usize,
    nonblack_pixels: usize,
    hash: u64,
}

fn frame_evidence(buffer: DisplayBuffer) -> FrameEvidence {
    let width = buffer.width();
    let height = buffer.height();
    let rgba = buffer.into_inner();

    let mut colors = HashSet::new();
    let mut nonblack_pixels = 0usize;
    for pixel in rgba.chunks_exact(4) {
        let packed = u32::from_be_bytes([pixel[0], pixel[1], pixel[2], pixel[3]]);
        colors.insert(packed);
        if pixel[0] != 0 || pixel[1] != 0 || pixel[2] != 0 {
            nonblack_pixels += 1;
        }
    }

    let mut hash = 0xcbf29ce484222325u64;
    for byte in &rgba {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }

    FrameEvidence {
        width,
        height,
        rgba,
        unique_colors: colors.len(),
        nonblack_pixels,
        hash,
    }
}

fn write_png(path: &Path, frame: &FrameEvidence) -> Result<()> {
    let mut encoder = png::Encoder::new(
        File::create(path).with_context(|| format!("creating {}", path.display()))?,
        u32::from(frame.width),
        u32::from(frame.height),
    );
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_compression(png::Compression::Best);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&frame.rgba)?;
    Ok(())
}

fn drain_events(
    event_recv: &snow_core::emulator::comm::EmulatorEventReceiver,
    final_pc: &mut Option<u32>,
    running: &mut bool,
    user_messages: &mut Vec<String>,
) {
    while let Ok(event) = event_recv.try_recv() {
        match event {
            EmulatorEvent::Status(status) => {
                *final_pc = Some(status.regs.pc);
                *running = status.running;
                info!(
                    "status: cycles={} pc={:08X} running={}",
                    status.cycles, status.regs.pc, status.running
                );
            }
            EmulatorEvent::UserMessage(kind, message) => {
                let kind_name = match kind {
                    UserMessageType::Success => "success",
                    UserMessageType::Notice => "notice",
                    UserMessageType::Warning => "warning",
                    UserMessageType::Error => "error",
                };
                warn!("guest/emulator message ({kind_name}): {message}");
                user_messages.push(format!("{kind_name}: {message}"));
            }
            _ => {}
        }
    }
}

fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_millis()
        .init();

    let args = Args::parse();
    fs::create_dir_all(&args.out_dir)?;

    let rom = fs::read(&args.rom)
        .with_context(|| format!("reading Macintosh IIci ROM {}", args.rom.display()))?;
    let detected = MacModel::detect_from_rom(&rom).context("ROM is not recognised by Snow")?;
    if detected != MacModel::MacIIci {
        bail!("ROM was detected as {detected}, not Macintosh IIci");
    }

    let ram_bytes = args
        .ram_mib
        .checked_mul(1024 * 1024)
        .context("RAM size overflow")?;
    let (mut emulator, frame_recv) = Emulator::new_with_extra(
        &rom,
        &[],
        MacModel::MacIIci,
        Some(MacMonitor::HiRes14),
        MouseMode::RelativeHw,
        Some(ram_bytes),
        None,
        false,
        None,
    )?;
    let cmd = emulator.create_cmd_sender();
    let event_recv = emulator.create_event_recv();

    cmd.send(EmulatorCommand::SetSpeed(EmulatorSpeed::Uncapped))?;
    cmd.send(EmulatorCommand::Run)?;

    let start = Instant::now();
    let host_timeout = Duration::from_secs(args.host_timeout);
    let mut timed_out = false;
    let mut running = true;
    let mut final_pc = None;
    let mut user_messages = Vec::new();
    let mut frames_received = 0usize;
    let mut best_frame: Option<FrameEvidence> = None;

    info!(
        "starting IIci ROM smoke test: target_cycles={} ram={} MiB",
        args.cycles, args.ram_mib
    );

    while emulator.get_cycles() < args.cycles && running {
        if start.elapsed() >= host_timeout {
            timed_out = true;
            break;
        }

        emulator.tick(1, ())?;
        drain_events(
            &event_recv,
            &mut final_pc,
            &mut running,
            &mut user_messages,
        );

        if let Some(buffer) = frame_recv.lock().unwrap().take() {
            frames_received += 1;
            let candidate = frame_evidence(buffer);
            let candidate_score = (candidate.unique_colors, candidate.nonblack_pixels);
            let best_score = best_frame
                .as_ref()
                .map(|frame| (frame.unique_colors, frame.nonblack_pixels))
                .unwrap_or_default();
            if candidate_score > best_score {
                info!(
                    "new best frame: {}x{} colors={} nonblack={} hash={:016x}",
                    candidate.width,
                    candidate.height,
                    candidate.unique_colors,
                    candidate.nonblack_pixels,
                    candidate.hash
                );
                best_frame = Some(candidate);
            }
        }
    }

    let completed_cycles = emulator.get_cycles();
    let stopped_early = !running && completed_cycles < args.cycles;

    // Force a final status snapshot without losing the evidence already captured.
    if running {
        cmd.send(EmulatorCommand::Stop)?;
        emulator.tick(1, ())?;
    }
    drain_events(
        &event_recv,
        &mut final_pc,
        &mut running,
        &mut user_messages,
    );

    let mut success = !timed_out && !stopped_early && completed_cycles >= args.cycles;
    if !args.allow_blank {
        success &= best_frame
            .as_ref()
            .is_some_and(|frame| frame.unique_colors >= 2 && frame.nonblack_pixels > 0);
    }

    if let Some(frame) = best_frame.as_ref() {
        let png_path = args.out_dir.join(format!("{}.png", args.prefix));
        let raw_path = args.out_dir.join(format!("{}.frame", args.prefix));
        write_png(&png_path, frame)?;
        fs::write(&raw_path, &frame.rgba)?;
        info!("wrote {}", png_path.display());
    }

    let report = SmokeReport {
        rom_path: args.rom.display().to_string(),
        rom_size: rom.len(),
        detected_model: detected.to_string(),
        target_cycles: args.cycles,
        completed_cycles,
        elapsed_host_seconds: start.elapsed().as_secs_f64(),
        timed_out,
        stopped_early,
        final_pc,
        frames_received,
        best_width: best_frame.as_ref().map(|frame| frame.width),
        best_height: best_frame.as_ref().map(|frame| frame.height),
        best_unique_colors: best_frame
            .as_ref()
            .map(|frame| frame.unique_colors)
            .unwrap_or_default(),
        best_nonblack_pixels: best_frame
            .as_ref()
            .map(|frame| frame.nonblack_pixels)
            .unwrap_or_default(),
        best_frame_hash_fnv1a64: best_frame
            .as_ref()
            .map(|frame| format!("{:016x}", frame.hash)),
        user_messages,
        success,
    };

    let report_path = args.out_dir.join(format!("{}.json", args.prefix));
    fs::write(&report_path, serde_json::to_vec_pretty(&report)?)?;
    println!("{}", serde_json::to_string_pretty(&report)?);

    if !success {
        bail!(
            "Macintosh IIci smoke test did not reach its acceptance gate; see {}",
            report_path.display()
        );
    }

    Ok(())
}
