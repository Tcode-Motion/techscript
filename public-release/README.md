# TechScript 2.0 Manual Testing Guide

This directory contains the Developer Debug Release of the TechScript 2.0 language environment.

## Installation Methods

### 1. Using the Portable Version
1. Extract `portable/TechScript_Portable.zip` to a directory of your choice (e.g. `C:\TechScript`).
2. Open PowerShell or Command Prompt.
3. Verify by running:
   ```powershell
   .\bin\tsc.exe doctor
   ```

### 2. Using the Setup Installer
1. Double-click `installer/TechScript_Setup.exe` to run the setup wizard.
2. Follow instructions to install to Program Files and register environment paths automatically.
3. Open a new PowerShell terminal and run:
   ```powershell
   tsc version
   ```

---

## Testing VS Code Extension
1. Install `vscode/techscript.vsix` directly in VS Code:
   - Command Palette -> `Extensions: Install from VSIX...`
   - Select the VSIX file in this package.
2. Open any `.txs` file. Verify syntax highlights and completion features.

---

## Running Examples & Templates
1. To run the Hello World example:
   ```powershell
   tsc run examples/hello_world/main.txs
   ```
2. Create a new project:
   ```powershell
   tsc new my_project --template console
   cd my_project
   tsc run src/main.txs
   ```
