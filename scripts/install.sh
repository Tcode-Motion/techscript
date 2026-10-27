#!/usr/bin/env bash
# ============================================================
#  TechScript v2.0.2 — Linux, macOS & Android Universal Installer
#  Run with:
#    curl -fsSL https://raw.githubusercontent.com/Tcode-Motion/techscript/main/scripts/install.sh -o install.sh && bash install.sh
# ============================================================

set -e

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
BLUE='\033[0;34m'; BOLD='\033[1m'; NC='\033[0m'

echo ""
echo -e "${BLUE}${BOLD}===================================================${NC}"
echo -e "${BLUE}${BOLD}   TechScript 2.0 — Universal Installer (POSIX)   ${NC}"
echo -e "${BLUE}${BOLD}===================================================${NC}"
echo ""

# ---------- Detect OS & Architecture ----------
OS="$(uname -s)"
ARCH="$(uname -m)"
IS_TERMUX=false

if [ -n "$TERMUX_VERSION" ] || [ -d "/data/data/com.termux" ]; then
    IS_TERMUX=true
    PLATFORM="Android Termux"
elif [[ "${OS}" == "Linux"* ]]; then
    PLATFORM="Linux"
elif [[ "${OS}" == "Darwin"* ]]; then
    PLATFORM="macOS"
else
    PLATFORM="Unknown"
fi

echo -e "Detected Platform: ${GREEN}${PLATFORM} (${ARCH})${NC}"

# ---------- Get Release Metadata ----------
echo ""
echo "[1/4] Retrieving latest release information..."
REPO="Tcode-Motion/techscript"
LATEST_TAG=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep -Po '"tag_name": "\K[^"]*' || true)
if [ -z "$LATEST_TAG" ]; then
    LATEST_TAG="v2.0.2"
fi
echo -e "Target Release: ${GREEN}${LATEST_TAG}${NC}"

# ---------- Select Asset Name Matching CI/CD Matrix ----------
ASSET_NAME=""
if [ "$IS_TERMUX" = true ] && [ "$ARCH" = "aarch64" ]; then
    ASSET_NAME="techscript-2.0.0-termux-aarch64.tar.gz"
elif [ "$PLATFORM" = "Linux" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        ASSET_NAME="techscript-2.0.0-linux-x86_64.tar.gz"
    elif [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then
        ASSET_NAME="techscript-2.0.0-linux-aarch64.tar.gz"
    fi
elif [ "$PLATFORM" = "macOS" ]; then
    if [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
        ASSET_NAME="techscript-2.0.0-macos-aarch64.tar.gz"
    elif [ "$ARCH" = "x86_64" ]; then
        ASSET_NAME="techscript-2.0.0-macos-x86_64.tar.gz"
    fi
fi

# ---------- Download and Extract ----------
echo ""
echo "[2/4] Downloading TechScript binaries..."
TEMP_DIR=$(mktemp -d)
DOWNLOAD_SUCCESS=false

if [ -n "$ASSET_NAME" ]; then
    DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${LATEST_TAG}/${ASSET_NAME}"
    echo "Downloading from: $DOWNLOAD_URL"
    if curl -fsSL "$DOWNLOAD_URL" -o "$TEMP_DIR/$ASSET_NAME"; then
        tar -xzf "$TEMP_DIR/$ASSET_NAME" -C "$TEMP_DIR"
        DOWNLOAD_SUCCESS=true
        echo -e "${GREEN}✓ Download and extraction complete.${NC}"
    else
        echo -e "${YELLOW}[WARN] Direct pre-compiled asset download not available for this tag. Checking fallback...${NC}"
    fi
fi

# ---------- Fallback: Source Compilation if Cargo Available ----------
if [ "$DOWNLOAD_SUCCESS" != "true" ]; then
    if command -v cargo &>/dev/null; then
        echo "[INFO] Compiling from source using Rust cargo..."
        git clone --depth 1 "https://github.com/${REPO}.git" "$TEMP_DIR/techscript"
        (cd "$TEMP_DIR/techscript" && cargo build --release -p techscript_cli)
        cp "$TEMP_DIR/techscript/target/release/tsc" "$TEMP_DIR/tsc"
        DOWNLOAD_SUCCESS=true
    else
        echo -e "${RED}[ERROR] Could not download binary and Rust 'cargo' is not installed.${NC}"
        echo "Please install Rust (curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh) or download binaries manually."
        exit 1
    fi
fi

# ---------- Set up Installation Destination ----------
echo ""
echo "[3/4] Installing 'tsc' executable..."

if [ "$IS_TERMUX" = true ]; then
    INSTALL_DIR="$PREFIX/bin"
elif [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
else
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
fi

if [ -f "$TEMP_DIR/tsc" ]; then
    SRC_BIN="$TEMP_DIR/tsc"
else
    SRC_BIN=$(find "$TEMP_DIR" -type f -name "tsc" | head -n 1)
fi

if [ -w "$INSTALL_DIR" ]; then
    cp "$SRC_BIN" "$INSTALL_DIR/tsc"
    ln -sf "$INSTALL_DIR/tsc" "$INSTALL_DIR/techscript"
    ln -sf "$INSTALL_DIR/tsc" "$INSTALL_DIR/webforge"
    ln -sf "$INSTALL_DIR/tsc" "$INSTALL_DIR/tech"
else
    sudo cp "$SRC_BIN" "$INSTALL_DIR/tsc"
    sudo ln -sf "$INSTALL_DIR/tsc" "$INSTALL_DIR/techscript"
    sudo ln -sf "$INSTALL_DIR/tsc" "$INSTALL_DIR/webforge"
    sudo ln -sf "$INSTALL_DIR/tsc" "$INSTALL_DIR/tech"
fi

chmod +x "$INSTALL_DIR/tsc"

# ---------- Set up PATH ----------
echo ""
echo "[4/4] Verifying PATH environment..."
SHELL_RC=""
case "${SHELL}" in
    */zsh)   SHELL_RC="$HOME/.zshrc" ;;
    */bash)  SHELL_RC="$HOME/.bashrc" ;;
    *)       SHELL_RC="$HOME/.profile" ;;
esac

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    if [ -f "$SHELL_RC" ]; then
        echo "" >> "$SHELL_RC"
        echo "# TechScript CLI" >> "$SHELL_RC"
        echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$SHELL_RC"
        echo -e "${GREEN}✓ Added $INSTALL_DIR to $SHELL_RC${NC}"
    fi
fi

# ---------- Verify ----------
echo ""
export PATH="$INSTALL_DIR:$PATH"
if command -v tsc &>/dev/null; then
    echo -e "${GREEN}✓ Verified TechScript installation:${NC}"
    tsc --version
else
    echo -e "${YELLOW}[WARN] tsc installed to $INSTALL_DIR. Restart shell or source $SHELL_RC to activate.${NC}"
fi

rm -rf "$TEMP_DIR"
echo ""
echo -e "${GREEN}${BOLD}===================================================${NC}"
echo -e "${GREEN}${BOLD}   TechScript Setup Complete! 🎉                   ${NC}"
echo -e "${GREEN}${BOLD}===================================================${NC}"
echo "Run 'tsc --help' to start coding."
echo ""
