# Binary Ninja Workflow

## Headless Example
```
export F1GP_REPO_ROOT=/home/md/language/experiment
export F1GP_SUBSYSTEMS=physics,input
binaryninja-headless f1gp-disasm/GP.EXE out.bndb \
    -s re/binary_ninja/plugins/f1gp_loader.py \
    -s re/binary_ninja/plugins/import_symbols.py \
    -s re/binary_ninja/plugins/emit_hlil.py
```

## Checklist
1. Ensure `re/artifacts/symbols.json` and `segments.yml` are up to date.
2. Run loader + import scripts to reconstruct segments and symbols.
3. Execute `emit_hlil.py` for target subsystems; commit outputs under `re/artifacts/hlil/`.
4. Attach diff summary plus telemetry trace IDs in PR descriptions.
