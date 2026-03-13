# TechScript AI Animation Guide (v1.1)

This guide is designed for AI models (LLMs) to understand, generate, and improve TechScript (`.txs`) animation code using the `anime` module.

## 1. Core Syntax Overview

TechScript uses the `use` keyword for modules and `make` for variables. Multi-line strings should use triple quotes `"""`.

```techscript
use anime

# Step 1: Create a workspace
make anim = anime.create()

# Step 2: Add styles (Optional but Recommended)
anim["styles"].append("""
  .star { width: 50px; height: 50px; background: cyan; clip-path: polygon(...); }
""")

# Step 3: Add elements
anim["elements"].append("<div class='star'></div>")

# Step 4: Define Animation Logic
# anime.animate(selector, config_map)
make config = {
  "scale": 1.5,
  "duration": 1000,
  "loop": true,
  "direction": "alternate",
  "easing": "easeInOutSine"
}
anim["animations"].append(anime.animate(".star", config))

# Step 5: Render
anime.render(anim)
```

## 2. API Reference (`anime` module)

| Function | Description |
| :--- | :--- |
| `anime.create()` | Initializes a workspace map with `elements`, `animations`, `styles`, and `scripts`. |
| `anime.target(tag, text, class)` | Returns a string of HTML (e.g., `<div class='...'>...</div>`). |
| `anime.animate(selector, map)` | Generates a JavaScript string for the animation configuration. |
| `anime.stagger(ms)` | Creates a staggered delay for multiple elements. |
| `anime.render(workspace)` | Generates the HTML file and launches the local viewer. |

## 3. Supported Animation Properties

Pass these in the configuration map to `anime.animate`:

- **Transform**: `translateX`, `translateY`, `rotate`, `rotateX`, `rotateY`, `scale`, `scaleX`, `scaleY`, `skew`.
- **Style**: `opacity`, `backgroundColor`, `borderRadius`, `filter` (e.g., `"blur(5px)"`), `boxShadow`.
- **Control**: `duration` (ms), `delay` (ms), `easing`, `loop` (bool/int), `direction` (`"alternate"`, `"reverse"`).

**Easings**: `linear`, `easeInOutSine`, `easeOutExpo`, `easeOutElastic(1, .5)`, `easeInOutBack`.

## 4. Advanced: The Studio Pattern

To make interactive interfaces, append raw JavaScript to `anim["scripts"]`:

```techscript
anim["scripts"].append("""
  function customBehavior() {
    // Standard JS here, techscript will bundle it
    console.log("Animation Interactivity Active");
  }
  customBehavior();
""")
```

---

## 5. 🚀 AI Prompt Template (Copy & Paste)

**Use the following prompt when asking an AI to improve your TechScript code:**

> "I am working with TechScript (a Rust-based VM for web animations). Below is my current `.txs` file. Please improve it by:
> 1. Adding premium CSS (Glassmorphism, Neon Glows, or Modern Gradients).
> 2. Using more complex `anime.js` properties like staggered delays, elastic easings, or multi-property transitions.
> 3. Ensuring the UI looks high-fidelity and 'premium'.
> 4. output ONLY the full TechScript code.
> 
> My Code:
> [PASTE YOUR CODE HERE]"

---

## 6. Pro-Tips for AI
- **Quoting**: Don't quote `anime.stagger()` in the config map; the TechScript runtime handles unquoting for `anime.` prefixed calls.
- **Triple Quotes**: ALWAYS use `"""` for multi-line CSS or scripts to avoid lexing errors.
- **None**: Use `none` instead of `null`.
