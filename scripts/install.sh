#!/usr/bin/env bash
# ============================================================
#  TechScript v2.0.0 — macOS & Linux Universal Installer
#  Downloads pre-compiled native binaries or builds from source.
#  Run with:  curl -fsSL https://raw.githubusercontent.com/Tcode-Motion/techscript/main/scripts/install.sh | bash
# ============================================================

set -e

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
BLUE='\033[0;34m'; BOLD='\033[1m'; NC='\033[0m'

echo ""
echo -e "${BLUE}${BOLD}  =======================================${NC}"
echo -e "${BLUE}${BOLD}   TechScript 2.0 — Universal Installer  ${NC}"
echo -e "${BLUE}${BOLD}  =======================================${NC}"
echo ""

# ---------- Detect OS & Architecture ----------
OS="$(uname -s)"
ARCH="$(uname -m)"
case "${OS}" in
    Linux*)  PLATFORM="Linux"  ;;
    Darwin*) PLATFORM="macOS"  ;;
    *)       PLATFORM="Unknown";;
esac
echo -e "  Detected Platform: ${GREEN}${PLATFORM} (${ARCH})${NC}"

# ---------- Get Latest Release Tag ----------
echo ""
echo "  [1/4] Retrieving latest release information..."
LATEST_TAG=$(curl -s "https://api.github.com/repos/Tcode-Motion/techscript/releases/latest" | grep -Po '"tag_name": "\K[^"]*' || true)
if [ -z "$LATEST_TAG" ] || [[ "$LATEST_TAG" == v1.* ]]; then
    LATEST_TAG="release-2.0.0"
fi
echo -e "  Target Release: ${GREEN}${LATEST_TAG}${NC}"

# ---------- Determine Asset Name ----------
ASSET_NAME=""
if [ "$PLATFORM" = "Linux" ] && [ "$ARCH" = "x86_64" ]; then
    ASSET_NAME="techscript-linux-x64.tar.gz"
elif [ "$PLATFORM" = "macOS" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        ASSET_NAME="techscript-macos-x64.tar.gz"
    elif [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
        ASSET_NAME="techscript-macos-arm64.tar.gz"
    fi
fi

# ---------- Download or Compile ----------
echo ""
echo "  [2/4] Fetching TechScript engine..."

TEMP_DIR=$(mktemp -d)
DOWNLOAD_SUCCESS=false

log_message() {
    echo -e "  [INFO] $1"
}

if [ -n "$ASSET_NAME" ]; then
    DOWNLOAD_URL="https://github.com/Tcode-Motion/techscript/releases/download/${LATEST_TAG}/${ASSET_NAME}"
    log_message "Pre-compiled native binary found: $ASSET_NAME"
    log_message "Downloading from $DOWNLOAD_URL..."
    
    if curl -fsSL "$DOWNLOAD_URL" -o "$TEMP_DIR/$ASSET_NAME"; then
        log_message "Extracting release files..."
        tar -xzf "$TEMP_DIR/$ASSET_NAME" -C "$TEMP_DIR"
        DOWNLOAD_SUCCESS=true
        echo -e "  ${GREEN}✓ Native binary downloaded and verified.${NC}"
    else
        log_message "Failed to download pre-compiled binary. Falling back to source build..."
    fi
fi

if [ "$DOWNLOAD_SUCCESS" != "true" ]; then
    log_message "No compatible pre-compiled binary found (or download failed)."
    log_message "Attempting to build from source using Rust toolchain..."
    
    if ! command -v cargo &>/dev/null; then
        echo -e "  ${RED}[ERROR] Rust cargo package manager not found.${NC}"
        echo "  Since no pre-compiled binary is available for your architecture,"
        echo "  you need to install Rust to compile TechScript from source."
        echo "  Install Rust with this command:"
        echo -e "    ${YELLOW}curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh${NC}"
        exit 1
    fi
    
    log_message "Rust toolchain detected: $(cargo --version)"
    log_message "Cloning repository..."
    git clone https://github.com/Tcode-Motion/techscript.git "$TEMP_DIR/techscript"
    
    cd "$TEMP_DIR/techscript"
    log_message "Compiling release binaries (cargo build --release)..."
    cargo build --workspace --release
    cp target/release/tsc "$TEMP_DIR/tsc"
    echo -e "  ${GREEN}✓ Built successfully from source.${NC}"
fi

# ---------- Set up Destination ----------
echo ""
echo "  [3/4] Setup 'tsc' executable binary..."

# Determine installation directory
INSTALL_DIR="/usr/local/bin"

if [ ! -w "$INSTALL_DIR" ]; then
    # If /usr/local/bin is not writable, try ~/.local/bin
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
fi

log_message "Installing binary to $INSTALL_DIR/tsc"
if [ -w "$INSTALL_DIR" ]; then
    cp "$TEMP_DIR/tsc" "$INSTALL_DIR/tsc"
else
    log_message "Requesting superuser privileges to copy binary to $INSTALL_DIR..."
    sudo cp "$TEMP_DIR/tsc" "$INSTALL_DIR/tsc"
fi
chmod +x "$INSTALL_DIR/tsc"

# ---------- Set up Shell PATH ----------
echo ""
echo "  [4/4] Setting up environment PATH..."

SHELL_RC=""
case "${SHELL}" in
    */zsh)   SHELL_RC="$HOME/.zshrc" ;;
    */bash)  SHELL_RC="$HOME/.bashrc" ;;
    *)       SHELL_RC="$HOME/.profile" ;;
esac

# Check if INSTALL_DIR is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    EXPORT_LINE="export PATH=\"$INSTALL_DIR:\$PATH\""
    if [ -f "$SHELL_RC" ]; then
        if ! grep -q "$INSTALL_DIR" "$SHELL_RC"; then
            echo "" >> "$SHELL_RC"
            echo "# TechScript v2 Toolchain" >> "$SHELL_RC"
            echo "$EXPORT_LINE" >> "$SHELL_RC"
            echo -e "  ${GREEN}✓ Added PATH configuration to $SHELL_RC${NC}"
        fi
    else
        echo -e "  ${YELLOW}[WARN] Please manually add $INSTALL_DIR to your shell PATH.${NC}"
    fi
else
    echo -e "  ${GREEN}✓ $INSTALL_DIR is already in your environment PATH.${NC}"
fi

# ---------- Verify Installation ----------
echo ""
export PATH="$INSTALL_DIR:$PATH"
if command -v tsc &>/dev/null; then
    echo -e "  ${GREEN}✓ 'tsc' command is successfully installed!${NC}"
    echo "  Version:"
    tsc version
else
    echo -e "  ${YELLOW}[WARN] 'tsc' binary installed but not active in current shell environment.${NC}"
    echo "  Please restart your terminal or run:  source $SHELL_RC"
fi

# ---------- Clean Up ----------
rm -rf "$TEMP_DIR"

echo ""
echo -e "${GREEN}${BOLD}  =======================================${NC}"
echo -e "${GREEN}${BOLD}   TechScript v2.0.0 Setup Complete! 🎉  ${NC}"
echo -e "${GREEN}${BOLD}  =======================================${NC}"
echo ""
echo "  Create and run your first project:"
echo "    tsc new my_project"
echo "    cd my_project"
echo "    tsc run"
echo ""
