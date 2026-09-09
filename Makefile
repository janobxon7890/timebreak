SHELL := /bin/zsh
export PATH := $(HOME)/.cargo/bin:/opt/homebrew/bin:$(PATH)

.PHONY: all setup dev dev-game test lint build clean

all: dev

setup:
	@echo "==> Setting up TimeBreak dependencies..."
	@which rustc > /dev/null || (echo "Rust is required. Install from https://rustup.rs"; exit 1)
	@which node > /dev/null || (echo "Node is required. Install via brew install node"; exit 1)
	@which pnpm > /dev/null || (npm install -g pnpm --force)
	pnpm install
	@echo "==> Dependencies setup complete."

dev:
	@echo "==> Starting TimeBreak desktop application..."
	pnpm --filter desktop tauri dev

dev-game:
	@echo "==> Launching TimeBreak Instant Overlay Game (Dev Break Mode)..."
	TIMEBREAK_DEV=1 TIMEBREAK_AUTO_BREAK=1 TIMEBREAK_SESSION_SECONDS=300 pnpm --filter desktop tauri dev

test:
	@echo "==> Running Rust workspace unit and integration tests..."
	cargo test --workspace --all-targets
	@echo "==> Running TypeScript unit tests..."
	pnpm test

lint:
	@echo "==> Checking Rust formatting and clippy..."
	cargo fmt --all --check
	cargo clippy --workspace --all-targets -- -D warnings
	@echo "==> Checking TypeScript lint and types..."
	pnpm typecheck

build:
	@echo "==> Building production desktop application..."
	pnpm --filter desktop tauri build

clean:
	@echo "==> Cleaning build artifacts..."
	cargo clean
	rm -rf node_modules apps/desktop/node_modules apps/desktop/dist
