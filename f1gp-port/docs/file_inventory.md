# F1GP File Inventory

**Extracted from**: Formula One Grand Prix (1996)(Microprose).iso
**Date**: 2025-11-14
**Total Size**: 28 MB
**Directories**: 12
**Files**: 254

---

## Directory Structure

```
assets/original/
├── HARDDISK/          # Main game files
├── MPS/
│   └── GPRIX/        # Game executables and data
└── EXTRAS/           # Additional content
    ├── BOOTDISC/     # Boot disk utilities
    └── DEMOS/        # Demo programs (Magic: The Gathering)
        └── MTG95/
```

---

## Critical Game Files

### Track Files (F1CT*.DAT)

Located in: `HARDDISK/` and `MPS/GPRIX/`

All 16 circuits from the 1991 F1 season:

| File | Size | Notes |
|------|------|-------|
| F1CT01.DAT | 17 KB | Circuit 1 |
| F1CT02.DAT | 15 KB | Circuit 2 |
| F1CT03.DAT | 16 KB | Circuit 3 |
| F1CT04.DAT | 20 KB | Circuit 4 |
| F1CT05.DAT | 15 KB | Circuit 5 |
| F1CT06.DAT | 15 KB | Circuit 6 |
| F1CT07.DAT | 14 KB | Circuit 7 |
| F1CT08.DAT | 14 KB | Circuit 8 |
| F1CT09.DAT | 14 KB | Circuit 9 |
| F1CT10.DAT | 14 KB | Circuit 10 |
| F1CT11.DAT | 14 KB | Circuit 11 |
| F1CT12.DAT | 15 KB | Circuit 12 |
| F1CT13.DAT | 14 KB | Circuit 13 |
| F1CT14.DAT | 14 KB | Circuit 14 |
| F1CT15.DAT | 15 KB | Circuit 15 |
| F1CT16.DAT | 15 KB | Circuit 16 |

**Total**: 16 track files (238 KB)

**Next Steps**: Reverse engineer format to extract:
- Track geometry (segments, coordinates)
- Elevation data
- Racing line
- AI behavior data
- Track objects/scenery

### Game Data Files

| File | Size | Description |
|------|------|-------------|
| F1GPDATA.DAT | Size TBD | Main game data |
| F1GPDATB.DAT | Size TBD | Additional game data |
| HELMETS.DAT | Size TBD | Driver helmet graphics |
| FLAGS.DAT | Size TBD | Country flags |
| TROPHY.DAT | Size TBD | Trophy graphics |
| CHAMP.DAT | 292 KB | Championship data |
| BACKDROP.DAT | 528 KB | Background graphics |

### Visual Assets

**FLI Files (Animations)**:
- EARTH.FLI (187 KB) - Opening sequence?
- DROPEN.FLI (160 KB) - Driver opening?
- FRNTEND.FLI - Frontend animation
- GPLOGO.FLI - GP logo animation

**LBM Files (Images)**:
- GPLOGO.LBM - GP logo image
- MPROSE.LBM - MicroProse logo

### Crash Animations

| File | Size | Description |
|------|------|-------------|
| CRASH1.DAT | 318 KB | Crash animation 1 |
| CRASH2.DAT | 161 KB | Crash animation 2 |
| CRASH3.DAT | 132 KB | Crash animation 3 |

**Total**: 611 KB of crash animations

### Audio Files

**Sound Banks (.BIN)**:
- AINTRO.BIN (18 KB) - AdLib intro sounds
- AINGAME.BIN (7 KB) - AdLib in-game sounds
- ASOUND.BIN (3 KB) - AdLib sound effects
- ACREDIT.BIN (27 KB) - AdLib credit sounds
- BINTRO.BIN (3 KB) - PC Speaker intro
- BINGAME.BIN (2 KB) - PC Speaker in-game
- BSOUND.BIN (1 KB) - PC Speaker sounds
- BCREDIT.BIN (57 bytes) - PC Speaker credits
- XINTRO.BIN - Unknown sound format intro
- XINGAME.BIN - Unknown sound format in-game
- XCREDIT.BIN - Unknown sound format credits
- RINTRO.BIN - Roland intro
- RSOUND.BIN - Roland sounds
- RCREDIT.BIN - Roland credits

**Sound Catalogs (.CAT)**:
- ADLIB.CAT (42 KB) - AdLib sound catalog
- BEEP.CAT (2 KB) - PC Speaker catalog
- ROLAND.CAT - Roland MT-32 catalog

**Audio Formats Detected**:
1. AdLib (FM synthesis)
2. PC Speaker (beeps)
3. Roland MT-32 (MIDI)
4. Unknown format (X prefix)

### Preference/Configuration Files

| File | Description |
|------|-------------|
| F1PREFS.DAT | Default preferences |
| F1PREFS.286 | 286 processor prefs |
| F1PREFS.386 | 386 processor prefs |
| F1PREFS.486 | 486 processor prefs |

### Executables

**Game Executables**:
- F1GP.$$$ - Main game executable
- CD1.EXE (6 KB) - CD version launcher
- CDPATCH.EXE (2 KB) - CD patch utility
- PLAYSCR.EXE - Screen playback utility
- MPSCOPY.EXE - Copy utility

**Batch Files**:
- F1GP.BAT - Game launcher script
- INSTALL.BAT - Installation script
- HDINST.BAT - Hard disk installer
- BOOTMAKE.BAT - Boot disk maker

### Installation Files

**Documentation**:
- README.NOW - Installation readme
- README.TXT (multiple) - Various readme files
- README.F1G - F1GP specific readme

**Boot Disk Utilities**:
- BOOTALL4.EXE (28 KB) - Boot disk creator
- BOOT.DAT (2 KB) - Boot configuration
- Various .LST files - File lists for boot disk

---

## File Format Analysis Priority

### High Priority (Stage 1.3)
1. **F1CT*.DAT** - Track format (critical for gameplay)
2. **F1GPDATA.DAT** - Main game data
3. **HELMETS.DAT** - Graphics format example

### Medium Priority (Stage 2+)
4. **CHAMP.DAT** - Championship structure
5. **.FLI files** - Animation format
6. **.CAT files** - Sound catalog format
7. **CRASH*.DAT** - Animation data

### Low Priority (Later stages)
8. **FLAGS.DAT**, **TROPHY.DAT** - Simple graphics
9. **BACKDROP.DAT** - Background graphics
10. Preference files - Configuration format

---

## Observations

### Track Files
- Relatively small (14-20 KB each)
- Likely contain:
  - Track geometry (segment list)
  - Elevation/banking data
  - Racing line coordinates
  - AI behavior parameters
  - Track objects/scenery
  - Pit lane configuration

### Data Organization
- Separate directories for hard disk vs. CD versions
- MPS/GPRIX contains CD-specific files
- HARDDISK contains installation files
- Sound files organized by sound card type

### Audio System
- Multiple sound card support (AdLib, Roland, PC Speaker)
- Separate sound banks for different parts of game
- Catalog files likely index into sound banks

### Graphics System
- FLI format for animations (standard Autodesk Animator format)
- LBM format for static images (IFF/ILBM format)
- DAT files for in-game graphics (custom format)

---

## Next Steps

1. **Hex Analysis** of F1CT01.DAT to understand track format
2. **Compare** multiple track files to identify patterns
3. **Research** existing F1GP file format documentation
4. **Implement** basic track file parser
5. **Visualize** track data to verify parsing

---

## Tools Needed

- Hex editor (for format analysis)
- Binary diff tool (compare similar files)
- FLI viewer (for animations)
- LBM viewer (for images)
- DOSBox (for testing original files)

---

## References

- F1GP Community: https://sites.google.com/view/f1gpwc
- F1GP Utils: https://github.com/sdidit/f1gp-utils
- ArgEditor/ArgTrack: Modern F1GP editing tools

---

**Document Status**: Initial inventory complete
**Last Updated**: 2025-11-14
