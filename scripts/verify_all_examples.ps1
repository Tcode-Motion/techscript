# TechScript Official Examples Verification Script (PowerShell)
$ErrorActionPreference = "Stop"

Write-Host "===================================================" -ForegroundColor Cyan
Write-Host "   TechScript Official Examples Verification       " -ForegroundColor Cyan
Write-Host "===================================================" -ForegroundColor Cyan
Write-Host ""

$tscPath = "target\release\tsc.exe"
if (-not (Test-Path $tscPath)) {
    $tscPath = "target\debug\tsc.exe"
}
if (-not (Test-Path $tscPath)) {
    Write-Host "[ERROR] Compiled tsc binary not found! Run 'cargo build --release' first." -ForegroundColor Red
    exit 1
}

Write-Host "Using binary: $tscPath" -ForegroundColor Gray
Write-Host ""

$files = Get-ChildItem -Path "examples" -Filter "*.txs" -Recurse -File
$total = $files.Count
$passed = 0
$failed = 0

foreach ($file in $files) {
    $relPath = $file.FullName.Replace((Get-Location).Path + "\", "")
    Write-Host "Verifying $relPath ... " -NoNewline

    $expectedFile = Join-Path $file.DirectoryName "expected.txt"
    $hasExpected = Test-Path $expectedFile

    try {
        $output = & $tscPath run $file.FullName 2>&1 | Out-String
        $exitCode = $LASTEXITCODE

        if ($exitCode -ne 0) {
            Write-Host "FAILED (Exit code $exitCode)" -ForegroundColor Red
            Write-Host $output -ForegroundColor Yellow
            $failed++
            continue
        }

        if ($hasExpected) {
            $expected = (Get-Content -Path $expectedFile -Raw -Encoding utf8).Replace("`r", "").Trim()
            $actual = $output.Replace("`r", "").Trim()
            if ($actual -ne $expected) {
                Write-Host "FAILED (Output mismatch)" -ForegroundColor Red
                Write-Host "Expected:`n$expected" -ForegroundColor Yellow
                Write-Host "Actual:`n$actual" -ForegroundColor Yellow
                $failed++
                continue
            }
        }

        Write-Host "OK" -ForegroundColor Green
        $passed++
    } catch {
        Write-Host "FAILED ($_)" -ForegroundColor Red
        $failed++
    }
}

Write-Host ""
Write-Host "===================================================" -ForegroundColor Cyan
Write-Host "Summary: $passed / $total passed, $failed failed." -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "===================================================" -ForegroundColor Cyan

if ($failed -gt 0) {
    exit 1
}
exit 0
