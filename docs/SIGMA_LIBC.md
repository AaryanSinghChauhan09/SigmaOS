# SigmaOS Native Library — `sigma_libc` & `native_core`

> **Path:** `sigma_core/sigma_libc.py` & `sigma_core/native_core/`
> **Philosophy:** One custom library to rule them all. Zero third-party packages. Zero high-level standard library wrappers. Zero low-level `libc`, `glibc`, `msvcrt`, or `std` usage. Instead of using standard low-level language libraries, SigmaOS completely customizes them using raw syscalls across multiple architectures.

---

## Why a Custom OS-Specific Low-Level Library?

SigmaOS enforces a "bare-metal-to-userland" direct link. High-level languages like Python typically rely on `glibc` or C-standard wrappers. Low-level languages like C/C++/Rust typically rely on `#include <stdlib.h>` or `std::`. 

For extreme isolation, security, and low-latency metrics, SigmaOS provides its **own** definitions. We absorb features from Arch, Alpine, and Debian without bringing along their heavy binary baggage.

- **OOP Principles Everywhere**: From C++ strings to Custom Memory Allocators, our `<stdlib.h>` replacements use true polymorphic architecture without `<memory>`.
- **Absolute Language Synthesis**: Written purely in Machine Language, Assembly (`sys_fast_ring.asm`), C++ (`LinuxAbsorber.hpp`), and Rust (`#[no_std]` `SigmaAutomation.rs`).
- **Complete Autonomy**: No `json`, `shutil`, `pathlib`, `psutil`, `hashlib`, `threading`, `<string>`, `<stdlib.h>`, or `alloc::vec`.

---

## ⚙️ Native Subystems (`native_core/`)

### 1. The `MemoryAllocator` (`MemoryAllocator.hpp`)
Replaces `<stdlib.h>` and `malloc/free`.
- Overloads the global `new` and `delete` operators securely.
- Connects directly to `mmap` (Linux) or `VirtualAlloc` (Windows) using custom syscall numbers and `extern "C"` imports. No intermediate buffer pooling vulnerabilities.

### 2. The `SigmaString` Class (`SigmaString.hpp`)
Replaces `<string>` and `<string.h>` (`memcpy`, `strcpy`).
- Custom OOP implementation of dynamically expanding character arrays utilizing `MemoryAllocator`.
- Powers internal native automation strings directly in memory without referencing C++ Standard Template Library.

### 3. The `LinuxAbsorber` (`LinuxAbsorber.hpp`)
Replaces `pacman`, `apk`, `apt`.
- Uses OOP (`AbstractDistroAbsorber` yielding `ArchAbsorber`, `AlpineAbsorber`, `DebianAbsorber`) to natively map package management APIs directly into SigmaOS architecture.

### 4. Zero-Std Rust Automation (`SigmaAutomation.rs`)
Replaces default Rust `std` execution.
- Employs `#![no_std]` and custom `#[panic_handler]`.
- Implements `DistroPackage` natively bridging the safety guarantees of Rust with the custom C++ memory pool for true personalisation tracking.

### 5. Custom Assembly Syscalls (`sys_fast_ring.asm`)
Replaces any standard `<sys/syscall.h>` usage.
- Pure x86_64 Machine Language bindings executing `syscall` and `movups` XMM registers for lightning-fast memory copies (`sigma_mem_copy_xmm`).

---

## 🐍 High-Level Py Wrapper (`sigma_libc.py`)

A pure-Python wrapper interacting natively with string-buffers to bypass `import json`, `hashlib`, `shutil`, `threading` completely.

| Engine | Replaces | How |
|---|---|---|
| **`SigmaJSON`** | `import json` | Custom tokenizer — handles nested dicts, booleans natively |
| **`SigmaHash`** | `import hashlib` | FNV-1a 64-bit — 4MB raw block file fingerprinting |
| **`SigmaFS`** | `shutil`, `pathlib` | Win32 `CopyFileW`, Linux `sendfile()`, 4MB raw streams |
| **`SigmaSys`** | `psutil` | `MEMORYSTATUSEX` struct or `/proc/meminfo` parsing natively |
| **`SigmaThread`** | `threading` | Built on raw `_thread` — CPython's primitive level |
| **`SigmaBase64`** | `import base64` | Pure bitwise encode/decode parsing |
| **`SigmaEntropy`** | `secrets` / `random` | Hooks `CryptGenRandom` / `/dev/urandom` directly |

---

## 🚀 Execution & Integration

Every `.md` guideline (from `ZERO_TRUST_ARCHITECTURE` to `AUTOMATION_GUIDE`) demands absolute performance. 
SigmaOS stands fully prepared for autonomous **automation, customization, and personalization**, relying exclusively on a fully custom, object-oriented, cross-language bare metal runtime.
