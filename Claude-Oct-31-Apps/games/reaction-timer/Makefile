# Makefile for Reaction Timer Game
# Comprehensive build system with optimization, testing, and profiling

.PHONY: all clean test help debug release coverage

# ============================================================================
# Configuration
# ============================================================================

CC ?= gcc
CSTD = -std=c11
WARNINGS = -Wall -Wextra -Wpedantic -Wshadow -Wstrict-prototypes \
           -Wwrite-strings -Wconversion -Wunused-parameter
COMMON_FLAGS = $(CSTD) $(WARNINGS) -D_POSIX_C_SOURCE=200809L

# Build directories
BUILD_DIR = build
BIN_DIR = bin
TEST_BIN = $(BIN_DIR)/test_runner

# Source files
SOURCES = $(wildcard *.c)
TEST_SOURCES = test_runner.c tests_timer.c tests_statistics.c tests_rng.c

# Filter out the main game source if building tests
IMPL_SOURCES = $(filter-out main.c test_runner.c tests_%.c, $(SOURCES))

# ============================================================================
# Build Configurations
# ============================================================================

# Default build: optimized release
CFLAGS ?= -O2 $(COMMON_FLAGS)

# Debug build: symbols and no optimization
DEBUG_FLAGS = -g3 -O0 -DDEBUG $(COMMON_FLAGS)

# Release build: maximum optimization
RELEASE_FLAGS = -O3 -DNDEBUG -march=native $(COMMON_FLAGS)

# Coverage build: instrumentation
COVERAGE_FLAGS = -fprofile-arcs -ftest-coverage -O0 -g $(COMMON_FLAGS)

# ASAN build: address sanitizer
ASAN_FLAGS = -fsanitize=address -fsanitize=undefined -g -O1 $(COMMON_FLAGS)

# ============================================================================
# Default Target
# ============================================================================

all: release

help:
	@echo "Reaction Timer Game - Build System"
	@echo ""
	@echo "Targets:"
	@echo "  make all              - Build release (default)"
	@echo "  make release          - Optimized release build (-O3)"
	@echo "  make debug            - Debug build with symbols (-g3 -O0)"
	@echo "  make test             - Build and run test suite"
	@echo "  make coverage         - Build with coverage, run tests, generate report"
	@echo "  make asan             - Build with AddressSanitizer"
	@echo "  make clean            - Remove all build artifacts"
	@echo "  make help             - Show this help message"
	@echo ""
	@echo "Variables:"
	@echo "  CC=clang              - Use Clang instead of GCC"
	@echo "  CFLAGS=\"-O2\"        - Override compiler flags"
	@echo "  V=1                   - Verbose output"
	@echo "  CATEGORY=timer        - Run specific test category"
	@echo "  TEST=name             - Run specific test"
	@echo ""

# ============================================================================
# Release Build (Default)
# ============================================================================

release: CFLAGS = $(RELEASE_FLAGS)
release: setup $(BIN_DIR)/reaction-timer
	@echo "✓ Release build complete: $(BIN_DIR)/reaction-timer"

$(BIN_DIR)/reaction-timer: $(IMPL_SOURCES:.c=.o) main.o
	@mkdir -p $(BIN_DIR)
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS) -lm
	@echo "  Linked: $@"

# ============================================================================
# Debug Build
# ============================================================================

debug: CFLAGS = $(DEBUG_FLAGS)
debug: setup $(BIN_DIR)/reaction-timer-debug
	@echo "✓ Debug build complete: $(BIN_DIR)/reaction-timer-debug"

$(BIN_DIR)/reaction-timer-debug: $(IMPL_SOURCES:.c=.o) main.o
	@mkdir -p $(BIN_DIR)
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS) -lm
	@echo "  Linked: $@"

# ============================================================================
# ASAN Build
# ============================================================================

asan: CFLAGS = $(ASAN_FLAGS)
asan: setup $(BIN_DIR)/reaction-timer-asan
	@echo "✓ ASAN build complete: $(BIN_DIR)/reaction-timer-asan"
	@echo "  Run with: $(BIN_DIR)/reaction-timer-asan"

$(BIN_DIR)/reaction-timer-asan: $(IMPL_SOURCES:.c=.o) main.o
	@mkdir -p $(BIN_DIR)
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS) -lm

# ============================================================================
# Test Build
# ============================================================================

test: setup $(TEST_BIN)
	@echo "✓ Test build complete: $(TEST_BIN)"
	@echo ""
	@echo "Running tests..."
	@./$(TEST_BIN)

$(TEST_BIN): $(IMPL_SOURCES:.c=.o) $(TEST_SOURCES:.c=.o)
	@mkdir -p $(BIN_DIR)
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS) -lm
	@echo "  Linked: $@"

# Test subcommands
test-category:
ifdef CATEGORY
	@./$(TEST_BIN) --category $(CATEGORY)
else
	@echo "Usage: make test CATEGORY=<name>"
	@echo "Categories: timer, statistics, rng"
endif

test-single:
ifdef TEST
	@./$(TEST_BIN) --single $(TEST)
else
	@echo "Usage: make test TEST=<name>"
endif

# ============================================================================
# Coverage Build
# ============================================================================

coverage: CFLAGS = $(COVERAGE_FLAGS)
coverage: LDFLAGS = --coverage
coverage: setup $(TEST_BIN)
	@echo "✓ Coverage build complete"
	@echo ""
	@echo "Running tests with coverage..."
	@./$(TEST_BIN)
	@echo ""
	@echo "Generating coverage report..."
	@mkdir -p coverage
	@gcov $(TEST_SOURCES) $(IMPL_SOURCES) -r 2>/dev/null || true
	@lcov --capture --directory . --output-file coverage/coverage.info 2>/dev/null || true
	@lcov --remove coverage/coverage.info '/usr/*' '*/test*' \
	      --output-file coverage/coverage.info 2>/dev/null || true
	@genhtml coverage/coverage.info --output-directory coverage/html 2>/dev/null || \
	  echo "Note: Install lcov and genhtml for HTML coverage report"
	@echo "✓ Coverage report generated in coverage/html/index.html"

# ============================================================================
# Object Files
# ============================================================================

%.o: %.c
	$(CC) $(CFLAGS) $(CPPFLAGS) -c $< -o $@
	@echo "  Compiled: $<"

# ============================================================================
# Setup
# ============================================================================

setup:
	@mkdir -p $(BUILD_DIR) $(BIN_DIR)
	@if [ -z "$(V)" ]; then echo "Building..."; fi

# ============================================================================
# Utilities
# ============================================================================

# Check for issues without building
check:
	@echo "Checking for common issues..."
	@cppcheck --enable=all --suppress=missingIncludeSystem . 2>/dev/null || \
	  echo "  (cppcheck not installed)"
	@echo "✓ Check complete"

# Format code with clang-format
format:
	@echo "Formatting code..."
	@clang-format -i *.c *.h 2>/dev/null || \
	  echo "  (clang-format not installed)"
	@echo "✓ Code formatted"

# Run benchmark
bench: release
	@echo "Running benchmark..."
	@./$(BIN_DIR)/reaction-timer --benchmark || true

# Show build info
info:
	@echo "Build Information:"
	@echo "  CC: $(CC)"
	@echo "  CFLAGS: $(CFLAGS)"
	@echo "  Sources: $(SOURCES)"
	@echo "  Build dir: $(BUILD_DIR)"
	@echo "  Bin dir: $(BIN_DIR)"

# ============================================================================
# Cleaning
# ============================================================================

clean:
	@echo "Cleaning build artifacts..."
	@rm -rf $(BUILD_DIR) $(BIN_DIR) *.o *.gcda *.gcno *.gcov coverage/
	@echo "✓ Clean complete"

distclean: clean
	@echo "Removing all generated files..."
	@rm -rf *~ *.bak
	@echo "✓ Distribution clean complete"

# ============================================================================
# Installation
# ============================================================================

install: release
	@echo "Installing reaction-timer..."
	@mkdir -p ~/bin
	@cp $(BIN_DIR)/reaction-timer ~/bin/
	@chmod +x ~/bin/reaction-timer
	@echo "✓ Installed to ~/bin/reaction-timer"

uninstall:
	@rm -f ~/bin/reaction-timer
	@echo "✓ Uninstalled"

# ============================================================================
# Development Targets
# ============================================================================

# Watch for changes and rebuild
watch:
	@while true; do \
		inotifywait -e modify *.c *.h 2>/dev/null || sleep 1; \
		clear; \
		make release; \
	done

# Quick test cycle
quick: clean debug test

# Full validation
validate: clean asan test coverage check
	@echo "✓ Full validation complete"

# Create distribution tarball
dist: distclean
	@echo "Creating distribution..."
	@mkdir -p reaction-timer
	@cp *.c *.h Makefile README.md HLD.md reaction-timer/
	@tar czf reaction-timer.tar.gz reaction-timer/
	@rm -rf reaction-timer
	@echo "✓ Distribution created: reaction-timer.tar.gz"

# ============================================================================
# Phony Targets (always run)
# ============================================================================

.PHONY: all clean test debug release coverage asan help check format bench info \
        install uninstall watch quick validate dist distclean setup test-category test-single
