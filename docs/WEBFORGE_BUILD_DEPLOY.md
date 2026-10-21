# TechScript WebForge: Build & Deployment Guide
**Production Optimization, Static Bundling, and Container Deployment**

---

## 1. Overview

WebForge provides a zero-hassle build pipeline that compiles declarative TechScript source code into production-ready web artifacts.

---

## 2. Generating Production Builds

To compile your application for production:

```bash
techscript build web --release
```

### Production Artifacts (`dist/`)
The compiler produces optimized files inside `dist/`:

- **`dist/index.html`:** Clean, semantic HTML5 structure with SEO meta tags and pre-linked assets.
- **`dist/styles.css`:** Extracted, minified CSS stylesheet containing all WebForge design tokens and responsive layout rules.
- **`dist/app.js`:** Lightweight client-side action runtime handling DOM events and asynchronous API communication.
- **`dist/schema.sql`:** DDL migration script for initializing the production database.

---

## 3. Deploying Static Websites

If your application consists of frontend views, documentation, or client-rendered 3D scenes:

### Deploy to Cloudflare Pages / Vercel / Netlify
1. Set the **Build Command** to:
   ```bash
   techscript build web --release
   ```
2. Set the **Output Directory** to:
   ```text
   dist
   ```

### Deploy to Nginx / Caddy / Apache
Simply copy the contents of `dist/` to your web server's public root:
```bash
cp -r dist/* /var/www/html/
```

---

## 4. Deploying Full-Stack Applications

For full-stack applications requiring active SQLite persistence and backend REST API processing:

### Option A: Using the CLI Deploy Helper
```bash
techscript deploy --target docker
```
This generates a production Dockerfile and container specification ready for deployment to any container runner (AWS ECS, Fly.io, Railway, Google Cloud Run).

### Option B: Docker Containerization
```dockerfile
# Multi-stage production container
FROM rust:1.80-alpine AS builder
WORKDIR /app
COPY . .
RUN cargo install --path cli --bin techscript
RUN techscript build web --release

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/dist ./dist
COPY --from=builder /usr/local/cargo/bin/techscript /usr/local/bin/
EXPOSE 3000
CMD ["techscript", "dev", "--port", "3000"]
```

---

## 5. Security & Secret Management

- **Client vs Server Secrets:** Never include private API keys or database credentials in client `.txs` or `.wtxs` files.
- **Environment Variables:** Access server-side environment variables exclusively in `.stxs` modules.
- **Static Assets:** Store public images, SVGs, and fonts in the `public/` directory; WebForge automatically copies them into `dist/assets/` during the build step.
