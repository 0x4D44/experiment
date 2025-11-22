# Repository Guidelines

## Project Structure & Modules
- Root crate lives in `src/` (`main.rs`, `lib.rs`) with feature areas split by folder: `ai/`, `physics/`, `game/`, `render/`, `render3d/`, `ui/`, `platform/`, `data/`, `telemetry/`, `audio/`, `utils/`.
- Workspace tools are under `tools/` (e.g., `asset_extractor`, `telemetry_cli`, `racing_line_cli`, `sprite_atlas_cli`, `track_viewer`). Each is a Cargo package you can run with `cargo run -p <tool>`.
- Integration tests live in `tests/`; fixtures and sanitized inputs sit in `data/fixtures/` and `data/samples/`. Documentation resides in `docs/`.

## Build, Test & Development Commands
- `cargo build --release` – build the game binary (`target/release/f1gp`).
- `cargo run --release` – launch the game; pass `-- --help` for runtime flags.
- `cargo test --workspace` – run all unit + integration tests.
- `cargo clippy --workspace --all-targets -- -D warnings` – lint (CI blocks on warnings).
- `cargo fmt --all` – format Rust sources.
- `./scripts/run_ci.sh` – regenerates fixtures, runs clippy, fmt check, and the full workspace test suite (use before PRs).
- Example tool runs: `cargo run -p telemetry_cli -- summary --input telemetry/<file>`; `cargo run -p asset_extractor -- --source <path> --dest assets/original/tracks`.

## Coding Style & Naming Conventions
- Rust 2021 edition; 4-space indentation; prefer expressive, small modules per domain folder.
- Follow standard Rust casing: modules/files `snake_case`, types `PascalCase`, constants `SCREAMING_SNAKE_CASE`.
- Keep functions small; favor `anyhow::Result` for fallible flows and `thiserror` for library errors.
- Run `cargo fmt` before commits; treat Clippy warnings as failures.

## Testing Guidelines
- Place unit tests alongside modules with `#[cfg(test)]`; integration/fixture-heavy tests go in `tests/`.
- When touching parsing, physics, or rendering pipelines, update/extend fixtures in `data/fixtures/` using `scripts/build_fixtures.sh` or targeted tool commands.
- Aim to keep existing test set green; add regression tests for reproduced bugs and parity tests when changing telemetry or track loading.

## Asset & Data Handling
- Do **not** commit copyrighted game assets. Use sanitized fixtures in `data/fixtures/` and generated outputs from the toolchain.
- Track provenance with manifests produced by extractors (see `docs/data_pipeline.md`); prefer environment overrides like `F1GP_DRIVER_DB_PATH` and `F1GP_TELEMETRY=off` instead of editing tracked data.

## Commit & Pull Request Guidelines
- Commit messages: present-tense verb + scope (e.g., `Add collision damping for kerbs`), reference issues when applicable.
- PRs should include: summary of change, test/CI results (or a note if not run), and screenshots/video for visual changes or telemetry deltas for parity work.
- Keep diffs focused; if a change affects assets or fixtures, describe how they were produced and which tool/command was used.
