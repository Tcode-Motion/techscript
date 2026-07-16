@echo off
rem TechScript 2.0 Release Verification Bat Wrapper
rem Runs the PowerShell verification and smoke testing suite.

powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0verify_release.ps1"
if %errorlevel% neq 0 (
    echo [FAIL] TechScript Release Verification Failed!
    exit /b %errorlevel%
)
echo [SUCCESS] TechScript Release Verification Passed!
