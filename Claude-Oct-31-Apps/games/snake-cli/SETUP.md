# Snake CLI - Setup and Build Instructions

## Quick Start

### Prerequisites

- **Go 1.21+**: Download from https://golang.org/
- **Git** (optional, for version control)
- **Unix/Linux/macOS or Windows Terminal**

### One-Command Setup

```bash
cd games/snake-cli
make build
./snake-cli
```

## Detailed Setup Instructions

### Step 1: Verify Go Installation

```bash
go version
# Output: go version go1.21 (or higher)
```

If Go is not installed:
- **Linux**: `sudo apt-get install golang-go`
- **macOS**: `brew install go`
- **Windows**: Download installer from golang.org

### Step 2: Navigate to Project

```bash
cd games/snake-cli
```

### Step 3: Download Dependencies

```bash
go mod download
go mod verify
```

Or use Makefile:

```bash
make deps
```

### Step 4: Build the Project

```bash
# Option 1: Using Makefile (Recommended)
make build

# Option 2: Direct Go build
go build -o snake-cli

# Option 3: Optimized build for distribution
CGO_ENABLED=0 go build -ldflags="-w -s" -o snake-cli
```

### Step 5: Run Tests (Optional)

```bash
# Option 1: Using Makefile
make test

# Option 2: Direct Go testing
go test -v -cover

# Option 3: With coverage report
make coverage
```

### Step 6: Run the Game

```bash
./snake-cli
```

## Build Targets

### Using Make

```bash
make help          # Show all available targets
make build         # Build the game
make test          # Run tests
make coverage      # Generate coverage report
make run           # Build and run
make clean         # Clean build artifacts
make fmt           # Format code
make lint          # Run linter
make install       # Install to GOPATH/bin
```

### Full Build Example

```bash
# Clean previous build
make clean

# Check code quality
make fmt
make lint

# Run tests
make test

# Generate coverage
make coverage

# Build final binary
make build

# Verify binary works
./snake-cli -h  # Should show no errors
```

## Compilation Variants

### Standard Build

```bash
go build -o snake-cli
```

**Pros**: Fast, small binary
**Cons**: Debug symbols included

### Optimized Build

```bash
CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build \
    -ldflags="-w -s" -o snake-cli-linux
```

**Pros**: Smallest binary (~3-5MB), fully static
**Cons**: No debug info, platform-specific

### Debug Build

```bash
go build -gcflags="all=-N -l" -o snake-cli-debug
```

**Pros**: Full debug symbols, can use debugger
**Cons**: Larger binary (~10-15MB)

## Cross-Platform Builds

### Build for All Platforms

```bash
#!/bin/bash

# Linux x64
GOOS=linux GOARCH=amd64 go build -o snake-cli-linux-amd64

# Linux ARM64
GOOS=linux GOARCH=arm64 go build -o snake-cli-linux-arm64

# Linux ARM (Raspberry Pi)
GOOS=linux GOARCH=arm GOARM=7 go build -o snake-cli-linux-arm7

# macOS Intel
GOOS=darwin GOARCH=amd64 go build -o snake-cli-macos-amd64

# macOS Apple Silicon
GOOS=darwin GOARCH=arm64 go build -o snake-cli-macos-arm64

# Windows 64-bit
GOOS=windows GOARCH=amd64 go build -o snake-cli-win-amd64.exe

# Windows ARM64
GOOS=windows GOARCH=arm64 go build -o snake-cli-win-arm64.exe
```

## Testing

### Run All Tests

```bash
go test -v
```

### Run Specific Test

```bash
go test -run TestSnakeMovement -v
```

### Run With Coverage

```bash
go test -cover
```

### Generate Coverage Report

```bash
go test -coverprofile=coverage.out
go tool cover -html=coverage.out -o coverage.html
open coverage.html  # macOS
# OR
xdg-open coverage.html  # Linux
```

### Check Code Quality

```bash
# Format check
go fmt ./...

# Vet (static analysis)
go vet ./...

# Lint (if installed)
golangci-lint run
```

## Installation

### Option 1: Install to GOPATH/bin

```bash
make install
```

Then add GOPATH/bin to PATH:

```bash
export PATH=$PATH:$(go env GOPATH)/bin
```

### Option 2: Manual Installation

```bash
# Build
go build -o snake-cli

# Copy to /usr/local/bin (Linux/macOS)
sudo cp snake-cli /usr/local/bin/

# Or copy to a directory in PATH
cp snake-cli ~/.local/bin/
```

### Option 3: Create Launcher Script

```bash
#!/bin/bash
# ~/.local/bin/snake-cli
exec /path/to/games/snake-cli/snake-cli "$@"
```

Make executable:

```bash
chmod +x ~/.local/bin/snake-cli
```

## Troubleshooting

### Build Fails: "go: command not found"

**Solution**: Install Go from golang.org or use package manager

### Build Fails: "no Go files in..."

**Solution**: Ensure you're in the `games/snake-cli` directory

### Build Fails: "undefined identifier"

**Solution**: Verify all .go files are in the directory

```bash
ls -la *.go  # Should show all source files
```

### Tests Fail With Import Errors

**Solution**: Download dependencies

```bash
go mod download
go mod verify
```

### Program Won't Start: "Permission Denied"

**Solution**: Make binary executable

```bash
chmod +x snake-cli
```

### Game Input Not Responding

**Solution**: Ensure terminal supports raw mode

- Try a different terminal emulator
- On Windows, use Windows Terminal or ConEmu
- On Linux, ensure you're not in SSH without TERM support

## Verification

### Quick Verification Checklist

```bash
# 1. Verify Go installation
go version
# Expected: go version go1.21 (or higher)

# 2. Verify project structure
ls -la game*.go main.go render.go input.go config.go utils.go
# Expected: All files exist

# 3. Build project
go build -o snake-cli
# Expected: No errors, binary created

# 4. Verify binary
file snake-cli
# Expected: ELF executable / Mach-O executable / PE executable

# 5. Check file size
ls -lh snake-cli
# Expected: 5-10 MB

# 6. Run tests
go test -v
# Expected: All tests pass

# 7. Run game (briefly)
timeout 2 ./snake-cli || true
# Expected: No panics, exits cleanly
```

## Performance Validation

### Check Build Size

```bash
ls -lh snake-cli

# Standard build: 6-8 MB
# Optimized: 3-5 MB
# Debug: 10-15 MB
```

### Check Startup Time

```bash
time ./snake-cli < /dev/null
# Expected: <100ms
```

### Memory Usage

```bash
# Run in background
./snake-cli &
PID=$!

# Check memory (Linux)
ps aux | grep $PID

# Expected: 5-10 MB
```

## Development Workflow

### Initial Setup

```bash
# Clone or download repo
cd games/snake-cli

# Setup
go mod download
go mod verify

# Development build
go build -o snake-cli
```

### During Development

```bash
# Build and run
make run

# Build and test
make test

# Format code
make fmt

# Run linter
make lint

# Generate coverage
make coverage
```

### Before Commit

```bash
# Full quality check
make check

# Clean build
make clean
make build

# Final test
make test
```

### Release Build

```bash
# Cross-platform builds
for os in linux darwin windows; do
    for arch in amd64 arm64; do
        if [ "$os" != "darwin" ] || [ "$arch" = "amd64" ] || [ "$arch" = "arm64" ]; then
            GOOS=$os GOARCH=$arch go build -o snake-cli-$os-$arch
        fi
    done
done
```

## IDE Setup

### Visual Studio Code

1. Install Go extension
2. Open folder: `games/snake-cli`
3. VSCode will prompt to install Go tools (click "Install All")
4. Create `.vscode/launch.json`:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "Launch Game",
            "type": "go",
            "request": "launch",
            "mode": "debug",
            "program": "${workspaceFolder}",
            "args": ["-width", "50", "-height", "25"]
        }
    ]
}
```

### GoLand / IntelliJ IDEA

1. Open folder: `games/snake-cli`
2. IDE recognizes Go project automatically
3. Run → Run 'main' or press Ctrl+Shift+F10

### Vim/Neovim

```vim
" Install vim-go plugin
" In vim: :!make build

" Run tests
" In vim: :!make test
```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Build and Test

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-go@v4
        with:
          go-version: '1.21'
      - run: cd games/snake-cli && make test
      - run: cd games/snake-cli && make build
```

## Support

For issues or questions:

1. Check the README.md
2. Review IMPLEMENTATION.md for technical details
3. Check HLD.md for design information
4. Examine error messages carefully
5. Try building in a clean environment

## Next Steps

After successful build:

1. Run the game: `./snake-cli`
2. Read README.md for gameplay instructions
3. Try different board sizes: `./snake-cli -width 60 -height 30`
4. Explore code: Start with main.go
5. Run tests: `make test`
6. Generate coverage: `make coverage`

Enjoy the game!
