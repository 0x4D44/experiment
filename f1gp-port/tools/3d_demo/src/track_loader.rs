//! Real F1GP track loader
//!
//! Loads authentic F1GP .DAT track files and provides them to the 3D demo.

use anyhow::Result;
use f1gp_port::data::Track;
use f1gp_port::parse_track;
use std::fs;

/// Track metadata
pub struct TrackInfo {
    pub filename: &'static str,
    pub name: &'static str,
    pub real_length_m: f32, // Real-world track length for reference
}

/// All 16 F1GP tracks with metadata
const TRACKS: &[TrackInfo] = &[
    TrackInfo {
        filename: "F1CT01.DAT",
        name: "Phoenix",
        real_length_m: 3720.0,
    },
    TrackInfo {
        filename: "F1CT02.DAT",
        name: "Interlagos",
        real_length_m: 4292.0,
    },
    TrackInfo {
        filename: "F1CT03.DAT",
        name: "Imola",
        real_length_m: 4895.0,
    },
    TrackInfo {
        filename: "F1CT04.DAT",
        name: "Monaco",
        real_length_m: 3340.0,
    },
    TrackInfo {
        filename: "F1CT05.DAT",
        name: "Montreal",
        real_length_m: 4361.0,
    },
    TrackInfo {
        filename: "F1CT06.DAT",
        name: "Mexico",
        real_length_m: 4421.0,
    },
    TrackInfo {
        filename: "F1CT07.DAT",
        name: "Magny-Cours",
        real_length_m: 4411.0,
    },
    TrackInfo {
        filename: "F1CT08.DAT",
        name: "Silverstone",
        real_length_m: 5226.0,
    },
    TrackInfo {
        filename: "F1CT09.DAT",
        name: "Hockenheim",
        real_length_m: 6823.0,
    },
    TrackInfo {
        filename: "F1CT10.DAT",
        name: "Hungaroring",
        real_length_m: 4381.0,
    },
    TrackInfo {
        filename: "F1CT11.DAT",
        name: "Spa-Francorchamps",
        real_length_m: 6940.0,
    },
    TrackInfo {
        filename: "F1CT12.DAT",
        name: "Monza",
        real_length_m: 5770.0,
    },
    TrackInfo {
        filename: "F1CT13.DAT",
        name: "Estoril",
        real_length_m: 4350.0,
    },
    TrackInfo {
        filename: "F1CT14.DAT",
        name: "Barcelona",
        real_length_m: 4730.0,
    },
    TrackInfo {
        filename: "F1CT15.DAT",
        name: "Suzuka",
        real_length_m: 5864.0,
    },
    TrackInfo {
        filename: "F1CT16.DAT",
        name: "Adelaide",
        real_length_m: 3780.0,
    },
];

/// Get the number of available tracks
pub fn get_track_count() -> usize {
    TRACKS.len()
}

/// Load a track by index
pub fn get_track(index: usize) -> Option<Track> {
    if index >= TRACKS.len() {
        log::error!("Invalid track index: {}", index);
        return None;
    }

    let track_info = &TRACKS[index];
    let path = format!("../../assets/original/{}", track_info.filename);

    log::info!("Loading track: {} from {}", track_info.name, path);

    // Read binary file
    let data = match fs::read(&path) {
        Ok(data) => data,
        Err(e) => {
            log::error!("Failed to read track file {}: {}", path, e);
            return None;
        }
    };

    // Parse track
    match parse_track(data, track_info.name.to_string()) {
        Ok(track) => {
            log::info!(
                "Loaded track '{}': {} sections, {:.2}km",
                track.name,
                track.sections.len(),
                track.length / 1000.0
            );

            // Warn if Montreal (known issue)
            if track.sections.is_empty() {
                log::warn!(
                    "Track '{}' has no sections! Parser may have failed.",
                    track.name
                );
                return None;
            }

            Some(track)
        }
        Err(e) => {
            log::error!("Failed to parse track {}: {}", track_info.name, e);
            None
        }
    }
}

/// Get track info without loading
pub fn get_track_info(index: usize) -> Option<&'static TrackInfo> {
    TRACKS.get(index)
}

/// Get all track names for display
pub fn get_track_names() -> Vec<&'static str> {
    TRACKS.iter().map(|t| t.name).collect()
}
