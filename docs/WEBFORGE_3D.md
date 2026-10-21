# TechScript WebForge: 3D WebGL Graphics Guide
**Interactive 3D Web Experiences Powered by Three.js**

---

## 1. Overview

WebForge provides native support for interactive 3D web graphics. Instead of configuring WebGL contexts, camera matrices, perspective projections, shaders, and render loops manually in JavaScript, TechScript lets developers declare 3D scenes using clean English syntax.

Behind the scenes, WebForge compiles your declarative scene graph into an optimized Three.js WebGL canvas.

---

## 2. Declarative 3D Syntax

To embed an interactive 3D viewport on any page:

```techscript
page Home:
    show heading "Interactive 3D World"
    show 3D scene called World:
        add cube:
            color is "indigo"
            rotation speed is 2
        enable mouse controls
```

### Components of a 3D Block

1. **`show 3D scene called <Name>:`**
   Initializes an HTML `<canvas>` with an interactive WebGL rendering context, perspective camera, ambient lighting, and a directional light.
2. **`add <shape>:`**
   Creates a 3D mesh and adds it to the scene graph.
   - `cube`: A standard 3D box geometry.
   - `sphere`: A smoothed spherical geometry.
   - `plane`: A flat surface or floor.
3. **`color is "<color>":`**
   Assigns a material color (supports named colors like `"blue"`, `"indigo"`, `"cyan"`, or hex codes like `"#6366F1"`).
4. **`rotation speed is <number>:`**
   Sets the continuous animation rotation velocity along the X and Y axes in the render loop.
5. **`enable mouse controls:`**
   Wires interactive mouse and touch drag listeners to orbit and tilt the camera around the scene.

---

## 3. Modular 3D Scene Files (`.3txs`)

In larger projects, complex 3D scenes can be isolated in specialized `.3txs` source files:

```techscript
// src/scenes/hero_logo.3txs
show 3D scene called TechScriptLogo:
    add cube:
        color is "#6366F1"
        rotation speed is 1.5
    enable mouse controls
```

This keeps 3D math and layout concerns cleanly separated from page text and UI markup.

---

## 4. Responsive Resizing & WebGL Optimization

WebForge's Three.js runtime automatically manages:
- **Automatic Aspect Ratio Resizing:** Window resize events automatically update the camera aspect ratio and renderer dimensions (`renderer.setSize()`), preventing distortion.
- **Resource Cleanup:** Cleans up textures, geometries, and animation frame request IDs when navigating between views.
- **Hardware Acceleration:** Runs at 60 FPS utilizing the user's GPU via WebGL 2.0 / WebGL 1.0 fallback.
