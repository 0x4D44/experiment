# F1GP Track File Format Specification

**Version:** 0.1 (Initial Research)
**Date:** 2025-11-14
**Status:** Work in Progress

---

## Overview

F1GP track files (F1CT01.DAT through F1CT16.DAT) contain all data for a racing circuit including:
- Track geometry and segments
- Trackside objects and scenery
- AI racing line and behavior
- Camera positions
- Pit lane configuration

**File Size Range:** 13,368 to 20,497 bytes (original tracks)
**Checksum:** Last 4 bytes of file (must be valid for F1GP to load track)

---

## File Structure

Based on ArgDocs documentation and binary analysis, the file contains these sections:

1. **Horizon Image** (offset unknown)
2. **Offsets Directory** (starts at 0x100E)
3. **Object Shapes** (directory at 0x100E, first object at 0x1010)
4. **Object Settings**
5. **Track Data Header**
6. **Track Sections**
7. **Computer Car Racing Line**
8. **Computer Car Setup and Behavior**
9. **Pit Lane Sections** (same format as track sections)
10. **Camera Definitions**
11. **Additional Data and Behavior**
12. **Checksum** (final 4 bytes)

---

## Binary Analysis Findings

### Byte Frequency (F1CT01.DAT, 16,924 bytes)

| Byte Value | Occurrences | Percentage | Interpretation |
|------------|-------------|------------|----------------|
| 0x00 | 5,400 | 31.91% | Padding/alignment or empty data |
| 0xEF | 676 | 3.99% | Fill byte / empty marker |
| 0x80 | 653 | 3.86% | Common value (bitmap flag?) |
| 0x02 | 575 | 3.40% | Flag/value |
| 0x01 | 409 | 2.42% | Flag/value |
| 0x2B | 347 | 2.05% | Common value |
| 0x2C | 336 | 1.99% | Common value |

**Observations:**
- High percentage of 0x00 suggests sparse data or alignment padding
- 0xEF appears to be used as a fill/empty marker
- Powers of 2 (0x01, 0x02, 0x04, 0x08) common - likely bit flags
- No ASCII text found - pure binary format

### Common Patterns

**Repeating Sequences:**
- `EF EF EF` - Fill bytes (10+ repetitions in places)
- `00 00 00` - Padding (many repetitions)
- `BB BB BB` - Data value repeated
- `28 28 28` - Data value repeated
- `AC AC AC` - Data value repeated
- `BB 88` - Alternating pattern (7 repetitions) - paired data?

**File Comparison (F1CT01, F1CT02, F1CT03):**
- **1,295 bytes identical** across all three files (~8.4%)
- All files start with `EF EF EF`
- Track-specific data begins at offset 0x0003
- Common bytes likely represent empty track sections or fixed data structures

---

## Object Shapes Format

*Source: ArgDocs*

### Directory Structure

**Location:** Starts at file offset 0x100E

| Offset | Size | Description |
|--------|------|-------------|
| 0x100E | 2 | Number of object shapes |
| 0x1010 | 4*n | Object offsets (4 bytes each) |

Each object's data extends from its offset to the byte before the next object's offset.

### Object Header

Each object shape contains paired header values and offsets:

| Header Value | Points To |
|--------------|-----------|
| Value 1 | Scale value data |
| Value 2 | Graphical elements |
| Value 3 | Point data |
| Value 4 | Vector data |
| Value 5 | Graphical element list |
| Value 6 | Unknown |

**Note:** Values 2-6 are reportedly always identical.

### Graphical Element Types

Elements identified by first byte:

| Byte Value | Type | Total Size | Description |
|------------|------|------------|-------------|
| 0x80, 0x88, 0xD0 | Bitmap | 4 bytes | Standard bitmap reference |
| 0x82, 0x86 | Extended Bitmap | 6 bytes | Bitmap with extra data |
| 0xA0 (160) | Line | 3 bytes | Line definition |
| Other | Polygon | Variable | Color indicates polygon, 3-12 sides |

### Polygon Format

```
[Color Byte] [Side1] [Side2] ... [SideN] [0x00]
```

- **Color Byte:** Determines polygon color
- **Side Bytes:** Signed bytes referencing vectors
  - Positive value: References start point
  - Negative value: References end point
- **Terminator:** 0x00 ends the polygon
- **Side Count:** 3-12 sides allowed

### Bitmap Format

**Standard Bitmap (4 bytes):**
```
[Flag Byte] [Point Ref] [Unknown Flag] [Bitmap Index]
```
- Flag Byte: 0x80, 0x88, or 0xD0
- Point Ref: Reference to a point
- Unknown Flag: Often 0xFF
- Bitmap Index: Which bitmap image to use

**Extended Bitmap (6 bytes):**
```
[Flag Byte] [Point Ref] [Unknown Flag] [Bitmap Index] [Extra1] [Extra2]
```
- Same as standard, plus 2 additional bytes of unknown purpose

### Line Format (3 bytes)

```
[0xA0] [Unknown Flag] [Vector/Point Ref]
```
- Flag: Always 0xA0
- Unknown Flag: Typically 0x08
- Reference: Points to vector or point data

---

## Unknowns and TODOs

### Not Yet Documented

- [ ] Exact location and format of Horizon Image
- [ ] Track Data Header structure
- [ ] Track Sections format (geometry, coordinates)
- [ ] Racing Line data format
- [ ] AI behavior parameters
- [ ] Camera definition format
- [ ] Pit lane section structure
- [ ] Checksum calculation algorithm

### Questions

1. What do the values 0x2B, 0x2C, 0x2A (common in analysis) represent?
2. Is 0xEF truly a fill byte or does it have semantic meaning?
3. What is the coordinate system for track geometry?
4. How are elevation and banking encoded?
5. What is the structure of the racing line data?

---

## Next Steps

### Immediate (Stage 1.3 completion)

1. **Hex analysis of track section area** (likely after object data)
2. **Compare track geometry** between circuits to identify segment format
3. **Attempt to parse object shapes** using documented format
4. **Create initial Rust structures** for known formats

### Future Stages

1. **Stage 1.4:** Define complete Rust data structures
2. **Stage 1.5:** Implement track loader
3. **Stage 2.2:** Implement track renderer to visualize loaded data
4. **Testing:** Validate parser against all 16 original tracks

---

## References

- **ArgDocs:** https://www.argtools.com/argdocs/file-formats/track/
- **F1GP Community:** https://sites.google.com/view/f1gpwc
- **Binary Analysis:** tools/analyze_track.py
- **File Inventory:** docs/file_inventory.md

---

## Revision History

| Version | Date | Changes |
|---------|------|---------|
| 0.1 | 2025-11-14 | Initial specification based on research and binary analysis |

---

**Document Status:** Active research document
**Completeness:** ~20% - Object shapes documented, track sections TBD
**Next Update:** After track section analysis complete
