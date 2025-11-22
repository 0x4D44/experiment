//! File loading and parsing utilities

use super::parser::parse_track;
use super::track::Track;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Load raw bytes from a file
pub fn load_file_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    fs::read(&path).with_context(|| format!("Failed to read file: {}", path.as_ref().display()))
}

const CHECKSUM_TAIL_BYTES: usize = 4;

/// Compute a simple checksum by summing all payload bytes (excluding footer)
pub fn calculate_checksum(data: &[u8]) -> u32 {
    if data.len() <= CHECKSUM_TAIL_BYTES {
        return 0;
    }
    data[..data.len() - CHECKSUM_TAIL_BYTES]
        .iter()
        .fold(0u32, |acc, byte| acc.wrapping_add(*byte as u32))
}

/// Read the stored checksum from the footer bytes (if present)
pub fn read_stored_checksum(data: &[u8]) -> Option<u32> {
    if data.len() < CHECKSUM_TAIL_BYTES {
        return None;
    }
    let idx = data.len() - CHECKSUM_TAIL_BYTES;
    Some(u32::from_le_bytes([
        data[idx],
        data[idx + 1],
        data[idx + 2],
        data[idx + 3],
    ]))
}

/// Verify file checksum (wrapping sum vs. stored footer)
pub fn verify_checksum(data: &[u8]) -> bool {
    match read_stored_checksum(data) {
        Some(stored) => stored == calculate_checksum(data),
        None => false,
    }
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
        if let Some(stored) = read_stored_checksum(&data) {
            let computed = calculate_checksum(&data);
            log::warn!(
                "Track file has invalid checksum: {} (stored=0x{:08X} computed=0x{:08X})",
                track_name,
                stored,
                computed
            );
        } else {
            log::warn!("Track file has missing checksum footer: {}", track_name);
        }
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
        let mut data = vec![0x01, 0x02, 0x03, 0x04, 0, 0, 0, 0];
        let expected = 0x0Au32; // sum of first four bytes
        let tail = data.len() - 4;
        data[tail..].copy_from_slice(&expected.to_le_bytes());
        assert_eq!(calculate_checksum(&data), expected);
        assert_eq!(read_stored_checksum(&data), Some(expected));
        assert!(verify_checksum(&data));
    }

    #[test]
    fn test_verify_checksum_failure() {
        let mut data = vec![0x01, 0x02, 0x03, 0x04, 0, 0, 0, 0];
        let tail = data.len() - 4;
        data[tail..].copy_from_slice(&0xFFFF_FFFFu32.to_le_bytes());
        assert!(!verify_checksum(&data));

        let empty: Vec<u8> = vec![];
        assert!(!verify_checksum(&empty));
        assert_eq!(read_stored_checksum(&empty), None);
    }
}
