# TechScript Public Website

This is the official public website and documentation hub for TechScript, rewritten in React + Vite + Tailwind CSS.

## Development

1. Install dependencies:
   ```bash
   npm install
   ```
2. Run the development server:
   ```bash
   npm run dev
   ```
3. Open `http://localhost:5173` in your browser.

## Building for Production

To build the optimized static bundle:

```bash
npm run build
```

This will output the compiled assets to the `dist` directory.

## Deployment to GitHub Pages

To deploy this site to `tech-script.github.io`:

1. Build the project:
   ```bash
   npm run build
   ```
2. Commit the changes and push the `dist/` directory (or use a GitHub Action like `peaceiris/actions-gh-pages` to auto-deploy the `dist` folder to the `gh-pages` branch).

## Maintenance Scripts

- `node scripts/generate_release.js`: Generates the `.zip` artifacts and SHA256 hashes for new releases.
- `node scripts/extract_readme.js`: Pulls the active README from the language root.
