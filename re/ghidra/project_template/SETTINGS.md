# Project Template Settings (Placeholder)

This document captures the configuration that will be applied once a real `.gpr` template is generated.

## Language & Processor
- Language: `x86:LE:16:Real Mode`
- Compiler Spec: `default`

## Memory Map Seeds
```
Segment  File Offset  Size    Notes
-------  -----------  ------  ------------------------------------
0000     0x0000       0x200   PSP (auto-generated)
0001     0x0200       ...     GP.EXE code+data (see `segments.yml` once exported)
```

## Overlay Configuration (Draft)
- Overlay Manager: Manual
- Bank descriptors to be imported from `re/artifacts/segments.yml` when available.

## Analyzer Defaults
Reference `re/ghidra/CONTRIBUTING.md` Section 3.

## To-Do
- [ ] Capture final segment table once `ExportSegmentsToYaml` is functional.
- [ ] Store `.gpr` under Git LFS with matching version metadata json.
