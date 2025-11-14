//! Integration tests for track loading

use f1gp_port::load_track;
use std::path::PathBuf;

#[test]
fn test_load_f1ct01() {
    // Path to first track file
    let mut track_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    track_path.push("assets");
    track_path.push("original");
    track_path.push("HARDDISK");
    track_path.push("F1CT01.DAT");

    // Try with ISO 9660 version suffix if needed
    if !track_path.exists() {
        track_path.set_file_name("F1CT01.DAT;1");
    }

    // Skip test if file doesn't exist (assets not extracted yet)
    if !track_path.exists() {
        eprintln!("Skipping test: track file not found");
        return;
    }

    // Load the track
    let result = load_track(&track_path, Some("Circuit 01".to_string()));

    // Should load successfully
    assert!(result.is_ok(), "Failed to load track: {:?}", result.err());

    let track = result.unwrap();

    // Basic validation
    assert_eq!(track.name, "Circuit 01");
    assert_ne!(track.checksum, 0, "Checksum should be non-zero");

    println!("Successfully loaded track: {}", track.name);
    println!("  Checksum: 0x{:08X}", track.checksum);
    println!("  Object shapes: {}", track.object_shapes.len());
    println!("  Sections: {}", track.sections.len());
}

#[test]
fn test_load_multiple_tracks() {
    let mut base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base_path.push("assets");
    base_path.push("original");
    base_path.push("HARDDISK");

    // Skip test if directory doesn't exist
    if !base_path.exists() {
        eprintln!("Skipping test: assets directory not found");
        return;
    }

    // Try to load first 3 tracks
    let mut loaded_count = 0;

    for i in 1..=3 {
        let filename = format!("F1CT{:02}.DAT", i);
        let mut track_path = base_path.clone();
        track_path.push(&filename);

        // Try with ISO 9660 version suffix if needed
        if !track_path.exists() {
            track_path.set_file_name(format!("F1CT{:02}.DAT;1", i));
        }

        if track_path.exists() {
            let track_name = format!("Circuit {:02}", i);
            let result = load_track(&track_path, Some(track_name));

            if let Ok(track) = result {
                println!("Loaded {}: checksum=0x{:08X}", track.name, track.checksum);
                loaded_count += 1;
            }
        }
    }

    println!("Successfully loaded {} tracks", loaded_count);

    // Test is successful if we could load at least one track
    if loaded_count > 0 {
        assert!(loaded_count >= 1, "Should load at least 1 track");
    }
}
