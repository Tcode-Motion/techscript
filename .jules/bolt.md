## 2024-05-24 - Optimized String Boundary Lookup with Byte Offsets
**Learning:** When searching for boundary characters using byte offsets in Rust strings (e.g., indices returned by `str::find`), avoid using `.chars().nth()` as it expects character indices and runs in O(N) time, which is slow for long lines and causes bugs with multi-byte Unicode characters.
**Action:** Slice the string using the byte index and use `.chars().next_back()` or `.chars().next()` (e.g., `line[..pos].chars().next_back()`) for safe, O(1) lookups in the future.
## 2024-05-24 - Optimized String Boundary Lookup with Byte Offsets
**Learning:** When searching for boundary characters using byte offsets in Rust strings (e.g., indices returned by `str::find`), avoid using `.chars().nth()` as it expects character indices and runs in O(N) time, which is slow for long lines and causes bugs with multi-byte Unicode characters.
**Action:** Slice the string using the byte index and use `.chars().next_back()` or `.chars().next()` (e.g., `line[..pos].chars().next_back()`) for safe, O(1) lookups in the future.
## 2024-05-24 - Optimized String Boundary Lookup with Byte Offsets
**Learning:** When searching for boundary characters using byte offsets in Rust strings (e.g., indices returned by `str::find`), avoid using `.chars().nth()` as it expects character indices and runs in O(N) time, which is slow for long lines and causes bugs with multi-byte Unicode characters.
**Action:** Slice the string using the byte index and use `.chars().next_back()` or `.chars().next()` (e.g., `line[..pos].chars().next_back()`) for safe, O(1) lookups in the future.
