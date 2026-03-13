@echo off
REM TechScript Update Script — replaces all system tech.exe with the latest build
echo.
echo ====================================================
echo  TechScript System Updater
echo ====================================================
echo.

set SRC=%~dp0runtime\target\x86_64-pc-windows-msvc\release\techscript.exe

if not exist "%SRC%" (
    echo [ERROR] No built binary found. Run build_v1.0.3.bat first!
    pause
    exit /b 1
)

echo [1/3] Updating AppData\Local\TechScript...
copy /Y "%SRC%" "%LOCALAPPDATA%\TechScript\bin\tech.exe" 2>nul
if %ERRORLEVEL% NEQ 0 echo      Skipped (path not found)

echo [2/3] Updating .techscript\bin...
copy /Y "%SRC%" "%USERPROFILE%\.techscript\bin\tech.exe" 2>nul
if %ERRORLEVEL% NEQ 0 echo      Skipped (path not found)

echo [3/3] Updating Python Scripts...
copy /Y "%SRC%" "%LOCALAPPDATA%\Programs\Python\Python310\Scripts\tech.exe" 2>nul
if %ERRORLEVEL% NEQ 0 echo      Skipped (path not found)

echo.
echo === Verification ===
tech version
echo.
echo ====================================================
echo  Update complete!
echo ====================================================
pause
