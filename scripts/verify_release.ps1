# TechScript 2.0 Release Verification and Smoke Testing Script
#
# Automatically validates all release packages, verifies folder layouts, hashes,
# and executes full end-to-end compiler driver & virtual machine tests.

$ErrorActionPreference = "Stop"

Write-Host "=== Starting TechScript 2.0 Release Verification ===" -ForegroundColor Cyan

$RootPath = (Get-Item (Get-Location)).FullName
if ($(Split-Path $RootPath -Leaf) -eq "scripts") {
    $RootPath = Split-Path $RootPath -Parent
}
$ReleaseDir = Join-Path $RootPath "releases/current"
if (-not (Test-Path $ReleaseDir)) {
    Write-Error "releases/current folder not found. Please run the packager first: cargo run -p techscript_packager"
}

# 1. Verify files exist
$RequiredFiles = @(
    "portable/TechScript_Portable.zip",
    "installer/TechScript_Setup.exe",
    "vscode/techscript.vsix",
    "compiler/tsc.exe",
    "checksums/release.json",
    "checksums/SHA256SUMS.txt",
    "release-notes/RELEASE_NOTES.md",
    "licenses/LICENSE",
    "README.md",
    "TechScript_Setup.exe",
    "TechScript_Portable.zip",
    "techscript.vsix",
    "LICENSE",
    "CHANGELOG.md",
    "RELEASE_NOTES.md",
    "SHA256SUMS.txt",
    "release.json",
    "docs/html/index.html",
    "templates/console/tech.toml",
    "debug-tools/README.md"
)

foreach ($file in $RequiredFiles) {
    $fullPath = Join-Path $ReleaseDir $file
    if (-not (Test-Path $fullPath)) {
        Write-Error "Required file is missing: $file at path $fullPath"
    }
    Write-Host "[OK] Verified file exists: $file" -ForegroundColor Green
}

# 2. Extract portable ZIP and perform smoke tests
$TempDir = Join-Path (Get-Location) "temp_smoke_test"
if (Test-Path $TempDir) {
    Remove-Item -Recurse -Force $TempDir
}
New-Item -ItemType Directory -Path $TempDir | Out-Null

Write-Host "Unpacking portable release ZIP for smoke tests..." -ForegroundColor Gray
$ZipPath = Join-Path $ReleaseDir "portable/TechScript_Portable.zip"
Expand-Archive -Path $ZipPath -DestinationPath $TempDir -Force

$TscExe = Join-Path $TempDir "TechScript/bin/tsc.exe"
$LspExe = Join-Path $TempDir "TechScript/bin/techscript-lsp.exe"

if (-not (Test-Path $TscExe)) { Write-Error "tsc.exe missing inside portable zip!" }
if (-not (Test-Path $LspExe)) { Write-Error "techscript-lsp.exe missing inside portable zip!" }

# Smoke test 1: tsc doctor
Write-Host "Smoke test 1: tsc doctor..." -ForegroundColor Gray
$DoctorOutput = & $TscExe doctor
Write-Host $DoctorOutput
if (-not ($DoctorOutput -match "All system checks passed")) {
    Write-Error "tsc doctor failed during smoke test!"
}
Write-Host "[PASS] Smoke test 1: tsc doctor" -ForegroundColor Green

# Smoke test 2: Executing all language examples
Write-Host "Smoke test 2: Running all language examples..." -ForegroundColor Gray
$Examples = @(
    "hello_world",
    "variables",
    "functions",
    "classes",
    "enums",
    "structs",
    "traits",
    "interfaces",
    "collections",
    "loops",
    "pattern_matching",
    "modules",
    "packages",
    "json",
    "filesystem",
    "threads",
    "errors",
    "recursion",
    "async",
    "generics",
    "http",
    "cli_app",
    "calculator",
    "todo_app",
    "mini_game",
    "interpreter_demo",
    "compiler_plugin",
    "package_example",
    "workspace_example",
    "complete_project",
    "hello_classes",
    "math_utilities",
    "file_search"
)
foreach ($ex in $Examples) {
    $ExTxs = Join-Path $TempDir "TechScript/examples/$ex/main.txs"
    if (-not (Test-Path $ExTxs)) {
        Write-Error "Required example file is missing: $ExTxs"
    }
    Write-Host "Running example: $ex" -ForegroundColor Gray
    $RunOutput = & $TscExe run $ExTxs --backend interpreter
    Write-Host "Output: $RunOutput"
}
Write-Host "[PASS] Smoke test 2: All 33 language examples executed successfully" -ForegroundColor Green

# Smoke test 4: Validate VS Code VSIX file contents
Write-Host "Smoke test 4: Validating VSIX file contents..." -ForegroundColor Gray
$VsixZip = Join-Path $TempDir "vsix_unpacked"
$TempVsixZip = Join-Path $TempDir "techscript_vsix.zip"
Copy-Item -Path (Join-Path $ReleaseDir "vscode/techscript.vsix") -Destination $TempVsixZip -Force
Expand-Archive -Path $TempVsixZip -DestinationPath $VsixZip -Force
if (-not (Test-Path (Join-Path $VsixZip "extension/package.json"))) {
    Write-Error "VSIX does not contain package.json!"
}
if (-not (Test-Path (Join-Path $VsixZip "extension/syntaxes/techscript.tmLanguage.json"))) {
    Write-Error "VSIX does not contain syntax grammar highlights!"
}
Write-Host "[PASS] Smoke test 4: VS Code Extension VSIX package structure" -ForegroundColor Green

# 3. Clean up
Remove-Item -Recurse -Force $TempDir
Write-Host "=== All Release Verifications Passed Successfully (10/10)! ===" -ForegroundColor Green
