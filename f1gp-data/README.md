# F1GP Original Game Data

This directory contains data extracted from the original Formula One Grand Prix (1996) ISO image.

## Directory Structure

### tracks/
Contains the 16 original circuit track files from the 1996 F1 season:

- `F1CT01.DAT` - Track 1
- `F1CT02.DAT` - Track 2
- `F1CT03.DAT` - Track 3
- `F1CT04.DAT` - Track 4
- `F1CT05.DAT` - Track 5
- `F1CT06.DAT` - Track 6
- `F1CT07.DAT` - Track 7
- `F1CT08.DAT` - Track 8
- `F1CT09.DAT` - Track 9
- `F1CT10.DAT` - Track 10
- `F1CT11.DAT` - Track 11
- `F1CT12.DAT` - Track 12
- `F1CT13.DAT` - Track 13
- `F1CT14.DAT` - Track 14
- `F1CT15.DAT` - Track 15
- `F1CT16.DAT` - Track 16

These `.DAT` files contain the track data in the F1GP binary format, equivalent to `.TRK` files used in some F1GP versions.

### cars-drivers/
Contains car and driver information:

- `F1GPDATA.DAT` - Primary game data including car and driver information (46KB)
- `F1GPDATB.DAT` - Additional game data (63KB)
- `HELMETS.DAT` - Driver helmet graphics and data (53KB)
- `FLAGS.DAT` - Team/nationality flag graphics (2.1KB)

## Source

Extracted from: `Formula One Grand Prix (1996)(Microprose).iso`

## Notes

These files use the original F1GP binary format. The track files (F1CT*.DAT) contain:
- Track geometry and layout
- Pit lane configuration
- Track sections and corners
- Elevation data
- Track-side objects

The car/driver data files contain information about the 1996 F1 season teams, drivers, and car performance characteristics.
