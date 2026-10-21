# TechScript Official Website

The official website for **TechScript** and the **WebForge** web framework, built entirely using TechScript WebForge itself.

## Architecture

This project is a canonical WebForge website demonstrating all major capabilities of the framework:

- **Framework:** WebForge
- **Language:** TechScript
- **Configuration:** `webforge.toml`
- **Primary Source Files:** All source files use the standard `.txs` extension:
  - `src/main.txs` — Canonical application entry point and header navigation
  - `src/pages/*.txs` — 12 interactive responsive pages (Home, Language, Framework, Docs, Playground, Examples, Packages, CLI, Showcase, Roadmap, Community, Download)
  - `src/server/api.txs` — Embedded SQLite database schema and REST API endpoints
  - `src/scenes/logo.txs` — Interactive Three.js 3D WebGL logo viewport
- **Styling & Aesthetics:** Dark-first palette (`#090A0F`, `#121520`, `#6366F1`, `#06B6D4`, `#8B5CF6`), modern typography, glassmorphism cards, and smooth navigation.

## Development

Start the WebForge development server:
```bash
webforge dev
```
Open [http://localhost:3000](http://localhost:3000) in your browser.

## Inspect Registered Routes

Inspect all UI pages, API endpoints, SQLite tables, and event handlers:
```bash
webforge routes
```

## Production Build

Compile optimized standalone static bundle into `dist/`:
```bash
webforge build web --release
```
The output in `dist/` contains:
- `dist/index.html`
- `dist/styles.css`
- `dist/app.js`

Deployable to GitHub Pages, Cloudflare Pages, Vercel, Netlify, or any static HTTP server.
