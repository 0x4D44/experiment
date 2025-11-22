use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct SpriteAtlas {
    atlas_file: PathBuf,
    sprites: HashMap<String, SpriteFrame>,
}

#[derive(Debug, Clone)]
pub struct SpriteFrame {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct SpriteSheet {
    pixels: Vec<u8>,
    width: u32,
    height: u32,
    cache_key: String,
}

impl SpriteAtlas {
    pub fn from_manifest<P: AsRef<Path>>(manifest_path: P) -> Result<Self> {
        let path = manifest_path.as_ref();
        let json = fs::read_to_string(path)
            .with_context(|| format!("Failed to read sprite manifest {}", path.display()))?;
        let manifest: AtlasManifest =
            serde_json::from_str(&json).context("Failed to deserialize sprite manifest")?;
        let atlas_file = path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(manifest.atlas_file);

        let sprites = manifest
            .sprites
            .into_iter()
            .map(|entry| {
                (
                    entry.name,
                    SpriteFrame {
                        x: entry.x,
                        y: entry.y,
                        width: entry.width,
                        height: entry.height,
                    },
                )
            })
            .collect();

        Ok(Self {
            atlas_file,
            sprites,
        })
    }

    pub fn frame(&self, name: &str) -> Option<&SpriteFrame> {
        self.sprites.get(name)
    }

    pub fn atlas_path(&self) -> &Path {
        self.atlas_file.as_path()
    }
}

impl SpriteSheet {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let img = image::open(path.as_ref())
            .with_context(|| format!("Failed to open sprite atlas {}", path.as_ref().display()))?
            .to_rgba8();
        let width = img.width();
        let height = img.height();
        Ok(Self {
            pixels: img.into_raw(),
            width,
            height,
            cache_key: path.as_ref().display().to_string(),
        })
    }

    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cache_key(&self) -> &str {
        &self.cache_key
    }
}

#[derive(Debug, Deserialize)]
struct AtlasManifest {
    atlas_file: String,
    sprites: Vec<ManifestSprite>,
}

#[derive(Debug, Deserialize)]
struct ManifestSprite {
    name: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn manifest_loads_frames() {
        let mut file = NamedTempFile::new().unwrap();
        let manifest = r#"{
            "atlas_file": "sprite_atlas.png",
            "sprites": [
                {"name": "car_body", "x": 0, "y": 0, "width": 64, "height": 32}
            ]
        }"#;
        file.write_all(manifest.as_bytes()).unwrap();

        let atlas = SpriteAtlas::from_manifest(file.path()).unwrap();
        let frame = atlas.frame("car_body").unwrap();
        assert_eq!(frame.width, 64);
        assert!(atlas.atlas_path().ends_with("sprite_atlas.png"));
    }
}
