# F1GP Modern Port

A modern reimplementation of the classic **Formula One Grand Prix** racing simulator by Geoff Crammond (MicroProse, 1991).

This project aims to port the legendary F1 racing game to modern platforms using Rust, while maintaining the authentic feel and behavior of the original game.

## Project Status

🚧 **Early Development** - Currently in Phase 1: Foundation & Data Extraction

### Current Progress
- ✅ Project structure initialized
- ✅ Rust workspace configured
- ✅ Core dependencies added
- ⏳ ISO extraction tools (in progress)
- ⏳ File format reverse engineering (planned)

## Features (Planned)

- **Native Performance**: Runs natively on modern Windows, Linux, and macOS
- **Authentic Physics**: Faithful recreation of the original car handling and physics
- **Modern Graphics**: Enhanced visuals while preserving the classic feel
- **Cross-Platform**: Play on any modern operating system
- **Moddable**: Open architecture for community modifications

## Original Game

**Formula One Grand Prix** (also known as **World Circuit**) was released in 1991 and is widely regarded as one of the greatest racing simulators ever created. It featured:

- All 16 circuits from the 1991 F1 season
- Revolutionary 3D graphics for its era
- Realistic physics and car handling
- Sophisticated AI opponents
- Comprehensive car setup options

## Technical Architecture

### Technology Stack
- **Language**: Rust (2021 edition)
- **Graphics**: SDL2 (initial), wgpu (future)
- **Audio**: rodio
- **Math**: glam (SIMD-accelerated)
- **Build System**: Cargo

### Project Structure
```
f1gp-port/
├── src/              # Main library and binary
│   ├── data/         # Data loading and file formats
│   ├── physics/      # Physics engine
│   ├── ai/           # AI system
│   ├── game/         # Game logic
│   ├── render/       # Rendering system
│   ├── audio/        # Audio engine
│   ├── platform/     # Platform abstractions
│   └── utils/        # Utilities
├── tools/            # Development tools
│   ├── extract_iso/  # ISO extraction utility
│   └── track_viewer/ # Track visualization tool
├── tests/            # Integration tests
├── benches/          # Performance benchmarks
├── assets/           # Game assets
└── docs/             # Documentation
```

## Building

### Prerequisites
- Rust 1.75+ (stable)
- Cargo

### Build Instructions
```bash
# Clone the repository
git clone https://github.com/yourusername/f1gp-port.git
cd f1gp-port

# Build the project
cargo build --release

# Run the game
cargo run --release
```

## Development Roadmap

See [Implementation Plan](../wrk_docs/2025.11.14%20-%20PLN%20-%20F1GP%20Modern%20Port%20Implementation.md) for detailed roadmap.

### Phase 1: Foundation (Weeks 1-3) - *Current*
- ISO extraction and data analysis
- File format reverse engineering
- Track and car data loaders

### Phase 2: Graphics (Weeks 4-6)
- Rendering system
- Track and car visualization
- Camera system

### Phase 3: Physics & Gameplay (Weeks 7-10)
- Physics engine
- Car dynamics
- Playable game

### Phase 4: AI & Complete Race (Weeks 11-14)
- AI opponents
- Race sessions
- Audio system

### Phase 5: Polish (Weeks 15-16)
- Championship mode
- Car setup
- UI polish
- Performance optimization

### Phase 6: Post-Release (Ongoing)
- Cross-platform support
- Advanced features
- Community contributions

## Contributing

This project is in early development. Contributions are welcome once the core architecture is established.

## License

This project is licensed under the GNU General Public License v3.0 - see the LICENSE file for details.

**Important**: This is a clean-room reimplementation. The original game assets and code are copyrighted by their respective owners. This project does not distribute any copyrighted materials.

## Credits

- **Original Game**: Geoff Crammond (Design), MicroProse (Publisher)
- **This Port**: F1GP Port Team and contributors

## Resources

- [Original Game Information](https://en.wikipedia.org/wiki/Formula_One_Grand_Prix_(video_game))
- [F1GP Community](https://sites.google.com/view/f1gpwc)
- [Implementation Plan](../wrk_docs/2025.11.14%20-%20PLN%20-%20F1GP%20Modern%20Port%20Implementation.md)

## Disclaimer

This project is a fan-made recreation and is not affiliated with, endorsed by, or connected to the original creators or publishers of Formula One Grand Prix. All trademarks and copyrights belong to their respective owners.
