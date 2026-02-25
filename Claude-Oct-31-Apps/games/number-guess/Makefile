# Number Guess Game - Makefile
# Compiler: gcc (or compatible C compiler)
# Standard: C99

CC = gcc
CFLAGS = -std=c99 -Wall -Wextra -O2 -g
LDFLAGS =
LIBS = -lm

# Source files
SOURCES = main.c game.c random.c hint.c ai.c stats.c ui.c
TEST_SOURCES = test.c random.c hint.c ai.c game.c stats.c
HEADERS = game.h random.h hint.h ai.h stats.h ui.h

# Object files
OBJECTS = $(SOURCES:.c=.o)
TEST_OBJECTS = $(TEST_SOURCES:.c=.o)

# Executables
TARGET = number-guess
TEST_TARGET = number-guess-test

# Default target
all: $(TARGET)

# Main game executable
$(TARGET): $(OBJECTS)
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS) $(LIBS)
	@echo "Build complete: $(TARGET)"

# Test executable
test: $(TEST_TARGET)
	./$(TEST_TARGET)

$(TEST_TARGET): $(TEST_OBJECTS)
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS) $(LIBS)
	@echo "Test build complete: $(TEST_TARGET)"

# Object file compilation rules
%.o: %.c $(HEADERS)
	$(CC) $(CFLAGS) -c $< -o $@

# Run the game
run: $(TARGET)
	./$(TARGET)

# Run tests
test-run: test
	./$(TEST_TARGET)

# Clean build artifacts
clean:
	rm -f $(OBJECTS) $(TEST_OBJECTS) $(TARGET) $(TEST_TARGET)
	@echo "Clean complete"

# Clean everything including executables
distclean: clean
	rm -rf DATA
	@echo "Distclean complete"

# Create data directory
data-dir:
	mkdir -p DATA

# Rebuild from scratch
rebuild: clean all

# Static analysis with cppcheck (if available)
analyze:
	@command -v cppcheck >/dev/null 2>&1 && cppcheck --enable=all $(SOURCES) $(HEADERS) || echo "cppcheck not found"

# Format code (if clang-format is available)
format:
	@command -v clang-format >/dev/null 2>&1 && clang-format -i $(SOURCES) $(HEADERS) || echo "clang-format not found"

# Run with valgrind memory checker (if available)
valgrind: $(TARGET)
	@command -v valgrind >/dev/null 2>&1 && valgrind --leak-check=full --show-leak-kinds=all ./$(TARGET) || echo "valgrind not found"

# Help target
help:
	@echo "Number Guess Game - Makefile targets:"
	@echo ""
	@echo "  make              - Build the game"
	@echo "  make run          - Build and run the game"
	@echo "  make test         - Build and run tests"
	@echo "  make test-run     - Run tests (alias)"
	@echo "  make clean        - Remove build artifacts"
	@echo "  make distclean    - Remove everything including DATA/"
	@echo "  make rebuild      - Clean and rebuild"
	@echo "  make analyze      - Run static analysis (cppcheck)"
	@echo "  make format       - Format code (clang-format)"
	@echo "  make valgrind     - Run with memory checker (valgrind)"
	@echo "  make help         - Show this help message"
	@echo ""

# Phony targets (not files)
.PHONY: all run test test-run clean distclean rebuild analyze format valgrind help data-dir
