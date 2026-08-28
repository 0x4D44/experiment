#!/usr/bin/env python3
"""Patch pinned m68k-rs with an embedding hook for external function-code buses."""

from __future__ import annotations

import pathlib
import sys


def replace_once(path: pathlib.Path, old: str, new: str) -> None:
    text = path.read_text()
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{path}: expected one patch anchor, found {count}: {old[:80]!r}")
    path.write_text(text.replace(old, new, 1))


def insert_before_all(path: pathlib.Path, needle: str, insertion: str, expected: int) -> None:
    text = path.read_text()
    count = text.count(needle)
    if count != expected:
        raise SystemExit(f"{path}: expected {expected} anchors, found {count}: {needle!r}")
    path.write_text(text.replace(needle, insertion + needle))


def insert_before_nth(path: pathlib.Path, needle: str, insertion: str, occurrence: int, expected: int) -> None:
    text = path.read_text()
    count = text.count(needle)
    if count != expected:
        raise SystemExit(f"{path}: expected {expected} anchors, found {count}: {needle!r}")
    start = -1
    for _ in range(occurrence):
        start = text.find(needle, start + 1)
        if start < 0:
            raise SystemExit(f"{path}: occurrence {occurrence} not found: {needle!r}")
    path.write_text(text[:start] + insertion + text[start:])


def main(root: pathlib.Path) -> None:
    memory = root / "src/core/memory.rs"
    cpu = root / "src/core/cpu.rs"
    ea = root / "src/core/ea.rs"
    exceptions = root / "src/core/exceptions.rs"
    execute = root / "src/core/execute.rs"

    replace_once(
        memory,
        "    fn sync(&mut self, _cpu_clocks: u32) {}\n",
        "    fn sync(&mut self, _cpu_clocks: u32) {}\n\n"
        "    /// Select the MC680x0 function code for the next bus transfer.\n"
        "    /// External MMUs such as Sun-3 use this to distinguish user,\n"
        "    /// supervisor, program, data, and control-space cycles.\n"
        "    fn set_function_code(&mut self, _function_code: u8) {}\n",
    )

    replace_once(
        cpu,
        "    /// Read byte from memory (data space).\n",
        "    #[inline]\n"
        "    pub(crate) fn external_data_function_code(&self) -> u8 {\n"
        "        self.mmu_fc_override.unwrap_or(if self.is_supervisor() { 5 } else { 1 })\n"
        "    }\n\n"
        "    #[inline]\n"
        "    pub(crate) fn external_program_function_code(&self) -> u8 {\n"
        "        if self.is_supervisor() { 6 } else { 2 }\n"
        "    }\n\n"
        "    /// Read byte from memory (data space).\n",
    )

    replace_once(
        cpu,
        "        // Read initial SSP from vector 0\n        let ssp = bus.read_long(0);\n",
        "        // Reset vector fetches are supervisor-program cycles.\n"
        "        bus.set_function_code(6);\n"
        "        // Read initial SSP from vector 0\n"
        "        let ssp = bus.read_long(0);\n",
    )
    replace_once(
        cpu,
        "        // Read initial PC from vector 1\n        self.pc = bus.read_long(4);\n",
        "        // Read initial PC from vector 1\n"
        "        bus.set_function_code(6);\n"
        "        self.pc = bus.read_long(4);\n",
    )

    # All external data transfers in these helpers use SFC/DFC for MOVES and
    # the ordinary user/supervisor data function code otherwise.
    for needle, expected in [
        ("        match bus.try_read_byte(addr) {\n", 1),
        ("        match bus.try_read_long(addr) {\n", 1),
        ("        match bus.try_read_three_bytes(addr) {\n", 1),
        ("        if let Err(f) = bus.try_write_byte(addr, value)\n", 1),
        ("        if let Err(f) = bus.try_write_word(addr, value)\n", 1),
        ("        if let Err(f) = bus.try_write_three_bytes(addr, value & 0x00FF_FFFF)\n", 1),
        ("        if let Err(f) = bus.try_write_long(addr, value)\n", 1),
    ]:
        insert_before_all(cpu, needle, "        bus.set_function_code(self.external_data_function_code());\n", expected)

    insert_before_nth(
        cpu,
        "        match bus.try_read_word(addr) {\n",
        "        bus.set_function_code(self.external_program_function_code());\n",
        occurrence=1,
        expected=2,
    )
    insert_before_nth(
        cpu,
        "        match bus.try_read_word(addr) {\n",
        "        bus.set_function_code(self.external_data_function_code());\n",
        occurrence=2,
        expected=2,
    )

    insert_before_all(
        ea,
        "        match bus.try_read_immediate_word(addr) {\n",
        "        bus.set_function_code(self.external_program_function_code());\n",
        2,
    )
    insert_before_all(
        ea,
        "        match bus.try_read_immediate_long(addr) {\n",
        "        bus.set_function_code(self.external_program_function_code());\n",
        1,
    )

    replace_once(
        exceptions,
        "        self.dar[15] = self.dar[15].wrapping_sub(2);\n        bus.write_word(self.address(self.dar[15]), value);\n",
        "        self.dar[15] = self.dar[15].wrapping_sub(2);\n"
        "        bus.set_function_code(5);\n"
        "        bus.write_word(self.address(self.dar[15]), value);\n",
    )
    replace_once(
        exceptions,
        "        self.dar[15] = self.dar[15].wrapping_sub(4);\n        bus.write_long(self.address(self.dar[15]), value);\n",
        "        self.dar[15] = self.dar[15].wrapping_sub(4);\n"
        "        bus.set_function_code(5);\n"
        "        bus.write_long(self.address(self.dar[15]), value);\n",
    )

    replace_once(
        execute,
        "        let addr = (vector << 2).wrapping_add(self.vbr);\n        self.pc = self.read_32(bus, addr);\n",
        "        let addr = (vector << 2).wrapping_add(self.vbr);\n"
        "        let previous_fc = self.mmu_fc_override.replace(6);\n"
        "        self.pc = self.read_32(bus, addr);\n"
        "        self.mmu_fc_override = previous_fc;\n",
    )

    marker = root / ".sun3-function-code-patch"
    marker.write_text("external function-code hook applied\n")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise SystemExit("usage: patch_m68k.py PATH_TO_M68K_RS")
    main(pathlib.Path(sys.argv[1]))
