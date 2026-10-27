#!/usr/bin/env bash
# ============================================================
# TechScript AI Skill Installer (Linux / macOS / POSIX)
# Installs TechScript skill for Antigravity, Cursor, Claude Code, Codex, and OpenClaw
# ============================================================
set -e

echo "[TechScript Skill Installer] Installing TechScript 2.0 AI Skill..."

BASE_URL="https://raw.githubusercontent.com/Tcode-Motion/techscript/main/skills/techscript/SKILL.md"

# 1. Antigravity & Gemini CLI global skills directory
AGY_SKILL_DIR="$HOME/.gemini/config/skills/techscript"
mkdir -p "$AGY_SKILL_DIR"
curl -fsSL "$BASE_URL" -o "$AGY_SKILL_DIR/SKILL.md"
echo "  ✓ Installed to Google Antigravity & Gemini CLI ($AGY_SKILL_DIR/SKILL.md)"

# 2. Local workspace .agents skills directory (if in a workspace)
if [ -d ".agents" ] || [ -d ".git" ]; then
    mkdir -p ".agents/skills/techscript"
    cp "$AGY_SKILL_DIR/SKILL.md" ".agents/skills/techscript/SKILL.md"
    echo "  ✓ Installed to workspace .agents directory (.agents/skills/techscript/SKILL.md)"
fi

# 3. Cursor rules directory
if [ -d ".cursor" ] || [ -d ".git" ]; then
    mkdir -p ".cursor/rules"
    cp "$AGY_SKILL_DIR/SKILL.md" ".cursor/rules/techscript.md"
    echo "  ✓ Installed to Cursor rules (.cursor/rules/techscript.md)"
fi

# 4. Claude Code guidelines
if [ -d ".claude" ] || [ -f "CLAUDE.md" ]; then
    cp "$AGY_SKILL_DIR/SKILL.md" "TECHSCRIPT_SKILL.md"
    echo "  ✓ Saved TECHSCRIPT_SKILL.md for Claude Code integration"
fi

echo "[TechScript Skill Installer] Installation successful! AI coding agents can now write canonical TechScript."
