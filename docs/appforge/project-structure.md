# Appforge Project Structure Specification

**Document Path**: `docs/appforge/project-structure.md`  

---

## 1. Standard Project Layout

When a developer runs `tsc app create <name>`, Appforge generates the following source-centered project structure:

```text
my-app/
├── .gitignore
├── .editorconfig
├── README.md
├── appforge.toml           # Application manifest (package ID, version, platforms, permissions)
├── techscript.toml         # Compiler and dependency settings
│
├── src/
│   ├── main.txs            # Application bootstrap and initialization
│   ├── app.txs             # Root application definition, theme, and router
│   ├── screens/
│   │   ├── home_screen.txs # Primary screen view
│   │   └── settings_screen.txs
│   ├── widgets/
│   │   └── custom_card.txs
│   ├── state/
│   │   └── app_state.txs   # Reactive application state
│   └── utils/
│       └── helpers.txs
│
├── assets/
│   ├── icon.png            # App launcher icon
│   ├── logo.svg
│   └── fonts/
│
├── test/
│   ├── unit/
│   │   └── logic_test.txs
│   └── app_test.txs
│
├── native/                 # Modular native language bridges
│   ├── kotlin/
│   ├── java/
│   ├── swift/
│   ├── rust/
│   ├── c/
│   └── cpp/
│
├── android/                # Android host project (Gradle, Manifest, Activity)
│   ├── build.gradle.kts
│   ├── settings.gradle.kts
│   ├── gradle.properties
│   ├── gradlew.bat
│   ├── gradlew
│   ├── gradle/wrapper/
│   │   ├── gradle-wrapper.properties
│   │   └── gradle-wrapper.jar
│   └── app/
│       ├── build.gradle.kts
│       └── src/main/
│           ├── AndroidManifest.xml
│           ├── java/com/techscript/app/MainActivity.kt
│           ├── res/
│           │   ├── values/strings.xml
│           │   ├── values/colors.xml
│           │   └── mipmap-anydpi-v26/ic_launcher.xml
│           └── assets/     # Compiled TechScript bytecode bundle & web assets
│
└── build/                  # Build cache and outputs (ignored by Git)
    ├── artifacts/          # Generated APK, AAB, EXE binaries
    ├── reports/            # Test and QA reports
    └── logs/               # Detailed build and compilation logs
```

---

## 2. Manifest Specification: `appforge.toml`

```toml
[app]
id = "com.techscript.myapp"
name = "My TechScript App"
version = "1.0.0"
version_code = 1
entry = "src/main.txs"
description = "Cross-platform mobile and desktop application created with TechScript Appforge."

[theme]
mode = "dark"
primary = "#4f46e5"
background = "#0f172a"

[platforms]
targets = ["android", "windows", "linux", "web"]

[android]
min_sdk = 26
target_sdk = 34
compile_sdk = 34
package = "com.techscript.myapp"
orientation = "portrait"
permissions = [
    "android.permission.INTERNET"
]

[windows]
executable_name = "my-app.exe"
company = "TechScript"

[build]
strip_symbols = true
optimize = true
```
