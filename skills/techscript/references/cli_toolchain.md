# TechScript CLI (`tsc`) Toolchain Guide

The unified `tsc` CLI drives compilation, formatting, linting, testing, and multi-platform packaging.

---

## 1. General Execution & Compilation

| Command | Usage | Description |
| :--- | :--- | :--- |
| `tsc run <file.txs>` | `tsc run main.txs` | Execute a `.txs` file directly using the runtime VM |
| `tsc check <file.txs>` | `tsc check src/app.txs` | Run strict semantic and type check without execution |
| `tsc build <file.txs>` | `tsc build main.txs --target native` | Compile TechScript to native target executable |
| `tsc test` | `tsc test` | Auto-discover and run all unit tests (`*_test.txs`) |
| `tsc fmt [paths...]` | `tsc fmt src/` | Format source files adhering to canonical 4-space colon indent |
| `tsc lint [paths...]` | `tsc lint --fix src/` | Check code health and automatically fix safe deprecations |
| `tsc repl` | `tsc repl` | Start interactive live REPL console |

---

## 2. WebForge Web Development

| Command | Usage | Description |
| :--- | :--- | :--- |
| `tsc web create <name>` | `tsc web create my-site` | Scaffold new full-stack WebForge project |
| `tsc web dev` | `tsc web dev` | Launch local dev server with hot-reload (http://localhost:3000) |
| `tsc web build` | `tsc web build` | Compile static production bundle to `dist/` |

---

## 3. AppForge Mobile & Desktop

| Command | Usage | Description |
| :--- | :--- | :--- |
| `tsc app create <name>` | `tsc app create my-app` | Scaffold cross-platform mobile/desktop application |
| `tsc app doctor` | `tsc app doctor` | Inspect JDK, Android SDK, and NDK requirements |
| `tsc app build android` | `tsc app build android` | Compile debug APK (`build/artifacts/android/app-debug.apk`) |
| `tsc app build android --release` | `tsc app build android --release` | Compile signed release APK |
| `tsc app run android` | `tsc app run android` | Compile, install via ADB, and launch on connected device |
| `tsc app build windows` | `tsc app build windows` | Compile native Windows desktop `.exe` executable |
