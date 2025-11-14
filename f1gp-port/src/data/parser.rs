//! Binary parser for F1GP track files
//!
//! This module implements parsing logic for the F1GP track file format (.DAT files).
//! The format is a complex binary structure with variable-length elements.

use anyhow::{Context, Result, bail};
use std::io::{Cursor, Read};
use super::objects::*;
use super::track::*;

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

/// Parse object shapes from the track file
///
/// According to ArgDocs, object shapes directory starts at 0x100E
pub fn parse_object_shapes(_parser: &mut TrackParser) -> Result<Vec<ObjectShape>> {
    // For now, return empty vec - will implement once we understand the format better
    // This is a placeholder for the full implementation
    Ok(Vec::new())
}

/// Parse a complete track file
pub fn parse_track(data: Vec<u8>, name: String) -> Result<Track> {
    let mut parser = TrackParser::new(data);

    // Basic validation
    if parser.file_size < 4 {
        bail!("File too small to be a valid track file");
    }

    // Read checksum from last 4 bytes
    parser.seek((parser.file_size - 4) as u64);
    let checksum = parser.read_u32()?;

    // TODO: Parse actual track data
    // For now, create a minimal track structure
    let track = Track {
        name,
        length: 0.0,
        object_shapes: Vec::new(),
        sections: Vec::new(),
        racing_line: RacingLine { points: Vec::new() },
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
