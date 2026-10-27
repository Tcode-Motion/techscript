# =============================================================================
# TechScript AppForge — Native Library Cross-Compilation Script
# examples/appforge/techscript-appforge/build_android_libs.ps1
#
# Builds release-optimized libts_android.so for both arm64-v8a and x86_64
# and installs them into android/app/src/main/jniLibs.
# =============================================================================

[CmdletBinding()]
param (
    [string]$NdkPath = "C:\AndroidSDK\ndk\27.1.12297006",
    [string]$Profile = "release"
)

$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectRoot = Resolve-Path (Join-Path $scriptDir "..\..\..")
$outJniLibs = Join-Path $scriptDir "android\app\src\main\jniLibs"

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host " TechScript AppForge Native Library Builder" -ForegroundColor Cyan
Write-Host " Project Root: $projectRoot" -ForegroundColor Gray
Write-Host " Output JNI:   $outJniLibs" -ForegroundColor Gray
Write-Host " NDK Path:     $NdkPath" -ForegroundColor Gray
Write-Host " Profile:      $Profile" -ForegroundColor Gray
Write-Host "============================================================" -ForegroundColor Cyan

if (-not (Test-Path $NdkPath)) {
    # Check alternate NDK paths under C:\AndroidSDK\ndk
    $found = Get-ChildItem -Path "C:\AndroidSDK\ndk" -Directory -ErrorAction SilentlyContinue | Select-Object -Last 1
    if ($found) {
        $NdkPath = $found.FullName
        Write-Host " Auto-detected NDK: $NdkPath" -ForegroundColor Yellow
    } else {
        Write-Error "Android NDK not found at $NdkPath. Please specify -NdkPath or install via Android Studio SDK Manager."
    }
}

$env:ANDROID_NDK_HOME = $NdkPath

Set-Location $projectRoot

$buildArgs = @(
    "ndk",
    "-t", "arm64-v8a",
    "-t", "x86_64",
    "-o", $outJniLibs,
    "build",
    "-p", "techscript_android_runtime"
)

if ($Profile -eq "release") {
    $buildArgs += "--release"
}

Write-Host " Running: cargo $($buildArgs -join ' ')" -ForegroundColor Green
& cargo $buildArgs

if ($LASTEXITCODE -ne 0) {
    Write-Error "cargo-ndk build failed with exit code $LASTEXITCODE"
}

Write-Host "`n Output Binaries:" -ForegroundColor Cyan
Get-ChildItem -Path $outJniLibs -Recurse -File | Select-Object Name, Length, LastWriteTime | Format-Table -AutoSize
Write-Host " AppForge native libraries successfully built and staged!" -ForegroundColor Green
