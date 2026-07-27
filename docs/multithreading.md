# Multithreading and Concurrency

TechScript uses real operating system threads and parallel blocks to achieve multi-core hardware execution.

---

## 🏗️ The `parallel` Block
The `parallel` block schedules statements to run concurrently on separate threads:

```txs
parallel
    do_heavy_comp_1()
    do_heavy_comp_2()
    do_heavy_comp_3()
end
# Execution blocks here until all three tasks finish
say "All processing threads completed."
```

---

## 🧬 Thread Spawning
To spin off a task in a background thread that does not block current execution, use the `thread` module:

```txs
use thread

handle = thread.spawn(do()
    loop 5
        say "Working in background..."
        sleep(500)
    end
end)

# Continue working in main thread
say "Main thread is still responsive."

# Wait for background thread if desired
thread.join(handle)
```

---

## 🔒 Shared State and Synchronization
To pass data between threads safely, use Channels or Mutexes provided by the standard library:

```txs
use sync

# Create a thread-safe mutex wrapper
counter = sync.mutex(0)

parallel
    sync.lock(counter)
    # Increment safely
    sync.set(counter, sync.get(counter) + 1)
    sync.unlock(counter)
end
```
See [Memory Model](memory-model.md) for data transfer rules.
