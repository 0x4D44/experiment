use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use image::{ImageBuffer, Rgba, RgbaImage};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about = "Pack sanitized sprites into a texture atlas")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Pack PNG sprites into an atlas
    Pack(PackArgs),
    /// Generate placeholder sprites for CI/tests
    GenerateFixtures(FixtureArgs),
}

#[derive(Args, Debug)]
struct PackArgs {
    #[arg(long)]
    source: PathBuf,
    #[arg(long)]
    dest: PathBuf,
    #[arg(long, default_value_t = 1024)]
    width: u32,
    #[arg(long, default_value_t = 1024)]
    height: u32,
    #[arg(long, default_value_t = 2)]
    padding: u32,
}

#[derive(Args, Debug)]
struct FixtureArgs {
    #[arg(long)]
    dest: PathBuf,
}

#[derive(Debug, Serialize)]
struct SpriteEntry {
    name: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[derive(Debug, Serialize)]
struct AtlasManifest {
    atlas_file: String,
    sprites: Vec<SpriteEntry>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Pack(args) => pack(args),
        Command::GenerateFixtures(args) => generate_fixtures(args),
    }
}

fn pack(args: PackArgs) -> Result<()> {
    fs::create_dir_all(&args.dest)
        .with_context(|| format!("Failed to create {}", args.dest.display()))?;
    let mut atlas = RgbaImage::new(args.width, args.height);
    let mut cursor_x = 0;
    let mut cursor_y = 0;
    let mut row_height = 0;
    let mut entries = Vec::new();

    for entry in WalkDir::new(&args.source)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext.eq_ignore_ascii_case("png"))
                .unwrap_or(false)
        })
    {
        let path = entry.into_path();
        let img = image::open(&path)
            .with_context(|| format!("Failed to open {}", path.display()))?
            .to_rgba8();
        let (w, h) = img.dimensions();

        if w > args.width || h > args.height {
            anyhow::bail!("Sprite {} exceeds atlas size", path.display());
        }

        if cursor_x + w > args.width {
            cursor_x = 0;
            cursor_y += row_height + args.padding;
            row_height = 0;
        }
        if cursor_y + h > args.height {
            anyhow::bail!("Atlas full before placing {}", path.display());
        }

        image::imageops::overlay(&mut atlas, &img, cursor_x as i64, cursor_y as i64);
        entries.push(SpriteEntry {
            name: sprite_name(&path, &args.source),
            x: cursor_x,
            y: cursor_y,
            width: w,
            height: h,
        });

        cursor_x += w + args.padding;
        row_height = row_height.max(h);
    }

    let atlas_path = args.dest.join("sprite_atlas.png");
    atlas
        .save(&atlas_path)
        .with_context(|| format!("Failed to save atlas {}", atlas_path.display()))?;

    let manifest = AtlasManifest {
        atlas_file: atlas_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string(),
        sprites: entries,
    };
    let manifest_path = args.dest.join("sprite_manifest.json");
    let json = serde_json::to_string_pretty(&manifest)?;
    fs::write(&manifest_path, json)
        .with_context(|| format!("Failed to write manifest {}", manifest_path.display()))?;

    println!(
        "Packed {} sprites into {}",
        manifest.sprites.len(),
        atlas_path.display()
    );
    Ok(())
}

fn sprite_name(path: &Path, root: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .with_extension("")
        .to_string_lossy()
        .replace(std::path::MAIN_SEPARATOR, "/")
}

fn generate_fixtures(args: FixtureArgs) -> Result<()> {
    fs::create_dir_all(&args.dest)
        .with_context(|| format!("Failed to create {}", args.dest.display()))?;
    let samples = vec![
        ("car_body", Rgba([255, 0, 0, 255])),
        ("wheel", Rgba([0, 0, 0, 255])),
        ("track_tile", Rgba([200, 200, 200, 255])),
    ];
    for (name, color) in samples {
        let path = args.dest.join(format!("{}.png", name));
        write_colored_sprite(&path, color)?;
        println!("Generated {}", path.display());
    }
    Ok(())
}

fn write_colored_sprite(path: &Path, color: Rgba<u8>) -> Result<()> {
    let size = 32;
    let mut img: RgbaImage = ImageBuffer::new(size, size);
    for pixel in img.pixels_mut() {
        *pixel = color;
    }
    img.save(path)
        .with_context(|| format!("Failed to save {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn fixtures_generate_files() {
        let dir = tempdir().unwrap();
        let args = FixtureArgs {
            dest: dir.path().to_path_buf(),
        };
        generate_fixtures(args).unwrap();
        assert!(dir.path().join("car_body.png").exists());
    }
}
