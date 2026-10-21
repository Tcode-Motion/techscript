# TechScript WebForge Developer Guide & Architecture Manual
**Full-Stack Website Development Platform for Beginners and Professionals**

---

## 1. Overview & Philosophy

**TechScript WebForge** transforms TechScript into an English-like, full-stack website development platform. It allows beginners—including a 10-year-old knowing basic English—to create, run, debug, test, build, and deploy complete websites with interactive UI, real SQLite database persistence, and 3D WebGL scenes, while compiling down to performant native code and optimized web bundles.

---

## 2. Getting Started

### Prerequisites
- Rust compiler (`rustc` & `cargo` 1.80+)
- Windows 11, macOS, or Linux
- A modern web browser (Edge, Chrome, Firefox, Safari)

### CLI Commands
```bash
# 1. Create a new full-stack website project
tsc new website MyWebsite

# 2. Enter project directory
cd MyWebsite

# 3. Start local development server with live SQLite and 3D canvas
tsc dev

# 4. View registered routes, REST APIs, and client actions
tsc routes

# 5. Build production bundle into dist/ (HTML, CSS, JS, SQLite schema)
tsc build web --release

# 6. Deploy or inspect deployment targets
tsc deploy -t static
tsc deploy -t docker
```

### File Structure & Canonical Extensions
- **Primary Entry File:** `src/main.txs` (`.txs` is the official, canonical default).
- **Optional Modular Extensions:**
  - `src/pages/*.wtxs`: WebForge interactive pages, components, and templates.
  - `src/server/*.stxs`: Backend API endpoints, scheduled workers, database daemons.
  - `src/scenes/*.3txs`: Three.js / WebGL 3D scene viewports.
- **Legacy Compatibility:** Legacy `.tech` files continue to execute seamlessly. Run `tsc migrate` to upgrade to modern `.txs`.
- **Reference:** See [TechScript File Extension Standard](TECHSCRIPT_FILE_EXTENSION_STANDARD.md).

---

## 3. English-Like Syntax Reference

TechScript WebForge extends canonical TechScript with declarative, accessible English constructs:

### A. Minimal Interactive Website
```techscript
make a website called MyWebsite

page Home:
    show heading "Welcome!"
    show text "Built with TechScript WebForge"
    show button "Click Me" called GreetingButton

when GreetingButton is clicked:
    show message "Hello, World!"

start website
```

### B. Full-Stack Application with SQLite Database
```techscript
make a website called NotesApp

database Notes:
    table notes:
        title is text
        content is text

page Home:
    show heading "My Notes"
    show input "Write a note" called NoteInput
    show button "Save Note" called SaveButton

when SaveButton is clicked:
    send NoteInput to /api/notes
    show message "Note saved"

start website
```

### C. Declarative 3D Scene with Three.js
```techscript
make a website called ThreeDWorld

page Home:
    show heading "TechScript 3D WebForge"
    show text "Interactive WebGL scene powered by Three.js"
    show 3D scene called World:
        add cube:
            color is "purple"
            rotation speed is 3
        enable mouse controls
    show button "Explore Scene" called ExploreBtn

when ExploreBtn is clicked:
    show message "3D Orbit Controls Activated!"

start website
```

---

## 4. Frontend Design System

WebForge generates a high-aesthetic responsive dark mode interface:
- **Typography**: Google Fonts Outfit & JetBrains Mono
- **Visuals**: Modern radial gradient background (`#1e1b4b` -> `#0f172a` -> `#020617`), glassmorphic panels (`backdrop-filter: blur(12px)`), subtle neon borders (`#a855f7`).
- **Micro-Animations**: Hover elevations, spring button clicks, animated toast notification manager (`webforgeToast`).
- **Responsive Layout**: Fluid grid layout auto-adjusting from mobile screens to desktop ultra-wides.

---

## 5. Backend Server & Database Engine

### HTTP Server (`tiny_http`)
- Listens on `http://localhost:3000` (or user-defined port).
- Serves the generated reactive application at `/`.
- Automatic REST endpoints for declared tables:
  - `GET /api/<table_name>`: Returns JSON array of the latest records.
  - `POST /api/<table_name>`: Accepts JSON payload, validates, inserts into SQLite, and returns `{ "status": "ok", "message": "Note saved" }`.

### SQLite Persistence (`rusqlite`)
- Managed local database file: `webforge.db`.
- Auto-migrated table schema on startup (`CREATE TABLE IF NOT EXISTS`).
- Parameterized queries protecting against SQL injection vulnerabilities.

---

## 6. Three.js 3D WebGL Engine

Declarative 3D scene blocks compile into self-contained WebGL canvas elements:
- Lighting: Ambient fill light + directional accent spotlight.
- Mesh: Standard PBR material with metallic sheen and roughness.
- Animation: `requestAnimationFrame` render loop with dynamic rotation.
- Interaction: Orbit controls via mouse/pointer drag listeners (`pointerdown`, `pointermove`, `pointerup`).
- Responsiveness: Auto-resizes camera aspect ratio and renderer on browser viewport change.

---

## 7. Production Builds & Deployment

### `tsc build web --release`
Generates a standalone `dist/` directory:
- `dist/index.html`: Fully self-contained or structured production entry point.
- `dist/styles.css`: Minified WebForge design system stylesheet.
- `dist/app.js`: Extracted client runtime, toast engine, and event bindings.
- `dist/webforge.db`: Bundled SQLite database.

### Deployment Targets
- **Static Hosting (Vercel / Netlify / Cloudflare Pages)**:
  `tsc deploy -t static` creates pre-configured production files.
- **Docker Containers**:
  `tsc deploy -t docker` emits multi-stage `Dockerfile` and `docker-compose.yml`.

---

## 8. Verified Test Matrix

| Crate / Suite | Tests | Status |
|---|---|---|
| `compiler/parser` | 23/23 | ✅ 100% Passed |
| `compiler/semantic` | 16/16 | ✅ 100% Passed |
| `runtime/interpreter` | 12/12 | ✅ 100% Passed |
| `stdlib` | 49/49 | ✅ 100% Passed |
| `cli` (Unit & Snapshots) | 26/26 | ✅ 100% Passed |
| `webforge_e2e_tests` | 4/4 | ✅ 100% Passed |
| **Total Test Suite** | **130/130** | **✅ 100% Verified** |
