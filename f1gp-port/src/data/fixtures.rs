//! Fixture helpers for generating synthetic data (used in tests/CI) without shipping
//! proprietary assets.

use super::loader;
use super::track::{AIBehavior, RacingLine, Track, TrackSection};
use anyhow::{Context, Result};
use glam::Vec3;
use std::fs;
use std::path::Path;

pub const DEFAULT_TRACK_DATA_OFFSET: usize = 0x1100;
pub const DEFAULT_SECTION_COUNT: usize = 15;
const SECTION_BYTES: usize = 10;
const HEADER_BYTES: usize = 25;

/// Generate a simple synthetic track with the provided number of sections.
pub fn synthetic_track(section_count: usize, length_meters: f32) -> Track {
    let mut sections = Vec::new();
    let section_length = length_meters / section_count as f32;
    for i in 0..section_count {
        sections.push(TrackSection {
            position: Vec3::new(i as f32 * section_length, 0.0, 0.0),
            length: section_length,
            ..TrackSection::default()
        });
    }

    Track {
        name: "Synthetic Track".to_string(),
        length: length_meters,
        object_shapes: Vec::new(),
        sections,
        racing_line: RacingLine {
            displacement: 0,
            segments: Vec::new(),
        },
        ai_behavior: AIBehavior::default(),
        pit_lane: Vec::new(),
        cameras: Vec::new(),
        checksum: 0,
    }
}

/// Build synthetic track bytes that resemble a real DAT file enough for parser tests.
pub fn synthetic_track_bytes(section_count: usize, track_data_offset: usize) -> Vec<u8> {
    let section_block_size = HEADER_BYTES + section_count * SECTION_BYTES + 2; // header + terminator
    let total_size = track_data_offset + section_block_size + 4;
    let mut data = vec![0u8; total_size];

    // Populate offset table at 0x1000
    let track_data_relative = (track_data_offset as i32 - 0x1010) as i16;
    let mut cursor = 0x1000;
    for value in [0i16, 0i16, 0i16, 0i16, 0i16, 0i16, track_data_relative] {
        let bytes = value.to_le_bytes();
        data[cursor..cursor + 2].copy_from_slice(&bytes);
        cursor += 2;
    }

    // Advance past synthetic header bytes
    let mut section_cursor = track_data_offset + HEADER_BYTES;
    for _ in 0..section_count {
        data[section_cursor] = 50;
        data[section_cursor + 1] = 0;
        data[section_cursor + 2..section_cursor + 4].copy_from_slice(&0i16.to_le_bytes());
        data[section_cursor + 4..section_cursor + 6].copy_from_slice(&0i16.to_le_bytes());
        data[section_cursor + 6..section_cursor + 8].copy_from_slice(&0u16.to_le_bytes());
        data[section_cursor + 8] = 0;
        data[section_cursor + 9] = 0;
        section_cursor += SECTION_BYTES;
    }

    // Terminator
    data[section_cursor] = 0xFF;
    data[section_cursor + 1] = 0xFF;

    // Append checksum footer
    let checksum = loader::calculate_checksum(&data);
    let len = data.len();
    data[len - 4..].copy_from_slice(&checksum.to_le_bytes());

    data
}

/// Write default synthetic track fixture to disk.
pub fn write_synthetic_track_fixture<P: AsRef<Path>>(path: P) -> Result<()> {
    let bytes = synthetic_track_bytes(DEFAULT_SECTION_COUNT, DEFAULT_TRACK_DATA_OFFSET);
    let path_ref = path.as_ref();
    if let Some(parent) = path_ref.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }
    fs::write(path_ref, &bytes)
        .with_context(|| format!("Failed to write fixture to {}", path_ref.display()))
}
