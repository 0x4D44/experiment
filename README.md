# Experiment Monorepo

A single home for all of the AI tooling, MCP servers, and research projects being explored by the 0x4D44 team. The repository now ships with a clean `main` branch that contains every completed branch and their deliverables.

## What's Inside
- Production-quality **MCP servers** (eBay search, stock prices, Udio music) implemented in Rust.
- A **modern port** of Geoff Crammond's Formula 1 Grand Prix including extracted original assets.
- Developer productivity tools such as the **`jrnrvw` journal reviewer** and the **`mdperf` benchmarking harness**.
- Narrative coding experiments, design documents, and extensive work journals that capture the thought process behind each milestone.

## Repository Map
| Path | Purpose | Primary Tech | Typical Commands |
| --- | --- | --- | --- |
| `f1gp-port/` | Playable Rust reimplementation of the classic F1 Grand Prix, including physics, AI, renderer, and tooling. | Rust + WGPU + SDL2 | `cd f1gp-port && cargo test` · `cargo run --bin f1gp`
| `f1gp-data/` | Extracted canonical track/car `.DAT` files used by the port for validation. | Data assets | N/A (inputs for `f1gp-port` tests) |
| `f1gp-orig/` | Original 1996 ISO kept for archival/reference when reverse-engineering assets. | Binary data | Mount/read only |
| `ebay-mcp/` | MCP server that lets assistants perform eBay searches with headless browser automation, caching, and history tracking. | Rust + headless browser tooling | `cd ebay-mcp && cargo test` |
| `stock-price-mcp/` | MCP server that scrapes Yahoo Finance for live quotes and meta data. | Rust + reqwest/scraper | `cd stock-price-mcp && cargo test` |
| `udio-mcp/` | Experimental MCP server for interacting with the Udio music platform plus a comprehensive MCP guide. | Rust + tokio | `cd udio-mcp && cargo test` |
| `jrnrvw/` | CLI for discovering and analyzing markdown journals (`yyyy.mm.dd - JRN - …`). Supports filtering, grouping, and multiple output formats. | Rust | `cd jrnrvw && cargo test` |
| `mdperf/` | Benchmark + visualization harness (benchctl) that captures repeatable CPU/memory/disk/net metrics. | Rust | `cd mdperf && cargo test` (bench data lives in `bench_report.json`) |
| `coding-challenge-01/` | Collection of self-contained Rust CLI "games" exploring AI cognition themes (Consciousness Compiler, Stream of Consciousness, etc.). | Rust | See each game directory README; typically `cargo run --bin <game>` |
| `fin-mcp/` | Planning docs for an upcoming finance-focused MCP server. | Docs | N/A |
| `wrk/`, `wrk_docs/`, `wrk_journals/`, `wrk_jrn/` | Research reports, work journals, and planning artifacts shared across projects. | Markdown | N/A |

> **Licensing:** Each sub-project keeps its own license (for example `f1gp-port/LICENSE`). Consult the specific directory before redistributing code.

## Getting Started
1. **Clone & toolchains**
   ```bash
   git clone https://github.com/0x4D44/experiment.git
   cd experiment
   rustup toolchain install stable
   rustup default stable
   ```
2. **Install native deps (only needed for graphics/audio projects):** SDL2 (`libsdl2-dev`), system OpenGL/Vulkan drivers, and audio backends so that `f1gp-port` can link successfully.
3. **Per-project configuration:**
   - MCP servers read JSON over stdio; see `claude_desktop_config.json.example` inside each server folder for ready-to-use Claude Desktop entries.
   - `mdperf` expects permission to create temp files (`/tmp/mdperf-*`).
   - `f1gp-port` uses the assets under `f1gp-data/`; keep that directory next to the executable or set `F1GP_DATA_ROOT` if you relocate it.

## Build & Test Shortcuts
The repo does not use a single workspace manifest, so run commands inside individual sub-directories. Common checks:
```bash
cd f1gp-port && cargo test
cd ebay-mcp && cargo test
cd stock-price-mcp && cargo test
cd jrnrvw && cargo test
cd mdperf && cargo test
```
All of the commands above were run on 2025-11-17 after consolidating the branches, so you can treat them as a known-good baseline.

## Branching & Workflow
- `main` is now the default and only active branch; every historical `claude/*` branch has been merged and deleted.
- Use feature branches for new work, then open PRs against `main`.
- Large documents, journals, and ADRs live under the `wrk*` folders so code directories can stay focused.

## Working With Docs & Journals
- Project journals follow the `YYYY.MM.DD - JRN - <summary>.md` convention. Tools like `jrnrvw` rely on that naming to auto-parsed status reports.
- `wrk_docs/` holds plans, ADRs, and specifications (for example the comprehensive MCP guides).
- `wrk_journals/` contain session-by-session progress updates.
- Keep binary assets (such as the F1GP ISO) outside of Git LFS for now; they live in `f1gp-orig/` for forensic parity with previous work.

## Contributing
1. Fork or create a new feature branch off `main`.
2. Update the relevant directory README or docs when adding new capabilities.
3. Run the project-specific `cargo test` (and `cargo fmt`/`cargo clippy` if applicable) before opening a PR.
4. Document major research findings in `wrk_docs/` or `wrk_journals/` so future contributors can follow the intent behind complex changes.

Have fun! This repository is intentionally exploratory—ship fast, document everything, and prefer merging improvements back into `main` so the experiments stay discoverable.
