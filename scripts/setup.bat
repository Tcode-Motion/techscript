@echo off
setlocal enabledelayedexpansion

:: ============================================================
::  TechScript v1.0.6 — Native Windows Setup & Live Updater
::  NO PYTHON REQUIRED — 100% native Rust toolchain only.
:: ============================================================

cls
echo.
echo  ================================================
echo    TechScript v1.0.6 - Setup and Live Updater
echo  ================================================
echo.

:: Check if powershell is available
where powershell >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] PowerShell is required to run the setup script.
    echo Please install PowerShell or make sure it is in your PATH.
    pause
    exit /b 1
)

:: Run the powershell setup script directly
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0setup.ps1"
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [ERROR] Setup failed! See above errors for details.
    echo.
    pause
    exit /b %ERRORLEVEL%
)

exit /b 0
