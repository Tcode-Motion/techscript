# TechScript Language Specification (v1.0.5)

TechScript is a highly readable, dynamically typed but gradually typed, system-level scripting language written in pure Rust.

## Types

TechScript supports the following native data types:
* `int`: 64-bit signed integer (`42`, `0xFF`, `0b1010`)
* `float`: 64-bit floating point (`3.14`)
* `string`: UTF-8 string, optionally f-strings (`"Hello"`, `f"Hello {name}"`)
* `bool`: Boolean (`true` or `false`)
* `none`: The null type (`none`)
* `list`: dynamic array (`[1, 2, 3]`)
* `map`: hash map (`{"key": "value"}`)
* `function`: First-class callable objects
* `class`, `instance`: Object-oriented constructs

## Core Syntax

### Variables
```techscript
make x = 10         # Mutable by default
keep PI = 3.1415    # Immutable constant
make y: int = 20    # Gradual typing annotation
```

### Functions
```techscript
build greeting(name: string) {
    say f"Hello, {name}!"
    send true
}
```

### Async & Concurrency
```techscript
async build fetch_data() {
    # Async background logic
}
make data = await fetch_data()

# Fire and forget concurrent thread
spawn fetch_data()
```

### Control Flow
```techscript
when x > 10 {
    say "Large"
} elif x == 10 {
    say "Exact"
} else {
    say "Small"
}

repeat x < 20 {
    x = x + 1
}

each item in [1, 2, 3] {
    say item
}
```

### Pattern Matching
```techscript
match value {
    case 1 { say "One" }
    case 2 { say "Two" }
    case _ { say "Other" }
}
```

### Object Oriented
```techscript
model Person {
    build init(self, name) {
        self.name = name
    }
    build greet(self) {
        say f"Hi, I am {self.name}"
    }
}
```

### Error Handling
```techscript
attempt {
    make content = read_file("missing.txt")
} catch error {
    say f"Failed: {error}"
} finally {
    say "Cleanup"
}
```
