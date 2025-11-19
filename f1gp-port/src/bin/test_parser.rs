#!/usr/bin/env rust
//! Test the track parser with real F1GP track files

use anyhow::Result;
use f1gp_port::*;
use std::fs;

fn main() -> Result<()> {
    env_logger::init();

    println!("F1GP Track Parser Test\n");
    println!("======================\n");

    // Test Monaco (F1CT04.DAT)
    let monaco_path = "assets/original/F1CT04.DAT";
    println!("Loading Monaco track from: {}", monaco_path);

    let data = fs::read(monaco_path)?;
    println!("File size: {} bytes", data.len());

    // Parse the track
    let track = parse_track(data, "Monaco".to_string())?;

    println!("\n--- Track Info ---");
    println!("Name: {}", track.name);
    println!("Checksum: 0x{:08X}", track.checksum);
    println!("Sections: {}", track.sections.len());
    println!("Racing line segments: {}", track.racing_line.segments.len());
    println!("Racing line displacement: {}", track.racing_line.displacement);

    // Show first few sections
    println!("\n--- First 5 Sections ---");
    for (i, section) in track.sections.iter().take(5).enumerate() {
        println!("Section {}: len={:.1}m, curv={}, height={}, flags=0x{:04X}",
                 i, section.length, section.curvature, section.height, section.flags);
        if !section.commands.is_empty() {
            println!("  Commands: {} command(s)", section.commands.len());
        }
        if section.has_left_kerb || section.has_right_kerb {
            println!("  Kerbs: L={} R={}", section.has_left_kerb, section.has_right_kerb);
        }
    }

    // Show racing line info
    println!("\n--- Racing Line ---");
    for (i, segment) in track.racing_line.segments.iter().take(5).enumerate() {
        match &segment.segment_type {
            SegmentType::Normal { radius } => {
                println!("Segment {}: len={}, corr={}, radius={}",
                         i, segment.length, segment.correction, radius);
            }
            SegmentType::WideRadius { high_radius, low_radius } => {
                println!("Segment {}: len={}, corr={}, high={}, low={}",
                         i, segment.length, segment.correction, high_radius, low_radius);
            }
        }
    }

    // Calculate total track length
    let total_length: f32 = track.sections.iter().map(|s| s.length).sum();
    println!("\n--- Track Metrics ---");
    println!("Calculated length: {:.0}m ({:.2}km)", total_length, total_length / 1000.0);
    println!("Expected Monaco: ~3,340m (3.34km)");

    // Test all 16 tracks
    println!("\n\n=== Testing All 16 Tracks ===\n");

    let tracks = [
        ("F1CT01.DAT", "Phoenix"),
        ("F1CT02.DAT", "Interlagos"),
        ("F1CT03.DAT", "Imola"),
        ("F1CT04.DAT", "Monaco"),
        ("F1CT05.DAT", "Montreal"),
        ("F1CT06.DAT", "Mexico"),
        ("F1CT07.DAT", "Magny-Cours"),
        ("F1CT08.DAT", "Silverstone"),
        ("F1CT09.DAT", "Hockenheim"),
        ("F1CT10.DAT", "Hungaroring"),
        ("F1CT11.DAT", "Spa-Francorchamps"),
        ("F1CT12.DAT", "Monza"),
        ("F1CT13.DAT", "Estoril"),
        ("F1CT14.DAT", "Barcelona"),
        ("F1CT15.DAT", "Suzuka"),
        ("F1CT16.DAT", "Adelaide"),
    ];

    for (filename, name) in tracks.iter() {
        let path = format!("assets/original/{}", filename);
        match fs::read(&path) {
            Ok(data) => {
                let file_size = data.len();
                match parse_track(data, name.to_string()) {
                    Ok(track) => {
                        let length: f32 = track.sections.iter().map(|s| s.length).sum();
                        println!("{:20} {:>6} bytes  {:>4} sections  {:>6.0}m  checksum: 0x{:08X}",
                                 name, file_size, track.sections.len(), length, track.checksum);
                    }
                    Err(e) => {
                        println!("{:20} PARSE ERROR: {}", name, e);
                    }
                }
            }
            Err(e) => {
                println!("{:20} FILE ERROR: {}", name, e);
            }
        }
    }

    Ok(())
}
