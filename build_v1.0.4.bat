@echo off
REM TechScript v1.0.4 — Release Build Script
echo.
echo ====================================================
echo  TechScript v1.0.4 Release Build
echo ====================================================
echo.

cd /d "%~dp0runtime"

echo [STEP 1] Setting up MSVC environment...
set VCVARS=""
if exist "C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat" (
    set VCVARS="C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
) else if exist "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat" (
    set VCVARS="C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
)

if %VCVARS% == "" (
    echo [WARNING] Could not find vcvars64.bat automatically.
) else (
    call %VCVARS%
)

set "PATH=%PATH%;C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64"

echo [STEP 2] Building TechScript v1.0.4...
cargo build --release --target x86_64-pc-windows-msvc
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [ERROR] Build failed.
    pause
    exit /b 1
)

echo.
echo [OK] Build succeeded!
echo.

REM Copy the new exe to public-release/bin/
if not exist "..\public-release\bin" mkdir "..\public-release\bin"
copy /Y "target\x86_64-pc-windows-msvc\release\techscript.exe" "..\public-release\bin\techscriptv1.0.4.exe"
echo [OK] Copied to public-release\bin\techscriptv1.0.4.exe

REM Verify it works
echo.
echo === Verification ===
"..\public-release\bin\techscriptv1.0.4.exe" version
echo.
echo ====================================================
echo  DONE! techscriptv1.0.4.exe is ready!
echo  New modules: use api | use web | use gui | use 3d | use anime
echo ====================================================
pause
