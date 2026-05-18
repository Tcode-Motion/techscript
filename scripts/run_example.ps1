# Run a TechScript example: .\scripts\run_example.ps1 examples\hello.txs
param(
    [Parameter(Mandatory = $true, Position = 0)]
    [string]$File
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$RuntimeDir = Join-Path $Root "runtime"
$Tech = Join-Path $RuntimeDir "target\x86_64-pc-windows-msvc\release\tech.exe"
if (-not (Test-Path $Tech)) {
    $Tech = Join-Path $RuntimeDir "target\release\tech.exe"
}

$Path = if ([System.IO.Path]::IsPathRooted($File)) { $File } else { Join-Path $Root $File }
if (-not (Test-Path $Path)) {
    Write-Error "File not found: $Path"
}

if (Test-Path $Tech) {
    & $Tech run $Path
} else {
    Push-Location $RuntimeDir
    cargo run --release --bin tech -- run $Path
    Pop-Location
}

exit $LASTEXITCODE
