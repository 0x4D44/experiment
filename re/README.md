# Reverse Engineering Tooling

This tree houses all artifacts, tooling, and documentation required to implement the Ghidra + Binary Ninja reverse engineering pipeline described in `wrk_docs/2025.11.19 - HLD - Ghidra Binary Ninja Port Strategy.md` and the corresponding implementation plan.

## Layout
- `ghidra/` – Project templates, scripts, and contribution guidelines for analysts.
- `binary_ninja/` – Loader plugins, workflows, and export tooling for HLIL generation.
- `artifacts/` – Machine-generated outputs (symbols, segments, HLIL exports, logs). Keep tracked files text-based; binaries go through Git LFS as needed.
- `scripts/` – Cross-tool automation (e.g., `refresh_artifacts.py`) and schema validation utilities.
- `docs/` – Supporting documentation (CI overview, telemetry harness specs, SOP references).

Additional directories (e.g., telemetry captures, dashboards) will be added in later stages per the plan.
