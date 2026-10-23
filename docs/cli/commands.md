# TechScript CLI Command Reference (`tsc`)

**Document Path**: `docs/cli/commands.md`  

---

## 1. Primary Commands

### Appforge Commands (`tsc app`)
- `tsc app create <name> [--template <tpl>]`: Scaffolds a new cross-platform Appforge application.
- `tsc app build <platform> [--debug|--release|--aab]`: Compiles project to specified platform artifact (e.g. `tsc app build android`, `tsc app build windows`).
- `tsc app run <platform> [--device <id>]`: Builds, installs, and launches app on connected device or desktop host.
- `tsc app test`: Runs unit, widget, and integration tests for the Appforge application.
- `tsc app doctor`: Evaluates Android SDK, JDK, Gradle, ADB, and desktop toolchains.
- `tsc app clean`: Cleans build artifacts, caches, and temporary build outputs.
- `tsc app wrap web [--platform <target>]`: Packages a Webforge website as an installable native application.

### Webforge Commands (`tsc web`)
- `tsc web create <name>`: Scaffolds a Webforge website project.
- `tsc web dev [--port <p>]`: Starts development server with live reload.
- `tsc web build [--release]`: Compiles Webforge website into production static `dist/`.
- `tsc web test`: Runs Webforge page and route tests.

### Native Interoperability (`tsc native`)
- `tsc native add <lang>`: Adds native module scaffolding for `kotlin`, `swift`, `rust`, `cpp`, `c`, or `java`.

### Diagnostics & Inspection
- `tsc doctor`: Overall system health and toolchain verification.
- `tsc logs [--follow|--level]`: Inspects centralized build and diagnostic logs.
- `tsc report`: Generates a comprehensive system, environment, and build report.
- `tsc clean`: Cleans global compilation caches and build folders.
