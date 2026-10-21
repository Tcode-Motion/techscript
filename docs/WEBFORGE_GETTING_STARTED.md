# Getting Started with TechScript WebForge

Welcome to **TechScript WebForge**, the official full-stack web framework for the TechScript programming language. This guide takes you from zero to running your first full-stack application with interactive UI, real SQLite storage, and 3D graphics in under 5 minutes.

---

## 1. System Requirements & Installation

### Prerequisites
- **Operating System:** Windows 10/11, macOS (x86_64 or Apple Silicon), or Linux (Ubuntu 20.04+, Debian, Fedora, Arch).
- **Rust Toolchain:** Rust compiler (`rustc`) and `cargo` (1.80+).
- **Web Browser:** Any modern evergreen browser (Google Chrome, Microsoft Edge, Mozilla Firefox, or Safari).

### Verifying the CLI
Verify that the `techscript` compiler and CLI driver is available in your PATH:

```bash
techscript --version
# or using canonical aliases:
tsc --version
tech --version
```

If you need to check environment health, run:
```bash
techscript doctor
```

---

## 2. Creating Your First Project

The interactive project generator scaffolds a complete, ready-to-run WebForge application:

### Option A: Using the CLI Shortcut
```bash
techscript new fullstack my-site
```

Supported project templates include:
- `website`: Clean static and interactive web frontend.
- `fullstack`: Complete full-stack app with UI, SQLite database, and REST APIs.
- `api`: Lightweight backend REST API service.
- `3d`: Interactive Three.js WebGL graphics scene.

### Option B: Interactive Wizard
Simply execute:
```bash
techscript new
```
Choose `[8] Full-Stack App (WebForge + SQLite)` and provide your desired project name.

---

## 3. Project Directory Structure

Entering your new project folder (`cd my-site`), you will find:

```text
my-site/
├── techscript.toml          # Authoritative project manifest
├── src/
│   ├── main.txs             # Core full-stack entry point
│   ├── pages/
│   │   └── home.wtxs        # Modular page view
│   ├── server/
│   │   └── api.stxs         # Server endpoint declarations
│   └── scenes/
│       └── world.3txs       # 3D WebGL scene definition
├── public/                  # Static assets (images, fonts)
└── tests/
    └── app_test.txs         # Unit and integration tests
```

---

## 4. Running the Development Server

Launch the live local development server with hot-reloading:

```bash
techscript dev
```

Output:
```text
========================================================
TechScript WebForge Development Server
Serving on: http://127.0.0.1:3000
========================================================
[WebForge] Database initialized at: webforge.db
[WebForge] Registered REST endpoint: GET  /api/notes
[WebForge] Registered REST endpoint: POST /api/notes
[WebForge] 3D Scene 'World' compiled with Three.js engine.
Press Ctrl+C to stop.
```

Open your browser and navigate to:
**`http://127.0.0.1:3000`**

You will see:
1. The dark-first TechScript WebForge UI.
2. A live interactive Three.js 3D cube spinning with mouse controls.
3. An active input field and button that persists notes into the local SQLite database.

---

## 5. Inspecting Registered Routes

To view all pages, API endpoints, and event listeners configured in your project:

```bash
techscript routes
```

Output:
```text
Available WebForge Routes:
  GET  /               -> Page: Home
  GET  /api/notes      -> REST: Query table 'notes'
  POST /api/notes      -> REST: Insert into table 'notes'
  GET  /assets/app.js  -> Client Actions Runtime
  GET  /assets/3d.js   -> Three.js WebGL Engine
```

---

## 6. Building for Production

When you are ready to prepare your application for release, compile an optimized production bundle:

```bash
techscript build web --release
```

This compiles your application into the `dist/` directory:
- `dist/index.html`: Fully rendered semantic HTML5 document.
- `dist/styles.css`: Minified CSS stylesheet with design tokens.
- `dist/app.js`: Client-side reactivity and API communication logic.
- `dist/schema.sql`: Production SQLite database migration script.

---

## 7. Next Steps

- Explore the [WebForge Language Guide](WEBFORGE_LANGUAGE_GUIDE.md) to learn English-like syntax rules.
- Dive into [WebForge Databases](WEBFORGE_DATABASES.md) to master SQLite models and relations.
- Read [WebForge 3D](WEBFORGE_3D.md) to build immersive WebGL experiences.
