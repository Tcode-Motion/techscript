# TechScript WebForge: Comprehensive Architecture & Web Stack Audit

**Date:** 2026-10-21  
**Project:** TechScript + WebForge  
**Auditor:** Principal Compiler Engineer & Framework Architect  

---

## 1. Current Architecture

The ecosystem comprises two distinct identities:
1. **TechScript Language:**
   - Compiler frontend in Rust: Lexer (`techscript_lexer`), Parser (`techscript_parser`), AST (`techscript_ast`), Semantic Analysis (`techscript_semantic`).
   - Execution targets: AST Interpreter (`techscript_interpreter`), Bytecode Compiler & Virtual Machine (`techscript_bytecode`, `techscript_vm`), and Native LLVM backend (`techscript_llvm_backend`).
   - Standard Library: `techscript_stdlib` containing modules for IO, Math, Strings, Crypto, Compression, Testing, and Web/WebForge (`stdlib/src/web.rs`).
2. **WebForge Framework:**
   - Declarative DSL extensions embedded in TechScript: `site`, `page`, `header`, `nav`, `link`, `card`, `button`, `input`, `data` (SQLite), `api`, `scene` (Three.js 3D), and `edit`.
   - WebForge CLI driver: `webforge` (with aliases `techscript`, `tsc`, `tech`) in `cli/src/`.
   - Build system: `webforge build web --release` lowering AST blocks into static web bundles (`dist/index.html`, `dist/styles.css`, `dist/app.js`).
   - Server: Built-in `tiny_http` server in `stdlib/src/web.rs` handling real SQLite persistence (`webforge.db`) and serving pages.

---

## 2. Existing Working Features

- **Multi-Binary CLI Targets:** `webforge`, `techscript`, `tsc`, and `tech` binaries built and verified in `target/debug/`.
- **Manifest Multi-File Resolution:** Seamlessly discovers `webforge.toml` $\to$ `techscript.toml` $\to$ `tech.toml`.
- **Canonical `.txs` Extension:** Standardized on `.txs` across the ecosystem; legacy `.tech` and specialized `.wtxs`, `.stxs`, `.3txs` are supported with backward compatibility.
- **Concise Declarative Parser:** Supports `site`, `page Name "/path":`, `title`, `text`, `button "Label" -> open Page`, `component`, `data`, `api`, `scene`, `cube`, and `edit`.
- **SQLite Persistence:** Real embedded SQLite integration creating tables, inserting records, and querying records via `/api/<table_name>` and `webforge.db`.
- **Baseline 3D WebGL:** Three.js scene creation with camera, directional lighting, metallic materials, and pointer drag rotation.
- **Production Bundler:** Generates standalone HTML5, CSS3, and JavaScript in `dist/`.
- **Test Suite:** 38 unit and integration tests passing in `techscript_cli`.

---

## 3. Broken / Underperforming Features Identified

1. **Monolithic Single-Page Output:**
   - The generated site currently outputs all pages stacked together in one large document using CSS `#hash` navigation rather than a true multi-page router with distinct routes, breadcrumbs, history management, and clean URLs.
2. **Missing Specific Required Routes:**
   - Routes such as `/loading`, `/features`, `/showcase/3d`, `/docs/getting-started`, `/docs/syntax`, `/docs/routing`, `/docs/api`, `/docs/3d`, `/examples/fullstack`, and branded `/404` are either missing or merged into generic anchors.
3. **Limited 3D Capabilities:**
   - Only a single rotating cube is currently rendered. There is no multi-scene 3D laboratory, no model customization, no material/lighting controls, no wireframe toggle, and no procedural/interactive environments.
4. **Hardcoded API Routing:**
   - `/api/<table_name>` only queries SQLite tables. Missing dedicated endpoints for `GET /api/health`, `GET /api/examples`, and structured JSON 404 responses.
5. **Monochromatic Visual Aesthetics:**
   - The visual design relies on repetitive card blocks without distinct typography scales, syntax highlighting, interactive code editors, live reactive state counters, or varied grid layouts.
6. **Direct Route Entry in Static Mode:**
   - Entering `/showcase/3d` or `/docs/getting-started` directly into a static server results in HTTP 404 unless individual static route files or server-side URL rewrites are generated.

---

## 4. Missing Framework Capabilities

1. **Modern Client Router:**
   - A client-side router supporting both HTML5 History (`pushState`/`popstate`) and static pre-rendered routes, updating document title, meta tags, active navigation states, and scroll position.
2. **Multi-Scene 3D Engine Component:**
   - Configurable 3D laboratory with OrbitControls, geometry generators (workspace, procedural sculpture, planet/satellite, particle cosmos, product studio), material shader controls, lighting sliders, and TechScript code reflection.
3. **Interactive Code Playground:**
   - In-browser interactive TechScript code explorer with syntax highlighting, sample switching, live DOM preview, and reactive state updates.
4. **Static Route Pre-rendering:**
   - Compiler capability during `webforge build web` to emit directory-based `index.html` files for every route (`dist/docs/syntax/index.html`, etc.) and `dist/404.html`.
5. **Enhanced Server Endpoints:**
   - Built-in handler for `GET /api/health`, `GET /api/examples`, `POST /api/feedback`, and CORS/security headers.

---

## 5. Proposed Implementation Order

1. **Phase 1: WebForge Rendering Engine & Router Enhancement (`stdlib/src/web.rs`)**
   - Implement multi-route architecture with HTML5 History & static route generation.
   - Upgrade visual styling tokens: `#090A0F` background, `#121520` surface, `#6366F1` primary, `#06B6D4` cyan accent, `#8B5CF6` violet accent.
   - Implement dedicated API endpoints: `/api/health`, `/api/examples`, `/api/feedback`.
2. **Phase 2: Sophisticated 3D Laboratory Engine**
   - Integrate Three.js OrbitControls, 5 distinct procedural scenes (Futuristic Workspace, Orbiting Planet, Procedural Sculpture, Product Studio, Particle Cosmos).
   - Add interactive control panel: material switcher, lighting intensity, rotation speed, wireframe toggle, pause/resume, and live TechScript source reflection.
3. **Phase 3: Multi-Page Content & Route Definitions (`website/`)**
   - Author rich, structured TechScript source files for all required routes:
     `/`, `/home`, `/loading`, `/language`, `/framework`, `/features`, `/showcase`, `/showcase/3d`, `/playground`, `/docs`, `/docs/getting-started`, `/docs/syntax`, `/docs/routing`, `/docs/api`, `/docs/3d`, `/examples`, `/examples/fullstack`, `/packages`, `/roadmap`, `/community`, `/404`.
4. **Phase 4: Build System & Static Route Emission (`cli/src/commands/build.rs`)**
   - Update `build_web` to emit multi-route static folders (`dist/<route>/index.html`) so every route works when directly navigated, refreshed, or hosted on static CDNs.
5. **Phase 5: Security Audit & Vulnerability Checks**
   - Audit XSS prevention, HTML escaping, SQL injection parameterized queries, input validation, dependency security, and security headers.
6. **Phase 6: Browser Automation & End-to-End Verification**
   - Verify all routes, direct entry, 3D laboratory controls, API responses, console error-free status, and responsive layout across desktop and mobile.
7. **Phase 7: Knowledge Graph Update & Final Deliverable Report**
   - Run `graphify update .` and produce the complete verification report.
