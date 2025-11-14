//! Track object shapes and graphical elements
//!
//! Based on ArgDocs specification:
//! - Object shapes directory at 0x100E
//! - First object offset at 0x1010
//! - Three types: Polygons, Bitmaps, Lines

use serde::{Deserialize, Serialize};

/// An object shape in the track file
///
/// Object shapes contain visual and geometric data for trackside objects
/// like buildings, trees, barriers, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectShape {
    /// Scale values for this object
    pub scale_data: Vec<u8>,

    /// Graphical elements (polygons, bitmaps, lines)
    pub elements: Vec<GraphicalElement>,

    /// Point data (vertices)
    pub points: Vec<Point3D>,

    /// Vector data
    pub vectors: Vec<Vector3D>,
}

/// A 3D point
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point3D {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

/// A 3D vector
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vector3D {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

/// Graphical element types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GraphicalElement {
    /// Polygon with color and sides
    Polygon(Polygon),

    /// Standard bitmap (4 bytes)
    Bitmap(Bitmap),

    /// Extended bitmap (6 bytes)
    ExtendedBitmap(ExtendedBitmap),

    /// Line (3 bytes)
    Line(Line),
}

/// Polygon definition
///
/// Polygons can have 3-12 sides, terminated by 0x00
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Polygon {
    /// Color index
    pub color: u8,

    /// Side definitions
    /// Positive value = start point reference
    /// Negative value = end point reference
    pub sides: Vec<i8>,
}

/// Standard bitmap (4 bytes total)
///
/// Format: [flag_byte, point_ref, unknown_flag, bitmap_index]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bitmap {
    /// Flag byte (0x80, 0x88, or 0xD0)
    pub flag: BitmapFlag,

    /// Reference to a point
    pub point_ref: u8,

    /// Unknown flag (often 0xFF)
    pub unknown_flag: u8,

    /// Which bitmap image to use
    pub bitmap_index: u8,
}

/// Extended bitmap (6 bytes total)
///
/// Same as standard bitmap plus 2 extra bytes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedBitmap {
    /// Flag byte (0x82 or 0x86)
    pub flag: ExtendedBitmapFlag,

    /// Reference to a point
    pub point_ref: u8,

    /// Unknown flag
    pub unknown_flag: u8,

    /// Which bitmap image to use
    pub bitmap_index: u8,

    /// Extra data byte 1 (purpose unknown)
    pub extra1: u8,

    /// Extra data byte 2 (purpose unknown)
    pub extra2: u8,
}

/// Line definition (3 bytes total)
///
/// Format: [0xA0, unknown_flag, vector_ref]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    /// Unknown flag (typically 0x08)
    pub unknown_flag: u8,

    /// Reference to vector or point
    pub vector_ref: u8,
}

/// Standard bitmap flag values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BitmapFlag {
    /// Flag 0x80
    Type80 = 0x80,

    /// Flag 0x88
    Type88 = 0x88,

    /// Flag 0xD0
    TypeD0 = 0xD0,
}

/// Extended bitmap flag values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExtendedBitmapFlag {
    /// Flag 0x82
    Type82 = 0x82,

    /// Flag 0x86
    Type86 = 0x86,
}

impl Polygon {
    /// Create a new polygon
    pub fn new(color: u8) -> Self {
        Self {
            color,
            sides: Vec::new(),
        }
    }

    /// Add a side to the polygon
    pub fn add_side(&mut self, side_ref: i8) {
        if self.sides.len() < 12 {
            self.sides.push(side_ref);
        }
    }

    /// Check if polygon is valid (3-12 sides)
    pub fn is_valid(&self) -> bool {
        self.sides.len() >= 3 && self.sides.len() <= 12
    }
}

impl Bitmap {
    /// Check if flag byte is valid
    pub fn is_valid_flag(byte: u8) -> bool {
        matches!(byte, 0x80 | 0x88 | 0xD0)
    }

    /// Create from flag byte
    pub fn from_flag(flag_byte: u8) -> Option<BitmapFlag> {
        match flag_byte {
            0x80 => Some(BitmapFlag::Type80),
            0x88 => Some(BitmapFlag::Type88),
            0xD0 => Some(BitmapFlag::TypeD0),
            _ => None,
        }
    }
}

impl ExtendedBitmap {
    /// Check if flag byte is valid
    pub fn is_valid_flag(byte: u8) -> bool {
        matches!(byte, 0x82 | 0x86)
    }

    /// Create from flag byte
    pub fn from_flag(flag_byte: u8) -> Option<ExtendedBitmapFlag> {
        match flag_byte {
            0x82 => Some(ExtendedBitmapFlag::Type82),
            0x86 => Some(ExtendedBitmapFlag::Type86),
            _ => None,
        }
    }
}

impl Line {
    /// Line flag is always 0xA0
    pub const FLAG: u8 = 0xA0;

    /// Check if byte is a line flag
    pub fn is_line_flag(byte: u8) -> bool {
        byte == Self::FLAG
    }
}

/// Identify graphical element type from flag byte
pub fn identify_element_type(flag_byte: u8) -> ElementType {
    if Line::is_line_flag(flag_byte) {
        ElementType::Line
    } else if Bitmap::is_valid_flag(flag_byte) {
        ElementType::Bitmap
    } else if ExtendedBitmap::is_valid_flag(flag_byte) {
        ElementType::ExtendedBitmap
    } else {
        // Everything else is a polygon with the byte as color
        ElementType::Polygon
    }
}

/// Element type identification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementType {
    Polygon,
    Bitmap,
    ExtendedBitmap,
    Line,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polygon_validation() {
        let mut poly = Polygon::new(0xFF);
        assert!(!poly.is_valid()); // No sides

        poly.add_side(1);
        poly.add_side(2);
        assert!(!poly.is_valid()); // Only 2 sides

        poly.add_side(3);
        assert!(poly.is_valid()); // 3 sides = triangle, valid

        // Add more sides
        for i in 4..=12 {
            poly.add_side(i as i8);
        }
        assert!(poly.is_valid());
        assert_eq!(poly.sides.len(), 12); // Max sides

        poly.add_side(13);
        assert_eq!(poly.sides.len(), 12); // Should not exceed 12
    }

    #[test]
    fn test_bitmap_flags() {
        assert!(Bitmap::is_valid_flag(0x80));
        assert!(Bitmap::is_valid_flag(0x88));
        assert!(Bitmap::is_valid_flag(0xD0));
        assert!(!Bitmap::is_valid_flag(0xFF));

        assert!(Bitmap::from_flag(0x80).is_some());
        assert!(Bitmap::from_flag(0x00).is_none());
    }

    #[test]
    fn test_extended_bitmap_flags() {
        assert!(ExtendedBitmap::is_valid_flag(0x82));
        assert!(ExtendedBitmap::is_valid_flag(0x86));
        assert!(!ExtendedBitmap::is_valid_flag(0x80));
    }

    #[test]
    fn test_line_flag() {
        assert!(Line::is_line_flag(0xA0));
        assert!(!Line::is_line_flag(0x00));
        assert_eq!(Line::FLAG, 0xA0);
    }

    #[test]
    fn test_element_type_identification() {
        assert_eq!(identify_element_type(0xA0), ElementType::Line);
        assert_eq!(identify_element_type(0x80), ElementType::Bitmap);
        assert_eq!(identify_element_type(0x82), ElementType::ExtendedBitmap);
        assert_eq!(identify_element_type(0xFF), ElementType::Polygon);
        assert_eq!(identify_element_type(0x01), ElementType::Polygon);
    }
}
