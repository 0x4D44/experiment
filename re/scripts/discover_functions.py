#!/usr/bin/env python3
"""
Function discovery script for GP.EXE (F1GP DOS game)

Analyzes the binary to find likely function entry points using pattern recognition:
- Standard x86 16-bit function prologues (PUSH BP; MOV BP,SP)
- CALL instruction targets
- FAR CALL targets
- Interrupt handlers

Outputs to re/artifacts/symbols.json
"""

import json
import struct
import hashlib
import os
from datetime import datetime, timezone
from pathlib import Path
from dataclasses import dataclass, field, asdict
from typing import List, Set, Dict, Optional, Tuple

@dataclass
class FunctionInfo:
    name: str
    entry_linear: str  # Hex string like "0x1234"
    size_bytes: int = 0
    stack_frame_size: int = 0
    subsystem: str = "unknown"
    callers: List[str] = field(default_factory=list)
    int_usage: List[str] = field(default_factory=list)
    source_recording_id: str = ""
    notes: str = ""

class DOSExeAnalyzer:
    """Analyzer for MS-DOS MZ executables"""

    # x86 16-bit function prologue patterns
    PROLOGUE_PATTERNS = [
        bytes([0x55, 0x8B, 0xEC]),        # PUSH BP; MOV BP, SP
        bytes([0x55, 0x89, 0xE5]),        # PUSH BP; MOV BP, SP (alternate encoding)
        bytes([0x55, 0x8B, 0xE5]),        # PUSH BP; MOV BP, SP (alternate)
        bytes([0xC8]),                     # ENTER instruction
        bytes([0x55]),                     # Just PUSH BP (common in small funcs)
    ]

    # CALL instruction opcodes
    CALL_NEAR_REL16 = 0xE8     # CALL rel16
    CALL_FAR_PTR = 0x9A        # CALL ptr16:16
    CALL_RM16 = 0xFF           # CALL r/m16 (with modrm byte /2)

    # RET instructions (mark function ends)
    RET_NEAR = 0xC3
    RET_NEAR_IMM = 0xC2
    RET_FAR = 0xCB
    RET_FAR_IMM = 0xCA

    def __init__(self, exe_path: str):
        self.exe_path = Path(exe_path)
        self.data = self.exe_path.read_bytes()
        self.functions: Dict[int, FunctionInfo] = {}
        self.call_targets: Set[int] = set()
        self.call_sources: Dict[int, List[int]] = {}  # target -> [callers]

        # Parse MZ header
        self._parse_mz_header()

    def _parse_mz_header(self):
        """Parse the MS-DOS MZ executable header"""
        if self.data[:2] != b'MZ':
            raise ValueError("Not a valid MZ executable")

        # MZ header fields
        self.e_cblp = struct.unpack_from('<H', self.data, 2)[0]      # Bytes on last page
        self.e_cp = struct.unpack_from('<H', self.data, 4)[0]       # Pages in file
        self.e_crlc = struct.unpack_from('<H', self.data, 6)[0]     # Relocations
        self.e_cparhdr = struct.unpack_from('<H', self.data, 8)[0]  # Header size in paragraphs
        self.e_minalloc = struct.unpack_from('<H', self.data, 10)[0]
        self.e_maxalloc = struct.unpack_from('<H', self.data, 12)[0]
        self.e_ss = struct.unpack_from('<H', self.data, 14)[0]      # Initial SS
        self.e_sp = struct.unpack_from('<H', self.data, 16)[0]      # Initial SP
        self.e_csum = struct.unpack_from('<H', self.data, 18)[0]    # Checksum
        self.e_ip = struct.unpack_from('<H', self.data, 20)[0]      # Initial IP
        self.e_cs = struct.unpack_from('<H', self.data, 22)[0]      # Initial CS
        self.e_lfarlc = struct.unpack_from('<H', self.data, 24)[0]  # Relocation table offset

        # Calculate code start offset
        self.header_size = self.e_cparhdr * 16
        self.code_start = self.header_size

        # Calculate entry point linear address
        self.entry_point = self.e_cs * 16 + self.e_ip

        print(f"MZ Header parsed:")
        print(f"  Header size: {self.header_size} bytes ({self.e_cparhdr} paragraphs)")
        print(f"  Code start: 0x{self.code_start:X}")
        print(f"  Entry point: CS:IP = {self.e_cs:04X}:{self.e_ip:04X} (linear: 0x{self.entry_point:X})")
        print(f"  File size: {len(self.data)} bytes")

    def compute_sha256(self) -> str:
        """Compute SHA256 hash of the binary"""
        return hashlib.sha256(self.data).hexdigest()

    def find_prologue_patterns(self) -> Set[int]:
        """Find standard function prologues in the code section"""
        prologues = set()

        # Search from code start
        code = self.data[self.code_start:]

        for pattern in self.PROLOGUE_PATTERNS:
            pos = 0
            while True:
                idx = code.find(pattern, pos)
                if idx == -1:
                    break

                offset = self.code_start + idx

                # Filter: PUSH BP alone needs additional context check
                if pattern == bytes([0x55]):
                    # Check if followed by MOV BP, SP or reasonable code
                    if idx + 1 < len(code):
                        next_byte = code[idx + 1]
                        # Only accept if followed by stack frame setup or common opcodes
                        if next_byte not in [0x8B, 0x89, 0x83, 0x56, 0x57, 0x1E, 0x06]:
                            pos = idx + 1
                            continue

                prologues.add(offset)
                pos = idx + 1

        return prologues

    def find_call_targets(self) -> Set[int]:
        """Find all CALL instruction targets"""
        targets = set()
        code = self.data[self.code_start:]

        i = 0
        while i < len(code) - 3:
            opcode = code[i]

            if opcode == self.CALL_NEAR_REL16:
                # CALL rel16 - 3 byte instruction
                if i + 3 <= len(code):
                    rel16 = struct.unpack_from('<h', code, i + 1)[0]  # signed
                    # Target = current position + 3 (instruction size) + relative offset
                    target = self.code_start + i + 3 + rel16
                    if 0 <= target < len(self.data):
                        targets.add(target)
                        source = self.code_start + i
                        if target not in self.call_sources:
                            self.call_sources[target] = []
                        self.call_sources[target].append(source)
                i += 3

            elif opcode == self.CALL_FAR_PTR:
                # CALL ptr16:16 - 5 byte instruction (offset:segment)
                if i + 5 <= len(code):
                    offset = struct.unpack_from('<H', code, i + 1)[0]
                    segment = struct.unpack_from('<H', code, i + 3)[0]
                    # Convert segment:offset to linear
                    target = segment * 16 + offset
                    if 0 <= target < len(self.data):
                        targets.add(target)
                        source = self.code_start + i
                        if target not in self.call_sources:
                            self.call_sources[target] = []
                        self.call_sources[target].append(source)
                i += 5

            elif opcode == self.CALL_RM16:
                # CALL r/m16 - indirect call, skip for now
                # Need to decode modrm byte to know instruction length
                i += 1
            else:
                i += 1

        return targets

    def find_interrupt_handlers(self) -> Dict[int, List[str]]:
        """Find INT instructions and map which functions use which interrupts"""
        int_usage: Dict[int, List[str]] = {}
        code = self.data[self.code_start:]

        i = 0
        while i < len(code) - 1:
            if code[i] == 0xCD:  # INT imm8
                int_num = code[i + 1]
                offset = self.code_start + i
                int_str = f"0x{int_num:02X}"

                # Find which function this belongs to
                func_addr = self._find_containing_function(offset)
                if func_addr is not None:
                    if func_addr not in int_usage:
                        int_usage[func_addr] = []
                    if int_str not in int_usage[func_addr]:
                        int_usage[func_addr].append(int_str)
                i += 2
            else:
                i += 1

        return int_usage

    def _find_containing_function(self, addr: int) -> Optional[int]:
        """Find the function that contains the given address"""
        # Simple heuristic: find nearest function start that's before this address
        candidates = [f for f in self.functions.keys() if f <= addr]
        if not candidates:
            return None
        return max(candidates)

    def estimate_function_sizes(self):
        """Estimate function sizes based on next function start or RET instruction"""
        sorted_addrs = sorted(self.functions.keys())

        for i, addr in enumerate(sorted_addrs):
            # Find next function start
            if i + 1 < len(sorted_addrs):
                next_func = sorted_addrs[i + 1]
            else:
                next_func = len(self.data)

            # Find first RET instruction
            code = self.data[addr:next_func]
            ret_offset = None
            for j, b in enumerate(code):
                if b in [self.RET_NEAR, self.RET_NEAR_IMM, self.RET_FAR, self.RET_FAR_IMM]:
                    ret_offset = j + 1  # Include the RET instruction
                    break

            if ret_offset:
                self.functions[addr].size_bytes = ret_offset
            else:
                self.functions[addr].size_bytes = min(next_func - addr, 0xFFFF)

    def classify_subsystem(self, addr: int, int_usage: List[str]) -> str:
        """Attempt to classify function subsystem based on interrupts and patterns"""
        # Video/graphics
        if "0x10" in int_usage:
            return "video"
        # DOS services
        if "0x21" in int_usage:
            return "dos"
        # Keyboard
        if "0x16" in int_usage:
            return "input"
        # Timer
        if "0x08" in int_usage or "0x1C" in int_usage:
            return "timer"
        # Mouse
        if "0x33" in int_usage:
            return "mouse"
        # Sound
        if "0x80" in int_usage or "0x81" in int_usage:
            return "sound"

        return "unknown"

    def analyze(self):
        """Run full analysis"""
        print("\nAnalyzing GP.EXE...")

        # Step 1: Add entry point
        entry_info = FunctionInfo(
            name="entry",
            entry_linear=f"0x{self.entry_point:X}",
            notes="Program entry point"
        )
        self.functions[self.entry_point] = entry_info
        print(f"  Added entry point at 0x{self.entry_point:X}")

        # Step 2: Find prologue patterns
        prologues = self.find_prologue_patterns()
        print(f"  Found {len(prologues)} potential prologues")

        # Step 3: Find CALL targets
        call_targets = self.find_call_targets()
        print(f"  Found {len(call_targets)} CALL targets")

        # Step 4: Merge candidates (CALL targets that also have prologues are high confidence)
        high_confidence = prologues & call_targets
        print(f"  High confidence functions (prologue + call target): {len(high_confidence)}")

        # Add high confidence functions (must be in code section)
        for addr in high_confidence:
            if addr not in self.functions and addr >= self.code_start:
                callers = [f"0x{c:X}" for c in self.call_sources.get(addr, [])]
                self.functions[addr] = FunctionInfo(
                    name=f"FUN_{addr:05X}",
                    entry_linear=f"0x{addr:X}",
                    callers=callers,
                    notes="auto-discovered (prologue+call)"
                )

        # Add remaining call targets (medium confidence, must be in code section)
        for addr in call_targets - high_confidence:
            if addr not in self.functions and addr >= self.code_start:
                callers = [f"0x{c:X}" for c in self.call_sources.get(addr, [])]
                self.functions[addr] = FunctionInfo(
                    name=f"FUN_{addr:05X}",
                    entry_linear=f"0x{addr:X}",
                    callers=callers,
                    notes="auto-discovered (call target)"
                )

        print(f"  Total functions discovered: {len(self.functions)}")

        # Step 5: Find interrupt usage
        int_usage = self.find_interrupt_handlers()
        for addr, ints in int_usage.items():
            if addr in self.functions:
                self.functions[addr].int_usage = ints
                self.functions[addr].subsystem = self.classify_subsystem(addr, ints)

        # Step 6: Estimate function sizes
        self.estimate_function_sizes()

        # Count subsystems
        subsystems = {}
        for f in self.functions.values():
            subsystems[f.subsystem] = subsystems.get(f.subsystem, 0) + 1
        print(f"\nSubsystem breakdown:")
        for sub, count in sorted(subsystems.items()):
            print(f"  {sub}: {count}")

    def export_json(self, output_path: str):
        """Export discovered functions to JSON format compatible with Ghidra script"""
        output = {
            "program_name": "GP.EXE",
            "language": "x86:LE:16:Real Mode / default",
            "binary_hash": f"sha256:{self.compute_sha256()}",
            "generated_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
            "functions": []
        }

        # Sort by address
        for addr in sorted(self.functions.keys()):
            f = self.functions[addr]
            output["functions"].append({
                "name": f.name,
                "entry_linear": f.entry_linear,
                "size_bytes": f.size_bytes,
                "stack_frame_size": f.stack_frame_size,
                "subsystem": f.subsystem,
                "callers": f.callers[:10],  # Limit to 10 callers
                "int_usage": f.int_usage,
                "source_recording_id": f.source_recording_id,
                "notes": f.notes
            })

        Path(output_path).parent.mkdir(parents=True, exist_ok=True)
        with open(output_path, 'w') as f:
            json.dump(output, f, indent=2)

        print(f"\nExported {len(output['functions'])} functions to {output_path}")

    def export_coverage_summary(self, output_path: str):
        """Export coverage summary markdown"""
        subsystems = {}
        int_counts = {}

        for f in self.functions.values():
            subsystems[f.subsystem] = subsystems.get(f.subsystem, 0) + 1
            for i in f.int_usage:
                int_counts[i] = int_counts.get(i, 0) + 1

        md = f"""# Reverse Engineering Coverage Summary

Total functions: {len(self.functions)}
Named functions: {sum(1 for f in self.functions.values() if not f.name.startswith('FUN_'))}

## Breakdown by subsystem

"""
        for sub, count in sorted(subsystems.items(), key=lambda x: -x[1]):
            md += f"- **{sub}**: {count} functions\n"

        md += "\n## INT usage counts\n\n"
        if int_counts:
            for int_num, count in sorted(int_counts.items()):
                md += f"- **{int_num}**: {count} functions\n"
        else:
            md += "- (none recorded)\n"

        Path(output_path).parent.mkdir(parents=True, exist_ok=True)
        with open(output_path, 'w') as f:
            f.write(md)

        print(f"Exported coverage summary to {output_path}")


def main():
    # Find repo root
    script_dir = Path(__file__).parent
    repo_root = script_dir.parent.parent  # re/scripts -> re -> repo root

    exe_path = repo_root / "f1gp-disasm" / "GP.EXE"
    if not exe_path.exists():
        print(f"Error: GP.EXE not found at {exe_path}")
        return 1

    analyzer = DOSExeAnalyzer(str(exe_path))
    analyzer.analyze()

    # Export results
    symbols_path = repo_root / "re" / "artifacts" / "symbols.json"
    coverage_path = repo_root / "re" / "artifacts" / "coverage_summary.md"

    analyzer.export_json(str(symbols_path))
    analyzer.export_coverage_summary(str(coverage_path))

    return 0


if __name__ == "__main__":
    exit(main())
