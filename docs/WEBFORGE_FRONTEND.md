# TechScript WebForge: Frontend Framework Manual
**Accessible, Responsive, Dark-First Web User Interfaces**

---

## 1. Overview

WebForge Frontend is a declarative UI system compiled directly from TechScript. It eliminates the complexity of virtual DOMs, JSX tooling, and heavy JavaScript frameworks by emitting clean, semantic HTML5, high-performance CSS, and native JavaScript event listeners.

---

## 2. Design Tokens & Visual Aesthetics

WebForge uses an engineered, dark-first visual language tailored for developer tools and modern web products:

### Color Palette
- **Canvas Base (`--bg-base`):** `#090A0F` — Ultra-deep near-black canvas.
- **Surface Elevation (`--bg-surface`):** `#121520` — High-contrast container surface.
- **Surface Border (`--border-color`):** `#1E2333` — Subtle border delineation.
- **Primary Accent (`--accent-primary`):** `#6366F1` — Electric indigo for buttons and focus rings.
- **Secondary Accent (`--accent-cyan`):** `#06B6D4` — Vivid cyan for highlights and tags.
- **Tertiary Accent (`--accent-violet`):** `#8B5CF6` — Purple gradient accents.
- **Text Main (`--text-main`):** `#F3F4F6` — High-readability white/grey.
- **Text Muted (`--text-muted`):** `#9CA3AF` — Subtitle and placeholder grey.

### Typography
- Primary font: `Outfit`, `Inter`, system-ui, sans-serif.
- Code & monospace: `JetBrains Mono`, `Fira Code`, monospace.

---

## 3. UI Component Architecture

Every component is defined declaratively within a `page` block.

### Headers and Text
```techscript
page Home:
    show heading "Welcome to TechScript WebForge"
    show text "Build real websites without writing complex JavaScript."
```

### Cards & Layout Containers
```techscript
page Home:
    show card "Feature Highlights":
        show text "• Zero configuration setup"
        show text "• Embedded SQLite storage"
        show text "• Real Three.js 3D graphics"
```

### Forms & Interactive Controls
```techscript
page Home:
    show input "Enter task description" called TaskInput
    show button "Add Task" called AddTaskBtn
```

---

## 4. Client-Side Interactivity

Interactivity is wired through simple `when` triggers:

### Dynamic Toast Messages
```techscript
when AddTaskBtn is clicked:
    show message "Task added successfully!"
```

### API Submissions
```techscript
when AddTaskBtn is clicked:
    send TaskInput to /api/tasks
    show message "Persisted to SQLite database!"
```

---

## 5. Responsive Design & Accessibility

WebForge guarantees accessible output:
1. **WCAG AA Contrast:** High contrast text ratios on all dark surfaces.
2. **Keyboard Navigation:** Native `<button>` and `<input>` elements with clear focus outlines (`outline: 2px solid #6366F1`).
3. **Responsive Breakpoints:**
   - Desktop: `1200px+`
   - Tablet: `768px – 1199px`
   - Mobile: `< 768px` (Fluid single-column layouts).
