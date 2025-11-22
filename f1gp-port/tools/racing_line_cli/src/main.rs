use anyhow::{bail, Context, Result};
use clap::{Args, Parser, Subcommand};
use f1gp_port::{load_file_bytes, parse_track_asset};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Export racing-line and section metadata from F1GP track files"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Print a quick summary of a track file
    Summary(SummaryArgs),
    /// Export structured metadata (sections + racing line) as JSON
    Export(ExportArgs),
}

#[derive(Args, Debug)]
struct SummaryArgs {
    /// Path to the F1CTxx.DAT file
    #[arg(required = true)]
    input: PathBuf,

    /// Optional friendly name (otherwise derived from filename)
    #[arg(long)]
    name: Option<String>,
}

#[derive(Args, Debug)]
struct ExportArgs {
    /// One or more DAT files to export
    #[arg(required = true)]
    inputs: Vec<PathBuf>,

    /// Output file (only when exporting a single input) or directory
    #[arg(long)]
    output: Option<PathBuf>,

    /// Pretty-print JSON instead of compact output
    #[arg(long, default_value_t = false)]
    pretty: bool,
}

#[derive(Serialize)]
struct TrackExport {
    name: String,
    file: String,
    checksum: ChecksumInfo,
    offsets: OffsetExport,
    header: Option<HeaderExport>,
    section_data_offset: u64,
    section_skip_hint: u64,
    section_count: usize,
    sections: Vec<SectionExport>,
    racing_line: RacingLineExport,
}

#[derive(Serialize)]
struct ChecksumInfo {
    stored: u32,
    computed: u32,
    valid: bool,
}

#[derive(Serialize)]
struct OffsetExport {
    base_offset: i16,
    checksum_position: i16,
    object_data: i16,
    track_data: i16,
}

#[derive(Serialize)]
struct HeaderExport {
    start_width: i16,
    kerb_type: u8,
    pole_side: i16,
    pits_side: u8,
}

#[derive(Serialize)]
struct SectionExport {
    index: usize,
    length: f32,
    curvature: i16,
    height: i16,
    flags: u16,
    width: f32,
    position: [f32; 3],
    surface: String,
    commands: Vec<f1gp_port::TrackSectionCommand>,
}

#[derive(Serialize)]
struct RacingLineExport {
    displacement: i16,
    segments: Vec<f1gp_port::RacingLineSegment>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Summary(args) => summary(args),
        Command::Export(args) => export(args),
    }
}

fn summary(args: SummaryArgs) -> Result<()> {
    let asset = load_track(&args.input, args.name)?;
    println!("Track      : {}", asset.name);
    println!("File       : {}", args.input.display());
    println!(
        "Checksum   : stored=0x{:08X} computed=0x{:08X}",
        asset.checksum, asset.computed_checksum
    );
    println!("Sections   : {}", asset.sections.len());
    let total_length: f32 = asset.sections.iter().map(|s| s.length).sum();
    println!("Length     : {:.3} km", total_length / 1000.0);
    println!("Skip hint  : {} bytes", asset.section_skip_hint);
    println!("Racing seg : {}", asset.racing_line.segments.len());
    Ok(())
}

fn export(args: ExportArgs) -> Result<()> {
    if args.inputs.len() > 1 {
        if args.output.is_none() {
            bail!("--output <dir> is required when exporting multiple inputs");
        }
        let output_dir = args.output.unwrap();
        fs::create_dir_all(&output_dir).with_context(|| {
            format!("Failed to create output directory {}", output_dir.display())
        })?;
        for input in args.inputs {
            let asset = load_track(&input, None)?;
            let export = build_export(&input, asset);
            let file_name = format!("{}.racing.json", export.name.replace(' ', "_"));
            let dest = output_dir.join(file_name);
            write_json(&export, &dest, args.pretty)?;
            println!("Wrote {}", dest.display());
        }
    } else {
        let input = args.inputs[0].clone();
        let asset = load_track(&input, None)?;
        let export = build_export(&input, asset);
        if let Some(output_path) = args.output {
            if output_path.is_dir() {
                let file_name = format!("{}.racing.json", export.name.replace(' ', "_"));
                let dest = output_path.join(file_name);
                write_json(&export, &dest, args.pretty)?;
                println!("Wrote {}", dest.display());
            } else {
                write_json(&export, &output_path, args.pretty)?;
                println!("Wrote {}", output_path.display());
            }
        } else {
            let json = if args.pretty {
                serde_json::to_string_pretty(&export)?
            } else {
                serde_json::to_string(&export)?
            };
            println!("{}", json);
        }
    }

    Ok(())
}

fn load_track(path: &PathBuf, name_override: Option<String>) -> Result<f1gp_port::TrackAsset> {
    let bytes = load_file_bytes(path)
        .with_context(|| format!("Failed to read track file {}", path.display()))?;
    let inferred_name = name_override.unwrap_or_else(|| {
        path.file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "Track".to_string())
    });
    parse_track_asset(bytes, inferred_name)
        .with_context(|| format!("Failed to parse {}", path.display()))
}

fn build_export(input: &Path, asset: f1gp_port::TrackAsset) -> TrackExport {
    TrackExport {
        file: input.display().to_string(),
        name: asset.name.clone(),
        checksum: ChecksumInfo {
            stored: asset.checksum,
            computed: asset.computed_checksum,
            valid: asset.checksum == asset.computed_checksum,
        },
        offsets: OffsetExport {
            base_offset: asset.offsets.base_offset,
            checksum_position: asset.offsets.checksum_position,
            object_data: asset.offsets.object_data,
            track_data: asset.offsets.track_data,
        },
        header: asset.header.as_ref().map(|h| HeaderExport {
            start_width: h.start_width,
            kerb_type: h.kerb_type,
            pole_side: h.pole_side,
            pits_side: h.pits_side,
        }),
        section_data_offset: asset.section_data_offset,
        section_skip_hint: asset.section_skip_hint,
        section_count: asset.sections.len(),
        sections: asset
            .sections
            .iter()
            .enumerate()
            .map(|(idx, section)| SectionExport {
                index: idx,
                length: section.length,
                curvature: section.curvature,
                height: section.height,
                flags: section.flags,
                width: section.width,
                position: [section.position.x, section.position.y, section.position.z],
                surface: format!("{:?}", section.surface),
                commands: section.commands.clone(),
            })
            .collect(),
        racing_line: RacingLineExport {
            displacement: asset.racing_line.displacement,
            segments: asset.racing_line.segments.clone(),
        },
    }
}

fn write_json(export: &TrackExport, path: &Path, pretty: bool) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }
    let json = if pretty {
        serde_json::to_string_pretty(export)?
    } else {
        serde_json::to_string(export)?
    };
    fs::write(path, json).with_context(|| format!("Failed to write {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use f1gp_port::data::fixtures::{
        synthetic_track_bytes, DEFAULT_SECTION_COUNT, DEFAULT_TRACK_DATA_OFFSET,
    };

    #[test]
    fn export_structure_serializes() {
        let data = synthetic_track_bytes(DEFAULT_SECTION_COUNT, DEFAULT_TRACK_DATA_OFFSET);
        let asset = parse_track_asset(data, "Fixture".to_string()).unwrap();
        let export = build_export(&PathBuf::from("Fixture"), asset);
        let json = serde_json::to_string(&export).unwrap();
        assert!(json.contains("\"section_count\""));
    }
}
