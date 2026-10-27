# TechScript Release Media & Asset Guidelines

This directory houses official brand, application, website, and video assets for TechScript documentation, GitHub releases, and website showcases.

---

## 📁 Directory Structure

```text
docs/assets/
├── brand/       # Verified logos, vector graphics, and brand identity assets
│   ├── techscript-logo.png
│   └── logo-master-transparent.png
├── app/         # Application screenshots from real Android devices and desktop runs
│   ├── appforge-showcase-android.png
│   └── appforge-techscript-live.png
├── website/     # Webforge website screenshots and responsive captures
│   └── [TODO: Drop desktop and mobile website showcase captures here]
└── video/       # Release demonstration videos and animated previews
    └── [TODO: Drop release demo MP4 or preview thumbnail here]
```

---

## 📸 Adding New Phone Screenshots
When capturing new release screenshots from a physical device (e.g., Android phone `10ME8DF1CT0001Z`):

1. **Resolution & Format**: Use full-resolution PNG captures directly from the device via ADB (`adb exec-out screencap -p > screenshot.png`) or device screenshot.
2. **Naming Convention**: Name files descriptively:
   - `docs/assets/app/appforge-home-android.png`
   - `docs/assets/app/appforge-workspace-android.png`
   - `docs/assets/app/appforge-settings-android.png`
3. **Preservation Rule**: Keep original full-resolution captures; do not overwrite or delete existing screenshots until the new captures have been reviewed and approved.

---

## 🎥 Release Demonstration Video
- Place local demo videos in `docs/assets/video/techscript-release-demo.mp4`.
- For GitHub README presentation, link a high-quality preview GIF or thumbnail pointing to the official YouTube / hosted demo video URL once supplied.
