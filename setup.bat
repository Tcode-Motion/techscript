@echo off
echo ========================================
echo  TechScript Standalone Installer
echo ========================================

set INSTALL_DIR=%USERPROFILE%\.techscript\bin
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

echo [1/3] Copying executable...
copy /Y "bin\tech.exe" "%INSTALL_DIR%\tech.exe" >nul

echo [2/3] Adding to User PATH...
powershell -Command "$path = [Environment]::GetEnvironmentVariable('PATH', 'User'); if ($path -notlike '*\.techscript\bin*') { [Environment]::SetEnvironmentVariable('PATH', $path + ';' + $env:USERPROFILE + '\.techscript\bin', 'User') }"

echo [3/3] Installing VS Code extension...
where code >nul 2>&1
if not errorlevel 1 (
    set VSCODE_EXT=%USERPROFILE%\.vscode\extensions\techscript
    if exist "%VSCODE_EXT%" rmdir /s /q "%VSCODE_EXT%"
    mkdir "%VSCODE_EXT%" >nul 2>&1
    xcopy /s /q "vscode-extension\*" "%VSCODE_EXT%\" >nul
) else (
    echo [SKIP] VS Code not found in PATH.
)

echo.
echo ========================================
echo  TechScript Installed Successfully!
echo  Restart your terminal to use 'tech'.
echo ========================================
pause
