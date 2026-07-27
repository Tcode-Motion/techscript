# Async / Await Example

This example demonstrates how TechScript uses async functions and the `await` keyword to run non-blocking, concurrent operations.

## Code (`async.txs`)
```txs
async do fetch_resource(resource_name)
    say $"[Async] Starting fetch for: {resource_name}"
    # Sleep to simulate network delay
    sleep(200)
    send $"Data: {resource_name} content"
end

async do run_main()
    say "Spawning resource futures..."
    future_a = fetch_resource("Config")
    future_b = fetch_resource("User Profile")
    
    say "Awaiting Config..."
    data_a = await future_a
    say $"[Done] {data_a}"
    
    say "Awaiting User Profile..."
    data_b = await future_b
    say $"[Done] {data_b}"
end

await run_main()
```

## Running the Example
```bash
tech run async.txs
```

## Expected Output
```
Spawning resource futures...
[Async] Starting fetch for: Config
[Async] Starting fetch for: User Profile
Awaiting Config...
[Done] Data: Config content
Awaiting User Profile...
[Done] Data: User Profile content
```
