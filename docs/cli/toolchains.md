# TechScript Toolchain Requirements & Detection

**Document Path**: `docs/cli/toolchains.md`  

---

## 1. Toolchain Detection Logic

The TechScript toolchain detector inspects environment variables, standard installation directories, and system PATH to automatically locate compilers, SDKs, and build tools.

### Android SDK Detection
- Checks `$env:ANDROID_HOME` and `$env:ANDROID_SDK_ROOT`.
- Standard fallbacks:
  - Windows: `C:\AndroidSDK`, `C:\Android`, `%LOCALAPPDATA%\Android\Sdk`
  - Linux: `$HOME/Android/Sdk`
  - macOS: `$HOME/Library/Android/sdk`
- Validates:
  - `platforms/android-XX` (Requires API 33+)
  - `build-tools/XX.X.X` (`aapt2.exe`, `zipalign.exe`, `apksigner.bat`, `d8.bat`)
  - `platform-tools/adb.exe`

### Java Development Kit (JDK) Detection
- Checks `$env:JAVA_HOME` and `java -version`, `javac -version`.
- Compatible versions: JDK 17, JDK 21 LTS.

### Gradle Detection
- Checks project-local `gradlew` / `gradlew.bat`.
- Fallbacks to system `gradle` on PATH.
- Compatible versions: Gradle 8.0+.

### Desktop Compilers
- **Windows**: Detects Visual Studio C++ toolchain (`cl.exe`, `link.exe`), Windows SDK, or Rust toolchain (`rustc`, `cargo`).
- **Linux**: Detects `gcc` or `clang`, `pkg-config`, and GTK3 development headers.
- **macOS / iOS**: Detects `xcode-select -p` and `xcrun`.
