@echo off
title Welcome to TechScript!
color 0A
echo ===================================================
echo             Welcome to TechScript 2.0!
echo ===================================================
echo.
echo [x] Compiler Installed (tsc.exe)
echo [x] Runtime Installed (tsvm.exe)
echo [x] PATH Environment Variables Configured
echo.
set /p choice="Would you like to initialize your first console project? (Y/N): "
if /i "%choice%"=="Y" (
    echo.
    echo Running: tsc new hello_app --template console
    tsc new hello_app --template console
    echo.
    echo Project created! You can now edit hello_app/src/main.txs and run:
    echo cd hello_app
    echo tsc run src/main.txs
)
echo.
pause
