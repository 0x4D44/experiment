.PHONY: build test clean coverage docs run help

# Compiler settings
GO := go
GOFLAGS := -v
BINARY_NAME := textadventure
VERSION := 1.0.0
BUILD_TIME := $(shell date -u '+%Y-%m-%d_%H:%M:%S')
GIT_COMMIT := $(shell git rev-parse --short HEAD 2>/dev/null || echo "unknown")

# Build flags
LDFLAGS := -ldflags "-X main.Version=$(VERSION) -X main.BuildTime=$(BUILD_TIME) -X main.GitCommit=$(GIT_COMMIT)"

# Directories
BUILD_DIR := ./bin
DIST_DIR := ./dist
DOCS_DIR := ./docs
COVERAGE_DIR := ./coverage

help:
	@echo "Text Adventure Engine - Make targets:"
	@echo ""
	@echo "  build          - Build the game executable"
	@echo "  test           - Run all tests"
	@echo "  coverage       - Run tests with coverage report"
	@echo "  clean          - Remove build artifacts"
	@echo "  docs           - Generate documentation"
	@echo "  run            - Run the sample game"
	@echo "  release        - Create release package"
	@echo "  fmt            - Format code"
	@echo "  lint           - Run linter"
	@echo "  help           - Show this help message"

build: ensure-dirs
	@echo "Building $(BINARY_NAME)..."
	$(GO) build $(GOFLAGS) $(LDFLAGS) -o $(BUILD_DIR)/$(BINARY_NAME) ./cmd/main.go
	@echo "Build complete: $(BUILD_DIR)/$(BINARY_NAME)"

test:
	@echo "Running tests..."
	$(GO) test -v -race -timeout 30s ./...

coverage: ensure-dirs
	@echo "Running tests with coverage..."
	$(GO) test -v -race -coverprofile=$(COVERAGE_DIR)/coverage.out -covermode=atomic ./...
	$(GO) tool cover -html=$(COVERAGE_DIR)/coverage.out -o $(COVERAGE_DIR)/coverage.html
	@echo "Coverage report: $(COVERAGE_DIR)/coverage.html"
	$(GO) tool cover -func=$(COVERAGE_DIR)/coverage.out | tail -1

clean:
	@echo "Cleaning..."
	rm -rf $(BUILD_DIR) $(DIST_DIR) $(COVERAGE_DIR)
	$(GO) clean

docs:
	@echo "Generating documentation..."
	mkdir -p $(DOCS_DIR)
	cp HLD.md $(DOCS_DIR)/
	@echo "Documentation generated in $(DOCS_DIR)/"

run: build
	@echo "Running sample game..."
	$(BUILD_DIR)/$(BINARY_NAME) games/dungeon-escape/dungeon-escape.json

release: clean test coverage
	@echo "Creating release package..."
	mkdir -p $(DIST_DIR)
	$(GO) build $(GOFLAGS) $(LDFLAGS) -o $(DIST_DIR)/$(BINARY_NAME) ./cmd/main.go
	cp README.md $(DIST_DIR)/
	cp HLD.md $(DIST_DIR)/
	cp -r games/dungeon-escape $(DIST_DIR)/
	tar czf $(DIST_DIR)/text-adventure-$(VERSION).tar.gz -C $(DIST_DIR) \
		$(BINARY_NAME) README.md HLD.md dungeon-escape/
	@echo "Release package: $(DIST_DIR)/text-adventure-$(VERSION).tar.gz"

fmt:
	@echo "Formatting code..."
	$(GO) fmt ./...
	goimports -w .

lint:
	@echo "Running linter..."
	golint ./...
	$(GO) vet ./...

ensure-dirs:
	@mkdir -p $(BUILD_DIR) $(DIST_DIR) $(DOCS_DIR) $(COVERAGE_DIR)

.DEFAULT_GOAL := help
