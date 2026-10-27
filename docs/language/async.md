# Asynchronous Programming in TechScript

TechScript features a high-performance event loop and cooperative concurrency based on `async` and `await`.

---

## 🏗️ Async Functions
Declaring a function with `async` makes it return a `Future` immediately instead of running blockingly:

```txs
async to fetch_user_data(user_id):
    # Simulate network call using sleep
    sleep(100)
    return { "id": user_id, "name": "Alice" }
```

---

## 🧬 Awaiting Futures
To extract the resolved value of a `Future`, use the `await` keyword:

```txs
# This suspends the current async context until the future resolves
user_future is fetch_user_data(42)
user is await user_future
say "{user['name']}" # "Alice"
```

---

## 🔁 Running Concurrent Tasks
You can launch multiple async calls concurrently and await them all:

```txs
async to main():
    future_a is fetch_user_data(1)
    future_b is fetch_user_data(2)
    
    # Both calls run concurrently; await their resolutions
    user_a is await future_a
    user_b is await future_b
    
    say "{user_a['name']} and {user_b['name']} loaded."
    return "done"

await main()
```
For true parallelism utilizing CPU cores, see [Multithreading Guide](concurrency.md).
