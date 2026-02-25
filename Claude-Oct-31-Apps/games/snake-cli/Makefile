.PHONY: build test clean run coverage fmt lint help

# Variables
BINARY_NAME=snake-cli
GO=go
GOFLAGS=-v
LDFLAGS=-ldflags "-X main.Version=$(VERSION)"

# Targets
help: ## Show this help message
	@echo "Snake CLI - Makefile targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

build: ## Build the snake game binary
	@echo "Building $(BINARY_NAME)..."
	@$(GO) build $(GOFLAGS) -o $(BINARY_NAME) .
	@echo "Build complete: $(BINARY_NAME)"

test: ## Run all tests
	@echo "Running tests..."
	@$(GO) test $(GOFLAGS) -cover .

coverage: ## Generate and display test coverage
	@echo "Generating coverage report..."
	@$(GO) test -coverprofile=coverage.out .
	@$(GO) tool cover -html=coverage.out -o coverage.html
	@$(GO) tool cover -func=coverage.out | tail -1
	@echo "Coverage report: coverage.html"

fmt: ## Format code according to Go standards
	@echo "Formatting code..."
	@$(GO) fmt ./...
	@goimports -w .

lint: ## Run linter (requires golangci-lint)
	@echo "Running linter..."
	@golangci-lint run ./... || echo "Install golangci-lint to run linter: https://golangci-lint.run"

run: build ## Build and run the game
	@./$(BINARY_NAME)

run-debug: build ## Build and run with debug output
	@./$(BINARY_NAME) -width 60 -height 25

clean: ## Remove built binary and temporary files
	@echo "Cleaning..."
	@rm -f $(BINARY_NAME)
	@rm -f coverage.out coverage.html
	@$(GO) clean

install: build ## Build and install the binary to GOPATH/bin
	@echo "Installing $(BINARY_NAME)..."
	@$(GO) install $(GOFLAGS) .
	@echo "Installed to $(GOPATH)/bin/$(BINARY_NAME)"

deps: ## Download and verify dependencies
	@echo "Downloading dependencies..."
	@$(GO) mod download
	@$(GO) mod verify
	@echo "Dependencies ready"

check: fmt lint test ## Run format check, lint, and tests

all: clean check build ## Build everything after cleaning and checking
	@echo "All tasks completed successfully!"

.DEFAULT_GOAL := help
