//! File loading and parsing utilities

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

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
