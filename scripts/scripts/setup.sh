#!/usr/bin/env bash
# ================================================================
#  TechScript One-Command Installer for Linux & macOS
#  Run:  bash scripts/setup.sh           (normal install)
#        bash scripts/setup.sh --dev     (editable/dev install)
# ================================================================

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
INSTALL_MODE="release"
OS_TYPE="$(uname -s)"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

ok()   { echo -e "${GREEN}[OK]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
err()  { echo -e "${RED}[ERROR]${NC} $1"; }
info() { echo -e "${CYAN}[INFO]${NC} $1"; }

echo ""
echo "  ========================================"
echo "   TechScript Installer for ${OS_TYPE}"
echo "  ========================================"
echo ""

# --- Parse args ---
if [[ "${1:-}" == "--dev" ]]; then
    INSTALL_MODE="dev"
fi

# --- Check Python ---
if ! command -v python3 &>/dev/null; then
    err "Python 3 not found. Install Python 3.10+ first."
    exit 1
fi
PYVER=$(python3 -c "import sys; print(f'{sys.version_info.major}.{sys.version_info.minor}')")
ok "Python ${PYVER} found"

# --- Check pip ---
if ! python3 -m pip --version &>/dev/null; then
    err "pip not found. Run:  python3 -m ensurepip"
    exit 1
fi
ok "pip found"

cd "$PROJECT_ROOT"

# ===== Step 1: Install TechScript =====
echo ""
echo "[1/6] Installing TechScript..."
if [[ "$INSTALL_MODE" == "dev" ]]; then
    python3 -m pip install -e ".[dev]"
else
    python3 -m pip install .
fi
ok "TechScript installed"

# ===== Step 2: Build icons =====
echo ""
echo "[2/6] Building icon assets..."
python3 -m pip install Pillow -q 2>/dev/null || true
python3 scripts/build_icons.py || warn "Icon generation failed. Continuing..."

# ===== Step 3: Register MIME type and file association =====
echo ""
echo "[3/6] Registering .txs file association..."

ICON_SRC="$PROJECT_ROOT/assets/icons/icon-128.png"
TECH_BIN="$(command -v tech 2>/dev/null || echo "python3 -m techscript")"

if [[ "$OS_TYPE" == "Linux" ]]; then
    # --- Linux (freedesktop.org) ---

    # 3a. MIME type
    MIME_DIR="${HOME}/.local/share/mime"
    mkdir -p "${MIME_DIR}/packages"
    cat > "${MIME_DIR}/packages/techscript.xml" <<'MIMEXML'
<?xml version="1.0" encoding="UTF-8"?>
<mime-info xmlns="http://www.freedesktop.org/standards/shared-mime-info">
  <mime-type type="text/x-techscript">
    <comment>TechScript source file</comment>
    <glob pattern="*.txs"/>
    <glob pattern="*.tx"/>
    <sub-class-of type="text/plain"/>
    <icon name="techscript"/>
  </mime-type>
</mime-info>
MIMEXML
    update-mime-database "${MIME_DIR}" 2>/dev/null || true
    ok "MIME type registered (text/x-techscript)"

    # 3b. Desktop entry
    APPS_DIR="${HOME}/.local/share/applications"
    mkdir -p "$APPS_DIR"
    cat > "${APPS_DIR}/techscript.desktop" <<DESKTOP
[Desktop Entry]
Type=Application
Name=TechScript
Comment=Run TechScript (.txs) programs
Exec=${TECH_BIN} run %f
Icon=${ICON_SRC}
Terminal=true
MimeType=text/x-techscript;
Categories=Development;TextEditor;
DESKTOP
    chmod +x "${APPS_DIR}/techscript.desktop"
    update-desktop-database "$APPS_DIR" 2>/dev/null || true
    ok ".desktop entry created"

    # 3c. Install icon for file manager
    for SIZE in 16 32 64 128 256 512; do
        ICON_DIR="${HOME}/.local/share/icons/hicolor/${SIZE}x${SIZE}/mimetypes"
        mkdir -p "$ICON_DIR"
        ICON_FILE="$PROJECT_ROOT/assets/icons/icon-${SIZE}.png"
        if [[ -f "$ICON_FILE" ]]; then
            cp "$ICON_FILE" "${ICON_DIR}/text-x-techscript.png"
        fi
    done
    gtk-update-icon-cache "${HOME}/.local/share/icons/hicolor" 2>/dev/null || true
    ok "File icons installed for GNOME/KDE/XFCE"

    # 3d. Set default handler
    xdg-mime default techscript.desktop text/x-techscript 2>/dev/null || true

elif [[ "$OS_TYPE" == "Darwin" ]]; then
    # --- macOS ---
    info "macOS file association requires a .app bundle."
    info "For now, use 'open -a Terminal' with tech CLI."

    # Install UTI declaration via a Launch Services plist (best-effort)
    PLIST_DIR="${HOME}/Library/Preferences"
    ICNS_FILE="$PROJECT_ROOT/assets/icons/icon.icns"

    # Copy icon to a standard location
    ICON_DEST="${HOME}/Library/Application Support/TechScript"
    mkdir -p "$ICON_DEST"
    if [[ -f "$ICNS_FILE" ]]; then
        cp "$ICNS_FILE" "${ICON_DEST}/techscript.icns"
        ok "Icon copied to ~/Library/Application Support/TechScript/"
    fi

    # Create a minimal .app wrapper
    APP_DIR="${HOME}/Applications/TechScript.app"
    mkdir -p "$APP_DIR/Contents/MacOS" "$APP_DIR/Contents/Resources"

    cat > "$APP_DIR/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>TechScript</string>
    <key>CFBundleIdentifier</key>
    <string>com.techscript.app</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>CFBundleExecutable</key>
    <string>tech-run</string>
    <key>CFBundleIconFile</key>
    <string>techscript</string>
    <key>CFBundleDocumentTypes</key>
    <array>
        <dict>
            <key>CFBundleTypeName</key>
            <string>TechScript Source</string>
            <key>CFBundleTypeExtensions</key>
            <array>
                <string>txs</string>
                <string>tx</string>
            </array>
            <key>CFBundleTypeIconFile</key>
            <string>techscript</string>
            <key>CFBundleTypeRole</key>
            <string>Editor</string>
            <key>LSItemContentTypes</key>
            <array>
                <string>com.techscript.source</string>
            </array>
        </dict>
    </array>
    <key>UTExportedTypeDeclarations</key>
    <array>
        <dict>
            <key>UTTypeIdentifier</key>
            <string>com.techscript.source</string>
            <key>UTTypeDescription</key>
            <string>TechScript Source File</string>
            <key>UTTypeConformsTo</key>
            <array>
                <string>public.source-code</string>
                <string>public.plain-text</string>
            </array>
            <key>UTTypeTagSpecification</key>
            <dict>
                <key>public.filename-extension</key>
                <array>
                    <string>txs</string>
                    <string>tx</string>
                </array>
            </dict>
        </dict>
    </array>
</dict>
</plist>
PLIST

    # Create launcher script
    cat > "$APP_DIR/Contents/MacOS/tech-run" <<'LAUNCHER'
#!/bin/bash
if [ -n "$1" ]; then
    tech run "$1"
else
    tech repl
fi
LAUNCHER
    chmod +x "$APP_DIR/Contents/MacOS/tech-run"

    # Copy icon
    if [[ -f "$ICNS_FILE" ]]; then
        cp "$ICNS_FILE" "$APP_DIR/Contents/Resources/techscript.icns"
    fi

    # Refresh Launch Services
    /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/LaunchServices.framework/Versions/A/Support/lsregister -f "$APP_DIR" 2>/dev/null || true
    ok "macOS .app bundle created at ~/Applications/TechScript.app"
fi

# ===== Step 4: Install VS Code extension =====
echo ""
echo "[4/6] Installing VS Code extension..."
if command -v code &>/dev/null; then
    VSCODE_EXT="${HOME}/.vscode/extensions/techscript"
    rm -rf "$VSCODE_EXT"
    mkdir -p "$VSCODE_EXT"
    cp -r vscode-extension/* "$VSCODE_EXT/"
    ok "VS Code extension installed"
    info "Restart VS Code to activate."
else
    warn "VS Code 'code' command not found. Install extension manually."
fi

# ===== Step 5: Verify installation =====
echo ""
echo "[5/6] Verifying installation..."
if tech --version 2>/dev/null; then
    ok "CLI working"
else
    # Check if it's in user local bin
    if python3 -m techscript version 2>/dev/null; then
        ok "CLI working (via python3 -m techscript)"
        warn "'tech' not in PATH. Add to shell config:"
        PYBIN=$(python3 -c "import sysconfig; print(sysconfig.get_path('scripts'))")
        echo "    export PATH=\"\$PATH:${PYBIN}\""
    else
        err "'tech' command not found."
        exit 1
    fi
fi

# ===== Step 6: Run test program =====
echo ""
echo "[6/6] Running test program..."
tech run examples/hello.txs 2>/dev/null || python3 -m techscript run examples/hello.txs

echo ""
echo "  ========================================"
echo "   TechScript installed successfully! 🐉"
echo "  ========================================"
echo ""
echo "  Commands:"
echo "    tech run file.txs        Run a .txs file"
echo "    tech transpile file.txs  Transpile to Python"
echo "    tech repl                Interactive REPL"
echo "    tech check file.txs      Syntax check"
echo ""
