# TechScript 2.0 Canvas & Graphics Guide

TechScript provides a standard drawing canvas library via the `graphics` module for rendering 2D vector graphics, charts, and shapes.

## Rendering Vector Graphics

To draw vector graphics, create a canvas and draw shapes:

```txs
use graphics

# Initialize a new drawing canvas
canvas = graphics.Canvas(800, 600)

# Set brush styles
canvas.fill("#2E3440")
canvas.clear()

# Draw a rectangle
canvas.fill("#88C0D0")
canvas.rect(100, 100, 400, 300)

# Draw text
canvas.fill("#ECEFF4")
canvas.font("Outfit", 24)
canvas.text("TechScript 2.0 Vector Graphics", 120, 150)

# Draw circle
canvas.fill("#BF616A")
canvas.circle(600, 200, 80)

# Save output image
canvas.save("output.png")
```

## Integrating with UI / Web

The drawing canvas is designed to bind directly to standard UI frames or output to web canvas interfaces. See [GUI.md](GUI.md) for more information.
