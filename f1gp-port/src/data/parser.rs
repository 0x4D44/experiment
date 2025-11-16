//! Binary parser for F1GP track files
//!
//! This module implements parsing logic for the F1GP track file format (.DAT files).
//! The format is a complex binary structure with variable-length elements.
//!
//! Based on ArgData .NET library: https://github.com/codemeyer/ArgData

use anyhow::{Context, Result, bail};
use std::io::{Cursor, Read};
use byteorder::{LittleEndian, ReadBytesExt};
use super::objects::*;
use super::track::*;

// Unit conversion constants (from ArgData)
/// Track section length: 1 unit = 16 feet ≈ 4.87 meters
const UNIT_TO_METERS: f32 = 4.87;

/// Track section terminator bytes
const SECTION_TERMINATOR: [u8; 2] = [0xFF, 0xFF];

/// Wide radius segment marker
const WIDE_RADIUS_MARKER: u8 = 0x40;

/// Binary cursor with helper methods for F1GP format
pub struct TrackParser {
    cursor: Cursor<Vec<u8>>,
    file_size: usize,
}

impl TrackParser {
    /// Create a new parser from raw bytes
    pub fn new(data: Vec<u8>) -> Self {
        let file_size = data.len();
        Self {
            cursor: Cursor::new(data),
            file_size,
        }
    }

    /// Get current position in file
    pub fn position(&self) -> u64 {
        self.cursor.position()
    }

    /// Get remaining bytes
    pub fn remaining(&self) -> usize {
        self.file_size - (self.position() as usize)
    }

    /// Seek to absolute position
    pub fn seek(&mut self, pos: u64) {
        self.cursor.set_position(pos);
    }

    /// Read a single byte
    pub fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.cursor.read_exact(&mut buf)
            .context("Failed to read u8")?;
        Ok(buf[0])
    }

    /// Read a signed byte
    pub fn read_i8(&mut self) -> Result<i8> {
        Ok(self.read_u8()? as i8)
    }

    /// Read u16 (little-endian)
    pub fn read_u16(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.cursor.read_exact(&mut buf)
            .context("Failed to read u16")?;
        Ok(u16::from_le_bytes(buf))
    }

    /// Read i16 (little-endian)
    pub fn read_i16(&mut self) -> Result<i16> {
        Ok(self.read_u16()? as i16)
    }

    /// Read u32 (little-endian)
    pub fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.cursor.read_exact(&mut buf)
            .context("Failed to read u32")?;
        Ok(u32::from_le_bytes(buf))
    }

    /// Read i32 (little-endian)
    pub fn read_i32(&mut self) -> Result<i32> {
        Ok(self.read_u32()? as i32)
    }

    /// Read f32 (little-endian)
    pub fn read_f32(&mut self) -> Result<f32> {
        let mut buf = [0u8; 4];
        self.cursor.read_exact(&mut buf)
            .context("Failed to read f32")?;
        Ok(f32::from_le_bytes(buf))
    }

    /// Peek at next byte without advancing
    pub fn peek_u8(&self) -> Result<u8> {
        let pos = self.position() as usize;
        if pos >= self.file_size {
            bail!("Unexpected end of file");
        }
        let data = self.cursor.get_ref();
        Ok(data[pos])
    }

    /// Read bytes into buffer
    pub fn read_bytes(&mut self, count: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; count];
        self.cursor.read_exact(&mut buf)
            .with_context(|| format!("Failed to read {} bytes", count))?;
        Ok(buf)
    }
}

/// Parse a single graphical element from the byte stream
pub fn parse_graphical_element(parser: &mut TrackParser) -> Result<GraphicalElement> {
    let flag = parser.read_u8()?;

    // Identify element type from flag byte
    let element_type = identify_element_type(flag);

    match element_type {
        ElementType::Line => {
            // Line: 3 bytes total (flag + 2 data bytes)
            let unknown_flag = parser.read_u8()?;
            let vector_ref = parser.read_u8()?;
            Ok(GraphicalElement::Line(Line {
                unknown_flag,
                vector_ref,
            }))
        }

        ElementType::Bitmap => {
            // Standard bitmap: 4 bytes total
            let point_ref = parser.read_u8()?;
            let unknown_flag = parser.read_u8()?;
            let bitmap_index = parser.read_u8()?;

            let bitmap_flag = match flag {
                0x80 => BitmapFlag::Type80,
                0x88 => BitmapFlag::Type88,
                0xD0 => BitmapFlag::TypeD0,
                _ => bail!("Invalid bitmap flag: 0x{:02X}", flag),
            };

            Ok(GraphicalElement::Bitmap(Bitmap {
                flag: bitmap_flag,
                point_ref,
                unknown_flag,
                bitmap_index,
            }))
        }

        ElementType::ExtendedBitmap => {
            // Extended bitmap: 6 bytes total
            let point_ref = parser.read_u8()?;
            let unknown_flag = parser.read_u8()?;
            let bitmap_index = parser.read_u8()?;
            let extra1 = parser.read_u8()?;
            let extra2 = parser.read_u8()?;

            let extended_flag = match flag {
                0x82 => ExtendedBitmapFlag::Type82,
                0x86 => ExtendedBitmapFlag::Type86,
                _ => bail!("Invalid extended bitmap flag: 0x{:02X}", flag),
            };

            Ok(GraphicalElement::ExtendedBitmap(ExtendedBitmap {
                flag: extended_flag,
                point_ref,
                unknown_flag,
                bitmap_index,
                extra1,
                extra2,
            }))
        }

        ElementType::Polygon => {
            // Polygon: [color byte] [side1] ... [sideN] [0x00]
            let color = flag;  // First byte is the color
            let mut sides = Vec::new();

            // Read sides until we hit 0x00 terminator
            loop {
                let side = parser.read_i8()?;
                if side == 0 {
                    break;  // End of polygon
                }
                sides.push(side);

                // Safety check: polygons have 3-12 sides
                if sides.len() > 12 {
                    bail!("Polygon has too many sides: {}", sides.len());
                }
            }

            // Validate polygon
            if sides.len() < 3 {
                bail!("Polygon has too few sides: {}", sides.len());
            }

            Ok(GraphicalElement::Polygon(Polygon { color, sides }))
        }
    }
}

/// Parse track sections from binary data
///
/// Sections are read sequentially until terminator (0xFF 0xFF)
/// Based on ArgData TrackSectionReader.cs
pub fn parse_track_sections(parser: &mut TrackParser) -> Result<Vec<TrackSection>> {
    let mut sections = Vec::new();
    let mut pending_commands = Vec::new();

    loop {
        let byte1 = parser.read_u8()?;
        let byte2 = parser.read_u8()?;

        // Check for terminator
        if byte1 == SECTION_TERMINATOR[0] && byte2 == SECTION_TERMINATOR[1] {
            break;
        }

        if byte2 > 0 {
            // Command: byte2 is command ID, byte1 is first arg
            let command = parse_track_command(parser, byte2, byte1)?;
            pending_commands.push(command);
        } else {
            // Section data: byte2 == 0, byte1 is length
            let length_raw = byte1;
            let curvature = parser.read_i16()?;
            let height = parser.read_i16()?;
            let flags = parser.read_u16()?;
            let right_verge_width = parser.read_u8()?;
            let left_verge_width = parser.read_u8()?;

            // Parse flags bitfield
            let (has_left_kerb, has_right_kerb, kerb_height, pit_lane_entrance,
                 pit_lane_exit, road_signs, road_sign_arrow) = parse_section_flags(flags);

            let section = TrackSection {
                length: length_raw as f32 * UNIT_TO_METERS,
                curvature,
                height,
                flags,
                right_verge_width,
                left_verge_width,
                commands: std::mem::take(&mut pending_commands),
                has_left_kerb,
                has_right_kerb,
                kerb_height,
                pit_lane_entrance,
                pit_lane_exit,
                road_signs,
                road_sign_arrow,
                ..Default::default()
            };

            sections.push(section);
        }
    }

    Ok(sections)
}

/// Parse a track section command
fn parse_track_command(parser: &mut TrackParser, command_id: u8, first_arg: u8) -> Result<TrackSectionCommand> {
    // Commands have variable number of int16 arguments based on command ID
    // Source: ArgData TrackSectionCommandFactory.cs
    let mut args = vec![first_arg as i16];

    let arg_count = match command_id {
        0x80 | 0x81 | 0x82 => 2,
        0x83 | 0x84 | 0x86 | 0x87 => 1,
        0x85 => 3,
        0x88 | 0x89 | 0x8c | 0x8d | 0x90..=0x95 | 0x98 | 0x99 | 0xa9 => 2,
        0x8a | 0x8b => 6,
        0x8e | 0x8f | 0x9a | 0xa6 | 0xa7 | 0xab => 3,
        0x96 | 0x97 | 0x9b..=0xa5 | 0xa8 => 1,
        0xaa => 4,
        0xac => 5,
        // Commands below 0x80 are not documented in ArgData
        // Default to 0 additional args (just the first_arg)
        _ => 0,
    };

    for _ in 0..arg_count {
        args.push(parser.read_i16()?);
    }

    Ok(TrackSectionCommand {
        command_id,
        args,
    })
}

/// Parse section flags bitfield
///
/// TODO: Determine exact bit positions for each flag
/// This is a placeholder that needs refinement
fn parse_section_flags(flags: u16) -> (bool, bool, KerbHeight, bool, bool, bool, bool) {
    // Bit positions (to be determined from real data or ArgData source)
    let has_left_kerb = (flags & 0x0001) != 0;
    let has_right_kerb = (flags & 0x0002) != 0;
    let kerb_height_high = (flags & 0x0004) != 0;
    let pit_lane_entrance = (flags & 0x0008) != 0;
    let pit_lane_exit = (flags & 0x0010) != 0;
    let road_signs = (flags & 0x0020) != 0;
    let road_sign_arrow = (flags & 0x0040) != 0;

    let kerb_height = if kerb_height_high {
        KerbHeight::High
    } else {
        KerbHeight::Low
    };

    (has_left_kerb, has_right_kerb, kerb_height, pit_lane_entrance,
     pit_lane_exit, road_signs, road_sign_arrow)
}

/// Parse racing line from binary data
///
/// Based on ArgData ComputerCarLineReader.cs
pub fn parse_racing_line(parser: &mut TrackParser) -> Result<RacingLine> {
    // First segment has special format
    let first_length = parser.read_u8()?;
    let displacement = parser.read_i16()?;
    let correction = parser.read_i16()?;
    let radius = parser.read_i16()?;

    let mut segments = vec![RacingLineSegment {
        length: first_length,
        correction,
        segment_type: SegmentType::Normal { radius },
    }];

    // Subsequent segments
    loop {
        let length = parser.read_u8()?;
        let type_byte = parser.read_u8()?;
        let correction = parser.read_i16()?;

        let segment_type = if type_byte == WIDE_RADIUS_MARKER {
            // Wide radius segment
            let high_radius = parser.read_i16()?;
            let low_radius = parser.read_i16()?;
            SegmentType::WideRadius { high_radius, low_radius }
        } else {
            // Normal segment
            let radius = parser.read_i16()?;
            SegmentType::Normal { radius }
        };

        segments.push(RacingLineSegment {
            length,
            correction,
            segment_type,
        });

        // Check for terminator (int16 == 0)
        let pos = parser.position();
        let next = parser.read_i16()?;
        if next == 0 {
            break;  // End of racing line
        } else {
            // Rewind and continue
            parser.seek(pos);
        }
    }

    Ok(RacingLine {
        displacement,
        segments,
    })
}

/// Parse object shapes from the track file
///
/// According to ArgDocs, object shapes directory starts at 0x100E
pub fn parse_object_shapes(_parser: &mut TrackParser) -> Result<Vec<ObjectShape>> {
    // For now, return empty vec - will implement once we understand the format better
    // This is a placeholder for the full implementation
    Ok(Vec::new())
}

/// Parse a complete track file
///
/// This is the main entry point for parsing .DAT files
pub fn parse_track(data: Vec<u8>, name: String) -> Result<Track> {
    let mut parser = TrackParser::new(data);

    // Basic validation
    if parser.file_size < 4 {
        bail!("File too small to be a valid track file");
    }

    // Read checksum from last 4 bytes
    parser.seek((parser.file_size - 4) as u64);
    let checksum = parser.read_u32()?;

    // Reset to beginning
    parser.seek(0);

    // Skip first 4096 bytes (unused padding)
    if parser.file_size > 4096 {
        parser.seek(4096);
    }

    // TODO: Parse offsets section at 0x1000 to find section locations
    // For now, try several candidate offsets and pick the one that works best
    // Different tracks seem to have sections at different offsets

    // Try a wide range of offsets since different tracks have sections at different positions
    let candidate_offsets = vec![
        0x4000, 0x3C00, 0x3800, 0x3400, 0x3000, 0x2C00, 0x2800,
        0x3200, 0x3600, 0x3A00, 0x3E00, 0x4200, 0x4400, 0x4600,
        0x4800, 0x4A00, 0x4C00, 0x2400, 0x2000, 0x2200, 0x2600,
    ];
    let mut best_sections = Vec::new();
    let mut best_total_length = 0.0;

    for &offset in &candidate_offsets {
        if offset >= parser.file_size {
            continue;
        }

        parser.seek(offset as u64);

        // Try to parse sections from this offset
        match parse_track_sections(&mut parser) {
            Ok(sections) if !sections.is_empty() => {
                let total_length: f32 = sections.iter().map(|s| s.length).sum();

                // Monaco should be ~3340m, other tracks 3000-7000m range
                // Prefer results that give reasonable track lengths
                if total_length > 2000.0 && total_length < 8000.0 {
                    if sections.len() > best_sections.len() {
                        log::info!("Found {} sections at offset 0x{:04X}, total length: {:.0}m",
                                   sections.len(), offset, total_length);
                        best_sections = sections;
                        best_total_length = total_length;
                    }
                }
            }
            _ => continue,
        }
    }

    let sections = best_sections;
    let track_length = best_total_length;

    log::info!("Parsed track '{}': {} sections, {:.2}km", name, sections.len(), track_length / 1000.0);

    // For now, skip racing line parsing (need to find its offset)
    let racing_line = RacingLine {
        displacement: 0,
        segments: Vec::new(),
    };

    let track = Track {
        name,
        length: track_length,
        object_shapes: Vec::new(),
        sections,
        racing_line,
        ai_behavior: AIBehavior::default(),
        pit_lane: Vec::new(),
        cameras: Vec::new(),
        checksum,
    };

    Ok(track)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_basic_reads() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut parser = TrackParser::new(data);

        assert_eq!(parser.position(), 0);
        assert_eq!(parser.remaining(), 8);

        assert_eq!(parser.read_u8().unwrap(), 0x01);
        assert_eq!(parser.position(), 1);
        assert_eq!(parser.remaining(), 7);
    }

    #[test]
    fn test_parser_u16() {
        let data = vec![0xAA, 0xBB];  // Little-endian
        let mut parser = TrackParser::new(data);

        let value = parser.read_u16().unwrap();
        assert_eq!(value, 0xBBAA);
    }

    #[test]
    fn test_parser_u32() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let mut parser = TrackParser::new(data);

        let value = parser.read_u32().unwrap();
        assert_eq!(value, 0x04030201);
    }

    #[test]
    fn test_parser_peek() {
        let data = vec![0x42, 0x43];
        let parser = TrackParser::new(data);

        assert_eq!(parser.peek_u8().unwrap(), 0x42);
        assert_eq!(parser.position(), 0);  // Position unchanged
    }

    #[test]
    fn test_parse_line_element() {
        // Line: [0xA0] [unknown_flag] [vector_ref]
        let data = vec![0xA0, 0x01, 0x05];
        let mut parser = TrackParser::new(data);

        let element = parse_graphical_element(&mut parser).unwrap();

        if let GraphicalElement::Line(line) = element {
            assert_eq!(line.unknown_flag, 0x01);
            assert_eq!(line.vector_ref, 0x05);
        } else {
            panic!("Expected Line element");
        }
    }

    #[test]
    fn test_parse_bitmap_element() {
        // Bitmap: [flag] [point_ref] [unknown] [bitmap_index]
        let data = vec![0x80, 0x03, 0x00, 0x12];
        let mut parser = TrackParser::new(data);

        let element = parse_graphical_element(&mut parser).unwrap();

        if let GraphicalElement::Bitmap(bitmap) = element {
            assert_eq!(bitmap.point_ref, 0x03);
            assert_eq!(bitmap.bitmap_index, 0x12);
        } else {
            panic!("Expected Bitmap element");
        }
    }

    #[test]
    fn test_parse_polygon_element() {
        // Polygon: [color] [side1] [side2] [side3] [0x00]
        let data = vec![0x1F, 0x01, 0x02, 0x03, 0x00];
        let mut parser = TrackParser::new(data);

        let element = parse_graphical_element(&mut parser).unwrap();

        if let GraphicalElement::Polygon(polygon) = element {
            assert_eq!(polygon.color, 0x1F);
            assert_eq!(polygon.sides.len(), 3);
            assert_eq!(polygon.sides, vec![1, 2, 3]);
        } else {
            panic!("Expected Polygon element");
        }
    }

    #[test]
    fn test_parse_track_basic() {
        // Minimal valid track file (just checksum at end)
        let mut data = vec![0; 100];
        data.extend_from_slice(&[0xAA, 0xBB, 0xCC, 0xDD]);  // Checksum

        let track = parse_track(data, "Test Track".to_string()).unwrap();

        assert_eq!(track.name, "Test Track");
        assert_eq!(track.checksum, 0xDDCCBBAA);
    }
}
