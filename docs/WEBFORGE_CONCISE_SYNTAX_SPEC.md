# TechScript WebForge: Concise Syntax Specification

## 1. Identity & Philosophy

**TechScript** is the general-purpose programming language.  
**WebForge** is the official full-stack web framework built for TechScript.

### Core Design Philosophy
> **"Write less code. Express more intent. Get a working application."**

TechScript and WebForge are designed to be immediately understandable to:
1. **A child around 10 years old** who has basic English knowledge.
2. **A complete beginner** who has never written code in any language.
3. **A developer** who values concise, declarative, readable code without boilerplate.

WebForge provides sensible defaults for routing, layouts, responsive design, typography, dark themes, SQLite persistence, and Three.js 3D WebGL rendering without requiring manual HTML/CSS/JS boilerplate.

---

## 2. File Extension Standard

- **`.txs`** is the canonical, default, and standard source file extension across the ecosystem.
  - Used for pages, components, server routes, database schemas, 3D scenes, tests, and business logic.
- **Optional Specialized Extensions**:
  - `.wtxs`: Web UI pages (optional)
  - `.stxs`: Server routes & database tables (optional)
  - `.3txs`: 3D WebGL scenes (optional)
- Special extensions are **never required**; any `.txs` file can define any combination of WebForge blocks.

---

## 3. Project Configuration (`webforge.toml`)

WebForge projects use `webforge.toml` as their primary project manifest, with full backward compatibility for `techscript.toml` and legacy `tech.toml`:

```toml
[package]
name = "my-website"
version = "0.1.0"
entry = "app.txs"
capabilities = ["Network", "FileSystem"]
```

---

## 4. Canonical Project Layout

WebForge automatically discovers pages, server endpoints, and 3D scenes without requiring manual routing tables:

```text
my-website/
├── webforge.toml       # Primary configuration
├── app.txs             # Application entry point
├── pages/
│   ├── home.txs        # Route: "/"
│   └── about.txs       # Route: "/about"
├── server/             # (Optional) Backend APIs and SQLite schemas
│   └── api.txs
├── scenes/             # (Optional) Three.js 3D scenes
│   └── world.txs
├── assets/
│   ├── images/
│   └── models/
└── README.md
```

---

## 5. Concise Declarative Syntax

### 5.1 Starter Website (`app.txs` / `pages/home.txs`)

```techscript
site "My Website"

page Home "/":
    title "Welcome"
    text "My first website built with TechScript and WebForge"
    button "Get Started" -> open About

page About "/about":
    title "About Me"
    text "Made with TechScript and WebForge"
    button "Back Home" -> open Home
```

#### Explanation:
- `site "Name"`: Declares the website title, metadata, viewport, and injects the WebForge responsive dark styling (`#090A0F`, `#121520`, `#6366F1`, `#06B6D4`, `#8B5CF6`).
- `page <Name> "<route>":`: Defines an accessible, responsive page with client routing.
- `title "<text>"`: Renders a primary semantic heading.
- `text "<content>"`: Renders descriptive paragraph content.
- `button "<label>" -> open <Page>`: Interactive button with zero-JS declarative client navigation to `#<page>`.

---

### 5.2 Reusable Components

Components encapsulate reusable UI elements that can be embedded into any page:

```techscript
component Welcome:
    title "Hello, World!"
    text "My first reusable WebForge component"
```

---

### 5.3 Interactive Reactive State

State variables and reactive bindings update the DOM automatically:

```techscript
page Counter "/":
    count = 0

    title "Counter: {count}"
    button "Add" -> count += 1
```

---

### 5.4 Backend API Endpoints

Declare server routes cleanly:

```techscript
api "/hello":
    return { message: "Hello from WebForge!" }
```

---

### 5.5 Database-Backed Persistence (SQLite)

Declare data models and CRUD operations in plain English:

```techscript
data Tasks:
    title text
    done boolean = false

page Tasks "/tasks":
    show Tasks
    button "New Task" -> create Tasks
```

WebForge automatically provisions an embedded SQLite database (`webforge.db`) with tables, schema migrations, and REST endpoints (`GET /api/tasks`, `POST /api/tasks`).

---

### 5.6 3D WebGL Scenes (Three.js)

3D is a first-class feature in WebForge. Declare 3D scenes, cameras, lighting, materials, and models without manual WebGL boilerplate:

```techscript
scene World:
    camera
    light
    cube Box:
        color "#6366F1"
        position (0, 0, 0)
        size 2
        rotate auto
```

### 5.7 Editing & Reusing Existing 3D Objects

Modify previously declared 3D elements by name:

```techscript
edit Box:
    color "#06B6D4"
    size 3
    position (2, 0, 0)
```

---

## 6. CLI Commands Reference

All commands are executed via the `webforge` CLI (or aliases `techscript`, `tsc`, `tech`):

| Command | Description |
|---------|-------------|
| `webforge new <name>` | Scaffolds a new WebForge project with `webforge.toml`, `app.txs`, and `pages/` |
| `webforge dev` | Starts development server on `http://localhost:3000` with hot-reload and live SQLite |
| `webforge build web --release` | Compiles optimized production bundle into `dist/` |
| `webforge routes` | Displays all registered UI routes, API endpoints, and event handlers |
| `webforge test` | Runs the test suite |
| `webforge doctor` | Validates environment, compiler toolchain, and dependencies |
