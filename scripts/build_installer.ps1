# ==============================================================================
#  TechScript v1.0.6 — Integrated Release & Installer Builder
#  Automates Rust release builds, verification, and Inno Setup compilation.
# ==============================================================================

$ErrorActionPreference = 'Stop'
Write-Host '=============================================' -ForegroundColor Cyan
Write-Host '  TechScript Studio v1.0.6 — Installer Pipeline' -ForegroundColor Cyan
Write-Host '=============================================' -ForegroundColor Cyan
Write-Host

# Path configuration
$PSScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Definition
$WorkspaceRoot = Resolve-Path (Join-Path $PSScriptRoot '..')
$RuntimeDir = Join-Path $WorkspaceRoot 'runtime'
$InstallerScript = Join-Path $WorkspaceRoot 'installer_build\installer.iss'

# ---------- Step 1: Rust Release Compilation ----------
Write-Host '[1/3] Compiling high-performance release binaries...' -ForegroundColor Yellow
# Stop any running processes to prevent file lock compilation failures
Stop-Process -Name 'tech_studio' -ErrorAction SilentlyContinue
Stop-Process -Name 'tech' -ErrorAction SilentlyContinue
Push-Location $RuntimeDir
try {
    Write-Host 'Building tech.exe (CLI runtime)...' -ForegroundColor Gray
    cargo build --release --target x86_64-pc-windows-msvc --bin tech
    
    Write-Host 'Building tech_studio.exe (Studio IDE)...' -ForegroundColor Gray
    cargo build --release --target x86_64-pc-windows-msvc --bin tech_studio
} catch {
    Write-Host '[ERROR] Cargo build failed! Aborting.' -ForegroundColor Red
    Pop-Location
    Exit 1
}
Pop-Location

# Verify binaries exist
$ReleaseDir = Join-Path $RuntimeDir 'target\x86_64-pc-windows-msvc\release'
$TechExe = Join-Path $ReleaseDir 'tech.exe'
$StudioExe = Join-Path $ReleaseDir 'tech_studio.exe'

if (!(Test-Path $TechExe) -or !(Test-Path $StudioExe)) {
    Write-Host '[ERROR] Could not find compiled output binaries in release folder!' -ForegroundColor Red
    Exit 1
}

$TechSize = (Get-Item $TechExe).Length
$StudioSize = (Get-Item $StudioExe).Length
Write-Host ('[OK] Verified tech.exe - size: ' + $TechSize + ' bytes') -ForegroundColor Green
Write-Host ('[OK] Verified tech_studio.exe - size: ' + $StudioSize + ' bytes') -ForegroundColor Green
Write-Host

# ---------- Step 2: Locate Inno Setup Compiler ----------
Write-Host '[2/3] Locating Inno Setup Compiler (ISCC.exe)...' -ForegroundColor Yellow
$IsccPaths = @(
    'C:\Users\tanmoy\AppData\Local\Programs\Inno Setup 6\ISCC.exe',
    'C:\Program Files (x86)\Inno Setup 6\ISCC.exe',
    'C:\Program Files\Inno Setup 6\ISCC.exe'
)

$IsccPath = $null
foreach ($path in $IsccPaths) {
    if (Test-Path $path) {
        $IsccPath = $path
        break
    }
}

if ($null -eq $IsccPath) {
    # Check system PATH environment
    $IsccPath = Get-Command iscc -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source
}

if ($null -eq $IsccPath) {
    Write-Host '[ERROR] Inno Setup Compiler (ISCC.exe) was not found.' -ForegroundColor Red
    Write-Host 'Please install Inno Setup 6 and verify it is accessible.' -ForegroundColor Yellow
    Exit 1
}

Write-Host ('[OK] Found Inno Setup Compiler at: ' + $IsccPath) -ForegroundColor Green
Write-Host

# ---------- Step 3: Run Inno Setup Compiler ----------
Write-Host '[3/3] Generating Windows GUI Setup Installer...' -ForegroundColor Yellow
Write-Host ('Running ISCC on: ' + $InstallerScript) -ForegroundColor Gray

try {
    & $IsccPath '/Q' $InstallerScript
    
    $OutputExe = Join-Path $WorkspaceRoot 'installer_build\Output\TechScript_v1.0.6_x64.exe'
    if (Test-Path $OutputExe) {
        $OutputSize = (Get-Item $OutputExe).Length
        Write-Host '=============================================' -ForegroundColor Green
        Write-Host '  Success! Installer Setup Generated.' -ForegroundColor Green
        Write-Host ('  Output File: ' + $OutputExe) -ForegroundColor Gray
        Write-Host ('  File Size:   ' + $OutputSize + ' bytes') -ForegroundColor Gray
        Write-Host '=============================================' -ForegroundColor Green
    } else {
        Write-Host '[ERROR] Setup EXE was not found after compilation!' -ForegroundColor Red
        Exit 1
    }
} catch {
    Write-Host '[ERROR] Inno Setup compilation failed!' -ForegroundColor Red
    Exit 1
}
