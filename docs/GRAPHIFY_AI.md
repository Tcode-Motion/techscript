# Graphify AI Usage Guide

> **Target Audience**: AI Assistants / Compiler Agents / Core Contributors
> **Purpose**: Complete guide to Graphify-Labs knowledge graph integration in TechScript.
> **Parent Link**: [AI_BOOTSTRAP](../AI_BOOTSTRAP.md)
> **Child Links**: [00_PROJECT](../docs/ai/00_PROJECT.md) · [02_MEMORY](../docs/ai/02_MEMORY.md)

---

## 1. Requirements & Installation

Graphify is designed to be cross-platform, supporting Windows, Linux, macOS, and WSL.

### 1.1 Python & Rust Prerequisites
- **Python**: Version `3.10`, `3.11`, or `3.12` is required.
- **Rust Toolchain**: `rustc` and `cargo` should be installed (highly recommended for extracting Rust/Cargo crate dependencies). Download from [https://rustup.rs/](https://rustup.rs/).

### 1.2 Installation
To install the dependencies for Graphify utilities in this workspace:
```bash
pip install -r requirements.txt
```
This automatically installs the PyPI package `graphifyy` (using a safe compatible version range `graphifyy>=0.8.0,<1.0.0`), along with helper dependencies like `python-dotenv`, `requests`, and version-conditional `tomli` (for Python versions < 3.11).

*Note: The CLI command registered by the `graphifyy` package is `graphify` (with a single 'y').*

To register the knowledge graph capabilities as a project-scoped Google Antigravity skill:
```bash
graphify antigravity install
```
This writes the rules to `.agents/rules/graphify.md` and workflows to `.agents/workflows/graphify.md`.

---

## 2. Configuration

Exclusions are managed in the `.graphifyignore` configuration file at the root of the repository. By default, it ignores:
- `target/` (Rust build artifacts)
- `.git/` (VCS metadata)
- `node_modules/` (Node dependencies)
- `graphify-out/` (Graphify output directory itself)
- Temporary files (`*.tmp`, `*.log`)

Ensure `.graphifyignore` contains `graphify-out/` to avoid recursive indexing during graph generation.

---

## 3. CLI Commands

### 3.1 Update Command (`update_graphify.py`)
To refresh the index and regenerate all artifacts, run the official wrapper script from anywhere in the workspace:
```bash
python tools/update_graphify.py
```
This script performs a pre-flight checklist (Python, dependencies, Cargo, Rust, CLI), runs the Graphify extraction/clustering processes, logs details to `logs/graphify.log`, and validates the generated outputs.

#### Dry-Run Mode
Verify the environment setup and print planned commands without modifying files:
```bash
python tools/update_graphify.py --dry-run
```

### 3.2 Verification Command (`check_graphify.py`)
Verify the local environment setup, configurations, and validate generated output file structures (JSON schema correctness, HTML headers, non-empty files):
```bash
python tools/check_graphify.py
```

---

## 4. Exit Codes

All Graphify utilities return standardized exit codes to make CI/CD automation and local scripting easier:

| Exit Code | Meaning | Description / Resolution |
| :--- | :--- | :--- |
| **`0`** | **Success** | Run completed successfully. |
| **`1`** | **Validation Failed** | Python environment/check_graphify checks failed. |
| **`2`** | **Missing Dependency** | A Python package, toolchain element, or the Graphify CLI itself is missing. |
| **`3`** | **Graphify Execution Failed** | The CLI command returned a non-zero exit code during extraction or clustering. |
| **`4`** | **Invalid Graph Output** | Generated files are missing, empty, or structurally malformed. |
| **`5`** | **Configuration Error** | Workspace root not found or `.graphifyignore` configuration missing/invalid. |

---

## 5. Generated Files

All generated outputs reside in the `graphify-out/` directory:

- **`graphify-out/graph.json`**: Machine-readable JSON containing nodes and edges representing code ASTs and Cargo manifests.
- **`graphify-out/graph.html`**: Interactive D3-based force-directed visualization.
- **`graphify-out/GRAPH_REPORT.md`**: Textual summary of communities, core abstractions, and god nodes.

---

## 6. How to Run CLI Queries

### 6.1 `graphify explain`
Get a detailed summary of a specific concept node:
```bash
graphify explain "main"
```

### 6.2 `graphify path`
Determine the shortest path/dependency chain between two concepts:
```bash
graphify path "main" "check_api_keys"
```

### 6.3 `graphify query`
Perform a BFS traversal to answer natural-language questions about the codebase:
```bash
graphify query "What does check_api_keys do?"
```

---

## 7. CI/CD Integration

The GitHub Actions workflow is defined in `.github/workflows/graphify.yml`. It ensures the consistency of the knowledge graph across commits:
1. Runs a matrix test on **Python 3.10, 3.11, and 3.12**.
2. Pre-installs stable **Rust and Cargo**.
3. Caches Python dependencies (`pip`) and Cargo registry.
4. Logs detailed toolchain diagnostics.
5. Performs graph generation (`update_graphify.py`) and dynamic output validation (`check_graphify.py`).
6. Verifies that generated graph files have no uncommitted changes (`git diff --exit-code graphify-out/`).
7. Archives and uploads generated output and logs as workflow artifacts.

---

## 8. Troubleshooting & Limitations

- **Error: No LLM API Key**: If no `GEMINI_API_KEY` (or compatible key) is present, scanning markdown and PDF files will fail.
  - *Fix*: The update script automatically falls back to AST-only/code-only mode. It temporarily updates `.graphifyignore` to skip non-code files, avoiding LLM calls, and restores the configuration afterward. Set an API key in a `.env` file to enable full semantic extraction.
- **Missing Python Packages**:
  - *Fix*: Run `pip install -r requirements.txt` to install all necessary packages automatically without manual setup.
- **Rust/Cargo Missing Warnings**:
  - *Fix*: The script will warn if the Rust toolchain is missing and proceed without Cargo dependencies (AST-only code parsing only). Install Rust from [https://rustup.rs/](https://rustup.rs/) to enable full crate-level dependency analysis.
