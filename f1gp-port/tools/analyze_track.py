#!/usr/bin/env python3
"""
Analyze F1GP track file format
Examines binary structure to understand data organization
"""

import sys
import struct
from pathlib import Path
from collections import Counter

def analyze_file(filepath):
    """Analyze a single track file"""
    with open(filepath, 'rb') as f:
        data = f.read()

    print(f"=== {filepath.name} ===")
    print(f"File size: {len(data):,} bytes ({len(data)} bytes)")
    print()

    # Byte frequency analysis
    print("Top 10 most common bytes:")
    byte_counts = Counter(data)
    for byte_val, count in byte_counts.most_common(10):
        percentage = (count / len(data)) * 100
        print(f"  0x{byte_val:02X} ({byte_val:3d}): {count:5d} times ({percentage:5.2f}%)")
    print()

    # Look for repeating patterns
    print("Repeating byte sequences (4+ repetitions):")
    for pattern_len in [2, 3, 4]:
        found = False
        i = 0
        while i < len(data) - pattern_len * 4:
            pattern = data[i:i+pattern_len]
            count = 1
            j = i + pattern_len
            while j < len(data) - pattern_len and data[j:j+pattern_len] == pattern:
                count += 1
                j += pattern_len

            if count >= 4 and not found:
                found = True
                print(f"  Pattern length {pattern_len}:")

            if count >= 4:
                pattern_hex = ' '.join(f'{b:02x}' for b in pattern)
                print(f"    Offset {i:04x}: [{pattern_hex}] x{count}")
                i = j
            else:
                i += 1
    print()

    # Check for possible header
    print("First 64 bytes (possible header):")
    for i in range(0, min(64, len(data)), 16):
        hex_bytes = ' '.join(f'{b:02x}' for b in data[i:i+16])
        ascii_chars = ''.join(chr(b) if 32 <= b < 127 else '.' for b in data[i:i+16])
        print(f"  {i:04x}: {hex_bytes:<48} {ascii_chars}")
    print()

    # Look for 16-bit integers
    if len(data) >= 4:
        print("First few 16-bit values (little-endian):")
        for i in range(0, min(32, len(data) - 1), 2):
            val = struct.unpack('<H', data[i:i+2])[0]
            print(f"  Offset {i:04x}: {val:5d} (0x{val:04x})")
        print()

    # Search for possible track name or string data
    print("Searching for ASCII strings (4+ chars):")
    current_string = []
    string_start = 0
    for i, byte in enumerate(data):
        if 32 <= byte < 127:  # Printable ASCII
            if not current_string:
                string_start = i
            current_string.append(chr(byte))
        else:
            if len(current_string) >= 4:
                print(f"  Offset {string_start:04x}: {''.join(current_string)}")
            current_string = []
    print()

def compare_files(filepaths):
    """Compare multiple track files to find common patterns"""
    print("=== COMPARISON ===")
    print()

    files_data = []
    min_size = float('inf')

    for fp in filepaths:
        with open(fp, 'rb') as f:
            data = f.read()
            files_data.append((fp.name, data))
            min_size = min(min_size, len(data))

    print(f"Comparing first {min_size} bytes of {len(filepaths)} files")
    print()

    # Find common bytes
    common_positions = []
    for i in range(min_size):
        bytes_at_pos = [data[i] for _, data in files_data]
        if all(b == bytes_at_pos[0] for b in bytes_at_pos):
            common_positions.append((i, bytes_at_pos[0]))

    if common_positions:
        print(f"Found {len(common_positions)} positions with identical bytes across all files:")
        for i, (pos, byte_val) in enumerate(common_positions[:20]):  # Show first 20
            print(f"  Offset {pos:04x}: 0x{byte_val:02X} ({byte_val:3d})")
        if len(common_positions) > 20:
            print(f"  ... and {len(common_positions) - 20} more")
    else:
        print("No common bytes found in same positions")
    print()

    # Find differing regions
    print("First 10 positions where files differ:")
    diff_count = 0
    for i in range(min_size):
        bytes_at_pos = [data[i] for _, data in files_data]
        if not all(b == bytes_at_pos[0] for b in bytes_at_pos):
            print(f"  Offset {i:04x}:", end='')
            for name, data in files_data:
                print(f" {name}=0x{data[i]:02X}", end='')
            print()
            diff_count += 1
            if diff_count >= 10:
                break
    print()

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python3 analyze_track.py <track_file> [track_file2 ...]")
        sys.exit(1)

    filepaths = [Path(arg) for arg in sys.argv[1:]]

    # Analyze first file in detail
    analyze_file(filepaths[0])

    # If multiple files, compare them
    if len(filepaths) > 1:
        compare_files(filepaths)
