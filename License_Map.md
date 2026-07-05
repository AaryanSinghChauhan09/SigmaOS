# SigmaOS License Map

> Per-directory licensing for the entire repository.
> SigmaOS uses MIT as the default license. Some directories have different terms.

---

## Quick Reference

| Directory / File | License | Notes |
|---|---|---|
| `kernel/` | MIT | Freestanding, no glibc, original implementation |
| `arch/` | MIT | Architecture-specific boot + paging code |
| `drivers/` | MIT | SDF drivers; firmware blobs excluded (see below) |
| `fs/` | MIT | VFS, SigmaFS, Ext4 port |
| `net/` | MIT | TCP/IP stack, TLS, DHCP |
| `security/` | MIT | pledge/unveil, AVC, PQC |
| `crypto/` | MIT | Kyber-1024, Dilithium-5 implementations |
| `memory/` | MIT | Buddy + slab allocators |
| `scheduling/` | MIT | MLFQ, EDF, CFS schedulers |
| `hal/` | MIT | Hardware abstraction layer |
| `userland/` | MIT | shell, pkg, coreutils |
| `suites/` | MIT | 600+ capability shards |
| `zenith_desktop/` | MIT | Electron/Vite desktop prototype |
| `browser/` | MIT | WASM browser demo |
| `ui/` | MIT | UI toolkit |
| `sdk/` | MIT | Developer SDK |
| `runtime/wasm/` | MIT | WASM/WASI runtime |
| `runtime/containers/` | MIT | Linux ELF compat layer |
| `docs/` | MIT | Documentation |
| `wiki_repo/` | MIT | Wiki content |
| `tests/` | MIT | Test suite |
| `scripts/` | MIT | Build + CI automation |
| `tools/` | MIT | Developer tooling |
| `api/sigma.proto` | MIT | gRPC protocol definitions |
| `agents/` | MIT | Orchestration agents |
| `simulation/` | MIT | Kernel simulator |
| `S01_Genesis/` | MIT | Genesis shard |
| `include/` | MIT | All headers |
| `assets/` | MIT | Logos, images |
| `.github/` | MIT | CI/CD workflows |

---

## Special Cases

### Firmware Blobs (Non-Free, Optional)

Proprietary firmware files (NVIDIA, Broadcom Wi-Fi) are **never stored in this repo**.
They are downloaded at install time via `sigma-pkg install sigma-nonfree/<driver>`.
Those blobs retain their original vendor licenses. SigmaOS code that loads them (the
firmware loader shim) is MIT-licensed; the blobs themselves are not.

### Mesa / GPU Drivers (Cleanroom)

SigmaOS GPU drivers are clean-room reimplementations of the *interface* of Mesa's
Gallium3D and Linux's DRM/KMS. No GPL Mesa or Linux DRM source code is included.
The cleanroom process is documented in
[CANONICAL_CLEANROOM_ABSORPTION.md](../wiki_repo/CANONICAL_CLEANROOM_ABSORPTION.md).

### BSD-2-Clause Files

Some files in `lib/` originate from FreeBSD or OpenBSD (BSD-2-Clause compatible with MIT).
These are explicitly marked with `// SPDX-License-Identifier: BSD-2-Clause` headers.
Both MIT and BSD-2-Clause are permissive and compatible — downstream users may use
either term.

### GPL Study References

No GPL source code is included in this repository. Architectural inspiration from GPL
projects (Linux kernel, Mesa, GNOME, etc.) was used under cleanroom discipline.
Documentation of what was studied lives in [docs/OSS_Reference_Map.md](OSS_Reference_Map.md).

---

## SPDX Headers

Every source file carries an SPDX header as the first comment:

```rust
// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
```

```c
// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
```

```zig
// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
```

Missing headers are a CI lint failure (`make check-spdx`).

---

## License Text Locations

- `LICENSE` — MIT full text (root)

- `wiki_repo/LICENSE.md` — MIT (wiki)

- `wiki_repo/LICENCE.md` — BSD-2-Clause (where applicable)

---

## Downstream Usage

You may use, modify, and distribute SigmaOS under the MIT License terms.
If you incorporate SigmaOS code in a product:

1. Keep the MIT copyright notice in source files.

2. Include the LICENSE file in your distribution.

3. Do not use "SigmaOS" branding without permission (trademark, not copyright).

4. If you ship firmware blobs alongside SigmaOS, comply with each blob's vendor license.

---

*See also: [docs/OSS_Reference_Map.md](OSS_Reference_Map.md) · [CONTRIBUTING.md](../CONTRIBUTING.md) · [wiki_repo/CANONICAL_CLEANROOM_ABSORPTION.md](../wiki_repo/CANONICAL_CLEANROOM_ABSORPTION.md)*
