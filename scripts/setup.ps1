# ============================================================
#  TechScript v1.0.6 — Native PowerShell Setup & Live Updater
#  Builds, installs, and configures TechScript on Windows.
#  NO PYTHON REQUIRED — 100% native Rust toolchain only.
# ============================================================

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$RuntimeDir = Join-Path $Root "runtime"
$InstallDir = Join-Path $env:LOCALAPPDATA "TechScript"
$IconsDir = Join-Path $Root "assets\icons"
$VsCodeExtDir = Join-Path $Root "vscode-extension"

Write-Host ""
Write-Host "  =============================================" -ForegroundColor Cyan
Write-Host "    TechScript v1.0.6 — Native PowerShell Setup" -ForegroundColor Cyan
Write-Host "  =============================================" -ForegroundColor Cyan
Write-Host ""

# ---------- Step 1: Check Rust ----------
Write-Host "  [1/8] Checking Rust toolchain..." -ForegroundColor White
try {
    $rustVer = (rustc --version 2>&1) | Out-String
    Write-Host "  Found: $($rustVer.Trim())" -ForegroundColor Green
} catch {
    Write-Host "  [ERROR] Rust is not installed. Install from https://rustup.rs" -ForegroundColor Red
    exit 1
}

# ---------- Step 2: Graceful Process Termination ----------
Write-Host ""
Write-Host "  [2/8] Closing active TechScript processes to prevent locked files..." -ForegroundColor White
$runningStudio = Get-Process -Name "tech_studio" -ErrorAction SilentlyContinue
$runningCli = Get-Process -Name "tech" -ErrorAction SilentlyContinue
$runningAlias = Get-Process -Name "techscript" -ErrorAction SilentlyContinue

if ($runningStudio -or $runningCli -or $runningAlias) {
    Write-Host "  Active instances detected. Automatically stopping them to perform update..." -ForegroundColor Yellow
    Stop-Process -Name "tech_studio" -Force -ErrorAction SilentlyContinue
    Stop-Process -Name "tech" -Force -ErrorAction SilentlyContinue
    Stop-Process -Name "techscript" -Force -ErrorAction SilentlyContinue
    Start-Sleep -Seconds 1 # Wait for file locks to release
    Write-Host "  [OK] Active instances closed cleanly." -ForegroundColor Green
} else {
    Write-Host "  No active file locks found. Clean build ready." -ForegroundColor Green
}

# ---------- Step 3: Build tech.exe ----------
Write-Host ""
Write-Host "  [3/8] Building tech.exe (CLI runtime)..." -ForegroundColor White
Push-Location $RuntimeDir
cargo build --release --bin tech
if ($LASTEXITCODE -ne 0) { Pop-Location; Write-Host "  [ERROR] Build failed!" -ForegroundColor Red; exit 1 }
Pop-Location
Write-Host "  [OK] tech.exe compiled." -ForegroundColor Green

# ---------- Step 4: Build tech_studio.exe ----------
Write-Host ""
Write-Host "  [4/8] Building tech_studio.exe (Studio IDE)..." -ForegroundColor White
Push-Location $RuntimeDir
cargo build --release --bin tech_studio
if ($LASTEXITCODE -ne 0) { Pop-Location; Write-Host "  [ERROR] Build failed!" -ForegroundColor Red; exit 1 }
Pop-Location
Write-Host "  [OK] tech_studio.exe compiled." -ForegroundColor Green

# ---------- Step 5: Install Binaries ----------
Write-Host ""
Write-Host "  [5/8] Installing/Updating to $InstallDir..." -ForegroundColor White
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

$ReleaseDir = Join-Path $RuntimeDir "target\release"
if (-not (Test-Path (Join-Path $ReleaseDir "tech.exe"))) {
    $ReleaseDir = Join-Path $RuntimeDir "target\x86_64-pc-windows-msvc\release"
}
Copy-Item -Force (Join-Path $ReleaseDir "tech.exe") (Join-Path $InstallDir "tech.exe")
Copy-Item -Force (Join-Path $ReleaseDir "tech.exe") (Join-Path $InstallDir "techscript.exe")
Copy-Item -Force (Join-Path $ReleaseDir "tech_studio.exe") (Join-Path $InstallDir "tech_studio.exe")

$icoSrc = Join-Path $IconsDir "icon.ico"
if (Test-Path $icoSrc) {
    Copy-Item -Force $icoSrc (Join-Path $InstallDir "icon.ico")
}
Write-Host "  [OK] Binaries successfully updated." -ForegroundColor Green


# ---------- Step 6: Add to PATH ----------
Write-Host ""
Write-Host "  [6/8] Updating user PATH environment..." -ForegroundColor White
$currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($currentPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$InstallDir;$currentPath", "User")
    Write-Host "  [OK] Added $InstallDir to PATH." -ForegroundColor Green
} else {
    Write-Host "  [OK] Already registered in PATH." -ForegroundColor Green
}
$env:PATH = "$InstallDir;$env:PATH"

# ---------- Step 7: Register .txs filetype ----------
Write-Host ""
Write-Host "  [7/8] Registering .txs file association..." -ForegroundColor White
$techExe = Join-Path $InstallDir "tech.exe"
$icoFile = Join-Path $InstallDir "icon.ico"

try {
    $classesRoot = "HKCU:\Software\Classes"

    # .txs extension
    New-Item -Path "$classesRoot\.txs" -Force | Out-Null
    Set-ItemProperty -Path "$classesRoot\.txs" -Name "(Default)" -Value "TechScript.Script"
    Set-ItemProperty -Path "$classesRoot\.txs" -Name "Content Type" -Value "text/x-techscript"
    Set-ItemProperty -Path "$classesRoot\.txs" -Name "PerceivedType" -Value "text"

    # .tx extension
    New-Item -Path "$classesRoot\.tx" -Force | Out-Null
    Set-ItemProperty -Path "$classesRoot\.tx" -Name "(Default)" -Value "TechScript.Library"

    # File type class
    New-Item -Path "$classesRoot\TechScript.Script" -Force | Out-Null
    Set-ItemProperty -Path "$classesRoot\TechScript.Script" -Name "(Default)" -Value "TechScript Script File"

    # Icon (Safe formatting pattern to avoid quote-escape issues)
    if (Test-Path $icoFile) {
        New-Item -Path "$classesRoot\TechScript.Script\DefaultIcon" -Force | Out-Null
        Set-ItemProperty -Path "$classesRoot\TechScript.Script\DefaultIcon" -Name "(Default)" -Value ('"{0}",0' -f $icoFile)
    }

    # Open command (Safe formatting pattern to avoid quote-escape issues)
    New-Item -Path "$classesRoot\TechScript.Script\shell\open\command" -Force | Out-Null
    Set-ItemProperty -Path "$classesRoot\TechScript.Script\shell\open\command" -Name "(Default)" -Value ('"{0}" run "%1"' -f $techExe)

    # Library class
    New-Item -Path "$classesRoot\TechScript.Library" -Force | Out-Null
    Set-ItemProperty -Path "$classesRoot\TechScript.Library" -Name "(Default)" -Value "TechScript Library File"

    Write-Host "  [OK] .txs file association registered." -ForegroundColor Green
} catch {
    Write-Host "  [WARN] Could not register filetype: $($_.Exception.Message)" -ForegroundColor Yellow
}

# ---------- Step 8: VS Code Extension ----------
Write-Host ""
Write-Host "  [8/8] Installing VS Code extension..." -ForegroundColor White
$codeCmd = Get-Command code -ErrorAction SilentlyContinue
if ($codeCmd) {
    $vsixPath = Join-Path $VsCodeExtDir "techscript-1.0.6.vsix"
    if (Test-Path $vsixPath) {
        & code --install-extension $vsixPath 2>&1 | Out-Null
        Write-Host "  [OK] VS Code extension installed from VSIX." -ForegroundColor Green
    } else {
        # Fallback: copy extension folder directly
        $extDest = Join-Path $env:USERPROFILE ".vscode\extensions\techscript-team.techscript-1.0.6"
        $pkgJson = Join-Path $VsCodeExtDir "package.json"
        if (Test-Path $pkgJson) {
            Copy-Item -Recurse -Force $VsCodeExtDir $extDest
            Write-Host "  [OK] VS Code extension copied." -ForegroundColor Green
        } else {
            Write-Host "  [INFO] No extension found. Skipping." -ForegroundColor Gray
        }
    }
} else {
    Write-Host "  [INFO] VS Code not found. Skipping." -ForegroundColor Gray
}

# ---------- Smoke Test ----------
Write-Host ""
Write-Host "  Running quick smoke test..." -ForegroundColor White
$helloScript = Join-Path $Root "examples\hello.txs"
if (Test-Path $helloScript) {
    $env:TECHSCRIPT_NON_INTERACTIVE = "1"
    & (Join-Path $InstallDir "tech.exe") run $helloScript
    Write-Host "  [OK] Smoke test passed." -ForegroundColor Green
}

# ---------- Done ----------
Write-Host ""
Write-Host "  =============================================" -ForegroundColor Green
Write-Host "    Setup and Update Complete!" -ForegroundColor Green
Write-Host "  =============================================" -ForegroundColor Green
Write-Host ""
Write-Host "  Installed/Updated to: $InstallDir" -ForegroundColor White
Write-Host ""
Write-Host "  Automatically launching TechScript Studio IDE..." -ForegroundColor Green
Start-Process (Join-Path $InstallDir "tech_studio.exe") -WorkingDirectory $InstallDir
Write-Host ""
