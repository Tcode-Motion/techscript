# TechScript 1.0 to 2.0 Migration Guide

TechScript 2.0 introduces breaking compiler changes, stricter type validations, and the async event loop task runner.

## 1. Async Await
In 1.0, async behaviors were handled via callback functions. In 2.0, use the native prefix `await` operator:

```diff
-fetch_data(function(data) { print(data); });
+let data = await fetch_data_future();
+print(data);
```

## 2. Capability Sandboxing
All standard library modules executing operating system requests (e.g. `std.fs`, `std.net`, `std.process`) require declaration in `tech.toml`:

```toml
[package]
capabilities = ["FileSystem", "Network"]
```
