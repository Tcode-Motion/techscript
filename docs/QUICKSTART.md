# TechScript v1.0.6 — Quick Start

## Windows / macOS / Linux (developers)

1. Install [Rust](https://rustup.rs/).
2. Clone this repo and open a terminal in the project folder.
3. Run:

```powershell
cd runtime
cargo build --release --bin tech
cargo run --release --bin tech -- run ..\examples\hello.txs
```

4. Read the full guide: [WORKTHROUGH.md](WORKTHROUGH.md).

## One-line helper (Windows)

From repo root:

```powershell
.\scripts\run_example.ps1 examples\hello.txs
```

## Verify install

```powershell
cd runtime
cargo run --release --bin tech -- version
cargo test
```

## What's implemented?

See [V1.0.6_STATUS.md](V1.0.6_STATUS.md) (export to PDF via Print in your editor).

## Optional: Python parity

```powershell
pip install -e .
.\scripts\parity_check.ps1
```
