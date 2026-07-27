# Canvas Drawing Guide

The `canvas` standard library module provides a simple, direct interface for drawing 2D shapes, text, and lines on screen or offscreen buffers.

---

## 🏗️ Initialization
Before drawing, you must initialize the canvas viewport coordinates:

```txs
use canvas

# Initialize canvas with: width, height
canvas.init(800, 600)
```

---

## 🎨 Drawing Shapes & Colors

All shapes accept color strings in standard hex formats (`"#000000"`, `"#DF28B0"`) or RGB formats.

### Rectangles
* **`canvas.draw_rect(x, y, width, height, color)`**: Draws a filled rectangle.
* **`canvas.draw_rect_outline(x, y, width, height, thickness, color)`**: Draws a rectangle outline.

```txs
# Draw a solid neon green square
canvas.draw_rect(50, 50, 200, 200, "#0DF28B")
```

### Circles
* **`canvas.draw_circle(cx, cy, radius, color)`**: Draws a filled circle.

```txs
# Draw a blue circle in the center
canvas.draw_circle(400, 300, 50, "#00A3FF")
```

### Lines
* **`canvas.draw_line(x1, y1, x2, y2, thickness, color)`**: Draws a line.

```txs
# Draw a diagonal line
canvas.draw_line(0, 0, 800, 600, 2, "#CCCCCC")
```

---

## 📝 Drawing Text
Draw styled labels to the canvas:
* **`canvas.draw_text(text, x, y, size, color)`**: Draws text on screen.

```txs
canvas.draw_text("TechScript Canvas Engine", 50, 500, 24, "#FFFFFF")
```
