# Asynchronous Programming in TechScript

TechScript features a high-performance event loop and cooperative concurrency based on `async` and `await`.

---

## 🏗️ Async Functions
Declaring a function with `async` makes it return a `Future` immediately instead of running blockingly:

```txs
async do fetch_user_data(user_id)
    # Simulate network call using sleep
    sleep(100) 
    send { "id": user_id, "name": "Alice" }
end
```

---

## 🧬 Awaiting Futures
To extract the resolved value of a `Future`, use the `await` keyword:

```txs
# This suspends the current async context until the future resolves
user_future = fetch_user_data(42)
user = await user_future
say user["name"] # "Alice"
```

---

## 🔁 Running Concurrent Tasks
You can launch multiple async calls concurrently and await them all:

```txs
async do main()
    future_a = fetch_user_data(1)
    future_b = fetch_user_data(2)
    
    # Both calls run concurrently; await their resolutions
    user_a = await future_a
    user_b = await future_b
    
    say $"{user_a['name']} and {user_b['name']} loaded."
end

await main()
```
For true parallelism utilizing CPU cores, see [Multithreading Guide](multithreading.md).
