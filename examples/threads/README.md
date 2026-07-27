# Threads & Parallel Example

This example demonstrates how to use the `parallel` block to run multiple functions concurrently in distinct OS worker threads, automatically waiting for all of them to complete before continuing.

## Code (`threads.txs`)
```txs
do task_a()
    sleep(100)
    say "Task A completed on worker thread."
end

do task_b()
    sleep(50)
    say "Task B completed on worker thread."
end

say "Starting parallel block execution..."
parallel
    task_a()
    task_b()
end
say "Parallel execution finished. All threads joined main thread."
```

## Running the Example
```bash
tech run threads.txs
```

## Expected Output
```
Starting parallel block execution...
Task B completed on worker thread.
Task A completed on worker thread.
Parallel execution finished. All threads joined main thread.
```
*(Note: Task B prints first because it has a shorter sleep time than Task A).*
