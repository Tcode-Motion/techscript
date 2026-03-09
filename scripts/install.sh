#!/usr/bin/env bash
# ============================================================
#  TechScript v2 — macOS & Linux Universal Installer
#  Run with:  bash scripts/install.sh
# ============================================================

set -e

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
BLUE='\033[0;34m'; BOLD='\033[1m'; NC='\033[0m'

echo ""
echo -e "${BLUE}${BOLD}  ==============================${NC}"
echo -e "${BLUE}${BOLD}   TechScript v2 — Installer${NC}"
echo -e "${BLUE}${BOLD}  ==============================${NC}"
echo ""

# ---------- Detect OS ----------
OS="$(uname -s)"
case "${OS}" in
    Linux*)  PLATFORM="Linux"  ;;
    Darwin*) PLATFORM="macOS"  ;;
    *)       PLATFORM="Unknown";;
esac
echo -e "  Platform: ${GREEN}${PLATFORM}${NC}"

# ---------- Check Python ----------
echo ""
echo "  [1/4] Checking for Python 3.10+..."
if ! command -v python3 &>/dev/null; then
    echo -e "  ${RED}[ERROR] python3 not found.${NC}"
    if [ "$PLATFORM" = "macOS" ]; then
        echo "  Install it with:  brew install python"
    else
        echo "  Install it with:  sudo apt install python3 python3-pip"
    fi
    exit 1
fi

PY_VER=$(python3 -c "import sys; print(f'{sys.version_info.major}.{sys.version_info.minor}')")
PY_MAJOR=$(python3 -c "import sys; print(sys.version_info.major)")
PY_MINOR=$(python3 -c "import sys; print(sys.version_info.minor)")

if [ "$PY_MAJOR" -lt 3 ] || ([ "$PY_MAJOR" -eq 3 ] && [ "$PY_MINOR" -lt 10 ]); then
    echo -e "  ${RED}[ERROR] Python 3.10+ required, found ${PY_VER}${NC}"
    exit 1
fi
echo -e "  ${GREEN}✓ Python ${PY_VER}${NC}"

# ---------- Install pip if missing ----------
if ! command -v pip3 &>/dev/null && ! python3 -m pip --version &>/dev/null 2>&1; then
    echo ""
    echo "  [INFO] pip not found. Installing..."
    if [ "$PLATFORM" = "macOS" ]; then
        python3 -m ensurepip --upgrade
    else
        sudo apt-get install -y python3-pip 2>/dev/null || python3 -m ensurepip --upgrade
    fi
fi

# ---------- Install TechScript ----------
echo ""
echo "  [2/4] Installing TechScript..."
python3 -m pip install techscript --quiet --upgrade
echo -e "  ${GREEN}✓ TechScript installed${NC}"

# ---------- Ensure 'tech' is in PATH ----------
echo ""
echo "  [3/4] Setting up 'tech' command..."

# Find the Scripts/bin dir from pip
PIP_BIN=$(python3 -m site --user-base 2>/dev/null)/bin
SYSTEM_BIN=$(python3 -c "import sys, os; print(os.path.join(os.path.dirname(sys.executable)))")

TECH_PATH=""
for DIR in "$PIP_BIN" "$SYSTEM_BIN" "$HOME/.local/bin" "/usr/local/bin"; do
    if [ -f "$DIR/tech" ]; then
        TECH_PATH="$DIR"
        break
    fi
done

if [ -z "$TECH_PATH" ]; then
    TECH_PATH="$HOME/.local/bin"
fi

# Add to shell config
add_to_path() {
    local SHELL_RC="$1"
    local EXPORT_LINE="export PATH=\"$TECH_PATH:\$PATH\""
    if [ -f "$SHELL_RC" ]; then
        if ! grep -q "$TECH_PATH" "$SHELL_RC"; then
            echo "" >> "$SHELL_RC"
            echo "# TechScript" >> "$SHELL_RC"
            echo "$EXPORT_LINE" >> "$SHELL_RC"
            echo -e "  ${GREEN}✓ Added to $SHELL_RC${NC}"
        fi
    fi
}

add_to_path "$HOME/.bashrc"
add_to_path "$HOME/.zshrc"
add_to_path "$HOME/.profile"

export PATH="$TECH_PATH:$PATH"

if command -v tech &>/dev/null; then
    echo -e "  ${GREEN}✓ 'tech' command is available${NC}"
    tech version
else
    echo -e "  ${YELLOW}[WARN] 'tech' not immediately in PATH.${NC}"
    echo "  Restart your terminal or run:  source ~/.bashrc"
fi

# ---------- .txs file association (Linux only with xdg-utils) ----------
echo ""
echo "  [4/4] Configuring file associations..."
if [ "$PLATFORM" = "Linux" ] && command -v xdg-mime &>/dev/null; then
    cat > /tmp/techscript.xml << 'XMLEOF'
<?xml version="1.0" encoding="UTF-8"?>
<mime-info xmlns="http://www.freedesktop.org/standards/shared-mime-info">
  <mime-type type="text/x-techscript">
    <comment>TechScript file</comment>
    <glob pattern="*.txs"/>
  </mime-type>
</mime-info>
XMLEOF
    xdg-mime install --novendor /tmp/techscript.xml 2>/dev/null || true
    echo -e "  ${GREEN}✓ .txs mime type registered${NC}"
elif [ "$PLATFORM" = "macOS" ]; then
    echo "  (macOS file associations managed by VS Code extension)"
fi

# ---------- VS Code extension ----------
if command -v code &>/dev/null; then
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    VSIX="$SCRIPT_DIR/../vscode-extension/techscript-2.0.0.vsix"
    if [ -f "$VSIX" ]; then
        code --install-extension "$VSIX" &>/dev/null && \
            echo -e "  ${GREEN}✓ VS Code extension installed${NC}" || \
            echo "  [WARN] VS Code extension install failed."
    fi
fi

# ---------- Done ----------
echo ""
echo -e "${GREEN}${BOLD}  ==============================${NC}"
echo -e "${GREEN}${BOLD}   Setup Complete! 🎉${NC}"
echo -e "${GREEN}${BOLD}  ==============================${NC}"
echo ""
echo "  Try it:  tech run examples/hello.txs"
echo "  Web app: tech run examples/web_app_simple.txs"
echo "  Docs:    docs/QUICKSTART.md"
echo ""
echo "  (Restart your terminal if 'tech' is not found)"
echo ""
