# TechScript File Extension Specification & Specialized File Standard

**Standard Version:** 2.0.0  
**Status:** Approved & Implemented  
**Core Rule:** `.txs` is the official, default, and canonical file extension for TechScript.

---

## 1. Executive Summary & Design Philosophy

TechScript 2.0 establishes a unified, deterministic, and future-proof file-extension standard across the entire compiler, runtime interpreter, language server, package manager, and CLI tooling.

### The Golden Rule
> **`.txs` is the official, default, and overwhelmingly preferred TechScript source-file extension.**  
> More than 99% of all TechScript code—including scripts, libraries, applications, business logic, CLI utilities, and algorithms—must use `.txs`.

Specialized extensions are strictly optional domain annotations designed for scenarios where a specialized source role provides tangible value to developer tooling, editor linting, asset pipelines, or framework routing (e.g. WebForge full-stack web pages or 3D scene viewports).

### Core Architectural Principles
1. **One Compiler, One Grammar:** Specialized extensions do **not** create dialect silos or alter the core language grammar. Every valid `.wtxs`, `.stxs`, `.atxs`, or `.3txs` file is parsed by the exact same TechScript parser and semantic engine.
2. **Deterministic Module Resolution:** Resolution never guesses ambiguously. The canonical `.txs` file is always prioritized. If multiple specialized extensions collide without a `.txs` file, resolution immediately aborts with an informative `AmbiguousImport` error detailing all candidate paths.
3. **Seamless Backward Compatibility:** Existing legacy `.tech` files continue to parse and execute smoothly without breaking legacy workflows, while emitting non-blocking migration hints guiding users toward `tsc migrate`.
4. **Security-First Path Traversal Protection:** Relative import resolution strictly checks against directory escapes (`../`) and prevents path traversal vulnerabilities.

---

## 2. Complete Extension Reference Table

| Extension | Role | Stability | Preferred Entry / Path | Recommended Use Case | Tooling / Compiler Behavior |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **`.txs`** | `GeneralPurpose` | **Stable** | `src/main.txs`, `src/lib.txs` | **Official default for >99% of all TechScript code**. Scripts, CLI tools, libraries, server services, algorithms. | Native LLVM compilation, bytecode VM execution, interactive REPL, complete LSP analysis. |
| **`.wtxs`** | `WebComponent` | **Stable** | `src/pages/*.wtxs`, `src/components/*.wtxs` | TechScript WebForge UI pages, reactive component trees, HTML markup, and browser event handlers. | Automatically routed to interpreter/DOM evaluation; validated for reactive element hierarchy. |
| **`.stxs`** | `ServerTask` | **Stable** | `src/server/*.stxs`, `src/tasks/*.stxs` | Backend API routes, scheduled background workers, SQLite database queries, and async queues. | Validated for server capabilities, HTTP endpoints, and transactional safety. |
| **`.atxs`** | `AndroidScreen` | **Beta** | `src/screens/*.atxs`, `src/mobile/*.atxs` | Declarative mobile screens, Android Jetpack/Compose layout bindings, touch interactions. | Android mobile target code generator and mobile schema validator. |
| **`.3txs`** | `Scene3D` | **Experimental** | `src/scenes/*.3txs` | 3D scene viewports, Three.js WebGL canvas graphs, lighting, material parameters, and cameras. | Evaluated with WebGL/Three.js runtime runtime injector; inspected by `tsc routes`. |
| **`.gtxs`** | `Scene3D` | **Experimental** | `src/scenes/*.gtxs` | Alternative scene & shader graph descriptor for 3D/graphics workloads. | Alias for `.3txs` scene processing pipeline. |
| **`.tech`** | `Legacy` | **Deprecated** | Any legacy file | Legacy TechScript 1.0.8 source files retained for backward compatibility. | Full execution supported; compiler outputs diagnostic notice recommending `tsc migrate`. |

---

## 3. Detailed Extension Breakdown

### 3.1 `.txs` — Canonical TechScript Source File
- **Status:** Canonical Default
- **Use Case:** All general-purpose programming, standard libraries, CLI tools, backend servers, desktop applications, games, and WebForge full-stack websites.
- **Example:**
```techscript
// src/main.txs
import std.io;
import std.http;

function main() {
    std.io.print("TechScript 2.0 running with canonical .txs extension");
}

main();
```

### 3.2 `.wtxs` — WebForge Web Pages & Components
- **Status:** Specialized (Stable)
- **Use Case:** Multi-page WebForge projects dividing UI views into modular components.
- **Example:**
```techscript
// src/pages/dashboard.wtxs
page Dashboard:
    show heading "Analytics Overview"
    show text "Active sessions: 1,420"
    show button "Refresh Metrics" called RefreshBtn

when RefreshBtn is clicked:
    show message "Metrics updated"
```

### 3.3 `.stxs` — Server-Side Endpoints & Background Tasks
- **Status:** Specialized (Stable)
- **Use Case:** Backend database operations, background tasks, asynchronous workers.
- **Example:**
```techscript
// src/server/metrics_worker.stxs
database MetricsDB:
    table system_logs:
        cpu_usage is number
        memory_usage is number
        recorded_at is text
```

### 3.4 `.atxs` — Android Mobile UI Screens
- **Status:** Specialized (Beta)
- **Use Case:** Declarative Android screen specifications for cross-compilation to mobile platforms.
- **Example:**
```techscript
// src/screens/settings.atxs
screen SettingsScreen:
    show heading "Application Settings"
    show toggle "Dark Mode" called DarkModeToggle
```

### 3.5 `.3txs` / `.gtxs` — 3D Scene Viewport & Graphics
- **Status:** Specialized (Experimental)
- **Use Case:** Interactive 3D WebGL canvases embedded within WebForge applications.
- **Example:**
```techscript
// src/scenes/hero_mesh.3txs
canvas Viewport3D:
    shape is cube
    rotation is interactive
    material is metal
```

### 3.6 `.tech` — Legacy TechScript 1.0.8 Files
- **Status:** Deprecated (Fully Functional)
- **Behavior:** The compiler seamlessly parses and executes `.tech` files, while emitting:
  ```
  Notice: '.tech' is a legacy extension. TechScript official source files use '.txs'. You can migrate via 'tsc migrate'.
  ```

---

## 4. Deterministic Module Resolution Specification

When resolving a module import (e.g. `import user.service;` or `use "user/service";`), the TechScript resolution engine follows a strict, deterministic 3-tier algorithm:

```
                  Input: import "path/module"
                             │
                             ▼
               Does path specify an extension?
                ├── YES ──> Check file directly
                │
                └── NO
                     │
                     ▼
             Step 1: Probe Canonical Extension
             Is "path/module.txs" present?
                ├── YES ──> Resolve to "path/module.txs" (Winner)
                │
                └── NO
                     │
                     ▼
             Step 2: Collect Candidate Matches
             Search [.wtxs, .stxs, .atxs, .3txs, .gtxs, .tech]
                │
                ├── Exactly 1 match found ──> Resolve to candidate (Winner)
                │
                ├── Multiple matches found ──> ERROR: AmbiguousImport
                │                             (List conflicting files)
                │
                └── 0 matches found ──> ERROR: ModuleNotFound
```

### Ambiguity Handling Example
If a project contains:
- `src/user/service.wtxs`
- `src/user/service.stxs`

And code writes: `import user.service;`
- **Result:** The compiler aborts with:
  ```
  SourceResolutionError::AmbiguousImport: Multiple candidates found for module 'user/service':
    - src/user/service.wtxs
    - src/user/service.stxs
  Hint: Disambiguate by creating a canonical 'service.txs' or specifying the explicit extension.
  ```

---

## 5. Migration Tooling (`tsc migrate`)

TechScript provides a built-in, automated migration command:

```bash
# Run migration on the current project
tsc migrate

# Or target a specific folder
tsc migrate ./my-project
```

### What `tsc migrate` Does:
1. **Discovers Legacy Files:** Traverses all project files finding `.tech` sources.
2. **Collision-Safe Renaming:** Verifies that no colliding `.txs` file exists before safely renaming `.tech` -> `.txs`.
3. **Manifest Modernization:** Updates `tech.toml` entries (e.g. `entry = "src/main.tech"` -> `entry = "src/main.txs"`).
4. **Import Modernization:** Updates any explicit code imports targeting `.tech` files to canonical `.txs`.
5. **Syntax Modernization:** Upgrades deprecated legacy keywords and syntax constructs.

---

## 6. CLI Command Behavior Matrix

| Command | `.txs` | `.wtxs` | `.stxs` | `.atxs` | `.3txs` | `.tech` |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: |
| `tsc run <file>` | VM / Native / Interpreter | Interpreter | Interpreter | Diagnostic | Interpreter | Evaluates + Warning |
| `tsc dev` | Default Entry | Auto-loaded (`pages/`) | Auto-loaded (`server/`) | Skipped | Auto-loaded (`scenes/`) | Fallback Entry + Warning |
| `tsc build web` | Primary Entry | Modular Pages | Modular Server | Skipped | Modular Scenes | Fallback Entry + Warning |
| `tsc check` | Full Analysis | Full Analysis | Full Analysis | Full Analysis | Full Analysis | Full Analysis + Warning |
| `tsc fmt` | Formatted | Formatted | Formatted | Formatted | Formatted | Formatted |
| `tsc lint` | Linted | Linted | Linted | Linted | Linted | Linted |
| `tsc migrate` | Modernized | Modernized | Modernized | Modernized | Modernized | Renamed to `.txs` |

---

## 7. Editor & Ecosystem Configuration

### VS Code File Associations (`settings.json`)
```json
{
  "files.associations": {
    "*.txs": "techscript",
    "*.wtxs": "techscript",
    "*.stxs": "techscript",
    "*.atxs": "techscript",
    "*.3txs": "techscript",
    "*.gtxs": "techscript",
    "*.tech": "techscript"
  }
}
```

### Project Manifest Standard (`tech.toml`)
All modern TechScript projects declare entry points using `.txs`:
```toml
[package]
name = "my_app"
version = "0.1.0"
entry = "src/main.txs"
capabilities = ["Network", "FileSystem"]
```
