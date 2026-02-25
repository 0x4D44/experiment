use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

/// Simple parity harness stub – compares metadata JSON for now.
#[derive(Parser, Debug)]
#[command(version, about = "F1GP parity harness stub", long_about = None)]
struct Args {
    /// Path to DOS capture metadata JSON
    #[arg(long)]
    reference_meta: PathBuf,

    /// Path to Rust capture metadata JSON
    #[arg(long)]
    candidate_meta: PathBuf,
}

#[derive(Deserialize, Debug)]
struct CaptureMeta {
    run_id: String,
    binary: String,
}

fn load_meta(path: &PathBuf) -> CaptureMeta {
    let data = fs::read_to_string(path).expect("read metadata");
    serde_json::from_str(&data).expect("parse metadata")
}

fn main() {
    let args = Args::parse();
    let reference = load_meta(&args.reference_meta);
    let candidate = load_meta(&args.candidate_meta);
    println!(
        "Parity harness stub\n  reference:{}\n  candidate:{}",
        reference.run_id, candidate.run_id
    );
    if reference.run_id != candidate.run_id {
        eprintln!("Mismatch: run IDs differ");
        std::process::exit(1);
    }
    println!(
        "Run IDs match ({}). Replace with telemetry diff in Stage 5.",
        reference.run_id
    );
}
