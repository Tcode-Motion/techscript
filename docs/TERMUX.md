# TechScript on Android (Termux)

TechScript's Rust binary does not ship as a pre-compiled Android binary yet.
To build and install it natively on Termux, follow the steps below.

---

## Prerequisites

Open Termux and run:

```bash
pkg update && pkg upgrade -y
pkg install python rust binutils clang curl -y
```

## Option 1: Install via pip (Python runtime)

```bash
pip install techscript
tech version
```

## Option 2: Build the native Rust binary

```bash
# Clone the project (or copy the runtime/ folder to Termux)
git clone https://github.com/your-repo/techscript.git
cd techscript/runtime

# Build release binary
cargo build --release

# Copy to PATH
cp target/release/tech $PREFIX/bin/tech
chmod +x $PREFIX/bin/tech

# Verify
tech version
```

## Verify Installation

```bash
echo 'say "Hello from Termux!"' > hello.txs
tech run hello.txs
```

## Troubleshooting

| Problem | Fix |
|---|---|
| `linker cc not found` | `pkg install clang` |
| `openssl errors` | `pkg install openssl` |
| `could not compile` Rust errors | `rustup update stable` |
| `pip install` fails for extension crates | Build via Option 2 above |

---

## Notes

- Tested on Termux 0.118+ with Android 12+
- The native Rust binary (`tech`) runs significantly faster than the Python runtime
- File associations and VS Code integration are not available on Android
