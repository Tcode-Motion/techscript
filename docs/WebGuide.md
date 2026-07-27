# Web Application Builder Guide

The `web` module allows developers to create full-stack, responsive websites natively using only TechScript.

---

## 🏗️ Structure of a Web Page
Import `use web` and instantiate the `WebPage` model. Define layouts declaratively without writing HTML or CSS:

```txs
use web

# Create a page instance
page = WebPage("Home Page")

# Apply styles directly
page.style("body", {
    "background-color": "#0f0f11",
    "color": "#eeeeee",
    "font-family": "sans-serif",
    "padding": "40px"
})

# Define script methods (runs on browser client)
page.script("""
    do alert_hello()
        alert("Hello from TechScript!")
    end
""")

# Define document structure
page.body([
    page.h1("Welcome to My Website!"),
    page.p("This page is generated entirely in TechScript."),
    page.button("Click Me!", { "onclick": "alert_hello()" })
])

# Launch the page in the browser
page.run()
```

---

## 🧬 Supported Elements
The `WebPage` class exposes methods for creating standard elements:
* `page.h1(text)` to `page.h6(text)`: Heading elements.
* `page.p(text)`: Paragraph blocks.
* `page.button(text, attributes)`: Interactive buttons.
* `page.input(attributes)`: Text input elements.
* `page.div(children, attributes)`: Group layouts.

---

## 🚀 Execution & Hot-Reloading
When you run a web script using `tech run site.txs`, the engine spins up an HTTP server on port `8080` and opens your default browser. The server watches files and reloads automatically when changes are saved.
