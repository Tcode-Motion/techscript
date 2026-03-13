#!/usr/bin/env bash
# ============================================================
#  TechScript v2 — macOS & Linux Universal Installer
#  v1.0.4.7 "Sanitized Universal Edition"
# ============================================================

set -e

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
BLUE='\033[0;34m'; BOLD='\033[1m'; NC='\033[0m'

echo ""
echo -e "${BLUE}${BOLD}  ==============================${NC}"
echo -e "${BLUE}${BOLD}   TechScript v2 — Installer${NC}"
echo -e "${BLUE}${BOLD}  ==============================${NC}"
echo ""

# ---------- Detect Platform ----------
OS="$(uname -s)"
ARCH="$(uname -m)"
case "${OS}" in
    Linux*)  PLATFORM="Linux"  ;;
    Darwin*) PLATFORM="macOS"  ;;
    *)       PLATFORM="Unknown";;
esac
echo -e "  Platform: ${GREEN}${PLATFORM} (${ARCH})${NC}"

LATEST_VERSION="1.0.4.7"
REPO="Tcode-Motion/techscript"
NATIVE_SUCCESS=false

# ---------- Native Binary Check (Bypass Python if possible) ----------
echo ""
echo "  [1/4] Checking for High-Performance Native Engine..."
NATIVE_ASSET=""
if [ "$PLATFORM" = "Linux" ] && [ "$ARCH" = "x86_64" ]; then
    NATIVE_ASSET="tech-linux-x64"
elif [ "$PLATFORM" = "macOS" ]; then
    NATIVE_ASSET="tech-macos-x64"
fi

if [ -n "$NATIVE_ASSET" ]; then
    BIN_DIR="$HOME/.local/bin"
    mkdir -p "$BIN_DIR"
    echo "  [INFO] Native support detected. Attempting direct download..."
    
    if curl -fsSL -o "$BIN_DIR/tech" "https://github.com/$REPO/releases/download/v$LATEST_VERSION/$NATIVE_ASSET"; then
        chmod +x "$BIN_DIR/tech"
        echo -e "  ${GREEN}✓ Native Rust Engine installed successfully (No Python required).${NC}"
        NATIVE_SUCCESS=true
    else
        echo "  [INFO] Native binary not yet available for v$LATEST_VERSION. Moving to universal install."
    fi
fi

# ---------- Fallback: Universal Python Engine ----------
if [ "$NATIVE_SUCCESS" != "true" ]; then
    echo ""
    echo "  [2/4] Setting up Universal Engine (via Python)..."
    
    if ! command -v python3 &>/dev/null; then
        echo -e "  ${RED}[ERROR] python3 not found.${NC}"
        exit 1
    fi

    # Detect PEP 668 (externally-managed-environment)
    PIP_FLAGS=""
    if python3 -m pip install techscript-lang --dry-run 2>&1 | grep -q "externally-managed-environment"; then
        echo "  [INFO] Detected externally-managed environment (Kali/Debian/Ubuntu)."
        echo "  [INFO] Applying --break-system-packages workaround automatically..."
        PIP_FLAGS="--break-system-packages"
    fi

    if ! python3 -m pip install techscript-lang --quiet --upgrade $PIP_FLAGS; then
        echo -e "  ${YELLOW}[WARN] Primary install failed. Retrying with force flags...${NC}"
        python3 -m pip install techscript-lang --quiet --upgrade --break-system-packages || {
            echo -e "  ${RED}[ERROR] Python engine installation failed.${NC}"
            exit 1
        }
    fi
    echo -e "  ${GREEN}✓ Universal Engine installed via pip${NC}"
fi

# ---------- Path Setup ----------
echo ""
echo "  [3/4] Configuring environment..."
BIN_DIR="$HOME/.local/bin"
[ -d "$BIN_DIR" ] || mkdir -p "$BIN_DIR"

# If we used pip, the binary might be in the python user bin
PIP_USER_BIN=$(python3 -m site --user-base 2>/dev/null)/bin
if [ -f "$PIP_USER_BIN/tech" ] && [ "$NATIVE_SUCCESS" != "true" ]; then
    BIN_DIR="$PIP_USER_BIN"
fi

# Add to shell profile
add_to_path() {
    local RC="$1"
    if [ -f "$RC" ] && ! grep -q "$BIN_DIR" "$RC"; then
        echo -e "\n# TechScript\nexport PATH=\"$BIN_DIR:\$PATH\"" >> "$RC"
        echo -e "  ${GREEN}✓ Updated $RC${NC}"
    fi
}

add_to_path "$HOME/.bashrc"
add_to_path "$HOME/.zshrc"
add_to_path "$HOME/.profile"

export PATH="$BIN_DIR:$PATH"

if command -v tech &>/dev/null; then
    echo -e "  ${GREEN}✓ 'tech' command ready!${NC}"
    tech version
else
    echo -e "  ${YELLOW}[WARN] Please restart your terminal or run: source ~/.bashrc${NC}"
fi

# ---------- Done ----------
echo ""
echo -e "${GREEN}${BOLD}  ==============================${NC}"
echo -e "${GREEN}${BOLD}   TechScript v1.0.4.7 Installed! 🎉${NC}"
echo -e "${GREEN}${BOLD}  ==============================${NC}"
echo ""
echo "  Quick Start:"
echo "  tech run examples/hello.txs"
echo ""
