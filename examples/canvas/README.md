# Canvas Example

This example demonstrates how to initialize the viewport and render basic geometric shapes and text using the `canvas` module in TechScript.

## Code (`draw.txs`)
```txs
use canvas

say "Initializing canvas viewport..."
canvas.init(800, 600)

say "Drawing rectangles and shapes..."
canvas.draw_rect(0, 0, 800, 600, "#000000") # Background
canvas.draw_circle(400, 300, 100, "#0DF28B") # Core circle
canvas.draw_text("TechScript Canvas Engine", 50, 50, 24, "#FFFFFF")

say "Canvas drawings completed successfully."
```

## Running the Example
```bash
tech run draw.txs
```

## Expected Output
```
Initializing canvas viewport...
Drawing rectangles and shapes...
Canvas drawings completed successfully.
```
