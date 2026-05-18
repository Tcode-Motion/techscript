# Run all v1.0.6 example matrix through Rust tech
# $ErrorActionPreference = "Stop" # Removed to allow native stderr output without exceptions
$Root = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$RuntimeDir = Join-Path $Root "runtime"
$RustBin = Join-Path $RuntimeDir "target\x86_64-pc-windows-msvc\release\tech.exe"
if (-not (Test-Path $RustBin)) {
    $RustBin = Join-Path $RuntimeDir "target\release\tech.exe"
}
$UseCargoRun = -not (Test-Path $RustBin)
if ($UseCargoRun) {
    Push-Location $RuntimeDir
    cargo build --release --bin tech | Out-Null
    Pop-Location
    if (Test-Path (Join-Path $RuntimeDir "target\release\tech.exe")) {
        $RustBin = Join-Path $RuntimeDir "target\release\tech.exe"
        $UseCargoRun = $false
    } elseif (Test-Path (Join-Path $RuntimeDir "target\x86_64-pc-windows-msvc\release\tech.exe")) {
        $RustBin = Join-Path $RuntimeDir "target\x86_64-pc-windows-msvc\release\tech.exe"
        $UseCargoRun = $false
    }
}

$env:TECHSCRIPT_WEB_TEST = "1"
$env:TECHSCRIPT_GUI_TEST = "1"
$env:TECHSCRIPT_3D_TEST = "1"
$env:TECHSCRIPT_NON_INTERACTIVE = "1"

$Examples = @(
    "examples\hello.txs",
    "examples\calc.txs",
    "examples\calculator.txs",
    "examples\guessing_game.txs",
    "examples\hot_reload.txs",
    "examples\classes.txs",
    "examples\syntax_aliases.txs",
    "examples\fibonacci.txs",
    "examples\fizzbuzz.txs",
    "examples\web_complete.txs",
    "examples\web_app.txs",
    "examples\web_app_simple.txs",
    "examples\gui_app.txs",
    "examples\3d_scene.txs",
    "examples\anime_demo.txs",
    "runtime_examples\01_basics.txs",
    "runtime_examples\02_math_and_logic.txs",
    "runtime_examples\03_control_flow.txs",
    "runtime_examples\04_functions.txs",
    "runtime_examples\05_classes.txs",
    "runtime_examples\06_advanced.txs",
    "runtime_examples\07_performance_test.txs"
)

$Passed = 0
$Failed = 0

foreach ($rel in $Examples) {
    $path = Join-Path $Root $rel
    if (-not (Test-Path $path)) {
        Write-Host "  SKIP $rel (missing)" -ForegroundColor Yellow
        continue
    }
    Write-Host "  $rel ... " -NoNewline
    if ($UseCargoRun) {
        Push-Location $RuntimeDir
        cargo run --quiet --release --bin tech -- run "$path" *>$null
        Pop-Location
    } else {
        & "$RustBin" run "$path" *>$null
    }
    if ($LASTEXITCODE -eq 0) {
        Write-Host "OK" -ForegroundColor Green
        $Passed++
    } else {
        Write-Host "FAIL" -ForegroundColor Red
        $Failed++
    }
}

Write-Host ""
Write-Host "Smoke: $Passed passed, $Failed failed"
if ($Failed -gt 0) { exit 1 }
