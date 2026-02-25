use anyhow::{Context, Result};
use f1gp_port::data::fixtures::write_synthetic_track_fixture;
use f1gp_port::telemetry::{TelemetryRecording, TelemetrySample};
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    let track_path = PathBuf::from("data/fixtures/track_stub.bin");
    write_synthetic_track_fixture(&track_path)?;
    println!("Wrote {}", track_path.display());

    let driver_src = PathBuf::from("data/samples/driver_db.json");
    let driver_dst = PathBuf::from("data/fixtures/driver_db.json");
    if driver_src.exists() {
        if let Some(parent) = driver_dst.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create {}", parent.display()))?;
        }
        fs::copy(&driver_src, &driver_dst).with_context(|| {
            format!(
                "Failed to copy {} -> {}",
                driver_src.display(),
                driver_dst.display()
            )
        })?;
        println!("Copied {}", driver_dst.display());
    } else {
        println!(
            "Warning: missing sample driver DB at {}",
            driver_src.display()
        );
    }

    let telemetry_dir = PathBuf::from("data/fixtures/telemetry");
    fs::create_dir_all(&telemetry_dir)
        .with_context(|| format!("Failed to create {}", telemetry_dir.display()))?;
    let telemetry_path = telemetry_dir.join("synthetic_monaco.bin");
    write_sample_telemetry(&telemetry_path)?;
    println!("Wrote {}", telemetry_path.display());

    Ok(())
}

fn write_sample_telemetry(path: &PathBuf) -> Result<()> {
    let mut recording = TelemetryRecording::new("Synthetic Monaco", "FixtureLap");
    let mut sample_player = TelemetrySample::new(0, 0);
    sample_player.position = [0.0, 0.0, 0.0];
    sample_player.speed = 45.0;
    sample_player.rpm = 12000.0;
    sample_player.gear = 3;
    sample_player.throttle = 0.75;
    recording.push_sample(sample_player);

    let mut sample_ai = TelemetrySample::new(100, 1);
    sample_ai.position = [5.0, 0.0, 10.0];
    sample_ai.speed = 40.0;
    sample_ai.rpm = 11000.0;
    sample_ai.gear = 3;
    sample_ai.brake = 0.2;
    recording.push_sample(sample_ai);

    recording
        .write_to_file(path)
        .with_context(|| format!("Failed to write telemetry fixture to {}", path.display()))
}
