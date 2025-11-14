# Contributing to F1GP Modern Port

Thank you for your interest in contributing to the F1GP Modern Port project!

## Project Status

This project is currently in **early development** (Phase 1). The core architecture is being established, and we're focusing on:

1. Extracting and understanding the original game data
2. Reverse engineering file formats
3. Building the foundation for the physics engine
4. Creating the rendering system

## How to Contribute

### Reporting Issues
- Use GitHub Issues to report bugs or suggest features
- Provide detailed information about your environment
- Include steps to reproduce any bugs

### Code Contributions

#### Prerequisites
- Rust 1.75+ installed
- Familiarity with game development concepts
- Understanding of the original F1GP game (helpful)

#### Development Setup
```bash
# Clone the repository
git clone https://github.com/yourusername/f1gp-port.git
cd f1gp-port

# Build the project
cargo build

# Run tests
cargo test

# Run the binary
cargo run --bin f1gp
```

#### Code Style
- Follow Rust standard style guidelines
- Run `cargo fmt` before committing
- Run `cargo clippy` and address warnings
- Write doc comments for public APIs
- Add unit tests for new functionality

#### Commit Messages
- Use clear, descriptive commit messages
- Start with a verb in present tense (e.g., "Add", "Fix", "Update")
- Reference issue numbers when applicable

Example:
```
Add track data loader for F1CT format

Implements basic parser for track files including segment data,
elevation map, and racing line information.

Closes #123
```

### Pull Request Process

1. **Fork** the repository
2. **Create a branch** for your feature (`git checkout -b feature/amazing-feature`)
3. **Make your changes** following the code style guidelines
4. **Run tests** (`cargo test`) and ensure they pass
5. **Run clippy** (`cargo clippy`) and address any warnings
6. **Commit your changes** with clear messages
7. **Push to your fork** (`git push origin feature/amazing-feature`)
8. **Open a Pull Request** with a clear description

### What to Contribute

#### High Priority
- File format documentation
- Data loaders (track, car, driver data)
- Physics algorithms (based on original game)
- Test cases and validation

#### Medium Priority
- Performance optimizations
- Documentation improvements
- Code refactoring
- Bug fixes

#### Future Priorities
- Advanced features (multiplayer, VR, etc.)
- Modding tools
- Cross-platform testing

## Code Review Process

All contributions will be reviewed by project maintainers. We look for:

- **Correctness**: Does the code work as intended?
- **Quality**: Is the code clean, readable, and maintainable?
- **Testing**: Are there adequate tests?
- **Documentation**: Is the code properly documented?
- **Compatibility**: Does it maintain authenticity with the original game?

## Community Guidelines

- Be respectful and constructive
- Help newcomers learn
- Focus on the code, not the person
- Assume good intentions
- Follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct)

## Legal Considerations

### Clean-Room Implementation
This project is a **clean-room reimplementation**. Contributors must:

- NOT copy code from the original game
- NOT decompile or disassemble the original game executable
- Use only publicly available information
- Reverse engineer through black-box testing (observe behavior, not code)

### Copyrighted Assets
- Do NOT commit original game assets (graphics, sounds, data files)
- Do NOT distribute copyrighted materials
- Users must obtain original game files legally

### License
By contributing, you agree that your contributions will be licensed under the GNU General Public License v3.0.

## Getting Help

- **Questions**: Open a GitHub Discussion
- **Documentation**: Check the `docs/` directory and implementation plan
- **Community**: Join our community forum (link TBD)

## Recognition

Contributors will be recognized in:
- The project README
- Release notes
- In-game credits (when applicable)

Thank you for helping preserve and modernize this classic racing game!
