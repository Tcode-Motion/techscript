## 2026-08-19 - Removed redundant instruction array lookup in VM loop
**Learning:** The inner loop of the VM interpreter (`execute_loop`) had an expensive, redundant deep indexing operation to fetch `inst_operands` which was already available on the `inst` reference. Re-fetching it via `self.module.functions[...].chunk.instructions[...].operands` adds unnecessary bounds checks and pointer chasing in the hottest part of the VM.
**Action:** Always prefer using existing local references over redundant deep lookups, especially in tight loops like an interpreter fetch-decode-execute loop.
