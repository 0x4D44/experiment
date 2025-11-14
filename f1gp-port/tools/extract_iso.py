#!/usr/bin/env python3
"""
Quick ISO extraction script for F1GP
Uses pycdlib to extract all files from the ISO image
"""

import sys
import os
from pathlib import Path
import pycdlib

def extract_iso(iso_path, output_dir):
    """Extract all files from ISO to output directory"""

    print(f"F1GP ISO Extractor (Python)")
    print(f"Input ISO: {iso_path}")
    print(f"Output: {output_dir}")
    print()

    # Create output directory
    output_path = Path(output_dir)
    output_path.mkdir(parents=True, exist_ok=True)

    # Open ISO
    iso = pycdlib.PyCdlib()
    iso.open(iso_path)

    total_files = 0
    total_dirs = 0

    # Walk through all entries
    for dirname, dirlist, filelist in iso.walk(iso_path="/"):
        # Create subdirectories
        if dirname != "/":
            dir_path = output_path / dirname[1:]  # Remove leading /
            dir_path.mkdir(parents=True, exist_ok=True)
            total_dirs += 1
            print(f"Dir:  {dirname}")

        # Extract files
        for filename in filelist:
            iso_file_path = dirname + "/" + filename if dirname != "/" else "/" + filename

            # Determine output file path
            if dirname == "/":
                output_file = output_path / filename
            else:
                output_file = output_path / dirname[1:] / filename

            # Extract file from ISO directly to disk
            iso.get_file_from_iso(str(output_file), iso_path=iso_file_path)

            total_files += 1
            file_size = output_file.stat().st_size
            print(f"File: {iso_file_path} ({file_size:,} bytes)")

    iso.close()

    print()
    print("Extraction complete!")
    print(f"  Directories: {total_dirs}")
    print(f"  Files: {total_files}")
    print(f"  Output: {output_dir}")

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python3 extract_iso.py <iso_file> <output_dir>")
        sys.exit(1)

    iso_path = sys.argv[1]
    output_dir = sys.argv[2]

    if not os.path.exists(iso_path):
        print(f"Error: ISO file not found: {iso_path}")
        sys.exit(1)

    try:
        extract_iso(iso_path, output_dir)
    except Exception as e:
        print(f"Error: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)
