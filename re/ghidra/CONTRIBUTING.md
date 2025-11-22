# Contributing to the F1GP Ghidra Workspace

This guide defines the workflow for analysts working on the `GP.EXE` disassembly. Follow these requirements before committing any Ghidra artifacts or exporter outputs.

## 1. Prerequisites
- Ghidra 11.0 (or later) installed locally with OpenJDK 17.
- Git LFS configured (`git lfs install`) so `.gpr` and other large artifacts sync correctly.
- Access to `f1gp-disasm/GP.EXE` (SHA256 `0e805c2e11fe...`) from `re/SOURCE_MANIFEST.md`.

## 2. Project Template Usage
1. Copy `re/ghidra/project_template` to a writable workspace.
2. Launch Ghidra and import `GP.EXE` using the template project.
3. Ensure the language is `x86:LE:16:Real Mode` and enable overlays according to `re/ghidra/project_template/overlays.yml` (placeholder until Stage 1.2).

## 3. Analyzer Configuration
Enable the following analyzers (others remain default/off):
| Analyzer | Setting |
|----------|---------|
| Function ID | Enabled |
| x86 Emulation | Enabled |
| Constant Reference Analyzer | Enabled |
| Reference | Enabled |
| Stack | Enabled |
| Scalar Operand References | Enabled |
| PCode Propagation | Enabled |
| Symbol Analysis | Enabled |
| String Recovery | Enabled |
| Data Reference | Enabled |
| Demangler | Disabled (not useful for raw 16-bit) |

Record deviations in the project log.

## 4. Naming & Commenting
- Functions: `Subsystem_ActionDetail` (e.g., `Physics_UpdateSuspension`).
- Data: `DATA_<Context>_<Description>` (e.g., `DATA_Audio_AdlibPatchTable`).
- Interrupt handlers: `BIOS_IntXX_<Purpose>`.
- Add header comment describing side effects, register usage, dependencies, telemetry trace IDs.
- Tag subsystem using `@subsystem:<name>` comment prefix so exporter scripts can parse it.

## 5. Annotation Workflow
1. Identify entry point(s) for target subsystem.
2. Create functions for any unlabeled code blocks (use `P` hotkey).
3. Convert data references to labeled data types.
4. Add comments for INT instructions (`INT 10h`, etc.) noting expected host translation.
5. Run `ExportSymbolsToJson` script (Stage 1 deliverable) after finishing a work session.
6. Review diff of `re/artifacts/symbols.json`/`segments.yml` before committing.

## 6. Reviews & Quality Gates
- Every PR must include artifact diffs plus summary of coverage improvements.
- Attach at least one screenshot or textual proof for new subsystem annotations.
- Verify Git LFS pointers are committed, not raw `.gpr` data.
- Coordinate with Binary Ninja owners when renaming functions that are already consumed downstream.

Further SOP details will be captured in `wrk_docs/2025.11.19 - SOP - RE Contribution Guidelines (Outline).md` as Stage 1 progresses.
