# TechScript Logo Package

Generated from your transparent master logo (`source/logo-master-transparent.png`, 500x500).
Note: the "black background" image was a promo shot, not transparent — it was NOT used to
generate icons (only the transparent one was, since icons need to sit on any background).

## Where to use each file

### 1. VS Code Extension (`vscode-extension/`)
- `icon.png` (128x128) → set as `"icon"` field in your extension's `package.json`.
  This is what shows in the Extensions marketplace and sidebar.
- `icon@2x.png` (256x256) → optional, keep for retina/high-DPI marketplace rendering.

### 2. Windows file association icon — i.e. the icon `.txs` files show in File Explorer (`ico/file-icon.ico`)
- Multi-resolution `.ico` (16, 24, 32, 48, 64, 128, 256px) — this is what Explorer picks the
  right size for automatically depending on view (list/details/large icons/tiles).
- Point your registry entry or installer's file-association step at this file, e.g.:
  ```
  HKEY_CLASSES_ROOT\.txs\DefaultIcon = "C:\Program Files\TechScript\file-icon.ico"
  ```

### 3. App / EXE icon (`ico/app-icon.ico`, `windows/installer-icon.ico`)
- Use `app-icon.ico` as the icon compiled into your `.exe` (in Rust, via `winres` or
  `embed-resource` crate + a `build.rs` pointing at this file).
- Use `installer-icon.ico` for your installer (Inno Setup / NSIS / WiX) — identical file,
  separated only so you can swap one without touching the other.

### 4. Favicon (`ico/favicon.ico`)
- For the language's website/docs (`tech-script.github.io`) — drop in the site root.

### 5. Raw PNGs at every common size (`png/`)
Covers every size an OS, package manager, or doc site is likely to ask for:
`16, 20, 24, 32, 40, 48, 64, 72, 96, 128, 144, 150, 192, 256, 512, 1024`
- 16–48: taskbar, tray, small UI icons
- 128–256: Start menu tiles, README headers, GitHub social preview
- 512–1024: Play-Store-style listing/store pages, high-res docs, app icon source for
  further packaging (e.g. macOS `.icns` if you ever port TechScript there)

## Quality note
Your source logo is 500x500. Everything ≤512px is resampled cleanly. The 1024px PNG is
upscaled beyond the source resolution — usable, but if you ever need it pixel-perfect at
that size, re-export directly from your original vector/design file at 1024px+.
