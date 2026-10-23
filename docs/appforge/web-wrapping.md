# Appforge Web Wrapping: Packaging Webforge Apps for Mobile & Desktop

**Document Path**: `docs/appforge/web-wrapping.md`  

---

## 1. Concept

Webforge builds fast, full-stack, responsive websites with declarative TechScript. Appforge allows converting any Webforge project into an installable native mobile app (Android APK/AAB) or desktop application (Windows EXE):

```bash
tsc app wrap web --platform android
tsc app wrap web --platform windows
```

---

## 2. Architecture of Wrapped Web Apps

```text
[Webforge Project (pages, 3D scenes, SQLite)]
                 │
                 ▼
     [tsc build --target web]
                 │
                 ▼
      [Compiled dist/ bundle]
     (HTML + CSS + JS + SQLite)
                 │
                 ▼
  [Staged into Android assets/]
                 │
                 ▼
   [Android Native WebView Host]
  - AndroidX Webkit asset loader
  - Hardware-accelerated canvas/WebGL
  - Secure bidirectional JS bridge
  - Offline local asset serving
                 │
                 ▼
       [Verified APK / AAB]
```

---

## 3. Security Model

1. **Local Asset Protocol**: Assets are served locally via `https://appassets.androidplatform.net/` using `WebViewAssetLoader`, preventing CORS issues while avoiding insecure `file://` scheme usage.
2. **Restricted Native Bridge**: Exposed JavaScript interfaces require token validation and whitelist permission checking.
3. **Hardware Acceleration**: Automatically enabled in `AndroidManifest.xml` (`android:hardwareAccelerated="true"`) to ensure 60fps WebGL/3D canvas performance.
