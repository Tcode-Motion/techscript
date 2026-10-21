# TechScript WebForge: Testing & Quality Assurance
**Unit Testing, Integration Verification, and End-to-End Testing Guide**

---

## 1. Overview

Reliability is a first-class citizen in TechScript WebForge. The ecosystem provides built-in test runners for unit testing language code, integration testing full-stack HTTP endpoints, and validating production build artifacts.

---

## 2. Writing TechScript Tests

Tests are placed in the `tests/` directory and follow the `*_test.txs` naming convention:

```techscript
// tests/math_test.txs
test "addition works properly":
    let result = 2 + 2
    assert result == 4

test "string concatenation":
    let greeting = "Hello, " + "WebForge"
    assert greeting == "Hello, WebForge"
```

To run all project tests:
```bash
techscript test
```

---

## 3. WebForge End-to-End Tests

WebForge's compiler and runtime include a dedicated test suite (`cli/tests/webforge_e2e_tests.rs`) covering the 4 core workflows:

1. **`test_webforge_interactive_site`:**
   Validates compilation of declarative website declarations, buttons, input fields, and event listeners into a cohesive DOM tree.
2. **`test_webforge_notes_sqlite_site`:**
   Verifies automated SQLite schema migration, table creation, and API route generation for persistent data storage.
3. **`test_webforge_3d_webgl_scene`:**
   Validates Three.js 3D scene AST lowering, camera setup, lighting, and WebGL canvas mounting.
4. **`test_webforge_production_build_generation`:**
   Validates that `techscript build web --release` produces valid `dist/index.html`, `dist/styles.css`, `dist/app.js`, and `dist/schema.sql` files.

Run the test suite using Cargo:
```bash
cargo test --test webforge_e2e_tests
```

---

## 4. Testing Live REST APIs

You can write automated integration tests to probe running WebForge servers:

```bash
# Start server in test background mode
techscript dev --port 3001 &

# Execute curl verification
curl -f -X GET http://127.0.0.1:3001/api/notes || exit 1
```

---

## 5. Continuous Integration (CI) Workflow

Example GitHub Actions workflow (`.github/workflows/ci.yml`):

```yaml
name: CI
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Run Test Suite
        run: cargo test --workspace
      - name: Verify WebForge Production Build
        run: cargo run -p techscript_cli -- build web --release
```
