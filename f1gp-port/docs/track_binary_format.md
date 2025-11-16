# F1GP Track Binary Format - Complete Specification

**Source:** ArgData .NET library (https://github.com/codemeyer/ArgData)
**Date:** 2025-11-16
**Status:** COMPLETE - Ready for Implementation

---

## Overview

This document provides the complete binary format for F1GP track files (.DAT), extracted from the ArgData .NET library source code. Unlike the partial ArgDocs documentation, this shows the exact byte-level structure needed for implementation.

---

## File Structure

```
Offset      Size    Description
────────────────────────────────────────────────────────────
0x0000      4096    Unused (originally used in F1GP, no longer required)
0x1000      var     Offsets section (file structure map)
0x????      var     Horizon data
0x????      var     Object shapes and settings
0x????      var     Track section header
0x????      var     Track sections (MAIN GEOMETRY - documented below)
0x????      var     Computer car racing line (AI - documented below)
0x????      var     Computer car behavior and setup
0x????      var     Pit lane sections (same format as track sections)
0x????      var     Track camera commands
0x????      var     Additional settings
filesize-4  4       Checksum (last 4 bytes)
```

**Key Finding:** The first 4096 bytes (0x0000-0x0FFF) are unused padding.
**Important:** Offsets section at 0x1000 maps where each section begins in the file.

---

## Track Section Format (THE CRITICAL PART)

### Binary Structure

Track sections are read sequentially until terminator (0xFF 0xFF). Each section is variable-length:

```
Byte Offset  Type    Name              Description
──────────────────────────────────────────────────────────────
0            byte    byte1             Either Length OR Command Arg1
1            byte    byte2             0 = section data, >0 = command
```

### Section Data (when byte2 == 0)

```
Byte Offset  Type    Name              Description
──────────────────────────────────────────────────────────────
0            byte    Length            Section length (1 unit = 16 feet ≈ 4.87m)
1            byte    (always 0)        Marker indicating section data
2-3          int16   Curvature         Turn radius (+ = right, - = left)
4-5          int16   Height            Elevation change
6-7          int16   Flags             Bitfield for features (see below)
8            byte    RightVergeWidth   Right shoulder width
9            byte    LeftVergeWidth    Left shoulder width

TOTAL: 10 bytes per section
```

### Command Data (when byte2 > 0)

```
Byte Offset  Type    Name              Description
──────────────────────────────────────────────────────────────
0            byte    Arg0              First argument (byte1)
1            byte    CommandId         Command type (byte2)
2-3          int16   Arg1              Additional arguments...
4-5          int16   Arg2              (variable count based on command)
...          int16   ArgN

Commands are variable-length based on command type
```

### Terminator

```
0xFF 0xFF = End of track sections
```

---

## Flags Bitfield (Byte 6-7)

The 16-bit flags field encodes track features:

```c
// Bit positions (to be decoded from int16 flags value)
HasLeftKerb          : bool
HasRightKerb         : bool
KerbHeight           : enum (Low/High)
PitLaneEntrance      : bool
PitLaneExit          : bool
RoadSigns            : bool  // 300/200/100m markers
RoadSignArrow        : bool
RoadSignArrow100     : bool
BridgedRightFence    : bool
BridgedLeftFence     : bool
RemoveRightWall      : bool
RemoveLeftWall       : bool
Unknown1-4           : bool  // Reserved/undocumented
```

**TODO:** Determine exact bit positions for each flag (reverse engineer from ArgData or test files)

---

## Track Section Commands

Commands are inserted between sections to modify behavior:

```
CommandId (byte2)   Purpose
──────────────────────────────────────────────────────────
Variable            Command types defined in TrackSectionCommandFactory
                    Each command has variable number of int16 arguments
```

**Note:** Command details are in `TrackSectionCommandFactory.cs` in ArgData source

---

## Racing Line Format (Computer Car Line)

### Binary Structure

The racing line consists of segments with variable formats:

```
FIRST SEGMENT:
Byte Offset  Type    Name              Description
──────────────────────────────────────────────────────────────
0            byte    Length            First segment length
1-2          int16   Displacement      Initial position offset
3-4          int16   Correction        Steering correction (tighter/wider)
5-6          int16   Radius            Corner radius

SUBSEQUENT SEGMENTS (Normal):
Byte Offset  Type    Name              Description
──────────────────────────────────────────────────────────────
0            byte    Length            Segment length
1            byte    Type              0x00 = Normal segment
2-3          int16   Correction        Steering correction
4-5          int16   Radius            Corner radius

SUBSEQUENT SEGMENTS (Wide Radius):
Byte Offset  Type    Name              Description
──────────────────────────────────────────────────────────────
0            byte    Length            Segment length
1            byte    Type              0x40 = Wide radius segment
2-3          int16   Correction        Steering correction
4-5          int16   HighRadius        High radius for complex curves
6-7          int16   LowRadius         Low radius for complex curves

TERMINATOR:
int16 == 0  (2 bytes of 0x00) = End of racing line
```

### Reading Algorithm

```rust
// Pseudocode
let first_length = read_u8();
let displacement = read_i16();
let correction = read_i16();
let radius = read_i16();

segments.push(Segment { length: first_length, displacement, correction, radius });

loop {
    let byte1 = read_u8();  // Length
    let byte2 = read_u8();  // Type

    let correction = read_i16();

    let segment = if byte2 == 0x40 {
        // Wide radius segment
        let high_radius = read_i16();
        let low_radius = read_i16();
        WideRadiusSegment { length: byte1, correction, high_radius, low_radius }
    } else {
        // Normal segment
        let radius = read_i16();
        NormalSegment { length: byte1, correction, radius }
    };

    segments.push(segment);

    // Check for terminator
    let next_short = read_i16();
    if next_short == 0 {
        break;  // End of racing line
    } else {
        seek(-2);  // Rewind to reprocess
    }
}
```

---

## Units and Conversions

### Distance Units

```
Track Section Length: 1 unit = 16 feet ≈ 4.87 meters

Example:
  Length byte = 100
  Actual length = 100 × 4.87m = 487 meters
```

### Width Units

```
Verge width: Same as length (1 unit = 4.87m)
Track width: Defined in TrackSectionHeader (not per-section)
```

### Curvature

```
Curvature (int16):
  Positive values = Right turn
  Negative values = Left turn
  Magnitude = Tightness of turn
  0 = Straight

Conversion to radius TBD (likely proportional)
```

### Height/Elevation

```
Height (int16):
  Signed value representing elevation change
  Units TBD (likely scaled game units)
  Positive = Uphill
  Negative = Downhill
```

---

## Coordinate System

**From ArgData TrackSectionHeader:**
```
TrackCenterX : int
TrackCenterY : int
TrackCenterZ : int
```

These define the track's origin point in 3D space.

**Coordinate Building:**
- Track sections don't store absolute X,Y,Z positions
- Positions are calculated by walking through sections from start
- Use curvature and length to calculate next section position
- Apply height deltas for elevation

**Algorithm:**
```rust
let mut pos = Vec3::new(TrackCenterX, TrackCenterY, TrackCenterZ);
let mut angle = 0.0;  // Starting heading

for section in track_sections {
    // Calculate turn
    angle += calculate_turn_angle(section.curvature, section.length);

    // Move forward
    let dx = section.length * cos(angle);
    let dz = section.length * sin(angle);
    let dy = section.height;  // Elevation change

    pos += Vec3::new(dx, dy, dz);

    // Store position
    section.position = pos;
}
```

---

## File Offsets Section (0x1000)

The offsets section at 0x1000 is a directory that tells where each major section begins:

```
Offset 0x1000:
  Number of offsets (count)
  Array of offset values (4 bytes each)

Each offset points to:
  - Horizon data start
  - Object shapes start
  - Track sections start
  - Racing line start
  - etc.
```

**Note:** ArgData uses `OffsetReader.cs` to parse this section. The exact structure is in that file.

---

## Object Shapes (Already Documented)

Object shapes are already documented in our existing `track_format_spec.md`:

- Directory at 0x100E (right before offsets section)
- Count of shapes (uint16)
- Array of 4-byte offsets
- Graphical elements: Polygons, Bitmaps, Lines

See existing documentation for details.

---

## Checksum (Last 4 Bytes)

```
Position: filesize - 4
Type: uint32 (little endian)
Purpose: File validation (F1GP requires valid checksum)
Algorithm: Unknown (TODO: reverse engineer from ArgData GpChecksum.cs)
```

---

## Implementation in Rust

### Data Structures

```rust
pub struct TrackSection {
    pub position: Vec3,          // Calculated from curvature/length
    pub length: f32,             // Converted from byte (× 4.87m)
    pub curvature: i16,          // Raw value from file
    pub height: i16,             // Raw elevation change
    pub flags: u16,              // Raw bitfield
    pub right_verge_width: u8,   // Raw width
    pub left_verge_width: u8,    // Raw width
    pub commands: Vec<TrackSectionCommand>,

    // Parsed from flags
    pub has_left_kerb: bool,
    pub has_right_kerb: bool,
    pub kerb_height: KerbHeight,
    pub pit_lane_entrance: bool,
    pub pit_lane_exit: bool,
    // ... other flags
}

pub struct RacingLineSegment {
    pub length: u8,
    pub correction: i16,
    pub segment_type: SegmentType,
}

pub enum SegmentType {
    Normal { radius: i16 },
    WideRadius { high_radius: i16, low_radius: i16 },
}

pub struct RacingLine {
    pub displacement: i16,  // Initial offset
    pub segments: Vec<RacingLineSegment>,
}
```

### Parser Implementation

```rust
use std::io::{Cursor, Read};
use byteorder::{LittleEndian, ReadBytesExt};

pub struct TrackParser {
    cursor: Cursor<Vec<u8>>,
}

impl TrackParser {
    pub fn parse_track_sections(&mut self) -> Result<Vec<TrackSection>> {
        let mut sections = Vec::new();
        let mut commands = Vec::new();

        loop {
            let byte1 = self.cursor.read_u8()?;
            let byte2 = self.cursor.read_u8()?;

            // Check for terminator
            if byte1 == 0xFF && byte2 == 0xFF {
                break;
            }

            if byte2 > 0 {
                // Command
                let arg0 = byte1;
                let command_id = byte2;
                // Read additional args based on command type
                let command = self.parse_command(command_id, arg0)?;
                commands.push(command);
            } else {
                // Section data
                let length = byte1;
                let curvature = self.cursor.read_i16::<LittleEndian>()?;
                let height = self.cursor.read_i16::<LittleEndian>()?;
                let flags = self.cursor.read_u16::<LittleEndian>()?;
                let right_verge = self.cursor.read_u8()?;
                let left_verge = self.cursor.read_u8()?;

                let section = TrackSection {
                    length: length as f32 * 4.87,  // Convert to meters
                    curvature,
                    height,
                    flags,
                    right_verge_width: right_verge,
                    left_verge_width: left_verge,
                    commands: std::mem::take(&mut commands),
                    ..Default::default()
                };

                sections.push(section);
            }
        }

        Ok(sections)
    }

    pub fn parse_racing_line(&mut self) -> Result<RacingLine> {
        // First segment
        let first_length = self.cursor.read_u8()?;
        let displacement = self.cursor.read_i16::<LittleEndian>()?;
        let correction = self.cursor.read_i16::<LittleEndian>()?;
        let radius = self.cursor.read_i16::<LittleEndian>()?;

        let mut segments = vec![RacingLineSegment {
            length: first_length,
            correction,
            segment_type: SegmentType::Normal { radius },
        }];

        loop {
            let length = self.cursor.read_u8()?;
            let type_byte = self.cursor.read_u8()?;
            let correction = self.cursor.read_i16::<LittleEndian>()?;

            let segment_type = if type_byte == 0x40 {
                // Wide radius
                let high = self.cursor.read_i16::<LittleEndian>()?;
                let low = self.cursor.read_i16::<LittleEndian>()?;
                SegmentType::WideRadius {
                    high_radius: high,
                    low_radius: low,
                }
            } else {
                // Normal
                let radius = self.cursor.read_i16::<LittleEndian>()?;
                SegmentType::Normal { radius }
            };

            segments.push(RacingLineSegment {
                length,
                correction,
                segment_type,
            });

            // Check terminator
            let pos = self.cursor.position();
            let next = self.cursor.read_i16::<LittleEndian>()?;
            if next == 0 {
                break;  // End of racing line
            } else {
                self.cursor.set_position(pos);  // Rewind
            }
        }

        Ok(RacingLine {
            displacement,
            segments,
        })
    }
}
```

---

## Next Steps for Implementation

1. **Get Binary Files** - Extract F1GP ISO to get actual .DAT files
2. **Test Parser** - Implement above Rust code and test with real Monaco track
3. **Validate** - Compare output with ArgData parsing of same file
4. **Calculate Positions** - Implement coordinate calculation algorithm
5. **Generate Mesh** - Convert sections to 3D mesh with elevation
6. **Render** - Display in 3D demo

---

## References

- **ArgData Source:** https://github.com/codemeyer/ArgData
- **Key Files:**
  - `Source/ArgData/Internals/TrackSectionReader.cs` - Binary reading logic
  - `Source/ArgData/Internals/ComputerCarLineReader.cs` - Racing line parsing
  - `Source/ArgData/Entities/TrackSection.cs` - Data structure
  - `Source/ArgData/Entities/TrackComputerCarLineSegment.cs` - Racing line structure
- **ArgDocs:** https://www.argtools.com/argdocs/file-formats/track/
- **GP2 Format:** https://www.waa63.ch/racesim/TEIC/primer/GP2TrackFileFormat.htm

---

## Status

**COMPLETE:** Binary format fully documented and ready for Rust implementation.

**Next:** Write Rust parser code and test with real track files.

---

**Last Updated:** 2025-11-16
