## Deploying `public-website/`

This folder is a **static site** (plain HTML/CSS/JS). You can host it anywhere.

### GitHub Pages (simple option)
- Create a repo (or use this repo).
- Ensure `public-website/` is committed.
- In GitHub: **Settings → Pages**
  - **Source**: Deploy from a branch
  - **Branch**: `main` / `/root` (recommended: copy files to root of a `gh-pages` branch), or use an Actions workflow.

### Netlify / Vercel
- Set the publish directory to `public-website/`.
- No build command needed.

### Notes
- Replace the disabled “GitHub / Releases” links in `index.html` with real URLs for your repo.

