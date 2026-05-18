@echo off
setlocal enabledelayedexpansion
:: ============================================================
::  TechScript v1.0.6 — Release Build Pipeline
::  Compiles tech.exe (CLI) and tech_studio.exe (IDE) in
::  release mode with LTO, single codegen-unit, and abort panic
::  for maximum performance and minimal binary size.
:: ============================================================

echo.
echo  =============================================
echo    TechScript v1.0.6 — Release Builder
echo  =============================================
echo.

cd /d "%~dp0..\runtime"

:: ---------- Step 1: Compile tech.exe (CLI) ----------
echo  [1/3] Building tech.exe (CLI runtime)...
cargo build --release --target x86_64-pc-windows-msvc --bin tech
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo  [ERROR] Failed to build tech.exe!
    pause
    exit /b 1
)
echo  tech.exe built successfully.

:: ---------- Step 2: Compile tech_studio.exe (IDE) ----------
echo.
echo  [2/3] Building tech_studio.exe (Studio IDE)...
cargo build --release --target x86_64-pc-windows-msvc --bin tech_studio
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo  [ERROR] Failed to build tech_studio.exe!
    pause
    exit /b 1
)
echo  tech_studio.exe built successfully.

:: ---------- Step 3: Verification ----------
echo.
echo  [3/3] Verifying output binaries...

set RELEASE_DIR=%CD%\target\x86_64-pc-windows-msvc\release
set TECH_EXE=%RELEASE_DIR%\tech.exe
set STUDIO_EXE=%RELEASE_DIR%\tech_studio.exe

if exist "%TECH_EXE%" (
    for %%A in ("%TECH_EXE%") do set TECH_SIZE=%%~zA
    echo  [OK] tech.exe         — !TECH_SIZE! bytes
) else (
    echo  [FAIL] tech.exe not found at %TECH_EXE%
    pause
    exit /b 1
)

if exist "%STUDIO_EXE%" (
    for %%A in ("%STUDIO_EXE%") do set STUDIO_SIZE=%%~zA
    echo  [OK] tech_studio.exe  — !STUDIO_SIZE! bytes
) else (
    echo  [FAIL] tech_studio.exe not found at %STUDIO_EXE%
    pause
    exit /b 1
)

echo.
echo  =============================================
echo    Release build complete!
echo  =============================================
echo.
echo  Output directory:
echo    %RELEASE_DIR%
echo.
echo  Next step: Run the Inno Setup compiler on
echo    installer_build\installer.iss
echo.
pause
