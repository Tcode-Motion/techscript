//! # WebForge CSS Design System
//!
//! Provides standardized dark-first design tokens, sleek typography (Outfit, Inter, JetBrains Mono),
//! glassmorphism surfaces, responsive layouts, interactive components, and micro-animations.

pub fn get_webforge_stylesheet() -> &'static str {
    r#"
@import url('https://fonts.googleapis.com/css2?family=Outfit:wght@400;500;600;700;800;900&family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500;600&display=swap');

:root {
    --bg-color: #F8FAFC;
    --surface: #FFFFFF;
    --surface-elevated: #F1F5F9;
    --surface-glass: rgba(255, 255, 255, 0.90);
    --border: #E2E8F0;
    --border-hover: rgba(2, 132, 199, 0.45);
    --primary: #0284C7;
    --primary-light: #0EA5E9;
    --primary-dark: #0369A1;
    --cyan: #0284C7;
    --cyan-light: #38BDF8;
    --teal: #0D9488;
    --teal-light: #14B8A6;
    --silver: #475569;
    --text-main: #0F172A;
    --text-muted: #64748B;
    --glow: 0 4px 20px rgba(2, 132, 199, 0.12);
    --glow-teal: 0 4px 20px rgba(13, 148, 136, 0.12);
    --radius-sm: 8px;
    --radius-md: 12px;
    --radius-lg: 16px;
    --radius-xl: 24px;
    --font-display: 'Outfit', sans-serif;
    --font-sans: 'Inter', -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    --font-mono: 'JetBrains Mono', 'Fira Code', monospace;
}

[data-theme="dark"], body.dark-theme {
    --bg-color: #08090E;
    --surface: #10141D;
    --surface-elevated: #161B26;
    --surface-glass: rgba(16, 20, 29, 0.85);
    --border: rgba(255, 255, 255, 0.08);
    --border-hover: rgba(6, 182, 212, 0.45);
    --primary: #06B6D4;
    --primary-light: #22D3EE;
    --primary-dark: #0891B2;
    --cyan: #06B6D4;
    --cyan-light: #22D3EE;
    --teal: #14B8A6;
    --teal-light: #2DD4BF;
    --silver: #E8EDF3;
    --text-main: #F8FAFC;
    --text-muted: #94A3B8;
    --glow: 0 0 24px rgba(6, 182, 212, 0.25);
    --glow-teal: 0 0 24px rgba(20, 184, 166, 0.25);
}

* {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
}

body {
    background-color: var(--bg-color);
    color: var(--text-main);
    font-family: var(--font-sans);
    line-height: 1.6;
    -webkit-font-smoothing: antialiased;
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    transition: background-color 0.25s ease, color 0.25s ease;
}

/* Header & Navigation Bar */
header.webforge-header, .webforge-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.85rem 2.5rem;
    background: var(--surface-glass);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border-bottom: 1px solid var(--border);
    position: sticky;
    top: 0;
    z-index: 1000;
}

.brand-logo {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    text-decoration: none;
    color: inherit;
    flex-shrink: 0;
}

.brand-logo-img {
    width: 36px;
    height: 36px;
    border-radius: var(--radius-sm);
    object-fit: contain;
    filter: drop-shadow(0 0 10px rgba(6, 182, 212, 0.4));
}

.brand-logo-icon {
    width: 32px;
    height: 32px;
    border-radius: var(--radius-sm);
    background: linear-gradient(135deg, var(--primary), var(--teal));
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 800;
    font-size: 0.95rem;
    font-family: var(--font-display);
    color: #fff;
    box-shadow: 0 0 16px rgba(6, 182, 212, 0.4);
}

.brand-logo-text {
    font-family: var(--font-display);
    font-size: 1.25rem;
    font-weight: 800;
    letter-spacing: -0.02em;
    color: var(--text-main);
}

[data-theme="dark"] .brand-logo-text, body.dark-theme .brand-logo-text {
    background: linear-gradient(135deg, #fff 0%, #cbd5e1 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
}

.brand-badge {
    font-family: var(--font-mono);
    font-size: 0.7rem;
    font-weight: 700;
    background: rgba(2, 132, 199, 0.12);
    color: var(--cyan);
    border: 1px solid rgba(2, 132, 199, 0.3);
    padding: 0.2rem 0.55rem;
    border-radius: 9999px;
    letter-spacing: 0.04em;
}

nav.webforge-nav, .webforge-nav {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    flex-wrap: nowrap;
    overflow: visible;
}

nav.webforge-nav a, .webforge-nav a {
    color: var(--text-muted);
    text-decoration: none;
    font-family: var(--font-sans);
    font-size: 0.865rem;
    font-weight: 500;
    padding: 0.45rem 0.75rem;
    border-radius: var(--radius-sm);
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    white-space: nowrap;
    letter-spacing: -0.01em;
}

nav.webforge-nav a:hover, .webforge-nav a:hover {
    color: var(--primary);
    background: rgba(2, 132, 199, 0.08);
}
[data-theme="dark"] nav.webforge-nav a:hover, [data-theme="dark"] .webforge-nav a:hover,
body.dark-theme nav.webforge-nav a:hover, body.dark-theme .webforge-nav a:hover {
    color: #fff;
    background: rgba(255, 255, 255, 0.06);
}

nav.webforge-nav a.active, .webforge-nav a.active {
    color: var(--primary);
    background: rgba(2, 132, 199, 0.12);
    border: 1px solid rgba(2, 132, 199, 0.3);
    font-weight: 600;
}
[data-theme="dark"] nav.webforge-nav a.active, [data-theme="dark"] .webforge-nav a.active,
body.dark-theme nav.webforge-nav a.active, body.dark-theme .webforge-nav a.active {
    color: var(--cyan-light);
    background: rgba(6, 182, 212, 0.12);
    border: 1px solid rgba(6, 182, 212, 0.3);
}

.header-actions {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex-shrink: 0;
}

.header-action-link {
    color: var(--text-muted);
    text-decoration: none;
    font-size: 0.85rem;
    font-weight: 500;
    padding: 0.45rem 0.85rem;
    border-radius: var(--radius-sm);
    transition: all 0.2s;
}
.header-action-link:hover {
    color: var(--primary);
    background: rgba(2, 132, 199, 0.08);
}
[data-theme="dark"] .header-action-link:hover, body.dark-theme .header-action-link:hover {
    color: #fff;
    background: rgba(255, 255, 255, 0.06);
}

.header-action-github {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    padding: 0.45rem 0.85rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface-elevated);
    color: var(--text-main);
    font-size: 0.85rem;
    font-weight: 600;
    text-decoration: none;
    transition: all 0.2s;
}
.header-action-github:hover {
    border-color: var(--primary);
    color: var(--primary);
    box-shadow: var(--glow);
}

/* Theme toggle button */
.theme-toggle-btn {
    background: var(--surface-elevated) !important;
    border: 1px solid var(--border) !important;
    color: var(--text-main) !important;
    padding: 0.45rem !important;
    border-radius: var(--radius-sm) !important;
    cursor: pointer !important;
    display: inline-flex !important;
    align-items: center !important;
    justify-content: center !important;
    width: 36px !important;
    height: 36px !important;
    box-shadow: none !important;
    transform: none !important;
    transition: all 0.2s !important;
}
.theme-toggle-btn:hover {
    border-color: var(--primary) !important;
    background: rgba(2, 132, 199, 0.08) !important;
    transform: none !important;
}
[data-theme="dark"] .theme-icon-moon, body.dark-theme .theme-icon-moon { display: none; }
[data-theme="dark"] .theme-icon-sun, body.dark-theme .theme-icon-sun { display: block; }
:root:not([data-theme="dark"]) .theme-icon-sun { display: none; }
:root:not([data-theme="dark"]) .theme-icon-moon { display: block; }

.mobile-toggle {
    display: none;
    background: none;
    border: none;
    color: #fff;
    font-size: 1.5rem;
    cursor: pointer;
}

/* Dropdown nav group */
.nav-group {
    position: relative;
    display: inline-flex;
    align-items: center;
}
.nav-group-trigger {
    color: var(--text-muted);
    text-decoration: none;
    font-size: 0.825rem;
    font-weight: 500;
    padding: 0.35rem 0.65rem;
    border-radius: var(--radius-sm);
    transition: all 0.2s;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.3rem;
    white-space: nowrap;
}
.nav-group-trigger:hover { color: #fff; background: rgba(255,255,255,0.06); }
.nav-dropdown {
    display: none;
    position: absolute;
    top: calc(100% + 8px);
    left: 0;
    background: rgba(18,22,31,0.97);
    backdrop-filter: blur(20px);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    min-width: 200px;
    padding: 0.5rem;
    z-index: 2000;
    box-shadow: 0 16px 48px rgba(0,0,0,0.5), var(--glow);
    flex-direction: column;
    gap: 0.2rem;
}
.nav-group:hover .nav-dropdown { display: flex; }
.nav-dropdown a {
    color: var(--text-muted) !important;
    padding: 0.5rem 0.75rem !important;
    border-radius: 8px !important;
    font-size: 0.875rem !important;
    display: flex !important;
    align-items: center !important;
    gap: 0.6rem !important;
}
.nav-dropdown a:hover { color: #fff !important; background: rgba(6,182,212,0.12) !important; }
.nav-sep {
    width: 1px;
    height: 20px;
    background: var(--border);
    margin: 0 0.4rem;
}

/* Main Container & Page Views */
main.webforge-main, .webforge-main {
    flex: 1;
    max-width: 1280px;
    width: 100%;
    margin: 0 auto;
    padding: 2.5rem 1.5rem;
}

.webforge-page-view {
    display: none;
    animation: webforgeFade 0.25s ease-out forwards;
}

.webforge-page-view.active-page {
    display: block;
}

@keyframes webforgeFade {
    from { opacity: 0; transform: translateY(8px); }
    to { opacity: 1; transform: translateY(0); }
}

.webforge-breadcrumbs {
    font-size: 0.88rem;
    color: var(--text-muted);
    margin-bottom: 1.5rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.webforge-breadcrumbs a {
    color: var(--text-muted);
    text-decoration: none;
}

.webforge-breadcrumbs a:hover {
    color: var(--cyan);
}

.webforge-breadcrumbs .sep {
    color: rgba(255, 255, 255, 0.2);
}

.webforge-breadcrumbs .current {
    color: #fff;
    font-weight: 600;
}

/* Typography */
.webforge-heading, h1 {
    font-size: clamp(2.2rem, 5vw, 3.4rem);
    font-weight: 800;
    letter-spacing: -0.03em;
    margin-bottom: 1rem;
    color: var(--text-main);
    line-height: 1.1;
}

[data-theme="dark"] .webforge-heading, [data-theme="dark"] h1,
body.dark-theme .webforge-heading, body.dark-theme h1 {
    background: linear-gradient(135deg, #ffffff 0%, #e2e8f0 40%, var(--primary) 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
}

.webforge-subheading, h2 {
    font-size: 1.75rem;
    font-weight: 700;
    color: var(--text-main);
    margin-bottom: 0.75rem;
}

.webforge-text, p {
    font-size: 1.05rem;
    color: var(--text-muted);
    line-height: 1.7;
    margin-bottom: 1.5rem;
}

/* Sections & Grids */
.hero {
    text-align: center;
    padding: 4.5rem 2rem 3rem 2rem;
    margin-bottom: 3.5rem;
    border-radius: var(--radius-xl);
    background:
        radial-gradient(ellipse at 30% -20%, rgba(2, 132, 199, 0.12) 0%, transparent 60%),
        radial-gradient(ellipse at 70% 120%, rgba(13, 148, 136, 0.08) 0%, transparent 60%),
        linear-gradient(180deg, #FFFFFF 0%, #F8FAFC 100%);
    border: 1px solid var(--border);
    position: relative;
    overflow: hidden;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.04);
}

[data-theme="dark"] .hero, body.dark-theme .hero {
    background:
        radial-gradient(ellipse at 30% -20%, rgba(6, 182, 212, 0.18) 0%, transparent 60%),
        radial-gradient(ellipse at 70% 120%, rgba(20, 184, 166, 0.12) 0%, transparent 60%),
        linear-gradient(180deg, rgba(18, 22, 31, 0.9) 0%, rgba(9, 10, 15, 0.95) 100%);
    border: 1px solid rgba(6, 182, 212, 0.15);
    box-shadow: 0 0 80px rgba(6, 182, 212, 0.06), inset 0 1px 0 rgba(255,255,255,0.05);
}

.hero::before {
    content: '';
    position: absolute;
    inset: 0;
    background: url("data:image/svg+xml,%3Csvg width='60' height='60' viewBox='0 0 60 60' xmlns='http://www.w3.org/2000/svg'%3E%3Cg fill='none' fill-rule='evenodd'%3E%3Cg fill='%2306B6D4' fill-opacity='0.03'%3E%3Cpath d='M36 34v-4h-2v4h-4v2h4v4h2v-4h4v-2h-4zm0-30V0h-2v4h-4v2h4v4h2V6h4V4h-4zM6 34v-4H4v4H0v2h4v4h2v-4h4v-2H6zM6 4V0H4v4H0v2h4v4h2V6h4V4H6z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E");
    pointer-events: none;
    opacity: 0.5;
}

.hero .subtitle {
    max-width: 720px;
    margin: 0 auto 2rem auto;
    font-size: 1.15rem;
    color: var(--text-muted);
    line-height: 1.7;
}

[data-theme="dark"] .hero .subtitle, body.dark-theme .hero .subtitle {
    color: #cbd5e1;
}

.hero-brand-pill {
    display: inline-flex;
    align-items: center;
    gap: 0.65rem;
    padding: 0.4rem 1.1rem;
    background: rgba(6, 182, 212, 0.08);
    border: 1px solid rgba(6, 182, 212, 0.28);
    border-radius: 9999px;
    margin-bottom: 1.5rem;
    font-size: 0.88rem;
    font-weight: 600;
    color: var(--primary-light);
    backdrop-filter: blur(8px);
    box-shadow: 0 4px 16px rgba(6, 182, 212, 0.12);
}

.hero-brand-icon {
    width: 22px;
    height: 22px;
    object-fit: contain;
    filter: drop-shadow(0 0 6px rgba(6, 182, 212, 0.5));
}

.hero-brand-link {
    color: var(--primary-light);
    text-decoration: none;
    display: inline-flex;
    align-items: center;
    gap: 0.3rem;
}

.hero-brand-link:hover {
    color: #fff;
    text-decoration: underline;
}

.content-section {
    margin-bottom: 4rem;
}

.content-section > .webforge-subheading {
    font-size: 2rem;
    margin-bottom: 0.5rem;
}

.section-desc {
    color: var(--text-muted);
    font-size: 1.05rem;
    margin-bottom: 2rem;
    max-width: 680px;
}

.grid-layout {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
    margin-top: 1.5rem;
}

/* Stats row */
.stats-row {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
    gap: 1.5rem;
    margin: 2.5rem 0;
}
.stat-card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: 2rem 1.5rem;
    text-align: center;
    transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
    position: relative;
    overflow: hidden;
}
.stat-card::before {
    content: '';
    position: absolute;
    top: 0; left: 0; right: 0;
    height: 2px;
    background: linear-gradient(90deg, var(--primary), var(--teal));
    opacity: 0;
    transition: opacity 0.3s;
}
.stat-card:hover { border-color: var(--border-hover); box-shadow: var(--glow); transform: translateY(-4px); }
.stat-card:hover::before { opacity: 1; }
.stat-number {
    font-size: 2.5rem;
    font-weight: 800;
    background: linear-gradient(135deg, #fff 0%, var(--primary) 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    line-height: 1;
    margin-bottom: 0.4rem;
    display: block;
}
.stat-label {
    font-size: 0.875rem;
    color: var(--text-muted);
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.06em;
}

/* Feature highlight row: icon + text */
.feature-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-top: 1rem;
}
.feature-item {
    display: flex;
    align-items: flex-start;
    gap: 1rem;
    padding: 1rem 1.25rem;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    transition: all 0.2s;
}
.feature-item:hover { border-color: var(--border-hover); box-shadow: var(--glow); }
.feature-icon {
    font-size: 1.5rem;
    flex-shrink: 0;
    margin-top: 0.1rem;
}
.feature-info h4 { font-size: 1rem; font-weight: 700; color: #fff; margin-bottom: 0.25rem; }
.feature-info p { font-size: 0.9rem; color: var(--text-muted); margin: 0; line-height: 1.5; }

/* Terminal/code showcase */
.terminal-showcase {
    background: #050608;
    border: 1px solid rgba(6, 182, 212, 0.2);
    border-radius: var(--radius-lg);
    overflow: hidden;
    margin: 1.5rem 0;
    box-shadow: 0 24px 64px rgba(0,0,0,0.5), 0 0 0 1px rgba(255,255,255,0.04);
}
.terminal-topbar {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.85rem 1.25rem;
    background: rgba(255,255,255,0.03);
    border-bottom: 1px solid rgba(255,255,255,0.05);
}
.terminal-dot { width: 12px; height: 12px; border-radius: 50%; }
.terminal-dot.red { background: #ff5f57; }
.terminal-dot.yellow { background: #ffbd2e; }
.terminal-dot.green { background: #28c940; }
.terminal-title { font-size: 0.8rem; color: var(--text-muted); margin-left: auto; font-family: 'JetBrains Mono', monospace; }
.terminal-body {
    padding: 1.5rem;
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.88rem;
    line-height: 1.8;
    color: #a5f3fc;
    overflow-x: auto;
}
.terminal-body .cmd { color: #22d3ee; }
.terminal-body .comment { color: #475569; }
.terminal-body .output { color: #86efac; }
.terminal-body .path { color: #fbbf24; }

/* Comparison table */
.compare-table {
    width: 100%;
    border-collapse: collapse;
    margin: 1.5rem 0;
    border-radius: var(--radius-lg);
    overflow: hidden;
    border: 1px solid var(--border);
}
.compare-table th {
    background: rgba(6, 182, 212, 0.1);
    color: #fff;
    font-weight: 700;
    padding: 1rem 1.25rem;
    text-align: left;
    font-size: 0.9rem;
    border-bottom: 1px solid var(--border);
}
.compare-table td {
    padding: 0.85rem 1.25rem;
    font-size: 0.9rem;
    color: var(--text-muted);
    border-bottom: 1px solid rgba(255,255,255,0.04);
}
.compare-table tr:last-child td { border-bottom: none; }
.compare-table tr:hover td { background: rgba(255,255,255,0.02); color: #fff; }
.compare-table .check { color: #22d3ee; font-weight: 700; }
.compare-table .cross { color: #475569; }

/* Cards */
.card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: 1.75rem;
    transition: all 0.25s cubic-bezier(0.16, 1, 0.3, 1);
    position: relative;
    overflow: hidden;
    box-shadow: 0 4px 14px rgba(0, 0, 0, 0.03);
}

.card:hover {
    transform: translateY(-4px);
    border-color: var(--border-hover);
    box-shadow: var(--glow);
}

.card h3 {
    font-size: 1.25rem;
    font-weight: 700;
    color: var(--text-main);
    margin-bottom: 0.75rem;
}

.card p {
    font-size: 0.95rem;
    color: var(--text-muted);
    line-height: 1.6;
    margin-bottom: 1rem;
}

.card .badge {
    display: inline-block;
    font-size: 0.75rem;
    font-weight: 700;
    padding: 0.25rem 0.6rem;
    border-radius: 6px;
    background: rgba(6, 182, 212, 0.12);
    color: #22d3ee;
    border: 1px solid rgba(6, 182, 212, 0.28);
    margin-bottom: 0.75rem;
}

.card a:not(.webforge-link) {
    color: var(--cyan);
    text-decoration: none;
    font-weight: 500;
}

.card a:not(.webforge-link):hover {
    text-decoration: underline;
    color: #fff;
}

/* IDE-Style Code Window & Syntax Highlighting */
.code-window {
    background: #070a12;
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 12px;
    margin: 1.1rem 0 1.35rem 0;
    overflow: hidden;
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.5);
}

.code-window-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.55rem 0.95rem;
    background: rgba(255, 255, 255, 0.035);
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
}

.code-dots {
    display: flex;
    align-items: center;
    gap: 6px;
}

.code-dots .dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    display: inline-block;
}

.code-dots .dot-red {
    background: #ff5f56;
    box-shadow: 0 0 6px rgba(255, 95, 86, 0.4);
}

.code-dots .dot-yellow {
    background: #ffbd2e;
    box-shadow: 0 0 6px rgba(255, 189, 46, 0.4);
}

.code-dots .dot-green {
    background: #27c93f;
    box-shadow: 0 0 6px rgba(39, 201, 63, 0.4);
}

.code-window-title {
    font-size: 0.76rem;
    font-family: var(--font-mono);
    color: #94a3b8;
    font-weight: 500;
    letter-spacing: 0.04em;
}

.code-window-bar .copy-btn {
    position: static;
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.15);
    color: #94a3b8;
    padding: 0.22rem 0.65rem;
    font-size: 0.72rem;
    border-radius: 6px;
    cursor: pointer;
    box-shadow: none;
    transform: none;
}

.code-window-bar .copy-btn:hover {
    background: rgba(6, 182, 212, 0.25);
    border-color: rgba(6, 182, 212, 0.5);
    color: #fff;
    transform: none;
}

.code-window .webforge-code-block {
    background: transparent;
    border: none;
    border-radius: 0;
    padding: 1.15rem 1.25rem;
    margin: 0;
    font-family: 'JetBrains Mono', 'Fira Code', var(--font-mono), monospace;
    font-size: 0.86rem;
    line-height: 1.7;
    color: #e2e8f0;
    overflow-x: auto;
}

.code-window .webforge-code-block code {
    font-family: inherit;
    white-space: pre;
}

/* Syntax Highlighting Tokens */
.tok-keyword, .tok-kw {
    color: #38bdf8;
    font-weight: 600;
    text-shadow: 0 0 12px rgba(56, 189, 248, 0.25);
}

.tok-type {
    color: #c084fc;
    font-weight: 600;
}

.tok-string, .tok-str {
    color: #a3e635;
}

.tok-comment, .tok-com {
    color: #64748b;
    font-style: italic;
}

.tok-number, .tok-num {
    color: #fbbf24;
}

.tok-literal, .tok-bool {
    color: #f43f5e;
    font-weight: 600;
}

.tok-func {
    color: #60a5fa;
}

/* Standard Code Block fallback */
.webforge-code-block {
    background: #06080d;
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 10px;
    padding: 1.1rem 1.25rem;
    position: relative;
    font-family: var(--font-mono);
    font-size: 0.84rem;
    line-height: 1.65;
    color: #7dd3fc;
    overflow-x: auto;
    margin: 1rem 0 1.25rem 0;
}

.webforge-code-block code {
    font-family: inherit;
    white-space: pre;
}

.copy-btn {
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.15);
    color: var(--text-muted);
    font-size: 0.72rem;
    font-weight: 600;
    padding: 0.22rem 0.6rem;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
    box-shadow: none;
}

.copy-btn:hover {
    background: rgba(6, 182, 212, 0.25);
    border-color: rgba(6, 182, 212, 0.5);
    color: #fff;
    transform: none;
    box-shadow: none;
}

/* 3D Scene HUD Buttons & Controls */
.webforge-3d-scene button.scene-btn,
.scene-btn {
    background: rgba(255, 255, 255, 0.08) !important;
    border: 1px solid rgba(255, 255, 255, 0.16) !important;
    color: #cbd5e1 !important;
    padding: 0.28rem 0.65rem !important;
    font-size: 0.75rem !important;
    font-weight: 500 !important;
    border-radius: 6px !important;
    cursor: pointer !important;
    box-shadow: none !important;
    transform: none !important;
    transition: all 0.15s ease !important;
    line-height: 1.2 !important;
}

.webforge-3d-scene button.scene-btn:hover,
.scene-btn:hover {
    background: rgba(255, 255, 255, 0.18) !important;
    color: #fff !important;
    border-color: rgba(255, 255, 255, 0.3) !important;
    transform: translateY(-1px) !important;
}

.webforge-3d-scene button.scene-btn.active,
.scene-btn.active {
    background: rgba(6, 182, 212, 0.25) !important;
    border-color: #06b6d4 !important;
    color: #22d3ee !important;
    font-weight: 600 !important;
    box-shadow: 0 0 10px rgba(6, 182, 212, 0.3) !important;
}

.webforge-3d-scene button.color-pill,
.color-pill {
    padding: 0 !important;
    box-shadow: none !important;
    transform: none !important;
    transition: transform 0.2s, box-shadow 0.2s !important;
}

.webforge-3d-scene button.color-pill:hover,
.color-pill:hover {
    transform: scale(1.25) !important;
    box-shadow: 0 0 10px rgba(255, 255, 255, 0.6) !important;
}

.webforge-3d-scene button.color-pill.active,
.color-pill.active {
    transform: scale(1.15) !important;
    border: 2px solid #fff !important;
    box-shadow: 0 0 12px currentColor !important;
}

.pulse-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #22d3ee;
    box-shadow: 0 0 10px #22d3ee;
    animation: pulse 2s ease-in-out infinite;
}

.hero-actions {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-wrap: wrap;
    gap: 0.85rem;
    margin-top: 2rem;
}

/* Buttons & Links */
.webforge-btn, button {
    background: linear-gradient(135deg, #06B6D4 0%, #0891B2 100%);
    color: #fff;
    border: none;
    padding: 0.75rem 1.6rem;
    border-radius: 10px;
    font-weight: 600;
    font-size: 0.95rem;
    font-family: inherit;
    cursor: pointer;
    transition: all 0.2s;
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    box-shadow: 0 4px 14px rgba(6, 182, 212, 0.35);
}

.webforge-btn:hover, button:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(6, 182, 212, 0.5);
    filter: brightness(1.1);
}

.webforge-btn.btn-secondary {
    background: var(--surface-elevated);
    color: var(--text-main);
    border: 1px solid var(--border);
    box-shadow: none;
}

.webforge-btn.btn-secondary:hover {
    background: var(--surface);
    border-color: var(--primary);
    color: var(--primary);
}

.webforge-link {
    display: inline-flex;
    align-items: center;
    gap: 0.45rem;
    color: var(--cyan);
    font-weight: 600;
    text-decoration: none;
    padding: 0.5rem 0.95rem;
    border-radius: var(--radius-sm);
    background: rgba(6, 182, 212, 0.08);
    border: 1px solid rgba(6, 182, 212, 0.22);
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    font-size: 0.9rem;
    margin-top: 0.6rem;
    cursor: pointer;
}

.webforge-link:hover {
    background: rgba(6, 182, 212, 0.2);
    border-color: rgba(6, 182, 212, 0.45);
    color: #fff;
    transform: translateY(-2px);
    box-shadow: 0 4px 14px rgba(6, 182, 212, 0.25);
}

.webforge-link.external-link svg {
    transition: transform 0.2s;
}

.webforge-link.external-link:hover svg {
    transform: translate(2px, -2px);
}

/* Inputs & Form Controls */
.webforge-input, input, select {
    background: rgba(18, 21, 32, 0.85);
    border: 1px solid var(--border);
    color: #fff;
    padding: 0.75rem 1rem;
    border-radius: 10px;
    font-size: 0.95rem;
    font-family: inherit;
    outline: none;
    width: 100%;
    transition: all 0.2s;
    margin-bottom: 1rem;
}

.webforge-input:focus, input:focus, select:focus {
    border-color: var(--primary);
    box-shadow: 0 0 0 3px rgba(6, 182, 212, 0.25);
}

/* Code Blocks */
.webforge-code-block {
    background: #050608;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: var(--radius-md);
    padding: 1.25rem;
    position: relative;
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.88rem;
    color: #e2e8f0;
    overflow-x: auto;
    margin: 1rem 0;
}

.webforge-code-block .copy-btn {
    position: absolute;
    top: 0.6rem;
    right: 0.6rem;
    background: rgba(255, 255, 255, 0.08);
    color: #94a3b8;
    border: 1px solid rgba(255, 255, 255, 0.12);
    padding: 0.25rem 0.65rem;
    border-radius: 6px;
    font-size: 0.75rem;
    cursor: pointer;
    box-shadow: none;
}

.webforge-code-block .copy-btn:hover {
    color: #fff;
    background: rgba(255, 255, 255, 0.15);
    transform: none;
}

/* 3D Laboratory & Viewport Styles */
.lab-container {
    display: grid;
    grid-template-columns: 1fr 340px;
    gap: 1.5rem;
    margin-top: 1.5rem;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-xl);
    overflow: hidden;
}

.lab-viewport {
    position: relative;
    min-height: 520px;
    width: 100%;
    background: radial-gradient(circle at center, #131726 0%, #090A0F 100%);
}

.lab-controls {
    background: var(--surface-elevated);
    padding: 1.5rem;
    border-left: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    gap: 1rem;
    overflow-y: auto;
    max-height: 560px;
}

.control-group {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
}

.control-group label {
    font-size: 0.8rem;
    font-weight: 700;
    color: #94a3b8;
    text-transform: uppercase;
    letter-spacing: 0.05em;
}

.color-palette {
    display: flex;
    gap: 0.5rem;
}

.color-dot {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    cursor: pointer;
    border: 2px solid transparent;
    transition: transform 0.2s;
}

.color-dot:hover, .color-dot.active {
    transform: scale(1.2);
    border-color: #fff;
}

.slider-row {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-size: 0.85rem;
}

.slider-row span:first-child {
    width: 100px;
    color: #94a3b8;
}

.slider-row input[type="range"] {
    flex: 1;
    margin: 0;
}

.scene-tabs {
    display: flex;
    gap: 0.35rem;
    flex-wrap: wrap;
    margin-bottom: 0.5rem;
}

.scene-btn {
    background: rgba(255, 255, 255, 0.05);
    color: #cbd5e1;
    border: 1px solid var(--border);
    padding: 0.35rem 0.75rem;
    border-radius: 6px;
    font-size: 0.8rem;
    cursor: pointer;
    box-shadow: none;
}

.scene-btn:hover, .scene-btn.active {
    background: rgba(6, 182, 212, 0.2);
    border-color: var(--primary);
    color: #fff;
    transform: none;
}

/* Code Playground Styles */
.playground-container {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1.5rem;
    margin-top: 1.5rem;
}

.editor-pane, .preview-pane {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    overflow: hidden;
    display: flex;
    flex-direction: column;
}

.pane-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1.25rem;
    background: rgba(255, 255, 255, 0.03);
    border-bottom: 1px solid var(--border);
    font-size: 0.85rem;
    font-weight: 600;
}

.playground-textarea {
    width: 100%;
    height: 340px;
    background: #050608;
    border: none;
    color: #a5f3fc;
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.88rem;
    padding: 1.25rem;
    outline: none;
    resize: none;
    line-height: 1.6;
}

.preview-content {
    padding: 1.5rem;
    flex: 1;
    background: var(--bg-color);
    min-height: 340px;
    overflow-y: auto;
}

/* Skeleton Loading */
.skeleton {
    background: linear-gradient(90deg, rgba(255, 255, 255, 0.03) 25%, rgba(255, 255, 255, 0.08) 50%, rgba(255, 255, 255, 0.03) 75%);
    background-size: 200% 100%;
    animation: shimmer 1.5s infinite;
    border-radius: var(--radius-sm);
}

@keyframes shimmer {
    0% { background-position: 200% 0; }
    100% { background-position: -200% 0; }
}

/* Toasts */
#webforge-toast-container {
    position: fixed;
    bottom: 24px;
    right: 24px;
    z-index: 9999;
    display: flex;
    flex-direction: column;
    gap: 10px;
    pointer-events: none;
}

.webforge-toast {
    pointer-events: auto;
    background: rgba(18, 21, 32, 0.95);
    backdrop-filter: blur(16px);
    -webkit-backdrop-filter: blur(16px);
    border: 1px solid var(--border-hover);
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5), var(--glow);
    color: #fff;
    padding: 0.85rem 1.4rem;
    border-radius: var(--radius-md);
    font-size: 0.92rem;
    display: flex;
    align-items: center;
    gap: 0.6rem;
    animation: toastIn 0.3s cubic-bezier(0.16, 1, 0.3, 1) forwards;
}

@keyframes toastIn {
    from { opacity: 0; transform: translateY(12px); }
    to { opacity: 1; transform: translateY(0); }
}

@keyframes toastOut {
    from { opacity: 1; transform: translateY(0); }
    to { opacity: 0; transform: translateY(12px); }
}

/* Footer */
footer.webforge-footer, .webforge-footer {
    background: var(--surface-elevated);
    border-top: 1px solid var(--border);
    padding: 0;
    color: var(--text-muted);
    font-size: 0.9rem;
    margin-top: auto;
}
[data-theme="dark"] footer.webforge-footer, [data-theme="dark"] .webforge-footer,
body.dark-theme footer.webforge-footer, body.dark-theme .webforge-footer {
    background: #020305;
    border-top: 1px solid rgba(255,255,255,0.06);
}

.footer-inner {
    max-width: 1360px;
    margin: 0 auto;
    padding: 3.5rem 2rem 2.5rem;
    display: grid;
    grid-template-columns: 1.6fr repeat(4, 1fr);
    gap: 2.25rem;
}
.footer-brand p { font-size: 0.9rem; line-height: 1.7; margin: 1rem 0 1.5rem; max-width: 280px; }
.footer-col h4 { font-size: 0.8rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.1em; color: var(--primary); margin-bottom: 1.25rem; }
.footer-col a {
    display: block;
    color: var(--text-muted);
    text-decoration: none;
    font-size: 0.875rem;
    padding: 0.3rem 0;
    transition: color 0.2s;
}
.footer-col a:hover { color: var(--primary); }
[data-theme="dark"] .footer-col a:hover, body.dark-theme .footer-col a:hover { color: #fff; }
.footer-bottom {
    border-top: 1px solid var(--border);
    padding: 1.5rem 2rem;
    max-width: 1280px;
    margin: 0 auto;
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    gap: 0.75rem;
    font-size: 0.8rem;
}
.footer-bottom a { color: var(--text-muted); text-decoration: none; transition: color 0.2s; }
.footer-bottom a:hover { color: var(--primary); }
.footer-status {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.8rem;
    color: var(--text-muted);
}
.footer-status-dot {
    width: 8px; height: 8px; border-radius: 50%;
    background: #22c55e;
    box-shadow: 0 0 8px #22c55e;
    animation: pulse 2s ease-in-out infinite;
}
@keyframes pulse { 0%,100% { opacity: 1; } 50% { opacity: 0.4; } }

@media (max-width: 1100px) {
    nav.webforge-nav a:nth-child(n+9) { display: none; }
}
@media (max-width: 900px) {
    header.webforge-header nav.webforge-nav, .webforge-header nav.webforge-nav {
        display: none;
    }
    .mobile-toggle { display: block; }
    #mobile-drawer { display: none !important; }
    #mobile-drawer.open { display: flex !important; }
    .lab-container, .playground-container { grid-template-columns: 1fr; }
    .lab-controls { border-left: none; border-top: 1px solid var(--border); }
    .footer-inner { grid-template-columns: 1fr 1fr; }
    .stats-row { grid-template-columns: repeat(2, 1fr); }
}
@media (max-width: 640px) {
    .footer-inner { grid-template-columns: 1fr; gap: 2rem; }
    .footer-bottom { flex-direction: column; align-items: flex-start; }
    .stats-row { grid-template-columns: 1fr; }
}

/* ========================================================
   WEBFORGE 3D STUDIO & WORKBENCH STYLING
   ======================================================== */
.webforge-3d-studio {
    display: grid;
    grid-template-columns: minmax(0, 1.45fr) minmax(360px, 440px);
    gap: 1.5rem;
    margin: 2rem 0 3.5rem;
    align-items: stretch;
}

.webforge-3d-scene {
    position: relative;
    background: radial-gradient(ellipse at 50% 40%, #0e1726 0%, #05080e 100%);
    border: 1px solid rgba(6, 182, 212, 0.25);
    border-radius: 20px;
    overflow: hidden;
    box-shadow: 0 25px 65px rgba(0, 0, 0, 0.7), 0 0 35px rgba(6, 182, 212, 0.1);
    display: flex;
    flex-direction: column;
    min-height: 580px;
}

.webforge-3d-topbar {
    position: absolute;
    top: 14px;
    left: 16px;
    right: 16px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    z-index: 10;
    pointer-events: none;
    gap: 0.75rem;
}

.topbar-left {
    display: flex;
    align-items: center;
    gap: 8px;
    background: rgba(8, 14, 24, 0.75);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    padding: 6px 14px;
    border-radius: 9999px;
    border: 1px solid rgba(255, 255, 255, 0.1);
}

.topbar-title {
    font-size: 0.78rem;
    font-weight: 700;
    letter-spacing: 0.05em;
    text-transform: uppercase;
    color: #e2e8f0;
}

.fps-val {
    color: #22d3ee;
    font-family: 'JetBrains Mono', monospace;
    font-weight: 700;
}

.topbar-badge {
    font-size: 0.74rem;
    color: #94a3b8;
    background: rgba(255, 255, 255, 0.06);
    padding: 2px 8px;
    border-radius: 6px;
    font-family: 'JetBrains Mono', monospace;
}

.topbar-right {
    display: flex;
    align-items: center;
    gap: 6px;
    pointer-events: auto;
}

.topbar-btn {
    background: rgba(12, 20, 32, 0.85);
    backdrop-filter: blur(12px);
    border: 1px solid rgba(255, 255, 255, 0.14);
    color: #cbd5e1;
    font-size: 0.75rem;
    font-weight: 600;
    padding: 5px 12px;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
}

.topbar-btn:hover {
    background: rgba(6, 182, 212, 0.2);
    color: #fff;
    border-color: rgba(6, 182, 212, 0.5);
    box-shadow: 0 0 10px rgba(6, 182, 212, 0.25);
}

.topbar-btn.active {
    background: rgba(6, 182, 212, 0.3);
    color: #22d3ee;
    border-color: #06b6d4;
}

.webforge-3d-canvas {
    width: 100%;
    height: 100%;
    flex: 1;
    display: block;
    cursor: grab;
    outline: none;
    min-height: 460px;
}

.webforge-3d-dock {
    position: absolute;
    bottom: 12px;
    left: 12px;
    right: 12px;
    background: rgba(8, 14, 25, 0.92);
    backdrop-filter: blur(18px);
    -webkit-backdrop-filter: blur(18px);
    border: 1px solid rgba(255, 255, 255, 0.14);
    border-radius: 14px;
    padding: 10px 14px;
    z-index: 10;
    box-shadow: 0 12px 36px rgba(0, 0, 0, 0.6);
}

.dock-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 6px;
}

.dock-title {
    font-size: 0.72rem;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    font-weight: 700;
    color: #94a3b8;
}

.dock-hint {
    font-size: 0.7rem;
    color: #64748b;
}

.dock-models-grid {
    display: flex;
    align-items: center;
    gap: 6px;
    overflow-x: auto;
    padding-bottom: 3px;
    scrollbar-width: thin;
    scrollbar-color: rgba(6, 182, 212, 0.3) transparent;
}

.scene-model-btn {
    white-space: nowrap;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: #94a3b8;
    font-size: 0.75rem;
    font-weight: 600;
    padding: 6px 12px;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    flex-shrink: 0;
}

.scene-model-btn:hover {
    background: rgba(6, 182, 212, 0.12);
    color: #fff;
    border-color: rgba(6, 182, 212, 0.4);
}

.scene-model-btn.active {
    background: linear-gradient(135deg, rgba(6, 182, 212, 0.28), rgba(20, 184, 166, 0.2));
    color: #22d3ee;
    border-color: #06b6d4;
    box-shadow: 0 0 14px rgba(6, 182, 212, 0.3);
}

/* Sidepanel Cards */
.webforge-3d-sidepanel {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
}

.sidepanel-card {
    background: rgba(10, 16, 28, 0.85);
    backdrop-filter: blur(18px);
    border: 1px solid rgba(6, 182, 212, 0.2);
    border-radius: 16px;
    padding: 1.25rem;
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.5);
}

.code-window-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-bottom: 0.75rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    margin-bottom: 0.75rem;
}

.window-dots {
    display: flex;
    align-items: center;
    gap: 6px;
}

.dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
}
.dot-red { background: #ff5f56; }
.dot-yellow { background: #ffbd2e; }
.dot-green { background: #27c93f; }

.window-tab {
    font-size: 0.78rem;
    font-family: 'JetBrains Mono', monospace;
    color: #94a3b8;
    font-weight: 500;
}

.copy-code-btn {
    background: rgba(6, 182, 212, 0.12);
    border: 1px solid rgba(6, 182, 212, 0.35);
    color: #22d3ee;
    font-size: 0.75rem;
    font-weight: 600;
    padding: 4px 10px;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
    display: inline-flex;
    align-items: center;
    gap: 4px;
}

.copy-code-btn:hover {
    background: rgba(6, 182, 212, 0.25);
    color: #fff;
    border-color: #06b6d4;
    box-shadow: 0 0 12px rgba(6, 182, 212, 0.3);
}

.copy-code-btn.copied {
    background: rgba(16, 185, 129, 0.25) !important;
    border-color: #10b981 !important;
    color: #34d399 !important;
}

.code-editor-pre {
    margin: 0;
    padding: 0.85rem;
    background: #05080e;
    border-radius: 10px;
    border: 1px solid rgba(255, 255, 255, 0.06);
    overflow-x: auto;
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.82rem;
    line-height: 1.55;
    color: #e2e8f0;
}

.code-note {
    margin-top: 0.85rem;
    padding: 0.65rem 0.85rem;
    background: rgba(6, 182, 212, 0.06);
    border-left: 3px solid #06b6d4;
    border-radius: 6px;
    font-size: 0.76rem;
    color: #94a3b8;
    line-height: 1.5;
}

.code-note-title {
    font-weight: 700;
    color: #22d3ee;
    margin-bottom: 2px;
}

.code-note-desc {
    color: #94a3b8;
}

.controls-section-title {
    font-size: 0.75rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: #94a3b8;
    margin-bottom: 0.65rem;
}

.custom-color-control-group {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 0.75rem;
}

.color-picker-wrapper {
    position: relative;
    flex: 1;
    display: flex;
    align-items: center;
}

.custom-color-native {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    opacity: 0;
    cursor: pointer;
    z-index: 2;
}

.color-picker-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: 100%;
    padding: 7px 12px;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 8px;
    font-size: 0.8rem;
    font-weight: 600;
    color: #cbd5e1;
    transition: all 0.2s ease;
}

.color-picker-wrapper:hover .color-picker-label {
    border-color: rgba(6, 182, 212, 0.4);
    background: rgba(6, 182, 212, 0.08);
}

.color-swatch-preview {
    width: 18px;
    height: 18px;
    border-radius: 50%;
    border: 2px solid #fff;
    box-shadow: 0 0 8px rgba(0,0,0,0.5);
    flex-shrink: 0;
}

.hex-input-wrapper {
    display: flex;
    align-items: center;
    background: rgba(0, 0, 0, 0.4);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 8px;
    padding: 0 8px;
    width: 110px;
}

.hex-prefix {
    font-size: 0.7rem;
    font-weight: 700;
    color: #64748b;
    margin-right: 4px;
    font-family: 'JetBrains Mono', monospace;
}

.hex-text-input {
    width: 100%;
    background: transparent;
    border: none;
    outline: none;
    color: #22d3ee;
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.82rem;
    font-weight: 600;
    padding: 7px 0;
    text-transform: uppercase;
}

.palette-swatches-grid {
    display: flex;
    align-items: center;
    gap: 7px;
    flex-wrap: wrap;
    margin-bottom: 0.5rem;
}

.color-swatch {
    width: 22px;
    height: 22px;
    border-radius: 50%;
    border: 2px solid rgba(255, 255, 255, 0.3);
    cursor: pointer;
    transition: transform 0.2s, border-color 0.2s, box-shadow 0.2s;
}

.color-swatch:hover {
    transform: scale(1.2);
    border-color: #fff;
}

.color-swatch.active {
    border-color: #fff;
    transform: scale(1.25);
    box-shadow: 0 0 10px rgba(255, 255, 255, 0.6);
}

.shading-pills-row {
    display: flex;
    flex-wrap: wrap;
    gap: 5px;
}

.shading-pill {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: #94a3b8;
    font-size: 0.72rem;
    font-weight: 600;
    padding: 5px 9px;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
}

.shading-pill:hover {
    color: #fff;
    background: rgba(6, 182, 212, 0.1);
    border-color: rgba(6, 182, 212, 0.3);
}

.shading-pill.active {
    background: rgba(6, 182, 212, 0.25);
    color: #22d3ee;
    border-color: #06b6d4;
    box-shadow: 0 0 10px rgba(6, 182, 212, 0.25);
}

.sliders-container {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
}

.studio-slider-row {
    display: flex;
    flex-direction: column;
    gap: 4px;
}

.slider-meta {
    display: flex;
    justify-content: space-between;
    font-size: 0.73rem;
    color: #94a3b8;
    font-weight: 600;
}

.slider-val {
    color: #22d3ee;
    font-family: 'JetBrains Mono', monospace;
}

.custom-range-slider {
    -webkit-appearance: none;
    appearance: none;
    width: 100%;
    height: 5px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 5px;
    outline: none;
}

.custom-range-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 15px;
    height: 15px;
    border-radius: 50%;
    background: #06b6d4;
    cursor: pointer;
    box-shadow: 0 0 8px #06b6d4;
    border: 2px solid #fff;
}

@media (max-width: 1024px) {
    .webforge-3d-studio {
        grid-template-columns: 1fr;
    }
    .webforge-3d-scene {
        min-height: 520px;
    }
}

/* ==========================================================================
   WebForge Modern UI Components: Buttons, Timelines, Accordions, Tabs, Stats
   ========================================================================== */

/* Enhanced Animated Buttons */
.btn-glow {
    background: linear-gradient(135deg, #06b6d4, #14b8a6) !important;
    box-shadow: 0 0 20px rgba(6, 182, 212, 0.45) !important;
    border: 1px solid rgba(255, 255, 255, 0.2) !important;
}
.btn-glow:hover {
    box-shadow: 0 0 30px rgba(6, 182, 212, 0.75), 0 0 10px #22d3ee !important;
    transform: translateY(-2px) scale(1.02);
}

.btn-shimmer {
    position: relative;
    overflow: hidden;
    background: linear-gradient(135deg, #1e293b, #0f172a) !important;
    border: 1px solid rgba(6, 182, 212, 0.4) !important;
}
.btn-shimmer::after {
    content: '';
    position: absolute;
    top: -50%;
    left: -60%;
    width: 40%;
    height: 200%;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.25), transparent);
    transform: rotate(25deg);
    animation: shimmerMove 3.5s infinite;
}
@keyframes shimmerMove {
    0% { left: -60%; }
    25%, 100% { left: 140%; }
}

.btn-cyber {
    background: transparent !important;
    border: 1px solid #06b6d4 !important;
    color: #22d3ee !important;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    font-weight: 700 !important;
    position: relative;
}
.btn-cyber:hover {
    background: rgba(6, 182, 212, 0.15) !important;
    box-shadow: inset 0 0 15px rgba(6, 182, 212, 0.3), 0 0 15px rgba(6, 182, 212, 0.4) !important;
}

.btn-outline {
    background: transparent !important;
    border: 1px solid rgba(255, 255, 255, 0.2) !important;
    color: #f8fafc !important;
}
.btn-outline:hover {
    border-color: #06b6d4 !important;
    color: #22d3ee !important;
}

.btn-pill {
    border-radius: 9999px !important;
}

.btn-glass {
    background: rgba(255, 255, 255, 0.06) !important;
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    border: 1px solid rgba(255, 255, 255, 0.12) !important;
    color: #f8fafc !important;
}
.btn-glass:hover {
    background: rgba(255, 255, 255, 0.12) !important;
    border-color: rgba(255, 255, 255, 0.25) !important;
}

.btn-sm {
    padding: 0.35rem 0.85rem !important;
    font-size: 0.8rem !important;
}
.btn-lg {
    padding: 0.85rem 2rem !important;
    font-size: 1.05rem !important;
}

/* Button Animations */
.btn-anim-pulse {
    animation: btnPulse 2s infinite ease-in-out;
}
@keyframes btnPulse {
    0%, 100% { transform: scale(1); box-shadow: 0 0 15px rgba(6, 182, 212, 0.3); }
    50% { transform: scale(1.03); box-shadow: 0 0 25px rgba(6, 182, 212, 0.6); }
}

.btn-anim-bounce:hover {
    animation: btnBounce 0.6s ease infinite alternate;
}
@keyframes btnBounce {
    from { transform: translateY(0); }
    to { transform: translateY(-4px); }
}

.btn-anim-float {
    animation: btnFloat 3s infinite ease-in-out;
}
@keyframes btnFloat {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(-5px); }
}

/* Timeline Component */
.webforge-timeline {
    position: relative;
    padding: 1rem 0;
    margin: 1.5rem 0;
}
.webforge-timeline::before {
    content: '';
    position: absolute;
    top: 0;
    bottom: 0;
    left: 20px;
    width: 2px;
    background: linear-gradient(180deg, #06b6d4, #14b8a6, rgba(255, 255, 255, 0.08));
}
.timeline-item {
    position: relative;
    padding-left: 54px;
    margin-bottom: 2rem;
    transition: all 0.2s;
}
.timeline-item:last-child {
    margin-bottom: 0;
}
.timeline-marker {
    position: absolute;
    left: 8px;
    top: 0;
    width: 26px;
    height: 26px;
    border-radius: 50%;
    background: #0f172a;
    border: 2px solid #06b6d4;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 0 12px rgba(6, 182, 212, 0.5);
    z-index: 2;
}
.timeline-icon {
    font-size: 0.75rem;
    color: #22d3ee;
    font-weight: 700;
}
.timeline-item.status-done .timeline-marker {
    background: #06b6d4;
    border-color: #22d3ee;
}
.timeline-item.status-done .timeline-icon {
    color: #041017;
}
.timeline-item.status-active .timeline-marker {
    border-color: #f59e0b;
    box-shadow: 0 0 16px rgba(245, 158, 11, 0.6);
}
.timeline-item.status-upcoming .timeline-marker {
    border-color: rgba(255, 255, 255, 0.2);
    box-shadow: none;
}
.timeline-content {
    background: rgba(16, 20, 29, 0.7);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    padding: 1.25rem 1.5rem;
    backdrop-filter: blur(12px);
    transition: all 0.2s;
}
.timeline-content:hover {
    border-color: var(--border-hover);
    transform: translateX(4px);
}
.timeline-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex-wrap: wrap;
    margin-bottom: 0.4rem;
}
.timeline-title {
    font-size: 1.15rem;
    font-weight: 700;
    color: #f8fafc;
    margin: 0;
}
.timeline-badge {
    background: rgba(6, 182, 212, 0.15);
    color: #22d3ee;
    border: 1px solid rgba(6, 182, 212, 0.3);
    padding: 0.15rem 0.55rem;
    border-radius: 9999px;
    font-size: 0.72rem;
    font-weight: 600;
}
.timeline-date {
    margin-left: auto;
    font-size: 0.8rem;
    color: #94a3b8;
    font-family: 'JetBrains Mono', monospace;
}
.timeline-desc {
    color: #cbd5e1;
    font-size: 0.92rem;
    margin: 0;
    line-height: 1.5;
}

/* Accordion Component */
.webforge-accordion {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin: 1.5rem 0;
}
.accordion-item {
    background: rgba(16, 20, 29, 0.75);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    overflow: hidden;
    transition: all 0.2s;
}
.accordion-item.open {
    border-color: var(--border-hover);
    box-shadow: 0 0 20px rgba(6, 182, 212, 0.1);
}
.accordion-trigger {
    width: 100%;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.1rem 1.4rem;
    background: transparent;
    border: none;
    color: #f8fafc;
    font-family: var(--font-display);
    font-size: 1.05rem;
    font-weight: 600;
    cursor: pointer;
    text-align: left;
}
.accordion-chevron {
    transition: transform 0.25s ease;
    color: #06b6d4;
    flex-shrink: 0;
}
.accordion-item.open .accordion-chevron {
    transform: rotate(180deg);
}
.accordion-body {
    max-height: 0;
    overflow: hidden;
    transition: max-height 0.3s ease-out;
}
.accordion-item.open .accordion-body {
    max-height: 500px;
}
.accordion-inner {
    padding: 0 1.4rem 1.25rem 1.4rem;
    color: #94a3b8;
    font-size: 0.95rem;
    line-height: 1.6;
    border-top: 1px solid rgba(255, 255, 255, 0.04);
    padding-top: 0.85rem;
}

/* Tabs Component */
.webforge-tabs {
    margin: 1.5rem 0;
}
.tabs-nav {
    display: flex;
    gap: 0.5rem;
    border-bottom: 1px solid var(--border);
    padding-bottom: 0.5rem;
    overflow-x: auto;
}
.tab-btn {
    background: transparent;
    border: 1px solid transparent;
    color: #94a3b8;
    padding: 0.6rem 1.2rem;
    border-radius: var(--radius-sm);
    font-family: var(--font-display);
    font-size: 0.95rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
}
.tab-btn:hover {
    color: #f8fafc;
    background: rgba(255, 255, 255, 0.04);
}
.tab-btn.active {
    color: #22d3ee;
    background: rgba(6, 182, 212, 0.12);
    border-color: rgba(6, 182, 212, 0.3);
}
.tabs-content {
    padding: 1.25rem 0;
}
.tab-panel {
    display: none;
    animation: fadeIn 0.25s ease;
}
.tab-panel.active {
    display: block;
}

/* Stats Component */
.webforge-stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 1.25rem;
    margin: 1.5rem 0;
}
.webforge-stat-card {
    background: rgba(16, 20, 29, 0.8);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    padding: 1.25rem 1.5rem;
    backdrop-filter: blur(12px);
    transition: all 0.2s;
}
.webforge-stat-card:hover {
    border-color: var(--border-hover);
    transform: translateY(-2px);
    box-shadow: 0 8px 24px rgba(6, 182, 212, 0.15);
}
.stat-top {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
}
.stat-label {
    font-size: 0.85rem;
    font-weight: 600;
    color: #94a3b8;
    text-transform: uppercase;
    letter-spacing: 0.05em;
}
.stat-icon {
    font-size: 1.1rem;
}
.stat-trend {
    font-size: 0.75rem;
    font-weight: 700;
    color: #10b981;
    background: rgba(16, 185, 129, 0.15);
    padding: 0.15rem 0.45rem;
    border-radius: 4px;
}
.stat-value {
    font-family: var(--font-display);
    font-size: 2rem;
    font-weight: 800;
    color: #f8fafc;
    letter-spacing: -0.02em;
}

/* Alert Component */
.webforge-alert {
    display: flex;
    gap: 1rem;
    align-items: flex-start;
    padding: 1rem 1.25rem;
    border-radius: var(--radius-md);
    margin: 1rem 0;
    border: 1px solid;
}
.alert-info {
    background: rgba(6, 182, 212, 0.08);
    border-color: rgba(6, 182, 212, 0.3);
}
.alert-success {
    background: rgba(16, 185, 129, 0.08);
    border-color: rgba(16, 185, 129, 0.3);
}
.alert-warning {
    background: rgba(245, 158, 11, 0.08);
    border-color: rgba(245, 158, 11, 0.3);
}
.alert-error {
    background: rgba(239, 68, 68, 0.08);
    border-color: rgba(239, 68, 68, 0.3);
}
.alert-tip {
    background: rgba(168, 85, 247, 0.08);
    border-color: rgba(168, 85, 247, 0.3);
}
.alert-icon {
    font-size: 1.2rem;
    flex-shrink: 0;
}
.alert-body {
    flex: 1;
}
.alert-title {
    font-weight: 700;
    font-size: 0.98rem;
    margin-bottom: 0.2rem;
    color: #f8fafc;
}
.alert-text {
    font-size: 0.9rem;
    color: #cbd5e1;
    line-height: 1.5;
}

/* ==========================================================================
   WebForge Custom 3D Studio & CAD Modeling Workbench Styles
   ========================================================================== */

.webforge-3d-workbench {
    background: #090d16;
    border: 1px solid rgba(6, 182, 212, 0.3);
    border-radius: var(--radius-lg);
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.7), 0 0 32px rgba(6, 182, 212, 0.15);
    overflow: hidden;
    margin: 1.5rem 0 3rem 0;
    display: flex;
    flex-direction: column;
}

.workbench-header {
    background: #0d121f;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    padding: 0.75rem 1.25rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    flex-wrap: wrap;
}

.header-left {
    display: flex;
    align-items: center;
    gap: 1.25rem;
    flex-wrap: wrap;
}

.workbench-brand {
    display: flex;
    align-items: center;
    gap: 0.5rem;
}
.brand-title {
    font-family: var(--font-display);
    font-weight: 800;
    font-size: 1.05rem;
    color: #fff;
    letter-spacing: -0.01em;
}
.brand-tag {
    background: rgba(6, 182, 212, 0.15);
    color: #22d3ee;
    border: 1px solid rgba(6, 182, 212, 0.3);
    font-size: 0.7rem;
    font-weight: 600;
    padding: 0.1rem 0.5rem;
    border-radius: 9999px;
}

.template-selector-group {
    display: flex;
    align-items: center;
    gap: 0.4rem;
}
.selector-label {
    font-size: 0.75rem;
    color: #94a3b8;
    font-weight: 600;
}
.template-btn {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: #cbd5e1;
    font-size: 0.78rem;
    font-weight: 600;
    padding: 0.35rem 0.75rem;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
}
.template-btn:hover {
    background: rgba(6, 182, 212, 0.12);
    border-color: rgba(6, 182, 212, 0.4);
    color: #22d3ee;
}
.template-btn.active {
    background: #06b6d4;
    color: #041017;
    border-color: #22d3ee;
    font-weight: 700;
    box-shadow: 0 0 12px rgba(6, 182, 212, 0.4);
}

.header-actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
}
.wb-action-btn {
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.12);
    color: #f8fafc;
    font-size: 0.8rem;
    font-weight: 600;
    padding: 0.4rem 0.85rem;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
}
.wb-action-btn:hover {
    background: rgba(255, 255, 255, 0.12);
    border-color: rgba(6, 182, 212, 0.4);
    color: #22d3ee;
}
.wb-action-btn.export-btn {
    background: linear-gradient(135deg, rgba(6, 182, 212, 0.2), rgba(20, 184, 166, 0.2));
    border-color: #06b6d4;
    color: #22d3ee;
}
.dropdown-export .export-menu {
    display: none;
    position: absolute;
    top: calc(100% + 4px);
    right: 0;
    background: #0d121f;
    border: 1px solid rgba(6, 182, 212, 0.3);
    border-radius: 8px;
    box-shadow: 0 10px 24px rgba(0, 0, 0, 0.8);
    z-index: 100;
    min-width: 170px;
    padding: 0.35rem 0;
}
.dropdown-export .export-menu.show {
    display: flex;
    flex-direction: column;
}
.dropdown-export .export-menu button {
    background: transparent;
    border: none;
    color: #cbd5e1;
    padding: 0.55rem 1rem;
    font-size: 0.82rem;
    text-align: left;
    cursor: pointer;
    transition: all 0.15s;
}
.dropdown-export .export-menu button:hover {
    background: rgba(6, 182, 212, 0.15);
    color: #22d3ee;
}

/* 3-Column Workspace Layout */
.workbench-body {
    display: grid;
    grid-template-columns: 280px 1fr 340px;
    height: 720px;
    background: #07090e;
}

.workbench-panel {
    background: #0d121f;
    overflow-y: auto;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.85rem;
}
.panel-left {
    border-right: 1px solid rgba(255, 255, 255, 0.08);
}
.panel-right {
    border-left: 1px solid rgba(255, 255, 255, 0.08);
    padding: 0.75rem 1rem;
}

.panel-section-title {
    font-size: 0.78rem;
    font-weight: 700;
    color: #94a3b8;
    text-transform: uppercase;
    letter-spacing: 0.06em;
}

/* Primitives & Tools Grid */
.primitives-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.4rem;
}
.prim-btn {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    color: #cbd5e1;
    font-size: 0.78rem;
    font-weight: 600;
    padding: 0.5rem 0.6rem;
    border-radius: 6px;
    cursor: pointer;
    text-align: left;
    transition: all 0.15s;
}
.prim-btn:hover {
    background: rgba(6, 182, 212, 0.12);
    border-color: rgba(6, 182, 212, 0.35);
    color: #22d3ee;
    transform: translateY(-1px);
}

/* CSG Boolean Actions */
.csg-actions-grid {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
}
.csg-btn {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    color: #cbd5e1;
    font-size: 0.78rem;
    font-weight: 600;
    padding: 0.5rem 0.75rem;
    border-radius: 6px;
    cursor: pointer;
    text-align: left;
    transition: all 0.15s;
}
.csg-btn:hover {
    background: rgba(6, 182, 212, 0.15);
    border-color: #06b6d4;
    color: #22d3ee;
}
.csg-btn.subtract:hover {
    border-color: #f43f5e;
    color: #fb7185;
    background: rgba(244, 63, 94, 0.12);
}

/* Scene Hierarchy Tree */
.scene-tree-list {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
    max-height: 250px;
    overflow-y: auto;
    padding-right: 4px;
}
.tree-node {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.45rem 0.65rem;
    border-radius: 6px;
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid transparent;
    cursor: pointer;
    transition: all 0.15s;
}
.tree-node:hover {
    background: rgba(255, 255, 255, 0.06);
}
.tree-node.active {
    background: rgba(6, 182, 212, 0.18);
    border-color: rgba(6, 182, 212, 0.45);
}
.tree-node-title {
    font-size: 0.8rem;
    font-weight: 600;
    color: #f8fafc;
    display: flex;
    align-items: center;
    gap: 0.4rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}
.tree-node-actions {
    display: flex;
    gap: 0.25rem;
}
.tree-mini-btn {
    background: transparent;
    border: none;
    color: #94a3b8;
    cursor: pointer;
    font-size: 0.8rem;
    padding: 0.15rem 0.3rem;
    border-radius: 4px;
}
.tree-mini-btn:hover {
    color: #fff;
    background: rgba(255, 255, 255, 0.1);
}

/* Center Viewport & Timeline Column */
.workbench-center {
    display: flex;
    flex-direction: column;
    position: relative;
    background: #07090e;
}
.viewport-toolbar {
    background: rgba(13, 18, 31, 0.85);
    backdrop-filter: blur(8px);
    border-bottom: 1px solid rgba(255, 255, 255, 0.06);
    padding: 0.45rem 1rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    z-index: 10;
}
.vp-tools-left, .vp-tools-right {
    display: flex;
    align-items: center;
    gap: 0.5rem;
}
.vp-stat-item {
    font-size: 0.75rem;
    color: #cbd5e1;
    font-weight: 600;
    font-family: 'JetBrains Mono', monospace;
}
.vp-sep {
    color: rgba(255, 255, 255, 0.2);
    font-size: 0.75rem;
}
.vp-tool-btn {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: #cbd5e1;
    font-size: 0.74rem;
    font-weight: 600;
    padding: 0.25rem 0.65rem;
    border-radius: 5px;
    cursor: pointer;
    transition: all 0.15s;
}
.vp-tool-btn:hover, .vp-tool-btn.active {
    background: rgba(6, 182, 212, 0.18);
    border-color: #06b6d4;
    color: #22d3ee;
}

.viewport-canvas-container {
    flex: 1;
    position: relative;
    overflow: hidden;
}
.workbench-canvas {
    width: 100% !important;
    height: 100% !important;
    display: block;
    cursor: grab;
}
.workbench-canvas:active {
    cursor: grabbing;
}

/* Bottom Animation Timeline Bar */
.workbench-timeline-bar {
    background: #0d121f;
    border-top: 1px solid rgba(255, 255, 255, 0.08);
    padding: 0.65rem 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    z-index: 10;
}
.timeline-playback-controls {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    flex-wrap: wrap;
}
.tl-btn {
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.12);
    color: #f8fafc;
    font-size: 0.78rem;
    font-weight: 600;
    padding: 0.35rem 0.75rem;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.15s;
}
.tl-btn:hover {
    background: rgba(6, 182, 212, 0.2);
    border-color: #06b6d4;
    color: #22d3ee;
}
.tl-btn.play-btn {
    background: #06b6d4;
    color: #041017;
    font-weight: 700;
    border-color: #22d3ee;
}
.tl-speed-select, .tl-anim-mode {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    margin-left: 0.5rem;
}
.tl-speed-label {
    font-size: 0.73rem;
    color: #94a3b8;
    font-weight: 600;
}
.tl-speed-select select, .tl-anim-mode select {
    background: #161c2c;
    border: 1px solid rgba(255, 255, 255, 0.12);
    color: #22d3ee;
    font-size: 0.75rem;
    font-weight: 600;
    padding: 0.25rem 0.5rem;
    border-radius: 5px;
    outline: none;
}

.timeline-scrubber-track {
    display: flex;
    align-items: center;
    gap: 1rem;
}
.timeline-scrubber-range {
    flex: 1;
    -webkit-appearance: none;
    appearance: none;
    height: 6px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    outline: none;
}
.timeline-scrubber-range::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: #06b6d4;
    cursor: pointer;
    box-shadow: 0 0 10px #06b6d4;
    border: 2px solid #fff;
}
.timeline-timecode {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.75rem;
    color: #94a3b8;
    min-width: 90px;
    text-align: right;
}

/* Right Panel: Inspector & Code Tabs */
.panel-tabs {
    display: flex;
    gap: 0.4rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    padding-bottom: 0.5rem;
}
.panel-tab-btn {
    flex: 1;
    background: transparent;
    border: 1px solid transparent;
    color: #94a3b8;
    font-size: 0.78rem;
    font-weight: 600;
    padding: 0.45rem 0.5rem;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.15s;
    text-align: center;
}
.panel-tab-btn:hover {
    color: #f8fafc;
    background: rgba(255, 255, 255, 0.04);
}
.panel-tab-btn.active {
    background: rgba(6, 182, 212, 0.15);
    border-color: rgba(6, 182, 212, 0.35);
    color: #22d3ee;
}
.right-tab-content {
    display: none;
    flex-direction: column;
    gap: 0.85rem;
    overflow-y: auto;
}
.right-tab-content.active {
    display: flex;
}

/* Inspector Form Elements */
.inspector-group {
    background: rgba(255, 255, 255, 0.02);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 8px;
    padding: 0.85rem;
    display: flex;
    flex-direction: column;
    gap: 0.65rem;
}
.group-title {
    font-size: 0.75rem;
    font-weight: 700;
    color: #22d3ee;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    border-bottom: 1px solid rgba(255, 255, 255, 0.04);
    padding-bottom: 0.35rem;
}
.field-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 0.5rem;
}
.field-label {
    font-size: 0.76rem;
    color: #94a3b8;
    font-weight: 600;
    min-width: 80px;
}
.field-input {
    flex: 1;
    background: #161c2c;
    border: 1px solid rgba(255, 255, 255, 0.12);
    color: #f8fafc;
    font-size: 0.78rem;
    padding: 0.3rem 0.5rem;
    border-radius: 5px;
    outline: none;
}
.field-input:focus {
    border-color: #06b6d4;
    box-shadow: 0 0 8px rgba(6, 182, 212, 0.3);
}
.field-val-badge {
    background: rgba(6, 182, 212, 0.15);
    color: #22d3ee;
    font-size: 0.72rem;
    font-weight: 700;
    padding: 0.15rem 0.5rem;
    border-radius: 4px;
    font-family: 'JetBrains Mono', monospace;
}

/* 3D Transform Grid */
.transform-3d-grid {
    display: grid;
    grid-template-columns: 24px 1fr 1fr 1fr;
    gap: 0.35rem;
    align-items: center;
}
.axis-header {
    font-size: 0.68rem;
    font-weight: 700;
    color: #64748b;
    text-align: center;
}
.axis-tag {
    font-size: 0.72rem;
    font-weight: 800;
    text-align: center;
    border-radius: 3px;
    padding: 0.15rem 0;
}
.axis-x { color: #f43f5e; background: rgba(244, 63, 94, 0.15); }
.axis-y { color: #10b981; background: rgba(16, 185, 129, 0.15); }
.axis-z { color: #38bdf8; background: rgba(56, 189, 248, 0.15); }
.axis-input {
    background: #161c2c;
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: #f8fafc;
    font-size: 0.75rem;
    font-family: 'JetBrains Mono', monospace;
    padding: 0.25rem 0.35rem;
    border-radius: 4px;
    width: 100%;
    text-align: center;
    outline: none;
}
.axis-input:focus {
    border-color: #06b6d4;
}

.color-picker-combo {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    flex: 1;
}
.color-picker-combo input[type="color"] {
    width: 28px;
    height: 28px;
    border: none;
    border-radius: 5px;
    background: transparent;
    cursor: pointer;
}
.hex-input {
    font-family: 'JetBrains Mono', monospace;
    text-transform: uppercase;
}
.swatches-quick-row {
    display: flex;
    gap: 0.35rem;
    flex-wrap: wrap;
    margin-top: 0.2rem;
}
.swatch-mini {
    width: 22px;
    height: 22px;
    border-radius: 4px;
    border: 1px solid rgba(255, 255, 255, 0.2);
    cursor: pointer;
    transition: transform 0.15s;
}
.swatch-mini:hover {
    transform: scale(1.15);
}
.slider-num {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.72rem;
    color: #22d3ee;
    min-width: 38px;
    text-align: right;
}

/* Code Editor Tab */
.code-editor-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    padding-bottom: 0.4rem;
}
.file-tag {
    font-size: 0.75rem;
    color: #22d3ee;
    font-family: 'JetBrains Mono', monospace;
    font-weight: 700;
}
.code-actions {
    display: flex;
    gap: 0.35rem;
}
.mini-btn {
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.12);
    color: #cbd5e1;
    font-size: 0.72rem;
    font-weight: 600;
    padding: 0.25rem 0.55rem;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.15s;
}
.mini-btn:hover {
    background: rgba(6, 182, 212, 0.15);
    color: #22d3ee;
    border-color: #06b6d4;
}
.mini-btn.compile-btn {
    background: #06b6d4;
    color: #041017;
    font-weight: 700;
    border-color: #22d3ee;
}
.txs-code-textarea {
    width: 100%;
    height: 480px;
    background: #070a12;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    color: #a5f3fc;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 0.82rem;
    line-height: 1.5;
    padding: 0.75rem;
    resize: none;
    outline: none;
    tab-size: 4;
}
.txs-code-textarea:focus {
    border-color: #06b6d4;
    box-shadow: 0 0 12px rgba(6, 182, 212, 0.25);
}
.code-status-bar {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    font-size: 0.72rem;
    color: #10b981;
    font-family: 'JetBrains Mono', monospace;
}
.status-indicator-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #10b981;
    box-shadow: 0 0 8px #10b981;
}

@media (max-width: 1100px) {
    .workbench-body {
        grid-template-columns: 1fr;
        height: auto;
    }
    .viewport-canvas-container {
        min-height: 450px;
    }
    .panel-left, .panel-right {
        border: none;
        border-top: 1px solid rgba(255, 255, 255, 0.08);
    }
}
"#
}
