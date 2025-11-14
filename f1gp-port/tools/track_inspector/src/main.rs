//! Track inspection and export tool
//!
//! Loads F1GP track files and exports them to JSON for analysis

use anyhow::{Context, Result};
use clap::Parser;
use env_logger;
use f1gp_port::load_track;
use log::info;
use serde_json;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "track_inspector")]
#[command(about = "Inspect and export F1GP track files", long_about = None)]
struct Args {
    /// Track file to inspect (F1CT*.DAT)
    #[arg(short, long)]
    input: PathBuf,

    /// Output JSON file (optional, defaults to stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Track name (optional, defaults to filename)
    #[arg(short = 'n', long)]
    name: Option<String>,

    /// Pretty-print JSON output
    #[arg(short, long, default_value = "true")]
    pretty: bool,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    info!("F1GP Track Inspector");
    info!("Loading track: {:?}", args.input);

    // Load the track
    let track = load_track(&args.input, args.name)
        .with_context(|| format!("Failed to load track from {:?}", args.input))?;

    info!("Successfully loaded track: {}", track.name);
    info!("  Checksum: 0x{:08X}", track.checksum);
    info!("  Object shapes: {}", track.object_shapes.len());
    info!("  Track sections: {}", track.sections.len());
    info!("  Racing line points: {}", track.racing_line.points.len());
    info!("  Pit lane sections: {}", track.pit_lane.len());
    info!("  Cameras: {}", track.cameras.len());

    // Serialize to JSON
    let json = if args.pretty {
        serde_json::to_string_pretty(&track)
            .context("Failed to serialize track to JSON")?
    } else {
        serde_json::to_string(&track)
            .context("Failed to serialize track to JSON")?
    };

    // Output
    if let Some(output_path) = args.output {
        info!("Writing to: {:?}", output_path);
        fs::write(&output_path, json)
            .with_context(|| format!("Failed to write to {:?}", output_path))?;
        info!("Export complete!");
    } else {
        println!("{}", json);
    }

    Ok(())
}
