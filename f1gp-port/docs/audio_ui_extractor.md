# Audio & UI Extractor Workflow

_Last updated: 2025-11-19_

The `audio_ui_extractor` crate provides two subcommands for converting original F1GP assets into open formats while
preserving provenance via manifests.

## 1. PCM Conversion

```bash
cargo run -p audio_ui_extractor -- pcm \
  --source assets/original/HARDDISK/SAMPLES \
  --dest build/audio \
  --sample-rate 11025 --channels 1
```

- Recursively finds files with `.pcm`/`.snd` (configurable via `--extensions`).
- Produces WAV files that match the directory structure under `--dest` and writes `pcm_manifest.json` capturing counts,
  hashes, sample rate, and channel configuration.
- Use `--dry-run`? (planned). For now rerun to overwrite existing outputs.

## 2. Font Atlas Generation

```bash
cargo run -p audio_ui_extractor -- font \
  --input assets/original/FONT.BIN \
  --glyph-width 8 --glyph-height 16 \
  --glyph-count 256 --columns 16 \
  --dest build/fonts --name vga_font --include-metadata
```

- Treats the binary as packed 1bpp glyphs, emits `<name>_atlas.png` plus `<name>_font_manifest.json` documenting hashes,
  glyph dimensions, and optional placement metadata.
- Supports bit inversion (`--invert-bits`) and color overrides via hex strings.

## 3. CI / Provenance
- Store WAV/PNG outputs outside the repository; keep only the manifests to prove origin and detect drift.
- `scripts/run_ci.sh` ensures fixtures are generated but does not run the PCM/font converter (requires proprietary data).
  When validating locally, run the relevant subcommands and attach manifests to review notes.

## 4. Next Steps
- Add spectral comparison tooling for PCM outputs.
- Automate hashed bundles for build reproducibility.
