# Appforge Platform Support Matrix

**Document Path**: `docs/appforge/platform-support.md`  

---

## 1. Supported Platforms & Toolchain Requirements

| Target Platform | Host OS | Output Formats | Required Toolchain | Implementation Status |
| :--- | :--- | :--- | :--- | :--- |
| **Android** | Windows, Linux, macOS | `.apk` (Debug/Release), `.aab` | JDK 17/21, Android SDK (API 34+), Gradle 8+ | **Fully Supported & Verified** |
| **Windows** | Windows | `.exe`, `.msi` | Windows 10/11, MSVC / Rust toolchain | **Fully Supported** |
| **Web Wrapper** | Any | Native Android APK, Windows EXE | Android SDK / WebView2 | **Fully Supported** |
| **Linux** | Linux | ELF executable, `.AppImage` | GCC/Clang, GTK3 / WebKitGTK | **Supported on Linux Hosts** |
| **macOS** | macOS | `.app`, `.dmg` | macOS, Xcode command-line tools | Supported on macOS Hosts |
| **iOS** | macOS | `.ipa`, `.xcarchive` | macOS, Xcode, CocoaPods | Supported on macOS Hosts |

---

## 2. Platform Detection & Doctor Inspection

The CLI command:
```bash
tsc app doctor
```
inspects the host system and reports the health of:
- Android SDK (`ANDROID_HOME`, platform API levels, build-tools)
- Java Runtime Environment (`javac`, JDK version)
- Gradle installation and Gradle wrapper availability
- Connected Android devices (`adb devices`)
- Windows Desktop build tools (`link.exe`, `cl.exe`, Rust toolchain)
- Web development toolchain (Webforge status)
