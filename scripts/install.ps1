# ============================================================
# TechScript Official Windows Installer (PowerShell One-Liner)
# Run with:
#   irm https://raw.githubusercontent.com/Tcode-Motion/techscript/main/scripts/install.ps1 | iex
# ============================================================
$ErrorActionPreference = "Stop"

Write-Host ""
Write-Host "===================================================" -ForegroundColor Cyan
Write-Host "    TechScript 2.0 — Windows Universal Installer   " -ForegroundColor Cyan
Write-Host "===================================================" -ForegroundColor Cyan
Write-Host ""

$installDir = Join-Path $env:LOCALAPPDATA "TechScript\bin"
New-Item -ItemType Directory -Force -Path $installDir | Out-Null

Write-Host "[1/4] Retrieving release metadata..." -ForegroundColor Gray
$repo = "Tcode-Motion/techscript"
$releaseUrl = "https://api.github.com/repos/$repo/releases/latest"

try {
    $release = Invoke-RestMethod -Uri $releaseUrl -UseBasicParsing
    $tag = $release.tag_name
} catch {
    $tag = "v2.0.2"
}

Write-Host "Target Release: $tag" -ForegroundColor Green

# Determine Asset
$zipName = "techscript-2.0.0-windows-x86_64.zip"
$downloadUrl = "https://github.com/$repo/releases/download/$tag/$zipName"

$tempZip = Join-Path $env:TEMP $zipName
Write-Host "[2/4] Downloading TechScript binary ($downloadUrl)..." -ForegroundColor Gray

$downloadSucceeded = $false
try {
    Invoke-WebRequest -Uri $downloadUrl -OutFile $tempZip -UseBasicParsing
    $downloadSucceeded = $true
} catch {
    Write-Host "[WARN] Direct release asset download failed. Checking fallback..." -ForegroundColor Yellow
}

if ($downloadSucceeded -and (Test-Path $tempZip)) {
    Write-Host "[3/4] Extracting binaries to $installDir..." -ForegroundColor Gray
    Expand-Archive -Path $tempZip -DestinationPath $installDir -Force
    Remove-Item -Force $tempZip
} else {
    Write-Host "[INFO] Installing local release binary if present..." -ForegroundColor Gray
    $localBin = "target\release\tsc.exe"
    if (Test-Path $localBin) {
        Copy-Item $localBin -Destination (Join-Path $installDir "tsc.exe") -Force
        Copy-Item $localBin -Destination (Join-Path $installDir "techscript.exe") -Force
        Copy-Item $localBin -Destination (Join-Path $installDir "webforge.exe") -Force
        Copy-Item $localBin -Destination (Join-Path $installDir "tech.exe") -Force
    } else {
        Write-Host "[ERROR] Could not download release archive or find local compiled binary." -ForegroundColor Red
        exit 1
    }
}

Write-Host "[4/4] Configuring User PATH environment..." -ForegroundColor Gray
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$installDir", "User")
    $env:Path = "$env:Path;$installDir"
    Write-Host "  ✓ Added $installDir to PATH" -ForegroundColor Green
} else {
    Write-Host "  ✓ $installDir is already in PATH" -ForegroundColor Green
}

Write-Host ""
Write-Host "Verifying installation..." -ForegroundColor Cyan
$tscExe = Join-Path $installDir "tsc.exe"
if (Test-Path $tscExe) {
    & $tscExe --version
    Write-Host ""
    Write-Host "===================================================" -ForegroundColor Green
    Write-Host "  TechScript installed successfully! 🎉            " -ForegroundColor Green
    Write-Host "===================================================" -ForegroundColor Green
    Write-Host "Run 'tsc --help' to get started."
} else {
    Write-Host "[WARN] Installation finished, but tsc.exe was not found in $installDir." -ForegroundColor Yellow
}
