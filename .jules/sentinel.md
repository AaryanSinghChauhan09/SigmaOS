# Sentinel's Journal

## 2026-03-31 - [Subsystem Warning & Parameter Security Sanitization]
**Learning:** Prefixing unused syscall and performance tuning arguments with underscores (`_`) prevents compiler dead-code warnings while explicitly documenting unused parameters in security-sensitive kernel dispatcher entrypoints.
**Action:** Consistently sanitize unused function arguments in low-level syscall handlers and hardware tuning interfaces.
