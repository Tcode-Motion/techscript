# TechScript Language Cheat Sheet

A quick reference for TechScript v1.0.4. Save this for quick lookups!

---

## 📤 Output & Input

```techscript
say "Hello"               # Print text
say "Hi", name, "!"       # Print multiple values
say f"Hello {name}!"      # F-string (insert variable)
make name = ask "Name? "  # Read input from user
```

## 📦 Variables

```techscript
make x = 10              # Create variable
keep PI = 3.14159        # Create constant (can't change)
make name = "Alice"
make items = [1, 2, 3]   # List
make info = {"age": 25}  # Map/Dictionary
```

## 🔀 Conditions

```techscript
when x > 10 {
    say "Big"
} or when x == 10 {
    say "Exactly 10"
} else {
    say "Small"
}
```

## 🔁 Loops

```techscript
each item in [1, 2, 3] {
    say item
}

each i in 1..10 {    # Range 1 to 9
    say i
}

repeat x > 0 {       # While loop
    x -= 1
}
```

## 🔧 Functions

```techscript
build greet(name, greeting = "Hello") {
    say f"{greeting}, {name}!"
}

greet("Alice")             # Hello, Alice!
greet("Bob", "Hi")         # Hi, Bob!
```

## 🏗️ Classes

```techscript
model Dog {
    build init(self, name) {
        self.name = name
    }
    build speak(self) {
        say f"{self.name} says Woof!"
    }
}

make rex = Dog("Rex")
rex.speak()
```

## ⚠️ Error Handling

```techscript
attempt {
    # risky code here
} catch err {
    say f"Error: {err.message}"
}
```

---

## 🌐 use web — Build Websites

```techscript
use web
make page = web.page("My Site")
make body = page["body"]
make styles = page["styles"]

styles.append(web.style("h1", {"color": "#e94560"}))
body.append(web.h1("Hello World"))
body.append(web.p("Built with TechScript!"))
body.append(web.button("Click", {"onclick": "alert('Hi!')"}))

make path = web.build(page)
web.open(path)
```

**Elements:** `web.h1` `web.h2` `web.h3` `web.p` `web.div` `web.span` `web.button` `web.input` `web.img` `web.a` `web.ul` `web.table` `web.form`
**Layout:** `web.style` `web.css_class` `web.layout`
**Actions:** `web.build` `web.open` `web.serve`

---

## 🚀 use api — HTTP Server

```techscript
use api
say "Starting server..."
api.listen(3000)    # Starts on http://localhost:3000
```

**Functions:** `api.listen` `api.json` `api.text` `api.html` `api.status`

---

## 🖥️ use gui — Desktop Apps

```techscript
use gui
make win = gui.window("My App", 600, 400)
make els = win["elements"]
els.append(gui.label("Hello!"))
els.append(gui.button("Click Me", "alert('Hi!')"))
els.append(gui.textbox("Type here...", "input1"))
els.append(gui.dropdown(["Option 1", "Option 2"]))
gui.show(win)
```

**Widgets:** `gui.window` `gui.label` `gui.button` `gui.textbox` `gui.checkbox` `gui.dropdown` `gui.vbox` `gui.hbox` `gui.show`

---

## 🎮 use three_d — 3D Graphics

```techscript
use three_d
make s = scene.scene()
make objects = s["objects"]
objects.append(scene.box("#e94560", 1.0))
objects.append(scene.sphere("#4ecdc4", 0.7))
objects.append(scene.torus("#ffd93d"))
scene.render(s)
```

**Shapes:** `scene.box` `scene.sphere` `scene.cylinder` `scene.plane` `scene.torus`
**Setup:** `scene.scene` `scene.light` `scene.render`

---

## 🎬 use anime — Animations

```techscript
use anime
make anim = anime.create()
make els = anim["elements"]
els.append(anime.target("div", "", "anim-el"))
make anims = anim["animations"]
anims.append(anime.animate(".anim-el", {"translateX": "250", "duration": "2000"}))
anime.render(anim)
```

**Functions:** `anime.create` `anime.target` `anime.animate` `anime.timeline` `anime.stagger` `anime.render`

---

## 🔍 use debug — Debugging Tools

```techscript
use debug
debug.trace(myVar)           # Show value + type
debug.inspect(myList)        # Deep inspection
debug.timer_start("task")
# ... some code ...
debug.timer_end("task")      # Shows elapsed time
debug.assert(x > 0, "x must be positive")
debug.table(myList)          # Pretty-print as table
debug.log("INFO", "Server started")
debug.benchmark(1000000, "loop test")
```

---

## 📐 Built-in Functions

| Function | What it does |
|---|---|
| `len(x)` | Length of string/list |
| `range(n)` | List 0 to n-1 |
| `type(x)` | Get type of value |
| `int(x)` | Convert to integer |
| `str(x)` | Convert to string |
| `float(x)` | Convert to decimal |
| `abs(x)` | Absolute value |
| `round(x, n)` | Round to n places |
| `max(a, b)` | Maximum value |
| `min(a, b)` | Minimum value |

## 🔤 String Methods

```techscript
"hello".upper()       # "HELLO"
"HELLO".lower()       # "hello"
"hello world".split() # ["hello", "world"]
"  hi  ".trim()       # "hi"
"hello".replace("l", "r") # "herro"
"hello".contains("ell")   # true
```

## 📋 List Methods

```techscript
make nums = [3, 1, 2]
nums.append(4)
nums.sort()
nums.reverse()
nums.map((x) => x * 2)
nums.filter((x) => x > 2)
nums.length
```

## 📦 Built-in Modules

| Module | Namespace | Use |
|---|---|---|
| Math | `math.*` | Trig, sqrt, log, etc. |
| File System | `fs.*` | read, write, list_dir |
| OS | `os.*` | env, system, popen |
| Random | `random.*` | random, choice, uuid |
| JSON | `json.*` | encode, decode |
| Crypto | `crypto.*` | sha256, base64 |
| Date | `date.*` | now, unix, year |
