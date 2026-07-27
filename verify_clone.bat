@echo off
:: TechScript 2.0 Repository Verification Script
:: This script verifies that a fresh clone builds and executes successfully.

echo ===================================================
echo   TechScript 2.0 Repository Verification System
echo ===================================================
echo.

echo [1/4] Checking Rust/Cargo environment...
cargo --version >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Rust and Cargo are not installed! Install them via https://rustup.rs
    exit /b 1
)
echo [PASS] Cargo is available.
echo.

echo [2/4] Building workspace targets in release mode...
cargo build --workspace --release
if %errorlevel% neq 0 (
    echo [ERROR] Compilation failed! Check compiler logs.
    exit /b 1
)
echo [PASS] All targets compiled successfully.
echo.

echo [3/4] Running workspace test suite...
cargo test --workspace
if %errorlevel% neq 0 (
    echo [ERROR] Unit/integration tests failed!
    exit /b 1
)
echo [PASS] All tests passed.
echo.

echo [4/4] Verifying hello world example execution...
target\release\tsc.exe run examples\hello_world\hello.txs > temp_output.txt
if %errorlevel% neq 0 (
    echo [ERROR] Example run failed!
    del temp_output.txt >nul 2>&1
    exit /b 1
)

set /p ACTUAL_OUT=<temp_output.txt
del temp_output.txt >nul 2>&1

if "%ACTUAL_OUT%"=="Hello, World!" (
    echo [PASS] Execution output is correct.
    echo.
    echo ===================================================
    echo  SUCCESS: TechScript repository is release-ready!
    echo ===================================================
    exit /b 0
) else (
    echo [ERROR] Unexpected output: "%ACTUAL_OUT%"
    exit /b 1
)
