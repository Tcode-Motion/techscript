@echo off
REM ============================================================
REM  TechScript v1.0.6 — Universal Launcher
REM  Auto-builds if needed, then dispatches commands.
REM
REM  Usage:
REM    run.bat examples\hello.txs    Run a script
REM    run.bat repl                  Start interactive REPL
REM    run.bat studio                Launch TechScript Studio IDE
REM    run.bat build                 Build release binaries
REM    run.bat test                  Run Cargo tests
REM    run.bat smoke                 Run full smoke test suite
REM    run.bat version               Show version info
REM ============================================================
setlocal
cd /d "%~dp0"
set RUNTIME=%~dp0runtime
set TECH=%RUNTIME%\target\x86_64-pc-windows-msvc\release\tech.exe
if not exist "%TECH%" set TECH=%RUNTIME%\target\release\tech.exe
if not exist "%TECH%" (
  echo Building TechScript...
  pushd "%RUNTIME%"
  cargo build --release --bin tech
  popd
  set TECH=%RUNTIME%\target\x86_64-pc-windows-msvc\release\tech.exe
  if not exist "%TECH%" set TECH=%RUNTIME%\target\release\tech.exe
)

set STUDIO=%RUNTIME%\target\x86_64-pc-windows-msvc\release\tech_studio.exe
if not exist "%STUDIO%" set STUDIO=%RUNTIME%\target\release\tech_studio.exe

if "%~1"=="" (
  "%TECH%" version
  echo.
  echo Usage:
  echo   run.bat ^<script.txs^>     Run a TechScript file
  echo   run.bat repl             Start interactive REPL
  echo   run.bat studio           Launch TechScript Studio IDE
  echo   run.bat build            Build release binaries
  echo   run.bat test             Run unit tests
  echo   run.bat smoke            Run smoke test suite
  exit /b 0
)
if /i "%~1"=="repl" (
  "%TECH%" repl
  exit /b %ERRORLEVEL%
)
if /i "%~1"=="studio" (
  if exist "%STUDIO%" (
    start "" "%STUDIO%"
  ) else (
    echo Building TechScript Studio...
    pushd "%RUNTIME%"
    cargo build --release --bin tech_studio
    popd
    set STUDIO=%RUNTIME%\target\x86_64-pc-windows-msvc\release\tech_studio.exe
    if not exist "%STUDIO%" set STUDIO=%RUNTIME%\target\release\tech_studio.exe
    start "" "%STUDIO%"
  )
  exit /b 0
)
if /i "%~1"=="build" (
  pushd "%RUNTIME%"
  echo Building tech.exe...
  cargo build --release --bin tech
  echo Building tech_studio.exe...
  cargo build --release --bin tech_studio
  popd
  exit /b %ERRORLEVEL%
)
if /i "%~1"=="test" (
  pushd "%RUNTIME%"
  cargo test
  popd
  exit /b %ERRORLEVEL%
)
if /i "%~1"=="smoke" (
  set TECHSCRIPT_WEB_TEST=1
  set TECHSCRIPT_GUI_TEST=1
  set TECHSCRIPT_3D_TEST=1
  set TECHSCRIPT_NON_INTERACTIVE=1
  powershell -NoProfile -File "%~dp0scripts\smoke_all.ps1"
  exit /b %ERRORLEVEL%
)
if /i "%~1"=="version" (
  "%TECH%" version
  exit /b %ERRORLEVEL%
)
"%TECH%" %*
exit /b %ERRORLEVEL%
