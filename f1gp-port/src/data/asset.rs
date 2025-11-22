use super::objects::ObjectShape;
use super::track::{
    AIBehavior, Camera, RacingLine, Track, TrackOffsets, TrackSection, TrackSectionHeader,
};

/// Rich representation of a parsed track file, including metadata needed
/// for validation, tooling, and conversion into runtime-friendly structs.
#[derive(Debug, Clone)]
pub struct TrackAsset {
    /// Friendly name resolved from filename or user input
    pub name: String,
    /// Raw file size in bytes
    pub raw_size: usize,
    /// Stored checksum (footer bytes)
    pub checksum: u32,
    /// Computed checksum (wrapping sum of payload)
    pub computed_checksum: u32,
    /// Offset table decoded from the file
    pub offsets: TrackOffsets,
    /// Optional section header (to be populated when header parsing is complete)
    pub header: Option<TrackSectionHeader>,
    /// Parsed racing line data
    pub racing_line: RacingLine,
    /// Parsed track sections with derived geometry
    pub sections: Vec<TrackSection>,
    /// Parsed pit-lane sections (placeholder for future work)
    pub pit_lane: Vec<TrackSection>,
    /// Parsed camera definitions (placeholder for future work)
    pub cameras: Vec<Camera>,
    /// Parsed object shapes (placeholder until format is decoded)
    pub object_shapes: Vec<ObjectShape>,
    /// Absolute offset where section data begins
    pub section_data_offset: u64,
    /// Skip amount applied while scanning for sections (useful for diagnostics)
    pub section_skip_hint: u64,
}

impl TrackAsset {
    /// Convert this asset into the runtime `Track` representation.
    pub fn into_track(self) -> Track {
        Track {
            name: self.name,
            length: self.sections.iter().map(|s| s.length).sum(),
            object_shapes: self.object_shapes,
            sections: self.sections,
            racing_line: self.racing_line,
            ai_behavior: AIBehavior::default(),
            pit_lane: self.pit_lane,
            cameras: self.cameras,
            checksum: self.checksum,
        }
    }
}
