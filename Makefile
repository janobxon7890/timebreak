# Cross-platform Shell and PATH configuration
SHELL := /bin/bash
export PATH := $(HOME)/.cargo/bin:/opt/homebrew/bin:/usr/local/bin:$(HOME)/.local/bin:$(PATH)

.PHONY: all setup check-deps dev dev-game test lint build clean

all: dev

check-deps:
	@./scripts/check-deps.sh

setup:
	@./scripts/setup-os.sh

dev: check-deps
	@echo "==> Starting TimeBreak desktop application..."
	pnpm --filter desktop tauri dev

dev-game: check-deps
	@echo "==> Launching TimeBreak Instant Overlay Game (Dev Break Mode)..."
	TIMEBREAK_DEV=1 TIMEBREAK_AUTO_BREAK=1 TIMEBREAK_SESSION_SECONDS=40 pnpm --filter desktop tauri dev

test: check-deps
	@echo "==> Running Rust workspace unit and integration tests..."
	cargo test --workspace --all-targets
	@echo "==> Running TypeScript unit tests..."
	pnpm test

lint: check-deps
	@echo "==> Checking Rust formatting and clippy..."
	cargo fmt --all --check
	cargo clippy --workspace --all-targets -- -D warnings
	@echo "==> Checking TypeScript lint and types..."
	pnpm typecheck

build: check-deps
	@echo "==> Building production desktop application..."
	pnpm --filter desktop tauri build

clean:
	@echo "==> Cleaning build artifacts..."
	cargo clean
	rm -rf node_modules apps/desktop/node_modules apps/desktop/dist
