# TechScript Python vs Rust parity checker
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$RuntimeDir = Join-Path $Root "runtime"
$RustBin = Join-Path $RuntimeDir "target\release\tech.exe"

Write-Host "TechScript Parity Check" -ForegroundColor Cyan

if (-not (Test-Path $RustBin)) {
    Write-Host "Building Rust runtime..."
    Push-Location $RuntimeDir
    cargo build --release --bin tech
    Pop-Location
}

$Examples = @(
    "runtime_examples\01_basics.txs",
    "runtime_examples\02_math_and_logic.txs",
    "runtime_examples\03_control_flow.txs",
    "runtime_examples\04_functions.txs",
    "runtime_examples\05_classes.txs",
    "runtime_examples\06_advanced.txs",
    "examples\hello.txs",
    "examples\calc.txs",
    "examples\classes.txs",
    "examples\syntax_aliases.txs"
)

$Passed = 0
$Failed = 0

foreach ($rel in $Examples) {
    $path = Join-Path $Root $rel
    if (-not (Test-Path $path)) { continue }

    Write-Host "  Running $rel..." -NoNewline
    $rustOut = & "$RustBin" run "$path" 2>&1 | Out-String

    $pythonAvailable = Get-Command python -ErrorAction SilentlyContinue
    if ($pythonAvailable) {
        Push-Location $Root
        $pyOut = C:\Users\tanmoy\AppData\Local\Programs\Python\Python312\python.exe -m techscript.cli run $path 2>&1 | Out-String
        Pop-Location
        if ($rustOut.Trim() -eq $pyOut.Trim()) {
            Write-Host " OK" -ForegroundColor Green
            $Passed++
        } else {
            Write-Host " DIFF" -ForegroundColor Yellow
            Write-Host "    Rust:   $($rustOut.Trim())"
            Write-Host "    Python: $($pyOut.Trim())"
            $Failed++
        }
    } else {
        if ($LASTEXITCODE -eq 0) {
            Write-Host " OK (Rust only)" -ForegroundColor Green
            $Passed++
        } else {
            Write-Host " FAIL" -ForegroundColor Red
            $Failed++
        }
    }
}

Write-Host ""
Write-Host "Results: $Passed passed, $Failed failed/diff"
