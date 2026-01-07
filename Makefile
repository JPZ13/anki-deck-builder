.PHONY: help build build-release test test-verbose clean fmt lint check run install doc

# Default target
.DEFAULT_GOAL := help

# Colors for output
CYAN := \033[36m
GREEN := \033[32m
YELLOW := \033[33m
RESET := \033[0m

help: ## Show this help message
	@echo "$(CYAN)Anki Deck Builder - Available Make Targets$(RESET)"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-15s$(RESET) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(YELLOW)Tip: Use 'nix-shell' or 'direnv allow' to enter the development environment$(RESET)"

build: ## Build the project in debug mode
	@echo "$(CYAN)Building in debug mode...$(RESET)"
	cargo build

build-release: ## Build the project in release mode (optimized)
	@echo "$(CYAN)Building in release mode...$(RESET)"
	cargo build --release

test: ## Run all tests
	@echo "$(CYAN)Running tests...$(RESET)"
	cargo test

test-verbose: ## Run all tests with verbose output
	@echo "$(CYAN)Running tests (verbose)...$(RESET)"
	cargo test -- --nocapture

check: ## Check the project for errors without building
	@echo "$(CYAN)Checking project...$(RESET)"
	cargo check

fmt: ## Format code using rustfmt
	@echo "$(CYAN)Formatting code...$(RESET)"
	cargo fmt

fmt-check: ## Check if code is properly formatted
	@echo "$(CYAN)Checking code formatting...$(RESET)"
	cargo fmt -- --check

lint: ## Run clippy linter
	@echo "$(CYAN)Running clippy...$(RESET)"
	cargo clippy -- -D warnings

lint-fix: ## Run clippy and automatically fix issues
	@echo "$(CYAN)Running clippy with auto-fix...$(RESET)"
	cargo clippy --fix --allow-dirty --allow-staged

clean: ## Clean build artifacts
	@echo "$(CYAN)Cleaning build artifacts...$(RESET)"
	cargo clean
	@echo "$(GREEN)Clean complete!$(RESET)"

run: ## Run the CLI (use ARGS to pass arguments, e.g., make run ARGS="config --show")
	@echo "$(CYAN)Running CLI...$(RESET)"
	cargo run -- $(ARGS)

run-release: ## Run the CLI in release mode
	@echo "$(CYAN)Running CLI (release mode)...$(RESET)"
	cargo run --release -- $(ARGS)

install: ## Install the binary to ~/.cargo/bin
	@echo "$(CYAN)Installing binary...$(RESET)"
	cargo install --path .
	@echo "$(GREEN)Installed to ~/.cargo/bin/anki-deck-builder$(RESET)"

doc: ## Generate and open documentation
	@echo "$(CYAN)Generating documentation...$(RESET)"
	cargo doc --open --no-deps

doc-all: ## Generate documentation including dependencies
	@echo "$(CYAN)Generating documentation (with dependencies)...$(RESET)"
	cargo doc --open

watch: ## Watch for changes and rebuild (requires cargo-watch)
	@echo "$(CYAN)Watching for changes...$(RESET)"
	@command -v cargo-watch >/dev/null 2>&1 || { echo "$(YELLOW)cargo-watch not found. Install with: cargo install cargo-watch$(RESET)"; exit 1; }
	cargo watch -x build

watch-test: ## Watch for changes and run tests (requires cargo-watch)
	@echo "$(CYAN)Watching for changes and running tests...$(RESET)"
	@command -v cargo-watch >/dev/null 2>&1 || { echo "$(YELLOW)cargo-watch not found. Install with: cargo install cargo-watch$(RESET)"; exit 1; }
	cargo watch -x test

dev: ## Run all development checks (format, lint, test, build)
	@echo "$(CYAN)Running all development checks...$(RESET)"
	@$(MAKE) fmt
	@$(MAKE) lint
	@$(MAKE) test
	@$(MAKE) build
	@echo "$(GREEN)All checks passed!$(RESET)"

ci: ## Run CI checks (format check, lint, test, build)
	@echo "$(CYAN)Running CI checks...$(RESET)"
	@$(MAKE) fmt-check
	@$(MAKE) lint
	@$(MAKE) test
	@$(MAKE) build
	@echo "$(GREEN)CI checks passed!$(RESET)"

setup: ## Setup development environment (for NixOS users)
	@echo "$(CYAN)Setting up development environment...$(RESET)"
	@if command -v nix-shell >/dev/null 2>&1; then \
		echo "$(GREEN)Nix is installed. Run 'nix-shell' or 'direnv allow' to enter the development environment.$(RESET)"; \
	else \
		echo "$(YELLOW)Nix is not installed. Please install Nix: https://nixos.org/download.html$(RESET)"; \
	fi
	@if command -v direnv >/dev/null 2>&1; then \
		echo "$(GREEN)Direnv is installed. Run 'direnv allow' for automatic shell activation.$(RESET)"; \
	else \
		echo "$(YELLOW)Direnv not found (optional). Install for automatic nix-shell activation.$(RESET)"; \
	fi

version: ## Show version information
	@echo "$(CYAN)Version Information:$(RESET)"
	@grep "^version" Cargo.toml | head -1
	@echo "Rust version: $$(rustc --version 2>/dev/null || echo 'not installed')"
	@echo "Cargo version: $$(cargo --version 2>/dev/null || echo 'not installed')"
