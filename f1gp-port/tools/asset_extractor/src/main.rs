use anyhow::{Context, Result};
use clap::Parser;
use f1gp_port::{calculate_checksum, load_file_bytes, read_stored_checksum, verify_checksum};
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const TRACK_FILES: &[&str] = &[
    "F1CT01.DAT",
    "F1CT02.DAT",
    "F1CT03.DAT",
    "F1CT04.DAT",
    "F1CT05.DAT",
    "F1CT06.DAT",
    "F1CT07.DAT",
    "F1CT08.DAT",
    "F1CT09.DAT",
    "F1CT10.DAT",
    "F1CT11.DAT",
    "F1CT12.DAT",
    "F1CT13.DAT",
    "F1CT14.DAT",
    "F1CT15.DAT",
    "F1CT16.DAT",
];

#[derive(Parser, Debug)]
#[command(name = "asset_extractor")]
#[command(about = "Copy and verify F1GP track assets", long_about = None)]
struct Args {
    /// Directory containing the extracted original files (e.g., assets/original/HARDDISK)
    #[arg(long)]
    source: PathBuf,

    /// Directory where verified files should be copied
    #[arg(long)]
    dest: PathBuf,

    /// Skip copy; only report checksum status
    #[arg(long, default_value_t = false)]
    dry_run: bool,

    /// Overwrite existing files in destination
    #[arg(long, default_value_t = false)]
    force: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct TrackEntry {
    file: String,
    size: usize,
    stored_checksum: Option<u32>,
    computed_checksum: u32,
    checksum_valid: bool,
}

fn verify_and_copy(
    source: &Path,
    dest: &Path,
    dry_run: bool,
    force: bool,
) -> Result<Vec<TrackEntry>> {
    fs::create_dir_all(dest)
        .with_context(|| format!("Failed to create destination {}", dest.display()))?;
    let pb = ProgressBar::new(TRACK_FILES.len() as u64);
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} [{elapsed}] {pos}/{len} {msg}")
            .unwrap()
            .tick_chars("|/-\\"),
    );

    let mut manifest = Vec::new();

    for file in TRACK_FILES {
        pb.set_message(file.to_string());
        let src = source.join(file);
        if !src.exists() {
            warn!("Missing source file: {}", src.display());
            pb.inc(1);
            continue;
        }

        let data =
            load_file_bytes(&src).with_context(|| format!("Failed to read {}", src.display()))?;
        let stored = read_stored_checksum(&data);
        let computed = calculate_checksum(&data);
        let valid = verify_checksum(&data);
        if !valid {
            warn!(
                "Checksum mismatch for {} (stored={:?}, computed=0x{:08X})",
                file,
                stored.map(|v| format!("0x{:08X}", v)),
                computed
            );
        } else {
            info!("Validated checksum for {}", file);
        }

        let entry = TrackEntry {
            file: file.to_string(),
            size: data.len(),
            stored_checksum: stored,
            computed_checksum: computed,
            checksum_valid: valid,
        };
        manifest.push(entry);

        if dry_run {
            pb.inc(1);
            continue;
        }

        let dst = dest.join(file);
        if dst.exists() && !force {
            warn!(
                "Destination file exists, skipping (use --force): {}",
                dst.display()
            );
            pb.inc(1);
            continue;
        }

        fs::write(&dst, &data)
            .with_context(|| format!("Failed to copy {} -> {}", src.display(), dst.display()))?;
        pb.inc(1);
    }

    pb.finish_with_message("Done");
    Ok(manifest)
}

fn write_manifest(dest: &Path, manifest: &[TrackEntry]) -> Result<()> {
    let manifest_path = dest.join("asset_manifest.json");
    let json = serde_json::to_string_pretty(manifest).context("Failed to serialize manifest")?;
    fs::write(&manifest_path, json)
        .with_context(|| format!("Failed to write manifest to {}", manifest_path.display()))
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    if !args.source.exists() {
        error!("Source directory does not exist: {}", args.source.display());
        std::process::exit(1);
    }

    let manifest = verify_and_copy(&args.source, &args.dest, args.dry_run, args.force)?;
    if !args.dry_run {
        write_manifest(&args.dest, &manifest)?;
        info!("Wrote manifest for {} files", manifest.len());
    } else {
        info!("Dry run complete. No files copied.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use f1gp_port::data::fixtures::{
        synthetic_track_bytes, DEFAULT_SECTION_COUNT, DEFAULT_TRACK_DATA_OFFSET,
    };
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(prefix: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        path.push(format!("{}_{}", prefix, stamp));
        fs::create_dir_all(&path).unwrap();
        path
    }

    #[test]
    fn verify_and_copy_produces_manifest() {
        let source = temp_dir("asset_src");
        for track in TRACK_FILES {
            let bytes = synthetic_track_bytes(DEFAULT_SECTION_COUNT, DEFAULT_TRACK_DATA_OFFSET);
            fs::write(source.join(track), &bytes).unwrap();
        }

        let dest = temp_dir("asset_dest");
        let manifest = verify_and_copy(source.as_path(), dest.as_path(), false, true).unwrap();
        assert_eq!(manifest.len(), TRACK_FILES.len());

        write_manifest(dest.as_path(), &manifest).unwrap();
        let manifest_path = dest.join("asset_manifest.json");
        assert!(manifest_path.exists());

        let json = fs::read_to_string(&manifest_path).unwrap();
        let parsed: Vec<TrackEntry> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.len(), TRACK_FILES.len());

        let _ = fs::remove_dir_all(&source);
        let _ = fs::remove_dir_all(&dest);
    }
}
