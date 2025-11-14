use anyhow::{Context, Result};
use clap::Parser;
use log::{info, warn};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// Extract files from F1GP ISO image
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the ISO file
    #[arg(short, long)]
    input: PathBuf,

    /// Output directory for extracted files
    #[arg(short, long)]
    output: PathBuf,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging
    if args.verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    }

    info!("F1GP ISO Extractor");
    info!("Input ISO: {}", args.input.display());
    info!("Output directory: {}", args.output.display());

    // Check if ISO file exists
    if !args.input.exists() {
        anyhow::bail!("ISO file not found: {}", args.input.display());
    }

    // Create output directory
    fs::create_dir_all(&args.output)
        .context("Failed to create output directory")?;

    // Open ISO file
    info!("Opening ISO file...");
    let iso_file = File::open(&args.input)
        .context("Failed to open ISO file")?;

    // Read ISO filesystem
    info!("Reading ISO 9660 filesystem...");
    let iso = cdfs::ISO9660Reader::new(iso_file)
        .context("Failed to read ISO 9660 filesystem")?;

    // Get root directory
    let root = iso.root();
    info!("ISO filesystem loaded. Root directory entries: {}", root.len());

    // Extract all files recursively
    let mut total_files = 0;
    let mut total_dirs = 0;

    extract_directory(&iso, &root, &args.output, &mut total_files, &mut total_dirs)?;

    println!("\nExtraction complete!");
    println!("  Directories: {}", total_dirs);
    println!("  Files: {}", total_files);
    println!("  Output: {}", args.output.display());

    Ok(())
}

fn extract_directory(
    iso: &cdfs::ISO9660Reader<File>,
    entries: &[cdfs::DirectoryEntry],
    output_path: &Path,
    total_files: &mut usize,
    total_dirs: &mut usize,
) -> Result<()> {
    for entry in entries {
        let name = entry.name();

        // Skip "." and ".." entries
        if name == "." || name == ".." {
            continue;
        }

        let entry_path = output_path.join(name);

        if entry.is_dir() {
            // Create directory
            fs::create_dir_all(&entry_path)
                .context(format!("Failed to create directory: {}", entry_path.display()))?;

            *total_dirs += 1;
            info!("Created directory: {}", entry_path.display());

            // Recursively extract subdirectory
            let sub_entries = iso.read_dir(entry)
                .context(format!("Failed to read directory: {}", name))?;
            extract_directory(iso, &sub_entries, &entry_path, total_files, total_dirs)?;
        } else {
            // Extract file
            let mut file_data = Vec::new();
            iso.read_file(entry, &mut file_data)
                .context(format!("Failed to read file: {}", name))?;

            let mut output_file = File::create(&entry_path)
                .context(format!("Failed to create file: {}", entry_path.display()))?;

            output_file.write_all(&file_data)
                .context(format!("Failed to write file: {}", entry_path.display()))?;

            *total_files += 1;
            info!("Extracted: {} ({} bytes)", entry_path.display(), file_data.len());
        }
    }

    Ok(())
}
