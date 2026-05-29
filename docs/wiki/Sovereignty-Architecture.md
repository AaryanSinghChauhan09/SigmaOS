# Sovereignty Architecture: The SigmaOS Dependency Manifesto 🔒

SigmaOS is built on a single non-negotiable principle: **every dependency is a liability**. This page is the living audit of what SigmaOS uses, what it has replaced, and what remains.

---

## 🏗️ The Hybrid Language Model

SigmaOS deliberately uses different languages at different layers of the stack. This is not an accident — it is policy.

```
┌────────────────────────────────────────────────────────────────┐
│  LAYER                │  LANGUAGE        │  REASON             │
├───────────────────────┼──────────────────┼─────────────────────┤
│  Bootloader (stage1)  │  Assembly (x86)  │  Bare metal, no ABI │
│  Bootloader (stage2)  │  C               │  Minimal ABI        │
│  Kernel Core          │  C               │  POSIX-free control │
│  Drivers              │  C / C++         │  Hardware proximity  │
│  Sovereign LibC       │  C               │  Zero external deps  │
│  Orchestrator         │  C++ / Rust*     │  Memory safety      │
│  Userland tools       │  C++ / Rust*     │  Safety + speed      │
│  Zenith SDK           │  C++             │  Performance, ABI    │
│  Zenith SDK bindings  │  Rust / Python*  │  Developer ergonomics│
│  App developer layer  │  Any (via SDK)   │  Open to all         │
└────────────────────────────────────────────────────────────────┘
  * Rust introduced incrementally — not yet in kernel core.
```

> [!IMPORTANT]
> **Rule:** Rust is **never** allowed in the kernel core or bootloader. It may be used in the orchestrator, userland tools, and SDK bindings. This preserves full compile-time control over kernel ABI.

---

## 📊 Full Dependency Audit

### ✅ Replaced — Custom Implementations in Place

| Dependency | Replaced By | File |
|---|---|---|
| `malloc` / `free` | `sigma_malloc` / `sigma_free` (bump + free-list) | [`kernel/libc/sigma_libc_impl.c`](../kernel/libc/sigma_libc_impl.c) |
| `printf` / `fprintf` | `sys_print` (raw `write(2)` syscall, varargs) | [`kernel/libc/sigma_libc_impl.c`](../kernel/libc/sigma_libc_impl.c) |
| `strcpy` / `strcat` | `sigma_strcpy` / `sigma_strcat` | [`kernel/libc/sigma_libc_impl.c`](../kernel/libc/sigma_libc_impl.c) |
| `memcpy` / `memset` / `memmove` | `sigma_memcpy` / `sigma_memset` / `sigma_memmove` | [`kernel/libc/sigma_libc_impl.c`](../kernel/libc/sigma_libc_impl.c) |
| `atoi` / `itoa` | `sigma_atoi` / `sigma_itoa` | [`kernel/libc/sigma_libc_impl.c`](../kernel/libc/sigma_libc_impl.c) |
| `errno` | `ZEN-DRIVER-xxxx` / `ZEN-UPDATE-xxxx` structured codes | [`include/sigma_driver_codes.h`](../include/sigma_driver_codes.h) |
| GTK / Qt | Zenith Native Toolkit (Button, Label, ListView, Container) | [`zenith_desktop/sdk/include/zenith.h`](../zenith_desktop/sdk/include/zenith.h) |
| Wayland / X11 | Direct Compositor IPC (`sigma_compositor.cpp`) | [`zenith_desktop/compositor/sigma_compositor.cpp`](../zenith_desktop/compositor/sigma_compositor.cpp) |
| DKMS (external) | Sovereign Driver Registry + DKMS tracker | [`kernel/drivers/sigma_driver_registry.cpp`](../kernel/drivers/sigma_driver_registry.cpp) |
| apt / rpm / pacman | Sovereign App Store + `.spkg` bundles | [`zenith_desktop/appstore/sigma_appstore.cpp`](../zenith_desktop/appstore/sigma_appstore.cpp) |

### 🔲 Remaining — Known Liabilities (Planned for Replacement)

| Dependency | Risk Level | Plan |
|---|---|---|
| Compiler built-ins (`__builtin_va_list`, `__asm__`) | Low — compiler intrinsics, not libc | Acceptable; unavoidable for inline ASM |
| `posix_memalign` ABI signature | Low — only the name; implementation is custom | Rename to `sigma_aligned_alloc` in Phase 7 |
| Rust `std` crate in SDK bindings | Medium — links against system libc via Rust | Replace with `no_std` crate + custom allocator hook into `sigma_malloc` |
| Python bindings (future) | Medium — CPython runtime dependency | Evaluate PyPy or MicroPython for sovereign embedding |
| Flatcar-style update channel (network) | Low — we control the server | Already mediated through sovereign signature verification |

---

## 🛡️ Kernel Space Rules (Enforced)

The following are hard rules checked during code review:

> [!CAUTION]
> **FORBIDDEN in kernel space:**
> - `#include <string.h>`, `<stdlib.h>`, `<stdio.h>`, `<unistd.h>`
> - `malloc`, `free`, `printf`, `scanf`, `fopen`, `fclose`
> - `strcpy`, `strcat`, `memcpy`, `memset` (use `sigma_*` equivalents)
> - Any Qt, GTK, SDL, SFML, or similar framework header
> - Dynamic linking against external `.so` / `.dll`

> [!TIP]
> **PERMITTED in kernel space:**
> - `sigma_malloc`, `sigma_free`, `sigma_memcpy`, `sigma_memset`
> - `sigma_strcmp`, `sigma_strncpy`, `sigma_strlen`, `sigma_strstr`
> - `sys_print(fmt, ...)` for console output
> - `zenith_log_structured(code, comp, desc, cid)` for structured logging
> - Compiler built-ins (`__builtin_va_list`, `__asm__ volatile`)

---

## 🖥️ Zenith Native Toolkit — No GTK/Qt

The Zenith SDK (`zenith_desktop/sdk/include/zenith.h`) provides a complete, sovereign C++ widget toolkit. No external graphics library is ever linked.

### Widget Catalogue

| Widget | Purpose |
|---|---|
| `Zenith::UI::Button` | Pressable control with hover state and click callback |
| `Zenith::UI::Label` | Read-only text with custom color |
| `Zenith::UI::TextInput` | Editable single-line field with cursor |
| `Zenith::UI::ProgressBar` | 0–100% completion indicator |
| `Zenith::UI::ListView` | Scrollable, keyboard-navigable list |
| `Zenith::UI::Container` | Flexbox-style ROW/COLUMN layout |

### Rendering Path

```
App Widget::render()
    └─► sys_ipc_send(COMPOSITOR_IPC, DRAW_RECT, ...)
            └─► sigma_compositor.cpp
                    └─► Direct framebuffer write (no Wayland daemon)
```

---

## 🔧 Networking — Custom Stack

The networking layer (`kernel/net/`) is fully sovereign. Key modules:

| File | Protocol |
|---|---|
| `sigma_net_ipv4.cpp` | IPv4 packet processing |
| `sigma_ipv6.cpp` | IPv6 support |
| `sigma_net_tcp.cpp` | TCP state machine |
| `sigma_net_udp.cpp` | UDP datagram handling |
| `sigma_net_dns.cpp` | Recursive DNS resolver |
| `sigma_net_arp.cpp` | ARP table management |
| `sigma_firewall.cpp` | Packet filter (Whonix-style isolation) |

No external networking library (libcurl, OpenSSL, libuv) is used in the kernel. TLS will be implemented via a sovereign crypto module in Phase 7.

---

## 📁 Filesystem — Custom VFS

The VFS layer (`kernel/fs/`) provides:

| File | Description |
|---|---|
| `SovereignVFS.cpp` | Virtual filesystem layer (mount, open, read, write) |
| `SovereignFAT32.cpp` | FAT32 driver (removable media) |
| `sigma_vfs.cpp` | VFS syscall dispatch |
| `sigma_crypto_vol.cpp` | Encrypted volume support (LUKS-compatible) |
| `fs/ext4/` | ext4 read support |
| `fs/btrfs/` | btrfs read + snapshot support |

---

## 🚀 Phase Roadmap

| Phase | Sovereignty Goal | Status |
|---|---|---|
| Phase 1–2 | Kernel + bootloader in C/ASM, zero libc | ✅ Done |
| Phase 3 | Sovereign Orchestrator (no Docker/K8s) | ✅ Done |
| Phase 4 | Native Zenith DE (no GTK/Qt) | ✅ Done |
| Phase 5 | Full sovereign libc + driver codes | ✅ Done |
| Phase 6 | DKMS integration, HW test suite, A/B updates | ✅ Done |
| Phase 7 | `no_std` Rust for SDK bindings, sovereign TLS | 🔲 Planned |
| Phase 8 | Formal verification of allocator + VFS | 🔲 Planned |
