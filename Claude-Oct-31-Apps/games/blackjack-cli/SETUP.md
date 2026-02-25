# Blackjack CLI - Setup and Build Guide

This guide will help you set up, build, and run the Blackjack CLI game.

## Prerequisites

### System Requirements
- Linux, macOS, or Windows (with WSL2)
- 512 MB disk space for Zig and build artifacts
- Terminal/console with Unicode support

### Install Zig

The Blackjack CLI requires **Zig 0.13.0 or later**.

#### Option 1: Official Zig Binary (Recommended)

1. Download from https://ziglang.org/download/
   - Select your platform (Linux x86_64, macOS, Windows)

2. Extract the archive:
   ```bash
   tar -xf zig-linux-x86_64-*.tar.xz
   cd zig-linux-x86_64-*
   ```

3. Add to PATH:
   ```bash
   export PATH="$(pwd):$PATH"
   ```

4. Verify installation:
   ```bash
   zig version
   ```

#### Option 2: Build from Source

1. Install dependencies (Ubuntu/Debian):
   ```bash
   sudo apt-get install cmake llvm-dev clang lld
   ```

2. Clone and build:
   ```bash
   git clone https://github.com/ziglang/zig.git
   cd zig
   mkdir build && cd build
   cmake ..
   make
   ./zig version
   ```

#### Option 3: Package Manager

**Ubuntu/Debian**:
```bash
sudo apt-get install zig
```

**macOS**:
```bash
brew install zig
```

**Arch Linux**:
```bash
sudo pacman -S zig
```

## Building the Project

### 1. Navigate to Project Directory
```bash
cd /path/to/games/blackjack-cli
```

### 2. Build the Game

**Debug Build** (faster compilation, larger binary):
```bash
zig build
```

**Release Build** (slower compilation, optimized binary):
```bash
zig build -Doptimize=ReleaseSafe
```

**Fastest Executable**:
```bash
zig build -Doptimize=ReleaseFast
```

The compiled executable will be at:
```
zig-cache/bin/blackjack
```

### 3. Run the Game

```bash
zig build run
```

Or directly run the binary:
```bash
./zig-cache/bin/blackjack
```

### 4. Run Tests

```bash
zig build test
```

This runs the comprehensive test suite including:
- Deck shuffling and management tests
- Hand evaluation tests
- Payout calculation tests
- Game logic tests
- Integration tests

## Build Configuration

The `build.zig` file includes several build steps:

```bash
zig build                  # Default: builds the game
zig build run             # Build and run the game
zig build test            # Run the test suite
zig build -Doptimize=X    # Use optimization level X
```

### Optimization Levels
- `Debug`: Fast compilation, slow execution
- `ReleaseSafe`: Balanced (safety checks, optimizations)
- `ReleaseFast`: Maximum speed (may disable safety)
- `ReleaseSmall`: Smallest binary size

## Troubleshooting

### Issue: "zig: command not found"

**Solution**: Zig is not in your PATH
```bash
# Find where Zig is installed
which zig

# If not found, download and add to PATH
export PATH="/path/to/zig:$PATH"
```

### Issue: Build fails with "Unresolved reference to..."

**Solution**: Ensure all source files are present
```bash
ls src/
# Should show: config.zig, deck.zig, game.zig, hand.zig, main.zig, ui.zig, game_test.zig
```

### Issue: Build cache errors

**Solution**: Clear and rebuild
```bash
rm -rf zig-cache build/
zig build
```

### Issue: Terminal doesn't clear properly

**Solution**: Your terminal doesn't support ANSI codes
- Use a modern terminal (xterm, GNOME Terminal, iTerm2, Windows Terminal)
- Or update terminal emulator

### Issue: Cards not displaying (missing Unicode characters)

**Solution**: Terminal doesn't support Unicode
- Use UTF-8 capable terminal
- Check locale: `echo $LANG` (should contain UTF-8)
- Update terminal: `export LANG=en_US.UTF-8`

## Development Workflow

### During Development

Watch for changes and rebuild:
```bash
# Rebuild on file changes (requires 'watch' tool)
watch -n 1 zig build
```

Or use a simple loop:
```bash
while true; do zig build && clear; sleep 1; done
```

### Testing During Development

Run tests frequently:
```bash
zig build test 2>&1 | less
```

Run specific test:
```bash
zig build test 2>&1 | grep "hand value"
```

### Profiling

Build with debug info:
```bash
zig build -Doptimize=Debug
```

### Memory Checking

The code uses Zig's GeneralPurposeAllocator which catches memory issues:
```bash
zig build -Doptimize=Debug
MALLOC_CHECK_=3 ./zig-cache/bin/blackjack
```

## Platform-Specific Notes

### Linux
- No additional configuration needed
- Most distributions have ANSI terminal support
- WSL2 works excellent for Windows users

### macOS
- Zig works great on both Intel and Apple Silicon
- Terminal.app supports ANSI codes
- iTerm2 recommended for better support

### Windows
- Use Windows Terminal (recommended)
- Or WSL2 (Windows Subsystem for Linux 2)
- Does NOT work with CMD.exe or old PowerShell
- Install WSL2 first: https://docs.microsoft.com/en-us/windows/wsl/install

### WSL2 Setup (Windows)
```bash
# In PowerShell as Administrator
wsl --install

# Then in WSL2 terminal
sudo apt update
sudo apt install zig
git clone <repo>
cd games/blackjack-cli
zig build run
```

## Project File Structure

After successful build:
```
games/blackjack-cli/
├── src/
│   ├── main.zig          # Entry point
│   ├── config.zig        # Constants and types
│   ├── deck.zig          # Card/Deck management
│   ├── hand.zig          # Hand evaluation
│   ├── game.zig          # Game logic
│   ├── ui.zig            # Terminal UI
│   └── game_test.zig     # Test suite
├── build.zig             # Build configuration
├── zig-cache/            # Build artifacts (created)
│   └── bin/
│       └── blackjack     # Executable (created)
├── HLD.md                # Design document
├── README.md             # Game documentation
└── SETUP.md              # This file
```

## Performance Benchmarks

Typical build times on modern hardware:

| Build Type | Time |
|-----------|------|
| Debug | 2-3 seconds |
| ReleaseSafe | 5-10 seconds |
| ReleaseFast | 10-15 seconds |
| Clean build | 10-20 seconds |
| Run tests | 3-5 seconds |

## Next Steps

1. **Build the game**: `zig build`
2. **Run the game**: `zig build run`
3. **Read documentation**: Check README.md
4. **Play the game**: Follow in-game instructions
5. **Review code**: Check HLD.md for design details

## Getting Help

### Zig Resources
- Official Site: https://ziglang.org
- Documentation: https://ziglang.org/documentation/
- Community: https://discord.gg/gxD46YYu6q

### Game Issues
- Review HLD.md for game design details
- Check source code comments in src/ folder
- Run tests for validation: `zig build test`

## Advanced Topics

### Custom Build Options

Edit `build.zig` to:
- Change optimization levels
- Adjust binary name
- Add custom build steps
- Include additional tests

### Extending the Game

The modular design allows easy extensions:
- Add new card suits: Edit `config.zig`
- Change payouts: Edit `game.zig`
- Add UI features: Edit `ui.zig`
- Implement new rules: Edit `game.zig` and `hand.zig`

### Deployment

To distribute the compiled binary:
```bash
zig build -Doptimize=ReleaseFast
cp zig-cache/bin/blackjack ~/bin/blackjack
chmod +x ~/bin/blackjack
```

Then run anywhere:
```bash
blackjack
```

---

**Ready to play? Build and run the game with `zig build run`**
