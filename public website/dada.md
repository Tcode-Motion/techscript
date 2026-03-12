# TechScript Website Data Backup

## Global Information
- **Logo**: `/logo.png` (originally techscript-logo.png, recently restored to logo.png by user)
- **Brand Name**: TechScript
- **Taglines**: 
  - "A language built for humans." / "A high-performance language that reads like English."
  - "Powered by Rust. Built for Simplicity."
- **Social/External Links**:
  - GitHub: `https://github.com/Tcode-Motion/techscript`
  - YouTube: `https://youtube.com/@sachkaswitch`
  - Email: `mailto:contact@techscript.io`
  - Issue Tracker: `https://github.com/Tcode-Motion/techscript/issues`
- **Main Author**: Tanmoy (Tcode-Motion)
- **License**: MIT License
- **Version**: v1.0.2 Native Rust Runtime
- **Theme**: Dark theme (with a toggle for Light mode via context)

## Pages Structure & Links
1. `/` - Home
2. `/features` - Features (Deep Dive)
3. `/syntax` - Syntax Guide / Documentation
4. `/functions` - Standard Library built-in functions
5. `/examples` - Code Samples (Live Sandbox)
6. `/releases` - Distribution Hub (Changelogs)
7. `/about` - Project Story (The Architect)
8. `/contact` - Support & Contact
9. `/downloads` - Download Installer
10. `/login` - Builder Login (Mock)
11. `/admin` - Maintainer Zone (Release tool)

## Content by Page

### 1. Home (`/`)
- **Hero**:
  - "TechScript" (Heading)
  - Stats: 1M Cycles in 2.9s, 0 Dependencies, 4 Platforms, 64 Bit Arch
  - CTA: "Get Version 1.0.2" (links to `/releases`)
  - Commands: `pip install techscript-lang`, `tech run hello.txs`
- **Badges**: 🦀 Native Rust VM, Multi-Platform, MIT License, VS Code Extension, By Tcode-Motion
- **What is it?**:
  - "A language built for humans."
  - "Imagine telling a computer what to do using sentences that actually make sense. No brackets to hunt, no semicolons to forget."
  - "Blazing Fast" (0.00s cycle overhead)
  - "Web Native" (Build UIs without HTML)
  - "No Setup" (Single binary, no Python)
- **Code Snippet**:
  ```techscript
  make name = ask "Your name? "
  say f"Hello, {name}!"
  # No complex syntax.
  # No cryptic symbols.
  # Just TechScript.
  ```
- **Core Ecosystem Grid**:
  - Engineered for Modern Development.
  - Plain English Syntax (`say "Hello!"`), Native Rust VM (`Blazing Fast`), Web Builder (`use web`), OOP with Models (`model User { }`), Error Handling (`Safety First`), VS Code Extension (`v1.0.2 Extension`), Loops & Ranges (`1..1000`), REPL Environment (`tech repl`), Mobile Native (`📱 Android Support`)
- **Final CTA**: "Ready to Start?", "TechScript is open-source, free, and waiting for you."

### 2. Features (`/features`)
- **Heading**: Every Feature
- **Subtitle**: TechScript is designed to be readable like English, yet powerful enough to run a native VM, build web apps, and handle memory safely.
- **Items**:
  - English-like Syntax
  - Native Rust Virtual Machine (1,000,000 loops in under 2.9 seconds)
  - Web Builder Module (No HTML/CSS needed, auto-generates local server)
  - Models (OOP) (init builder, properties, methods)
  - Stack Unwinding & Error Handling
  - (More features likely present in the code)

### 3. Syntax (`/syntax`)
- Documentation for all keywords. Data is pulled from `src/data/syntaxData.ts` (contains ~200-300 keywords). Includes search and filtering by categories.

### 4. Functions (`/functions`)
- Standard library functions documentation. Also pulled from `src/data/syntaxData.ts`. Filtering function keywords vs language keywords. 300+ built-ins.

### 5. Examples (`/examples`)
- **Heading**: Code Examples. Live sandbox.
- **Snippets**:
  - 1. Hello World
  - 2. The FizzBuzz Challenge (Loops and conditions)
  - 3. Object Oriented Programming (Models and build init)
  - 4. Web Builder UI Module (Using `use web`, `WebPage`)

### 6. Releases (`/releases`)
- **Heading**: Distribution Hub. Official Releases.
- Currently highlights v1.0.2.

### 7. About (`/about`)
- **Heading**: Crafted by Tcode-Motion. The Architect.
- **Lead Developer**: Tanmoy. "Predict the future by coding it." ⚡
- **Projects**:
  - TechScript (Native Rust VM)
  - Project JARVIS (Local AI)
  - Stark OS (Dashboard)
  - 3D Experiences (WebGL)
  - AR Keyboard (OpenCV 200 WPM)
  - Satch Ka Switch (Indian education socio-economic audits)
- Pulls from `src/data/readmeData.ts` as well.

### 8. Contact (`/contact`)
- **Heading**: Connect & Report. Get in touch with the core team.
- **Methods**:
  - Bug Reports (GitHub Issues)
  - Community (YouTube Channel)
  - Source Code (GitHub Repository)
  - Direct Email (contact@techscript.io)

### 9. Downloads (`/downloads`)
- Download installers for Windows (.exe / .msi), Linux (.AppImage / .deb), macOS (.dmg). Platform auto-detection implemented.

### 10. Login / Admin
- Mocked login interfaces for managing deployed apps and pushing release artifacts (`releases.json`).

## Next Steps for Rewrite
- Separate all these hardcoded text fragments into a robust data layer (`src/data/siteContent.ts`).
- Ensure UI/UX remains pixel-perfect to the latest user revisions while components become generic, bug-free rendering engines mapping over the data layer.
