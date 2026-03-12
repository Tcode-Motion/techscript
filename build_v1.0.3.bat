@echo off
REM TechScript v1.0.3 — Release Build Script (MSVC Edition)
echo.
echo ====================================================
echo  TechScript v1.0.3 Release Build (MSVC)
echo ====================================================
echo.

cd /d "%~dp0runtime"

echo [STEP 1] Setting up MSVC environment...
REM Try to find vcvarsall.bat to set up the environment automatically
set VCVARS=""
if exist "C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat" (
    set VCVARS="C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
) else if exist "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat" (
    set VCVARS="C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
)

if %VCVARS% == "" (
    echo [WARNING] Could not find vcvars64.bat automatically. 
    echo If the build fails, please run this script from the "Developer Command Prompt for VS 2022".
) else (
    call %VCVARS%
)

echo [STEP 2] Building TechScript...
cargo build --release --target x86_64-pc-windows-msvc
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [ERROR] Build failed. 
    echo.
    echo Please ensure you have "Desktop development with C++" installed in VS Installer.
    echo Try running this script from the "Developer Command Prompt for VS 2022".
    pause
    exit /b 1
)

echo.
echo [OK] Build succeeded!
echo.

REM Copy the new exe to public-release/bin/
if not exist "..\public-release\bin" mkdir "..\public-release\bin"
copy /Y "target\x86_64-pc-windows-msvc\release\tech.exe" "..\public-release\bin\techscriptv1.0.3.exe"
echo [OK] Copied to public-release\bin\techscriptv1.0.3.exe

REM Verify it works
echo.
echo === Verification ===
"..\public-release\bin\techscriptv1.0.3.exe" version
echo.
echo ====================================================
echo  DONE! techscriptv1.0.3.exe is ready!
echo ====================================================
pause
