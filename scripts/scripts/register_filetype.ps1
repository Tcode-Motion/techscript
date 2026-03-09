# ================================================================
#  TechScript .txs File Type Registration — Windows (PowerShell)
#
#  Registers .txs extension in the Windows registry so that:
#    1. .txs files show the TechScript dragon icon
#    2. Double-clicking runs them with `tech run`
#    3. Right-click → "Edit with VS Code" is available
#
#  Must be run elevated (Administrator) for HKLM, or runs in HKCU
#  for current-user-only registration.
# ================================================================

param(
    [switch]$AllUsers  # Use HKLM (requires admin) instead of HKCU
)

$ErrorActionPreference = "Stop"

# --- Determine paths ---
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptDir
$IconPath = Join-Path $ProjectRoot "assets\icons\icon.ico"

# Find tech.exe or python entry point
$TechCmd = (Get-Command tech -ErrorAction SilentlyContinue).Source
if (-not $TechCmd) {
    $TechCmd = "python -m techscript"
}

# --- Choose registry hive ---
if ($AllUsers) {
    $Root = "HKLM:"
    Write-Host "[INFO] Registering for all users (requires admin)" -ForegroundColor Cyan
} else {
    $Root = "HKCU:"
    Write-Host "[INFO] Registering for current user" -ForegroundColor Cyan
}

$ClassesRoot = "$Root\Software\Classes"

try {
    # --- 1. Register .txs extension ---
    Write-Host "[1/4] Registering .txs extension..."
    $extKey = "$ClassesRoot\.txs"
    New-Item -Path $extKey -Force | Out-Null
    Set-ItemProperty -Path $extKey -Name "(Default)" -Value "TechScript.File"
    Set-ItemProperty -Path $extKey -Name "Content Type" -Value "text/x-techscript"
    Set-ItemProperty -Path $extKey -Name "PerceivedType" -Value "text"
    Write-Host "  [OK] .txs extension registered" -ForegroundColor Green

    # --- 2. Register .tx extension (fallback) ---
    $extKeyTx = "$ClassesRoot\.tx"
    New-Item -Path $extKeyTx -Force | Out-Null
    Set-ItemProperty -Path $extKeyTx -Name "(Default)" -Value "TechScript.File"
    Write-Host "  [OK] .tx extension registered" -ForegroundColor Green

    # --- 3. Register file type class ---
    Write-Host "[2/4] Registering TechScript file type..."
    $typeKey = "$ClassesRoot\TechScript.File"
    New-Item -Path $typeKey -Force | Out-Null
    Set-ItemProperty -Path $typeKey -Name "(Default)" -Value "TechScript Source File"
    Set-ItemProperty -Path $typeKey -Name "FriendlyTypeName" -Value "TechScript Source File (.txs)"

    # --- 4. Set icon ---
    Write-Host "[3/4] Setting file icon..."
    if (Test-Path $IconPath) {
        $iconKey = "$typeKey\DefaultIcon"
        New-Item -Path $iconKey -Force | Out-Null
        Set-ItemProperty -Path $iconKey -Name "(Default)" -Value "`"$IconPath`",0"
        Write-Host "  [OK] Icon set to $IconPath" -ForegroundColor Green
    } else {
        Write-Host "  [WARN] icon.ico not found at $IconPath" -ForegroundColor Yellow
        Write-Host "  [INFO] Run 'python scripts/build_icons.py' first" -ForegroundColor Yellow
    }

    # --- 5. Set open command ---
    Write-Host "[4/4] Setting open command..."
    $cmdKey = "$typeKey\shell\open\command"
    New-Item -Path $cmdKey -Force | Out-Null

    if ($TechCmd -like "*.exe") {
        Set-ItemProperty -Path $cmdKey -Name "(Default)" -Value "`"$TechCmd`" run `"%1`""
    } else {
        Set-ItemProperty -Path $cmdKey -Name "(Default)" -Value "cmd /c $TechCmd run `"%1`""
    }

    # --- 6. Add "Edit with TechScript" context menu ---
    $editKey = "$typeKey\shell\edit\command"
    New-Item -Path $editKey -Force | Out-Null
    $codeCmd = (Get-Command code -ErrorAction SilentlyContinue).Source
    if ($codeCmd) {
        Set-ItemProperty -Path $editKey -Name "(Default)" -Value "`"$codeCmd`" `"%1`""
        Write-Host "  [OK] 'Edit' action → VS Code" -ForegroundColor Green
    } else {
        Set-ItemProperty -Path $editKey -Name "(Default)" -Value "notepad `"%1`""
        Write-Host "  [OK] 'Edit' action → Notepad (VS Code not found)" -ForegroundColor Yellow
    }

    Write-Host ""
    Write-Host "[DONE] .txs file type registered successfully!" -ForegroundColor Green
    Write-Host "       Restart Explorer or log out/in to see icon changes." -ForegroundColor Gray

} catch {
    Write-Host "[ERROR] $($_.Exception.Message)" -ForegroundColor Red
    Write-Host "        Try running as Administrator." -ForegroundColor Gray
    exit 1
}
