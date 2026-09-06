# SigmaOS AI Agent Buffer Overrun Management Directive (`AGENTS_BUFFER_OVERRUN.md`)

This document defines technical directives, memory boundary guardrails, and audit guidelines for AI agents managing buffer overrun and off-by-one mitigation in SigmaOS.

---

## 1. Zero-Trust Buffer Overrun Mitigation Directives

Buffer overrun vulnerabilities occur when read or write indexing operations exceed destination array or slice boundaries (including off-by-one errors in null-terminated strings or loop bounds). AI agents modifying kernel, driver, or C-compatibility code in SigmaOS must observe the following rules:

1. **Strict Slice Indexing & Bounds Checking:**
   - Avoid direct unchecked array indexing (`buf[i]`). Use safe Rust methods such as `.get(i)`, `.get_mut(i)`, or explicit bounds assertions prior to indexing.
   - For C string operations (`klib::ffi`), use bounds-aware helpers (`cstrlen`, `cstrcmp`, `rust_string_to_cstr`) that enforce maximum length termination and guard against buffer read overruns.

2. **Off-By-One Protection in Iterators & Loops:**
   - Ensure loop counters check strict strict-inequality bounds (`i < capacity`) rather than non-strict inequality (`i <= capacity`).
   - Validate array and vector capacity before bulk copy operations (`copy_from_slice`, `memcpy` wrappers).

3. **Hardened Guard Page Isolation (`alloc_with_guard_page`):**
   - Critical dynamic allocations must sandwich memory regions between inaccessible guard pages (`alloc_with_guard_page`).
   - Thread stacks must enable stack clash guard zones (`has_guard_page = true`) to trap sequential overrun attempts immediately.

4. **Ring Buffer Overflow Handling:**
   - Ring buffers (`RingBuffer`, `RingBuf`) must check capacity limits before enqueueing and handle index wrap-around using modulo arithmetic (`idx % capacity`) or bitwise masking (`idx & (capacity - 1)` for power-of-two sizes).

---

## 2. Pre-Commit Buffer Overrun Audit Checklist

Before submitting code modifications, AI agents must verify:
- [ ] Array or string indexing operations check bounds explicitly or use safe option-returning primitives.
- [ ] Null-terminated C string helper conversions specify maximum scan limits to prevent infinite read overruns.
- [ ] Off-by-one loop conditions are thoroughly audited.
- [ ] `./run_sigma_tests.sh` executes with 100% test pass rate.
