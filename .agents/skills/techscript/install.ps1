# ============================================================
# TechScript AI Skill Installer (Windows PowerShell)
# Installs TechScript skill for Antigravity, Cursor, Gemini CLI, and VS Code AI
# ============================================================
$ErrorActionPreference = "Stop"

Write-Host "[TechScript Skill Installer] Installing TechScript 2.0 AI Skill..." -ForegroundColor Cyan

$baseUrl = "https://raw.githubusercontent.com/Tcode-Motion/techscript/main/skills/techscript/SKILL.md"

# 1. Antigravity & Gemini CLI global skills directory
$agySkillDir = Join-Path $env:USERPROFILE ".gemini\config\skills\techscript"
New-Item -ItemType Directory -Force -Path $agySkillDir | Out-Null
$destFile = Join-Path $agySkillDir "SKILL.md"
Invoke-WebRequest -Uri $baseUrl -OutFile $destFile -UseBasicParsing
Write-Host "  ✓ Installed to Google Antigravity & Gemini CLI ($destFile)" -ForegroundColor Green

# 2. Local workspace .agents skills directory
if (Test-Path ".agents" -Or Test-Path ".git") {
    $wsDir = ".agents\skills\techscript"
    New-Item -ItemType Directory -Force -Path $wsDir | Out-Null
    Copy-Item $destFile -Destination (Join-Path $wsDir "SKILL.md")
    Write-Host "  ✓ Installed to workspace .agents directory ($wsDir\SKILL.md)" -ForegroundColor Green
}

# 3. Cursor rules directory
if (Test-Path ".cursor" -Or Test-Path ".git") {
    $cursorDir = ".cursor\rules"
    New-Item -ItemType Directory -Force -Path $cursorDir | Out-Null
    Copy-Item $destFile -Destination (Join-Path $cursorDir "techscript.md")
    Write-Host "  ✓ Installed to Cursor rules ($cursorDir\techscript.md)" -ForegroundColor Green
}

Write-Host "[TechScript Skill Installer] Installation successful! AI coding agents can now write canonical TechScript." -ForegroundColor Green
