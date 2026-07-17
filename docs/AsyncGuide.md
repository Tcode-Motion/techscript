# TechScript 2.0 Async Guide

TechScript supports cooperative multitasking using futures and the `await` operator.

## Cooperative Multi-tasking

Async tasks are spawned using `std.async.spawn_async`. They yield cooperative execution back to the scheduler, which ticks whenever an `await` instruction is met on a pending future.

## Example

```techscript
import std.async;
import std.io;

function task_func() {
    std.io.print("Async task running...");
    return 42;
}

function main() {
    let future = std.async.spawn_async(task_func);
    std.io.print("Awaiting future...");
    let result = await future;
    std.io.print("Result:", result);
}

main();
```
