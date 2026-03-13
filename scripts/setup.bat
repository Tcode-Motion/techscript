@echo off
setlocal enabledelayedexpansion

:: ============================================================
::  TechScript v2 — Windows Universal Installer
::  - Installs Python dependency (techscript)
::  - Adds `tech` to PATH
::  - Registers .txs file association with icon
::  - Installs the VS Code extension
:: ============================================================

cls
echo.
echo  =============================================
echo    TechScript v2 — Windows Setup
echo  =============================================
echo.

:: ---------- Check Python ----------
echo  [1/5] Checking for Python...
python --version >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo  [ERROR] Python is not installed or not in PATH.
    echo.
    echo  Please install Python 3.10+ from:
    echo    https://www.python.org/downloads/
    echo.
    echo  IMPORTANT: Check "Add Python to PATH" during install!
    echo.
    pause
    exit /b 1
)
for /f "tokens=2" %%V in ('python --version 2^>^&1') do set PY_VER=%%V
echo  Found Python %PY_VER%

:: ---------- Install techscript ----------
echo.
echo  [2/5] Installing TechScript...
pip install techscript --quiet --upgrade
if %ERRORLEVEL% NEQ 0 (
    echo  [ERROR] pip install failed. Trying with python -m pip...
    python -m pip install techscript --quiet --upgrade
    if %ERRORLEVEL% NEQ 0 (
        echo  [ERROR] Failed to install TechScript.
        pause
        exit /b 1
    )
)
echo  TechScript installed!

:: ---------- Verify 'tech' command ----------
echo.
echo  [3/5] Verifying installation...
tech version >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    :: Try adding Scripts to path for this session
    for /f "delims=" %%P in ('python -c "import sys, os; print(os.path.join(os.path.dirname(sys.executable), 'Scripts'))"') do set SCRIPTS_DIR=%%P
    set PATH=!SCRIPTS_DIR!;%PATH%
    echo  Added %SCRIPTS_DIR% to session PATH.
    :: Persist to user PATH safely via PowerShell .NET API (no 1024-char limit)
    powershell -NoProfile -Command "[Environment]::SetEnvironmentVariable('PATH', '%SCRIPTS_DIR%;' + [Environment]::GetEnvironmentVariable('PATH', 'User'), 'User')" >nul 2>&1
    if %ERRORLEVEL% EQU 0 (
        echo  PATH updated safely via PowerShell.
    ) else (
        echo  [WARN] Could not update PATH automatically. Please add %SCRIPTS_DIR% manually.
    )
)
tech version
echo  'tech' command is available!

:: ---------- Register .txs file association ----------
echo.
echo  [4/5] Registering .txs file type...
assoc .txs=TechScript.File >nul 2>&1
:: Find where tech.exe lives
for /f "delims=" %%F in ('where tech 2^>nul') do set TECH_EXE=%%F
if defined TECH_EXE (
    ftype TechScript.File="%TECH_EXE%" run "%%1" >nul 2>&1
    echo  .txs files now open with 'tech run' by default.
) else (
    echo  [WARN] Could not find tech.exe to register file type.
)

:: ---------- Install VS Code extension ----------
echo.
echo  [5/5] Checking for VS Code...
where code >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    set VSIX_PATH=%~dp0vscode-extension\techscript-1.0.2.vsix
    if exist "!VSIX_PATH!" (
        echo  Installing VS Code extension...
        code --install-extension "!VSIX_PATH!" >nul 2>&1
        echo  VS Code extension installed!
    ) else (
        echo  [INFO] VS Code extension VSIX not found at !VSIX_PATH!
        echo         Install manually from the vscode-extension folder.
    )
) else (
    echo  [INFO] VS Code not found. Skipping extension install.
)

:: ---------- Done ----------
echo.
echo  =============================================
echo    Installation Complete!
echo  =============================================
echo.
echo  Try it now — open a new terminal and run:
echo.
echo    tech run examples\hello.txs
echo.
echo  Or build a website instantly:
echo.
echo    tech run examples\web_app_simple.txs
echo.
echo  Documentation: docs\QUICKSTART.md
echo.
pause
