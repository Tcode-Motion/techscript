# TechScript Skill Validation Helper (PowerShell)
$ErrorActionPreference = "Stop"

$tempFile = Join-Path $env:TEMP "verify_techscript_skill.txs"
$code = "constant message is ""TechScript AI Skill Verified!""`nsay ""{message}"""
Set-Content -Path $tempFile -Value $code -Encoding utf8

Write-Host "Running verification program with 'tsc run'..." -ForegroundColor Gray
$tscPath = "target\release\tsc.exe"
if (-not (Test-Path $tscPath)) {
    $tscPath = "tsc"
}

& $tscPath run $tempFile
$exitCode = $LASTEXITCODE

Remove-Item -Force $tempFile

if ($exitCode -eq 0) {
    Write-Host "[OK] TechScript compiler verified and operational." -ForegroundColor Green
    exit 0
} else {
    Write-Host "[ERROR] TechScript verification failed with exit code $exitCode." -ForegroundColor Red
    exit 1
}
