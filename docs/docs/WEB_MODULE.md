# TechScript v2 — Web Module Guide

Build full websites using only TechScript — no HTML, CSS, JavaScript, or React needed!

---

## The Basic Pattern

Every TechScript web app follows this simple 4-step pattern:

```techscript
use web                          # 1. Load web module

make page = WebPage("Title")     # 2. Create your page

page.body([ ... ])               # 3. Add your content

page.run()                       # 4. Open in browser
```

---

## Styling (replaces CSS)

```techscript
# Basic style
page.style("body", { "background": "#000", "color": "#fff" })

# Style a class
page.style(".card", {
    "padding": "20px",
    "border-radius": "10px",
    "background": "#1a1a1a"
})

# Style on hover
page.style(".card:hover", { "transform": "scale(1.02)" })
```

---

## Building the Page (replaces HTML)

```techscript
page.body([
    page.h1("Main Title"),           # <h1>
    page.h2("Sub Title"),            # <h2>
    page.p("Paragraph text"),        # <p>
    page.div([                       # <div>
        page.p("Inside a box")
    ], { "class": "card" }),
    page.button("Click Me", { "onclick": "doSomething()" }),
    page.span("Inline", { "style": "color:red" })
])
```

---

## Adding JavaScript (replaces React/Vue)

```techscript
page.script("""
    let count = 0;
    
    function increment() {
        count += 1;
        document.getElementById('num').textContent = count;
    }
""")

page.body([
    page.raw('<span id="num">0</span>'),
    page.button("Add +1", { "onclick": "increment()" })
])
```

---

## Fetching Live Data (replaces PHP/backend)

```techscript
page.script("""
    async function loadData() {
        const res = await fetch('https://api.example.com/data');
        const json = await res.json();
        document.getElementById('result').textContent = json.value;
    }
    loadData();
""")

page.body([
    page.h2("Live Data:"),
    page.raw('<p id="result">Loading...</p>')
])
```

---

## Advanced: Inject Raw HTML

When you need something specific, drop raw HTML directly:

```techscript
page.raw('<video src="video.mp4" controls></video>')
page.raw('<iframe src="https://youtube.com/embed/..." width="560" height="315"></iframe>')
```

---

## Custom Port

```techscript
page.run()        # Default: free port, opens browser
page.run(3000)    # Use port 3000
```

---

## Full Example

```techscript
use web

make page = WebPage("My Portfolio")

page.style("body", {
    "font-family": "sans-serif",
    "background": "#0f0f11",
    "color": "#eee",
    "text-align": "center",
    "padding": "40px"
})

page.script("""
    function downloadCV() {
        alert('CV download started!');
    }
""")

page.body([
    page.h1("Hi, I am Alex"),
    page.p("A TechScript developer."),
    page.button("Download CV", { "onclick": "downloadCV()" })
])

page.run()
```
