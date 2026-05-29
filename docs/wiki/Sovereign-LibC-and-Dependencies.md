# Sovereign LibC & Dependency Reduction 🔒⚙️

One of SigmaOS's core sovereignty principles is **eliminating hidden dependencies**. Kernel-space code must never call `malloc`, `printf`, `strcpy`, or any glibc/musl symbol. Instead, the entire runtime surface is provided by the **Sovereign LibC** (`kernel/libc/sigma_libc_impl.c`), a hand-written C library with zero external linkage.

---

## 🎯 Design Philosophy

> *"Don't trust what you didn't build."*

| Concern | Standard OS | SigmaOS |
|---|---|---|
| Memory allocator | glibc `malloc` (dlmalloc-based, opaque) | `sigma_malloc` — bump + free-list, fully auditable |
| String ops | glibc `strcpy`, `memcpy` (SIMD, hidden codegen) | Inline sovereign implementations, predictable |
| Console output | `printf` → FILE* → glibc → kernel | `sys_print` → raw `write(2)` syscall, no FILE* |
| Arch support | Conditional glibc ABI | Explicit `#ifdef __x86_64__` / `#ifdef __aarch64__` |

**Inspired by:**
- **musl libc** (Alpine Linux): Lean, spec-compliant, no hidden surprises.
- **diet libc**: Radically minimal for embedded targets.
- **Linux kernel `lib/`**: In-tree string helpers used where libc is unavailable.
- **BusyBox**: Replacing heavy glibc calls with inline helpers throughout.

---

## 📦 The Sovereign LibC Surface

Declared in [`include/sigma_libc.h`](../include/sigma_libc.h), implemented in [`kernel/libc/sigma_libc_impl.c`](../kernel/libc/sigma_libc_impl.c).

### Memory

```c
void*  sigma_malloc(sigma_size_t size);   // First-fit free-list allocator
void   sigma_free(void* ptr);             // Block reuse (zero-on-free for security: call sigma_memset first)
void*  sigma_memset(void* dst, sigma_u8 val, sigma_size_t n);
void*  sigma_memcpy(void* dst, const void* src, sigma_size_t n);
void*  sigma_memmove(void* dst, const void* src, sigma_size_t n); // Handles overlap
int    posix_memalign(void** ptr, sigma_size_t align, sigma_size_t size);
```

The allocator uses a **2 MiB sovereign heap** (`g_heap`) baked into the BSS segment at link time. On production hardware, this will be replaced by `sigma_page_alloc()` from the kernel's buddy allocator.

### Strings

```c
sigma_size_t sigma_strlen(const char* s);
int          sigma_strcmp(const char* a, const char* b);
int          sigma_strncmp(const char* a, const char* b, sigma_size_t n);
char*        sigma_strcpy(char* dst, const char* src);
char*        sigma_strncpy(char* dst, const char* src, sigma_size_t n);  // Always null-terminates
char*        sigma_strcat(char* dst, const char* src);
const char*  sigma_strchr(const char* s, char c);
const char*  sigma_strstr(const char* haystack, const char* needle);
```

### Number Conversion & Math

```c
sigma_i32 sigma_atoi(const char* str);            // Handles sign, whitespace
char*     sigma_itoa(sigma_i32 v, char* buf, sigma_u32 base); // Base 2–16
sigma_i32 sigma_abs(sigma_i32 val);
```

### Console Output

```c
void sys_print(const char* fmt, ...);  // Supports: %s %d %u %x %c %%
```

`sys_print` calls `sigma_vsnprint` internally (our varargs formatter), then issues a **raw `write(2)` syscall** via inline assembly — no `FILE*`, no buffering, no glibc. Both x86_64 and ARM64 assembly paths are implemented.

---

## 🔧 Dependency Reduction Rules

The following rules are enforced across the entire SigmaOS codebase:

> [!IMPORTANT]
> **Rule 1:** `#include <string.h>`, `<stdlib.h>`, `<stdio.h>`, or `<glibc/*>` are **forbidden** in kernel and driver code. Use `sigma_libc.h` equivalents.

> [!IMPORTANT]
> **Rule 2:** `printf`, `malloc`, `free`, `strcpy`, `memcpy` from external libs are **forbidden**. Use `sys_print`, `sigma_malloc`, `sigma_free`, `sigma_strcpy`, `sigma_memcpy`.

> [!TIP]
> **Rule 3:** For userland code, a minimal musl-compatible shim layer is acceptable, but it must wrap sovereign primitives — not call glibc directly.

> [!WARNING]
> **Rule 4:** `sys_print` must **never** be called from interrupt context (IRQ handlers, exception vectors). Use the ring-buffer logger (`sigma_log.h`) instead.

---

## 🛡️ Security Benefits

- **No hidden glibc allocator state**: No `ptmalloc` bins that can be heap-groomed.
- **Audit trail**: Every byte of `sigma_malloc`/`sigma_free` is traceable.
- **No format string exploits**: `sys_print` handles only known format specifiers — unknown `%` tokens emit `?`.
- **Heap corruption detection**: Each block carries a `0xSIGMA5A5` magic cookie; `sigma_free` validates it before marking the block free.

---

## 🚀 Roadmap

| Milestone | Status |
|---|---|
| Bump allocator + free-list | ✅ Done |
| Full string library | ✅ Done |
| `sys_print` (x86_64 + ARM64 inline asm) | ✅ Done |
| Replace bump allocator with buddy/slab | 🔲 Planned (Phase 6) |
| WASM / WASI sandboxing via sigma_wasi.h | 🔲 Planned (Phase 6) |
| Formal verification of allocator safety | 🔲 Planned (Phase 7) |
