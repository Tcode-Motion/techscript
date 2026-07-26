# TechScript 2.0 GUI Programming Guide

TechScript supports graphical user interface development using standard native widget systems.

## Basic Window Creation

```txs
use gui

# Create window
window = gui.Window("TechScript Hello", 400, 300)

# Add container
layout = gui.VLayout()
window.set_layout(layout)

# Add label
label = gui.Label("Welcome to TechScript GUI")
layout.add(label)

# Add button
btn = gui.Button("Click Me")
btn.on_click(do()
    label.set_text("Button clicked!")
end)
layout.add(btn)

# Show and block until closed
window.show()
```

## GUI Event Loops

GUI applications execute their main loop through `window.show()`, which is built on non-blocking native event dispatchers.
