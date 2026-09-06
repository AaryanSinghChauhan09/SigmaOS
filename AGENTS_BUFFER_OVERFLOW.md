# SigmaOS AI Agent Buffer Overflow Management Directive (`AGENTS_BUFFER_OVERFLOW.md`)

This document defines technical directives, memory protection protocols, and audit procedures for AI agents tasked with buffer overflow mitigation and memory safety management in SigmaOS.

---

## 1. Zero-Trust Memory Safety Directives

All AI agents operating on or modifying SigmaOS kernel, drivers, or userspace components must enforce strict zero-trust memory safety:

1. **No Unsafe Buffer Operations:**
   - Avoid raw pointer arithmetic, unaligned casts, and unchecked slicing.
   - Use Rust's native bounds-checked slices, `alloc::vec::Vec`, `RingBuffer`, and safe iterator primitives.
   - Verify that all fixed-size buffer indexing operations explicitly handle potential overflow or out-of-bounds conditions.

2. **Hardened Guard Page Allocations (`alloc_with_guard_page`):**
   - Ensure dynamic allocations requiring isolation allocate guard pages around data boundaries using `alloc_with_guard_page`.
   - Guard pages must be configured as inaccessible (`stack_guard_pages = 1`, `heap_guard_pages = 1`) to trigger immediate page faults upon sequential or off-by-one buffer overruns.

3. **ASLR / KASLR Entropy Verification (`SovereignCsprng`):**
   - Address Space Layout Randomization (ASLR) and Kernel ASLR (KASLR) offset generation must utilize `SovereignCsprng` entropy sources.
   - Ensure stack, heap, and memory-mapped bases are randomized on process/kernel init.

4. **Non-Executable Page Enforcement (NX / DEP / W^X):**
   - Memory pages must strictly conform to Write-XOR-Execute (`W^X`) policies.
   - Data buffers, stacks, and heap allocations must be mapped Non-Executable (`NX` / `DEP`).

---

## 2. Buffer Safety Verification Checklist for AI Agents

Before committing changes, AI agents must verify:
- [ ] No `unsafe` blocks introduce unchecked pointer dereferences or raw buffer writes.
- [ ] Ring buffers (`RingBuffer`, `RingBuf`) check overflow/wrap-around boundaries safely.
- [ ] Stack allocations are guarded against stack clash via guard pages (`has_guard_page = true`).
- [ ] `./run_sigma_tests.sh` executes without memory safety faults or assertions failures.
