@echo off
setlocal enabledelayedexpansion

cls
echo.
echo  ================================================
echo    TechScript v2 - Windows Installer
echo  ================================================
echo.
echo  Installing TechScript...

set INSTALL_DIR=%LOCALAPPDATA%\TechScript
mkdir "%INSTALL_DIR%" 2>nul

:: Extract tech.exe (bundled in this installer via sfx)
copy "%~dp0tech.exe" "%INSTALL_DIR%\tech.exe" >nul 2>&1

:: Add to user PATH
setx PATH "%INSTALL_DIR%;%PATH%" >nul 2>&1
echo  [OK] Added to PATH

:: Register .txs file association
assoc .txs=TechScript.File >nul 2>&1
ftype TechScript.File="%INSTALL_DIR%\tech.exe" run "%%1" >nul 2>&1
echo  [OK] .txs files registered

:: Done
echo.
echo  ================================================
echo    Installation Complete!
echo  ================================================
echo.
echo  Open a NEW Command Prompt or PowerShell and run:
echo.
echo    tech run examples\hello.txs
echo.
echo  Press any key to close.
pause >nul
