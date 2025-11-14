//! File loading and parsing utilities

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use super::parser::parse_track;
use super::track::Track;

/// Load raw bytes from a file
pub fn load_file_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    fs::read(&path).with_context(|| {
        format!("Failed to read file: {}", path.as_ref().display())
    })
}

/// Calculate F1GP checksum
///
/// The checksum is stored in the last 4 bytes of the file.
/// Algorithm TBD - needs reverse engineering.
pub fn calculate_checksum(data: &[u8]) -> u32 {
    // TODO: Implement actual F1GP checksum algorithm
    // For now, just read the existing checksum from file
    if data.len() >= 4 {
        let idx = data.len() - 4;
        u32::from_le_bytes([data[idx], data[idx + 1], data[idx + 2], data[idx + 3]])
    } else {
        0
    }
}

/// Verify file checksum
pub fn verify_checksum(data: &[u8]) -> bool {
    if data.len() < 4 {
        return false;
    }

    let stored = calculate_checksum(data);
    // TODO: Calculate actual checksum and compare
    // For now, just return true if checksum exists
    stored != 0
}

/// Load and parse a complete track file
///
/// Reads a track file (F1CT*.DAT), parses it, and returns a Track structure.
///
/// # Arguments
/// * `path` - Path to the track file
/// * `name` - Track name (optional, defaults to filename without extension)
///
/// # Example
/// ```no_run
/// use f1gp_port::load_track;
/// let track = load_track("assets/original/HARDDISK/F1CT01.DAT", Some("Monaco".to_string())).unwrap();
/// ```
pub fn load_track<P: AsRef<Path>>(path: P, name: Option<String>) -> Result<Track> {
    let path_ref = path.as_ref();

    // Determine track name
    let track_name = name.unwrap_or_else(|| {
        path_ref
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string()
    });

    // Load file bytes
    let data = load_file_bytes(path_ref)?;

    // Verify checksum (basic validation)
    if !verify_checksum(&data) {
        log::warn!("Track file has invalid or missing checksum: {}", track_name);
    }

    // Parse track
    let track = parse_track(data, track_name)?;

    // Validate parsed data
    // Note: Currently we don't have enough data to validate fully
    // track.validate()?;

    log::info!("Loaded track: {} ({} bytes)", track.name, track.checksum);

    Ok(track)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum_calculation() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0xAA, 0xBB, 0xCC, 0xDD];
        let checksum = calculate_checksum(&data);
        assert_eq!(checksum, 0xDDCCBBAA); // Little endian
    }

    #[test]
    fn test_verify_checksum() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        assert!(verify_checksum(&data));

        let empty: Vec<u8> = vec![];
        assert!(!verify_checksum(&empty));
    }
}
