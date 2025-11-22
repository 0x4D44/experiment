use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use csv::Writer;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Parse DOS serial telemetry logs into CSV/JSON"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Convert DOS capture log (serial1=raw file) to CSV
    ExportCsv(ExportCsvArgs),
    /// Convert DOS capture log to JSON array
    ExportJson(ExportJsonArgs),
}

#[derive(Args, Debug)]
struct ExportCsvArgs {
    #[arg(long)]
    input: PathBuf,
    #[arg(long)]
    output: PathBuf,
}

#[derive(Args, Debug)]
struct ExportJsonArgs {
    #[arg(long)]
    input: PathBuf,
    #[arg(long)]
    output: PathBuf,
    #[arg(long, default_value_t = false)]
    pretty: bool,
}

#[derive(Debug, Clone, Serialize)]
struct DosSample {
    timestamp_ms: u64,
    car_id: u8,
    speed_kmh: f32,
    rpm: f32,
    gear: i8,
    throttle: f32,
    brake: f32,
    steering: f32,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::ExportCsv(args) => export_csv(args),
        Command::ExportJson(args) => export_json(args),
    }
}

fn export_csv(args: ExportCsvArgs) -> Result<()> {
    let samples = parse_log(&args.input)?;
    if let Some(parent) = args.output.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }
    let mut writer = Writer::from_path(&args.output)
        .with_context(|| format!("Failed to create CSV {}", args.output.display()))?;
    writer.write_record([
        "timestamp_ms",
        "car_id",
        "speed_kmh",
        "rpm",
        "gear",
        "throttle",
        "brake",
        "steering",
    ])?;
    for sample in &samples {
        writer.write_record([
            sample.timestamp_ms.to_string(),
            sample.car_id.to_string(),
            format!("{:.3}", sample.speed_kmh),
            format!("{:.0}", sample.rpm),
            sample.gear.to_string(),
            format!("{:.3}", sample.throttle),
            format!("{:.3}", sample.brake),
            format!("{:.3}", sample.steering),
        ])?;
    }
    writer.flush()?;
    println!(
        "Wrote {} samples to {}",
        samples.len(),
        args.output.display()
    );
    Ok(())
}

fn export_json(args: ExportJsonArgs) -> Result<()> {
    let samples = parse_log(&args.input)?;
    if let Some(parent) = args.output.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }
    let json = if args.pretty {
        serde_json::to_string_pretty(&samples)?
    } else {
        serde_json::to_string(&samples)?
    };
    fs::write(&args.output, json)
        .with_context(|| format!("Failed to write {}", args.output.display()))?;
    println!(
        "Wrote {} samples to {}",
        samples.len(),
        args.output.display()
    );
    Ok(())
}

fn parse_log(path: &PathBuf) -> Result<Vec<DosSample>> {
    let bytes =
        fs::read(path).with_context(|| format!("Failed to read DOS capture {}", path.display()))?;
    let lines = bytes
        .split(|b| *b == b'\n' || *b == b'\r')
        .filter(|line| !line.is_empty());

    let mut samples = Vec::new();
    for line in lines {
        if let Ok(line_str) = std::str::from_utf8(line) {
            if let Some(sample) = parse_line(line_str.trim()) {
                samples.push(sample);
            }
        }
    }
    Ok(samples)
}

fn parse_line(line: &str) -> Option<DosSample> {
    // Expected format: "12345|CAR=00|SPD=120.0|RPM=11000|GEAR=3|THR=0.75|BRK=0.00|STR=-0.10"
    let mut timestamp = None;
    let mut car = None;
    let mut speed = None;
    let mut rpm = None;
    let mut gear = None;
    let mut throttle = None;
    let mut brake = None;
    let mut steering = None;

    for part in line.split('|') {
        if part.is_empty() {
            continue;
        }
        if let Some(value) = part.strip_prefix("CAR=") {
            if let Ok(parsed) = u8::from_str_radix(value, 16) {
                car = Some(parsed);
            }
        } else if let Some(value) = part.strip_prefix("SPD=") {
            speed = value.parse::<f32>().ok();
        } else if let Some(value) = part.strip_prefix("RPM=") {
            rpm = value.parse::<f32>().ok();
        } else if let Some(value) = part.strip_prefix("GEAR=") {
            gear = value.parse::<i8>().ok();
        } else if let Some(value) = part.strip_prefix("THR=") {
            throttle = value.parse::<f32>().ok();
        } else if let Some(value) = part.strip_prefix("BRK=") {
            brake = value.parse::<f32>().ok();
        } else if let Some(value) = part.strip_prefix("STR=") {
            steering = value.parse::<f32>().ok();
        } else if timestamp.is_none() {
            timestamp = part.parse::<u64>().ok();
        }
    }

    Some(DosSample {
        timestamp_ms: timestamp?,
        car_id: car.unwrap_or(0),
        speed_kmh: speed.unwrap_or(0.0),
        rpm: rpm.unwrap_or(0.0),
        gear: gear.unwrap_or(0),
        throttle: throttle.unwrap_or(0.0),
        brake: brake.unwrap_or(0.0),
        steering: steering.unwrap_or(0.0),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_extracts_fields() {
        let sample =
            parse_line("100|CAR=01|SPD=150.5|RPM=11000|GEAR=4|THR=0.80|BRK=0.10|STR=-0.20")
                .expect("line should parse");
        assert_eq!(sample.timestamp_ms, 100);
        assert_eq!(sample.car_id, 0x01);
        assert!((sample.speed_kmh - 150.5).abs() < f32::EPSILON);
        assert_eq!(sample.gear, 4);
    }
}
