@echo off
setlocal
echo ==========================================
echo    TechScript v1.0.5 — Windows Setup
echo ==========================================
echo.

set "INSTALL_DIR=%USERPROFILE%\.techscript"
set "BIN_DIR=%INSTALL_DIR%\bin"
set "EXT_DIR=%INSTALL_DIR%\extension"

echo [1/3] Creating directories...
if not exist "%BIN_DIR%" mkdir "%BIN_DIR%"
if not exist "%EXT_DIR%" mkdir "%EXT_DIR%"

echo [2/3] Copying binaries...
copy /Y "TechScript_TX.exe" "%BIN_DIR%\tech.exe" >nul
copy /Y "techscript-1.0.5.vsix" "%EXT_DIR%\" >nul

echo [3/3] Updating System PATH...
:: Add to User PATH if not already there
setx PATH "%BIN_DIR%;%PATH%" >nul

echo.
echo ==========================================
echo    Setup Complete! 🎉
echo ==========================================
echo.
echo Try it: Open a NEW terminal and type 'tech'
echo VS Code: The extension is ready in %EXT_DIR%
echo.
pause
