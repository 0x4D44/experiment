use anyhow::{anyhow, Context, Result};
use clap::{Args, Parser, Subcommand};
use hound::{SampleFormat, WavSpec, WavWriter};
use image::{Rgba, RgbaImage};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Pcm(args) => {
            let manifest = run_pcm(&args)?;
            println!(
                "Converted {} PCM files into WAV format",
                manifest.entries.len()
            );
        }
        Command::Font(args) => {
            let manifest = run_font(&args)?;
            println!(
                "Wrote atlas {} for {} glyphs",
                manifest.atlas_file, manifest.glyph_count
            );
        }
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Extract PCM samples and bitmap fonts from original F1GP assets"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Convert raw PCM files to standard WAV files and emit a manifest
    Pcm(PcmArgs),
    /// Convert 1-bit packed bitmap fonts into a PNG atlas
    Font(FontArgs),
}

#[derive(Args, Debug)]
struct PcmArgs {
    /// Directory containing raw PCM files
    #[arg(long)]
    source: PathBuf,

    /// Destination directory for WAV files + manifest
    #[arg(long)]
    dest: PathBuf,

    /// Sample rate (Hz) to encode the WAV output with
    #[arg(long, default_value_t = 11025)]
    sample_rate: u32,

    /// Number of channels in the output WAV
    #[arg(long, default_value_t = 1)]
    channels: u16,

    /// File extensions (comma separated) to treat as PCM files
    #[arg(long, value_delimiter = ',', default_value = "pcm,snd")]
    extensions: Vec<String>,
}

#[derive(Args, Debug)]
struct FontArgs {
    /// Path to the raw packed font binary
    #[arg(long, alias = "source")]
    input: PathBuf,

    /// Output directory for atlas and manifest
    #[arg(long)]
    dest: PathBuf,

    /// Width of each glyph in pixels
    #[arg(long)]
    glyph_width: u32,

    /// Height of each glyph in pixels
    #[arg(long)]
    glyph_height: u32,

    /// Number of glyphs to decode
    #[arg(long)]
    glyph_count: usize,

    /// Number of columns in the atlas grid
    #[arg(long, default_value_t = 16)]
    columns: u32,

    /// Optional base filename (without extension) for generated assets
    #[arg(long)]
    name: Option<String>,

    /// Skip the first N bytes before decoding glyph data
    #[arg(long, default_value_t = 0)]
    skip_bytes: usize,

    /// Treat bits as inverted (0 = on, 1 = off)
    #[arg(long, default_value_t = false)]
    invert_bits: bool,

    /// Hex color (e.g. #FF0000FF) representing lit pixels
    #[arg(long, default_value = "#FFFFFFFF")]
    on_color: String,

    /// Hex color for background pixels
    #[arg(long, default_value = "#00000000")]
    off_color: String,

    /// Include glyph placement metadata in the manifest
    #[arg(long, default_value_t = false)]
    include_metadata: bool,
}

#[derive(Debug, Serialize)]
struct PcmManifest {
    entries: Vec<PcmManifestEntry>,
}

#[derive(Debug, Serialize)]
struct PcmManifestEntry {
    source_file: String,
    output_file: String,
    bytes_in: usize,
    bytes_out: usize,
    sample_rate: u32,
    channels: u16,
    samples_per_channel: usize,
    input_sha256: String,
    output_sha256: String,
}

#[derive(Debug, Serialize)]
struct FontManifest {
    source_file: String,
    atlas_file: String,
    glyph_width: u32,
    glyph_height: u32,
    glyph_count: usize,
    columns: u32,
    rows: u32,
    input_sha256: String,
    atlas_sha256: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    glyphs: Option<Vec<GlyphPlacement>>,
}

#[derive(Debug, Serialize, Clone)]
struct GlyphPlacement {
    index: usize,
    x: u32,
    y: u32,
}

fn run_pcm(args: &PcmArgs) -> Result<PcmManifest> {
    if !args.source.exists() {
        return Err(anyhow!(
            "Source directory does not exist: {}",
            args.source.display()
        ));
    }

    let extensions: Vec<String> = args
        .extensions
        .iter()
        .map(|ext| ext.trim_start_matches('.').to_ascii_lowercase())
        .collect();

    fs::create_dir_all(&args.dest)
        .with_context(|| format!("Failed to create destination {}", args.dest.display()))?;

    let source_root = fs::canonicalize(&args.source)
        .with_context(|| format!("Failed to canonicalize {}", args.source.display()))?;

    let mut entries = Vec::new();

    for entry in WalkDir::new(&source_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.into_path();
        if !should_process(&path, &extensions) {
            continue;
        }

        let relative = path
            .strip_prefix(&source_root)
            .unwrap_or(&path)
            .to_path_buf();
        let output_rel = relative.with_extension("wav");
        let output_path = args.dest.join(&output_rel);

        let input_bytes = fs::read(&path)
            .with_context(|| format!("Failed to read PCM file {}", path.display()))?;
        let input_hash = sha256_hex(&input_bytes);

        convert_pcm_to_wav(&input_bytes, args.channels, args.sample_rate, &output_path)?;
        let output_bytes = fs::read(&output_path)
            .with_context(|| format!("Failed to read WAV file {}", output_path.display()))?;
        let output_hash = sha256_hex(&output_bytes);

        let entry = PcmManifestEntry {
            source_file: relative.to_string_lossy().to_string(),
            output_file: output_rel.to_string_lossy().to_string(),
            bytes_in: input_bytes.len(),
            bytes_out: output_bytes.len(),
            sample_rate: args.sample_rate,
            channels: args.channels,
            samples_per_channel: input_bytes.len(),
            input_sha256: input_hash,
            output_sha256: output_hash,
        };
        entries.push(entry);
    }

    let manifest = PcmManifest { entries };
    let manifest_path = args.dest.join("pcm_manifest.json");
    let json = serde_json::to_string_pretty(&manifest)?;
    fs::write(&manifest_path, json)
        .with_context(|| format!("Failed to write manifest to {}", manifest_path.display()))?;

    Ok(manifest)
}

fn run_font(args: &FontArgs) -> Result<FontManifest> {
    let bytes = fs::read(&args.input)
        .with_context(|| format!("Failed to read font file {}", args.input.display()))?;

    let (_, data) = bytes.split_at(args.skip_bytes.min(bytes.len()));
    let glyph_bits = (args.glyph_width * args.glyph_height) as usize;
    let bytes_per_glyph = (glyph_bits + 7) / 8;
    let required_bytes = bytes_per_glyph
        .checked_mul(args.glyph_count)
        .ok_or_else(|| anyhow!("Glyph parameters overflow"))?;

    if data.len() < required_bytes {
        return Err(anyhow!(
            "Font file too small (have {} bytes, need {})",
            data.len(),
            required_bytes
        ));
    }

    fs::create_dir_all(&args.dest)
        .with_context(|| format!("Failed to create {}", args.dest.display()))?;

    let base_name = args
        .name
        .clone()
        .or_else(|| {
            args.input
                .file_stem()
                .map(|stem| stem.to_string_lossy().to_string())
        })
        .unwrap_or_else(|| "font".to_string());

    let atlas_filename = format!("{}_atlas.png", base_name);
    let manifest_filename = format!("{}_font_manifest.json", base_name);
    let atlas_path = args.dest.join(&atlas_filename);
    let manifest_path = args.dest.join(&manifest_filename);

    let columns = args.columns.max(1);
    let rows = ((args.glyph_count as u32 + columns - 1) / columns).max(1);
    let mut atlas = RgbaImage::new(args.glyph_width * columns, args.glyph_height * rows);

    let on_color = parse_hex_color(&args.on_color)?;
    let off_color = parse_hex_color(&args.off_color)?;

    for glyph_index in 0..args.glyph_count {
        let glyph_offset = glyph_index * bytes_per_glyph;
        let glyph_slice = &data[glyph_offset..glyph_offset + bytes_per_glyph];
        let dest_x = (glyph_index as u32 % columns) * args.glyph_width;
        let dest_y = (glyph_index as u32 / columns) * args.glyph_height;

        for bit_idx in 0..glyph_bits {
            let byte = glyph_slice[bit_idx / 8];
            let mask = 0x80 >> (bit_idx % 8);
            let bit_set = (byte & mask) != 0;
            let is_on = if args.invert_bits { !bit_set } else { bit_set };
            let px = bit_idx as u32 % args.glyph_width;
            let py = bit_idx as u32 / args.glyph_width;
            let target = atlas.get_pixel_mut(dest_x + px, dest_y + py);
            *target = if is_on { on_color } else { off_color };
        }
    }

    atlas
        .save(&atlas_path)
        .with_context(|| format!("Failed to save atlas to {}", atlas_path.display()))?;

    let glyphs = if args.include_metadata {
        let mut placements = Vec::with_capacity(args.glyph_count);
        for glyph_index in 0..args.glyph_count {
            let dest_x = (glyph_index as u32 % columns) * args.glyph_width;
            let dest_y = (glyph_index as u32 / columns) * args.glyph_height;
            placements.push(GlyphPlacement {
                index: glyph_index,
                x: dest_x,
                y: dest_y,
            });
        }
        Some(placements)
    } else {
        None
    };

    let input_hash = sha256_hex(&bytes);
    let atlas_bytes = fs::read(&atlas_path)
        .with_context(|| format!("Failed to read atlas {}", atlas_path.display()))?;
    let atlas_hash = sha256_hex(&atlas_bytes);

    let manifest = FontManifest {
        source_file: args.input.to_string_lossy().to_string(),
        atlas_file: atlas_filename,
        glyph_width: args.glyph_width,
        glyph_height: args.glyph_height,
        glyph_count: args.glyph_count,
        columns,
        rows,
        input_sha256: input_hash,
        atlas_sha256: atlas_hash,
        glyphs,
    };

    let json = serde_json::to_string_pretty(&manifest)?;
    fs::write(&manifest_path, json)
        .with_context(|| format!("Failed to write manifest to {}", manifest_path.display()))?;

    Ok(manifest)
}

fn should_process(path: &Path, allowed: &[String]) -> bool {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => allowed.contains(&ext.to_ascii_lowercase()),
        None => false,
    }
}

fn convert_pcm_to_wav(
    data: &[u8],
    channels: u16,
    sample_rate: u32,
    output_path: &Path,
) -> Result<()> {
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }

    let spec = WavSpec {
        channels,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut writer = WavWriter::create(output_path, spec)
        .with_context(|| format!("Failed to create WAV {}", output_path.display()))?;

    for &byte in data {
        let sample = ((byte as i16) - 128) << 8;
        for _ in 0..channels {
            writer.write_sample(sample)?;
        }
    }

    writer
        .finalize()
        .with_context(|| format!("Failed to finalize WAV {}", output_path.display()))?;
    Ok(())
}

fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let digest = hasher.finalize();
    digest.iter().map(|b| format!("{:02x}", b)).collect()
}

fn parse_hex_color(value: &str) -> Result<Rgba<u8>> {
    let trimmed = value.trim_start_matches('#');
    if trimmed.len() != 6 && trimmed.len() != 8 {
        return Err(anyhow!(
            "Invalid color '{}'. Use #RRGGBB or #RRGGBBAA",
            value
        ));
    }

    let r = u8::from_str_radix(&trimmed[0..2], 16)?;
    let g = u8::from_str_radix(&trimmed[2..4], 16)?;
    let b = u8::from_str_radix(&trimmed[4..6], 16)?;
    let a = if trimmed.len() == 8 {
        u8::from_str_radix(&trimmed[6..8], 16)?
    } else {
        0xFF
    };
    Ok(Rgba([r, g, b, a]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(prefix: &str) -> PathBuf {
        let mut dir = std::env::temp_dir();
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        dir.push(format!("{}_{}", prefix, millis));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn pcm_manifest_produced_for_synthetic_data() {
        let source = temp_dir("pcm_source");
        let dest = temp_dir("pcm_dest");

        let mut file = fs::File::create(source.join("engine.pcm")).unwrap();
        for i in 0..128u8 {
            file.write_all(&[i]).unwrap();
        }

        let args = PcmArgs {
            source: source.clone(),
            dest: dest.clone(),
            sample_rate: 11025,
            channels: 1,
            extensions: vec!["pcm".to_string()],
        };

        let manifest = run_pcm(&args).unwrap();
        assert_eq!(manifest.entries.len(), 1);
        let wav_path = dest.join("engine.wav");
        assert!(wav_path.exists());

        let manifest_path = dest.join("pcm_manifest.json");
        assert!(manifest_path.exists());

        let _ = fs::remove_dir_all(source);
        let _ = fs::remove_dir_all(dest);
    }

    #[test]
    fn font_atlas_created_from_bitpacked_data() {
        let dest = temp_dir("font_dest");
        let input = dest.join("font.bin");

        // Build two 8x8 glyphs: diagonal and inverse diagonal
        let glyph_width = 8;
        let glyph_height = 8;
        let glyph_bits = (glyph_width * glyph_height) as usize;
        let bytes_per_glyph = (glyph_bits + 7) / 8;
        let mut data = vec![0u8; bytes_per_glyph * 2];

        for idx in 0..glyph_bits {
            let byte_idx = idx / 8;
            let bit_idx = idx % 8;
            if idx % (glyph_width as usize + 1) == 0 {
                data[byte_idx] |= 0x80 >> bit_idx;
            }
            let mirrored = ((idx / glyph_width as usize) * glyph_width as usize)
                + (glyph_width as usize - 1)
                - (idx % glyph_width as usize);
            let byte_idx_b = mirrored / 8 + bytes_per_glyph;
            let bit_idx_b = mirrored % 8;
            data[byte_idx_b] |= 0x80 >> bit_idx_b;
        }

        fs::write(&input, &data).unwrap();

        let args = FontArgs {
            input: input.clone(),
            dest: dest.clone(),
            glyph_width,
            glyph_height,
            glyph_count: 2,
            columns: 2,
            name: Some("test_font".into()),
            skip_bytes: 0,
            invert_bits: false,
            on_color: "#FFFFFFFF".into(),
            off_color: "#00000000".into(),
            include_metadata: true,
        };

        let manifest = run_font(&args).unwrap();
        assert_eq!(manifest.glyph_count, 2);
        assert_eq!(manifest.atlas_file, "test_font_atlas.png");
        assert!(dest.join("test_font_atlas.png").exists());
        assert!(dest.join("test_font_font_manifest.json").exists());
        assert!(manifest.glyphs.unwrap().len() == 2);

        let _ = fs::remove_dir_all(dest);
    }
}
