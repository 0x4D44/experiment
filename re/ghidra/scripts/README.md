# Ghidra Script Inventory

| Script | Language | Purpose |
|--------|----------|---------|
| `ExportSymbolsToJson.java` | Java | Traverse all functions, export metadata to `re/artifacts/symbols.json`. Honors `F1GP_REPO_ROOT` or defaults to current directory. |
| `ExportSegmentsToYaml.java` | Java | Dump memory block layout and permissions to `re/artifacts/segments.yml`. |
| `TagInterruptUsage.py` | Python | Annotate INT instructions with `@interrupt` comments for exporter consumption. |
| `EmitTypeDecls.py` | Python | Emit `/F1GP` data type declarations into `re/artifacts/types/f1gp_types.h`. |

Scripts assume they are executed inside the Ghidra project configured per `re/ghidra/CONTRIBUTING.md`.
