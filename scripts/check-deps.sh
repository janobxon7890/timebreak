#!/usr/bin/env bash
# ==============================================================================
# TimeBreak - Fast Dependency Verification
# Checks if environment is ready; triggers setup-os.sh only if anything is missing.
# ==============================================================================

export PATH="$HOME/.cargo/bin:/opt/homebrew/bin:/usr/local/bin:$HOME/.local/bin:$PATH"

NEEDS_SETUP=0

# 1. Check Toolchain Binaries
if ! command -v rustc >/dev/null 2>&1 || ! command -v cargo >/dev/null 2>&1; then
    NEEDS_SETUP=1
fi

if ! command -v node >/dev/null 2>&1; then
    NEEDS_SETUP=1
fi

if ! command -v pnpm >/dev/null 2>&1; then
    NEEDS_SETUP=1
fi

# 2. Check node_modules
if [ ! -d "node_modules" ] || [ ! -d "apps/desktop/node_modules" ]; then
    NEEDS_SETUP=1
fi

# 3. Check Linux GTK/WebKit dependencies
OS="$(uname -s)"
if [ "$OS" = "Linux" ]; then
    if command -v pkg-config >/dev/null 2>&1; then
        if ! pkg-config --exists gtk+-3.0 >/dev/null 2>&1; then
            NEEDS_SETUP=1
        fi
        if ! pkg-config --exists webkit2gtk-4.1 >/dev/null 2>&1 && ! pkg-config --exists webkit2gtk-4.0 >/dev/null 2>&1; then
            NEEDS_SETUP=1
        fi
    else
        NEEDS_SETUP=1
    fi
fi

if [ "$NEEDS_SETUP" -eq 1 ]; then
    echo "==> Yetishmayotgan tizim yoki dastur kutubxonalari aniqlandi."
    echo "==> Avtomatik sozlash ishga tushirilmoqda (scripts/setup-os.sh)..."
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    "$SCRIPT_DIR/setup-os.sh"
fi

exit 0
