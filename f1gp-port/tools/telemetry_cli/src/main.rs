use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use csv::Writer;
use f1gp_port::telemetry::{TelemetryRecording, TelemetrySample};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Inspect and convert F1GP telemetry captures")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Print aggregate information about a telemetry recording
    Summary(SummaryArgs),
    /// Export telemetry recording to JSON (pretty-print optional)
    ExportJson(ExportArgs),
    /// Export telemetry to CSV (one row per sample)
    ExportCsv(ExportCsvArgs),
    /// Compare two CSV exports and report basic drift metrics
    Diff(DiffArgs),
}

#[derive(Args, Debug)]
struct SummaryArgs {
    /// Telemetry .bin file produced by the game runtime
    #[arg(long)]
    input: PathBuf,

    /// Include per-car sample counts and last timestamps
    #[arg(long, default_value_t = false)]
    verbose: bool,
}

#[derive(Args, Debug)]
struct ExportArgs {
    #[arg(long)]
    input: PathBuf,
    #[arg(long)]
    output: PathBuf,
    /// Whether to pretty-print the JSON output
    #[arg(long, default_value_t = false)]
    pretty: bool,
}

#[derive(Args, Debug)]
struct ExportCsvArgs {
    #[arg(long)]
    input: PathBuf,
    #[arg(long)]
    output: PathBuf,
    /// Filter to a single car ID
    #[arg(long)]
    car: Option<u8>,
}

#[derive(Args, Debug)]
struct DiffArgs {
    #[arg(long)]
    reference: PathBuf,
    #[arg(long)]
    candidate: PathBuf,
    /// Optional car filter; when omitted compares all overlapping car samples
    #[arg(long)]
    car: Option<u8>,
}

#[derive(Serialize)]
struct SummaryRow {
    car_id: u8,
    samples: usize,
    duration_ms: u64,
}

#[derive(Deserialize)]
struct CsvRow {
    #[serde(rename = "timestamp_ms")]
    _timestamp_ms: f32,
    #[serde(rename = "car_id")]
    car_id: u8,
    speed: f32,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Summary(args) => summary(args),
        Command::ExportJson(args) => export_json(args),
        Command::ExportCsv(args) => export_csv(args),
        Command::Diff(args) => diff_csv(args),
    }
}

fn load_recording(path: &PathBuf) -> Result<TelemetryRecording> {
    TelemetryRecording::read_from_file(path)
        .with_context(|| format!("Failed to read telemetry recording from {}", path.display()))
}

fn summary(args: SummaryArgs) -> Result<()> {
    let recording = load_recording(&args.input)?;
    println!("Track      : {}", recording.track_name);
    println!("Session    : {}", recording.session_type);
    println!("Samples    : {}", recording.samples.len());

    if let Some(last) = recording.samples.last() {
        println!("Duration   : {:.3} s", last.timestamp_ms as f32 / 1000.0);
    }

    if args.verbose {
        let mut rows = Vec::new();
        for (car_id, group) in group_by_car(&recording.samples) {
            let duration = group.last().map(|s| s.timestamp_ms).unwrap_or(0);
            rows.push(SummaryRow {
                car_id,
                samples: group.len(),
                duration_ms: duration,
            });
        }
        println!("\nPer-car stats:");
        println!("car\tsamples\tlast_timestamp_ms");
        for row in rows {
            println!("{}\t{}\t{}", row.car_id, row.samples, row.duration_ms);
        }
    }

    Ok(())
}

fn export_json(args: ExportArgs) -> Result<()> {
    let recording = load_recording(&args.input)?;
    let json = if args.pretty {
        serde_json::to_string_pretty(&recording)?
    } else {
        serde_json::to_string(&recording)?
    };
    if let Some(parent) = args.output.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }
    fs::write(&args.output, json)
        .with_context(|| format!("Failed to write {}", args.output.display()))?;
    println!("Wrote JSON telemetry to {}", args.output.display());
    Ok(())
}

fn export_csv(args: ExportCsvArgs) -> Result<()> {
    let recording = load_recording(&args.input)?;
    if let Some(parent) = args.output.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }

    let mut writer = Writer::from_path(&args.output)
        .with_context(|| format!("Failed to create CSV {}", args.output.display()))?;

    writer.write_record([
        "timestamp_ms",
        "car_id",
        "pos_x",
        "pos_y",
        "pos_z",
        "speed",
        "rpm",
        "gear",
        "throttle",
        "brake",
        "steering",
    ])?;

    for sample in &recording.samples {
        if let Some(car_filter) = args.car {
            if sample.car_id != car_filter {
                continue;
            }
        }
        writer.write_record([
            sample.timestamp_ms.to_string(),
            sample.car_id.to_string(),
            sample.position[0].to_string(),
            sample.position[1].to_string(),
            sample.position[2].to_string(),
            sample.speed.to_string(),
            sample.rpm.to_string(),
            sample.gear.to_string(),
            sample.throttle.to_string(),
            sample.brake.to_string(),
            sample.steering.to_string(),
        ])?;
    }

    writer.flush()?;
    println!("Wrote CSV telemetry to {}", args.output.display());
    Ok(())
}

fn diff_csv(args: DiffArgs) -> Result<()> {
    let reference = load_csv_rows(&args.reference)?;
    let candidate = load_csv_rows(&args.candidate)?;

    let mut pairs = Vec::new();
    let len = reference.len().min(candidate.len());
    for idx in 0..len {
        let a = &reference[idx];
        let b = &candidate[idx];
        if args.car.map(|c| c == a.car_id).unwrap_or(true) && a.car_id == b.car_id {
            pairs.push((a, b));
        }
    }

    if pairs.is_empty() {
        println!("No overlapping samples found for car filter {:?}", args.car);
        return Ok(());
    }

    let mut speed_delta = 0.0_f32;
    for (a, b) in &pairs {
        speed_delta += (a.speed - b.speed).abs();
    }

    let avg_delta = speed_delta / pairs.len() as f32;
    println!("Compared {} overlapping samples", pairs.len());
    println!("Average speed delta: {:.3} km/h", avg_delta * 3.6);
    Ok(())
}

fn load_csv_rows(path: &PathBuf) -> Result<Vec<CsvRow>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)
        .with_context(|| format!("Failed to read CSV {}", path.display()))?;
    let mut rows = Vec::new();
    for record in reader.deserialize() {
        let row: CsvRow = record?;
        rows.push(row);
    }
    Ok(rows)
}

fn group_by_car(samples: &[TelemetrySample]) -> Vec<(u8, Vec<&TelemetrySample>)> {
    let mut map: std::collections::BTreeMap<u8, Vec<&TelemetrySample>> =
        std::collections::BTreeMap::new();
    for sample in samples {
        map.entry(sample.car_id).or_default().push(sample);
    }
    map.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_recording() -> TelemetryRecording {
        let mut rec = TelemetryRecording::new("Monaco", "Practice");
        let mut sample0 = TelemetrySample::new(0, 0);
        sample0.speed = 10.0;
        rec.push_sample(sample0);
        let mut sample1 = TelemetrySample::new(100, 1);
        sample1.speed = 12.0;
        rec.push_sample(sample1);
        rec
    }

    #[test]
    fn group_by_car_splits_samples() {
        let rec = build_recording();
        let grouped = group_by_car(&rec.samples);
        assert_eq!(grouped.len(), 2);
        assert_eq!(grouped[0].0, 0);
        assert_eq!(grouped[1].0, 1);
    }
}
