# Appforge Build System Specification

**Document Path**: `docs/appforge/build-system.md`  

---

## 1. Build Pipeline Workflow

```text
[Source .txs Code]
       │
       ▼
[1. TechScript Compiler Pipeline]
  - Syntax check & Pratt parsing
  - Semantic analysis & type verification
  - AST optimization & bundle bundling
  - Asset staging into host project
       │
       ▼
[2. Target Platform Adapter]
  ├── Android: Gradle invocation (assembleDebug / assembleRelease / bundleRelease)
  ├── Windows: Host runner & MSVC/GCC linking
  ├── Linux: Host packaging (ELF / AppImage)
  └── Apple: Xcode toolchain invocation
       │
       ▼
[3. Artifact Verification & Inspection]
  - File existence and non-zero size check
  - ZIP/JAR archive structure validation
  - Manifest & signature inspection
       │
       ▼
[4. Deployment & Execution]
  - Device discovery (ADB / local process)
  - Installation (adb install -r)
  - Launch & stream diagnostics
```

---

## 2. Command Reference

### Android
```bash
# Debug APK Build
tsc app build android
tsc app build android --debug

# Release APK Build
tsc app build android --release

# Android App Bundle (AAB for Google Play)
tsc app build android --aab

# Live Device Execution
tsc app run android
tsc app run android --device 10ME8DF1CT0001Z
```

### Windows & Desktop
```bash
# Build Windows Executable
tsc app build windows
tsc app build windows --release

# Run Windows Application
tsc app run windows
```

### Web Wrapping
```bash
# Wrap Webforge Website as Native Android App
tsc app wrap web --platform android

# Wrap Webforge Website for Windows
tsc app wrap web --platform windows
```

---

## 3. Build Artifact Outputs

All build outputs are staged into clearly separated artifact directories:
- **Android Debug APK**: `build/artifacts/android/app-debug.apk`
- **Android Release APK**: `build/artifacts/android/app-release-unsigned.apk` or `app-release.apk`
- **Android Release AAB**: `build/artifacts/android/app-release.aab`
- **Windows Executable**: `build/artifacts/windows/<name>.exe`
- **Build Logs**: `logs/build-<timestamp>-<target>.log`
