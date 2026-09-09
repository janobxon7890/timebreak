#!/usr/bin/env bash
set -e

# ==============================================================================
# TimeBreak - Cross-Platform OS & Dependency Auto-Installer
# Supports: Ubuntu / Debian, macOS, Fedora, Arch Linux
# ==============================================================================

BOLD='\033[1m'
GREEN='\033[0;32m'
SKY='\033[0;36m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Ensure standard binaries and cargo are in PATH
export PATH="$HOME/.cargo/bin:/opt/homebrew/bin:/usr/local/bin:$HOME/.local/bin:$PATH"

OS="$(uname -s)"
echo -e "${SKY}${BOLD}==> TimeBreak avtomatik tizim sozlash: [${OS}]${NC}"

# Detect root / sudo
if [ "$(id -u)" -eq 0 ]; then
    SUDO=""
elif command -v sudo >/dev/null 2>&1; then
    SUDO="sudo"
else
    SUDO=""
fi

case "$OS" in
    Linux)
        echo -e "${SKY}==> Linux tizimi aniqlandi. Distro tekshirilmoqda...${NC}"
        
        if [ -f /etc/os-release ]; then
            # Source OS release details
            . /etc/os-release
            DISTRO_ID="${ID:-unknown}"
            DISTRO_LIKE="${ID_LIKE:-}"
        else
            DISTRO_ID="unknown"
            DISTRO_LIKE=""
        fi

        echo -e "${SKY}==> Aniqlangan distro: ${DISTRO_ID} (${DISTRO_LIKE})${NC}"

        # 1. UBUNTU / DEBIAN / POP_OS / MINT
        if [[ "$DISTRO_ID" =~ ^(ubuntu|debian|pop|linuxmint|elementary|zorin|kali)$ ]] || [[ "$DISTRO_LIKE" =~ (ubuntu|debian) ]]; then
            echo -e "${SKY}==> Ubuntu/Debian uchun zarur tizim paketlari (Tauri & GTK) tekshirilmoqda...${NC}"
            
            # Update package lists
            if [ -n "$SUDO" ] || [ "$(id -u)" -eq 0 ]; then
                echo -e "${SKY}==> Paketlar yangilanmoqda (apt-get update)...${NC}"
                $SUDO apt-get update -y
                
                BASE_PKGS="build-essential curl wget file pkg-config libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev"
                
                # Check for webkit2gtk-4.1 (preferred on modern Ubuntu) or fallback to 4.0
                if apt-cache show libwebkit2gtk-4.1-dev >/dev/null 2>&1; then
                    WEBKIT_PKGS="libwebkit2gtk-4.1-dev javascriptcoregtk-4.1-dev"
                elif apt-cache show libwebkit2gtk-4.0-dev >/dev/null 2>&1; then
                    WEBKIT_PKGS="libwebkit2gtk-4.0-dev javascriptcoregtk-4.0-dev"
                else
                    WEBKIT_PKGS="libwebkit2gtk-4.1-dev"
                fi

                echo -e "${SKY}==> Tizim kutubxonalari o'rnatilmoqda: ${BASE_PKGS} ${WEBKIT_PKGS}${NC}"
                $SUDO apt-get install -y $BASE_PKGS $WEBKIT_PKGS
            else
                echo -e "${YELLOW}Ogohlantirish: sudo topilmadi. Tizim kutubxonalari qo'lda o'rnatilgan deb hisoblanadi.${NC}"
            fi

        # 2. FEDORA / RHEL / CENTOS
        elif [[ "$DISTRO_ID" =~ ^(fedora|rhel|centos)$ ]] || [[ "$DISTRO_LIKE" =~ (fedora|rhel) ]]; then
            echo -e "${SKY}==> Fedora/RHEL uchun zarur kutubxonalar o'rnatilmoqda...${NC}"
            if command -v dnf >/dev/null 2>&1; then
                $SUDO dnf install -y gcc-c++ make curl wget pkgconf-pkg-config openssl-devel gtk3-devel webkit2gtk4.1-devel libappindicator-gtk3-devel librsvg2-devel || \
                $SUDO dnf install -y gcc-c++ make curl wget pkgconf-pkg-config openssl-devel gtk3-devel webkit2gtk3-devel libappindicator-gtk3-devel librsvg2-devel
            fi

        # 3. ARCH LINUX / MANJARO
        elif [[ "$DISTRO_ID" =~ ^(arch|manjaro|endeavouros)$ ]] || [[ "$DISTRO_LIKE" =~ arch ]]; then
            echo -e "${SKY}==> Arch Linux uchun zarur kutubxonalar o'rnatilmoqda...${NC}"
            if command -v pacman >/dev/null 2>&1; then
                $SUDO pacman -Sy --needed --noconfirm base-devel curl wget openssl gtk3 webkit2gtk-4.1 libappindicator-gtk3 librsvg || \
                $SUDO pacman -Sy --needed --noconfirm base-devel curl wget openssl gtk3 webkit2gtk libappindicator-gtk3 librsvg
            fi
        else
            echo -e "${YELLOW}Noma'lum Linux distrosi. GTK3 va WebKit2GTK kutubxonalari o'rnatilganligiga ishonch hosil qiling.${NC}"
        fi
        ;;

    Darwin)
        echo -e "${SKY}==> macOS tizimi aniqlandi.${NC}"
        # Check Xcode CLI tools
        if ! xcode-select -p >/dev/null 2>&1; then
            echo -e "${SKY}==> Xcode Command Line Tools o'rnatilmoqda...${NC}"
            xcode-select --install || true
        fi

        # Check Homebrew
        if ! command -v brew >/dev/null 2>&1; then
            echo -e "${YELLOW}==> Homebrew topilmadi. O'rnatish tavsiya etiladi (https://brew.sh).${NC}"
        fi
        ;;

    *)
        echo -e "${YELLOW}Noma'lum operatsion tizim: ${OS}.${NC}"
        ;;
esac

# ==============================================================================
# Rust Toolchain (rustc & cargo)
# ==============================================================================
if ! command -v rustc >/dev/null 2>&1 || ! command -v cargo >/dev/null 2>&1; then
    echo -e "${SKY}==> Rust (rustc/cargo) topilmadi. Rustup orqali o'rnatilmoqda...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env" 2>/dev/null || true
    export PATH="$HOME/.cargo/bin:$PATH"
fi
echo -e "${GREEN}✓ Rust:${NC} $(rustc --version)"

# ==============================================================================
# Node.js Environment
# ==============================================================================
if ! command -v node >/dev/null 2>&1; then
    echo -e "${SKY}==> Node.js topilmadi. Node.js LTS o'rnatilmoqda...${NC}"
    if [ "$OS" = "Darwin" ] && command -v brew >/dev/null 2>&1; then
        brew install node
    elif [ "$OS" = "Linux" ] && ([ -n "$SUDO" ] || [ "$(id -u)" -eq 0 ]); then
        curl -fsSL https://deb.nodesource.com/setup_20.x | $SUDO -E bash -
        $SUDO apt-get install -y nodejs
    else
        echo -e "${RED}Node.js avtomatik o'rnatib bo'lmadi. Iltimos Node.js v18+ o'rnating.${NC}"
        exit 1
    fi
fi
echo -e "${GREEN}✓ Node.js:${NC} $(node --version)"

# ==============================================================================
# pnpm Package Manager
# ==============================================================================
if ! command -v pnpm >/dev/null 2>&1; then
    echo -e "${SKY}==> pnpm topilmadi. O'rnatilmoqda...${NC}"
    if command -v corepack >/dev/null 2>&1; then
        corepack enable >/dev/null 2>&1 || true
    fi
    if ! command -v pnpm >/dev/null 2>&1; then
        if command -v npm >/dev/null 2>&1; then
            npm install -g pnpm --force || curl -fsSL https://get.pnpm.io/install.sh | sh -
        else
            curl -fsSL https://get.pnpm.io/install.sh | sh -
        fi
    fi
    export PATH="$HOME/.local/share/pnpm:$PATH"
fi
echo -e "${GREEN}✓ pnpm:${NC} $(pnpm --version)"

# ==============================================================================
# Node & Workspace Dependencies
# ==============================================================================
echo -e "${SKY}==> Loyiha paketlari o'rnatilmoqda (pnpm install)...${NC}"
pnpm install

echo -e "${GREEN}${BOLD}==> Barcha tizim va loyiha sozlamalari muvaffaqiyatli yakunlandi!${NC}"
