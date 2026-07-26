# TechScript 2.0 Async Guide

> **Status**: Frozen Specification — 2.0.0 Stable
> **Last Updated**: 2026-07-26

TechScript 2.0 supports cooperative multitasking and parallel block execution
directly at the language level. This guide details how to declare async tasks,
await results, and run code in parallel.

---

## 1. Cooperative Multi-tasking

Async functions are declared with the `async do` prefix. Calling an async
function returns a **Future** object representation instead of executing the body
immediately. To wait for the execution to complete and fetch the return value,
use the `await` keyword.

### Declaring and Awaiting Tasks

```txs
use http
use json

async do fetch_user(user_id)
    say $"Fetching user {user_id}..."
    response = await http.get($"https://api.example.com/users/{user_id}")
    
    when response.status != 200
        throw $"Failed to load user: {response.status}"
    end
    
    send json.parse(response.body)
end

async do main()
    try
        user = await fetch_user(42)
        say $"Loaded user: {user["name"]}"
    catch err
        say $"Error: {err}"
    end
end

await main()
```

---

## 2. Parallel Execution Blocks

To run multiple independent operations concurrently, use the `parallel` block.
The runtime schedules all operations inside a `parallel` block concurrently
across available threads (or multiplexes them cooperatively on the single-threaded
event loop depending on the target compilation model).

A `parallel` block halts execution of the enclosing scope until **all** nested
statements inside it have finished executing.

```txs
do fetch_assets()
    say "Loading assets..."
    
    parallel
        load_images()
        load_audio()
        load_fonts()
    end
    
    say "All assets loaded successfully."
end
```

### Capturing Parallel Results

To collect values from concurrent tasks, assign them inside the parallel block
to variables declared in the outer scope:

```txs
do load_data()
    user_data = null
    posts_data = null
    
    parallel
        user_data = fetch_user_profile()
        posts_data = fetch_recent_posts()
    end
    
    # Execution resumes here only after both fetches complete
    render_page(user_data, posts_data)
end
```

---

## 3. Deprecated Async Syntax (1.x Legacy)

The following 1.x patterns are deprecated and will emit `TSW1xxx` warnings.
They should be migrated to canonical 2.0 form:

- `import std.async;` → `use async`
- `function main()` / `build main()` / `fun main()` → `do main()`
- `return value` / `give value` → `send value`
- `std.async.spawn_async(func)` → `async do` functions
- Semicolons `;` and curly braces `{}` → Newlines and `end` blocks
