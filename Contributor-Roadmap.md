# Contributor Roadmap

This page gives an honest picture of where SigmaOS stands today — what works, what is scaffolded but not implemented, and where contributors are most needed. It is sourced from the `prepare-sigmaos-launch` branch audit.

---

## Honest Status: What Actually Works

> The scaffolding is solid. True implementation is still needed in several areas. This roadmap tracks real feature development, not aspirational claims.

| Component | Status | Notes |
|---|---|---|
| Freestanding x86_64 kernel boot | ✓ Working | Boots in QEMU, IDT + Ring 3 transition complete |
| PID 1 signalfd event loop | ✓ Working | Infinite loop with SIGCHLD reaping |
| MLFQ + Round-Robin scheduler | ✓ Working | 4 priority levels, periodic boost |
| VMM 4-level paging | ✓ Working | PML4, PDPT, PD, PT; higher-half kernel |
| VFS + Ext4 read | ✓ Working | Superblock parsing, inode resolution |
| TCP/IP loopback | ✓ Working | 3-way handshake state machine |
| bwrap sandboxing | ✓ Working | PID/mount/net namespaces + seccomp |
| pledge / unveil | ✓ Working | Per-process syscall + FS restriction |
| Zero-trust workload IDs | ✓ Working | SPIFFE URIs, revocation check on every call |
| Ext4 JBD2 journaling | ⚠ Partial | Ordered-mode journal not fully wired |
| PMM fragmentation | ⚠ Known bug | Block allocator fragments under pressure |
| Zenith UI | ⚠ JS prototype | Native C++ compositor not yet built |
| NVMe / e1000 drivers | ☐ Planned | Real hardware drivers not committed |
| USB 3.0 controller | ☐ Planned | Only stub exists |
| Linux DRM/KMS shim | ☐ Planned | HAL stub only |
| Graphical installer | ☐ Planned | No Calamares equivalent yet |

---

## Competitive Gap Analysis

Targets from the `prepare-sigmaos-launch` branch gap audit:

| Category | Best Competitor | SigmaOS Target | Shard / File |
|---|---|---|---|
| UX & Accessibility | Zorin / Elementary | Adaptive UI scaling + screen reader | `SovereignAccessibility` |
| IoT / ARM | Raspberry Pi OS | Event-driven GPIO + sensor toolkit | `SovereignIoT` |
| Gaming | SteamOS | Dynamic GPU scheduler + controller manager | `SovereignGPUSched` |
| Performance | Clear Linux | Telemetry-driven auto-tuner + PGO | `SovereignAISched` |
| Reproducibility | NixOS | Declarative shard configs + rollbacks | `sigma-pkg sync` |
| Recovery | Rescuezilla | Snapshot diff + forensic toolkit | `SovereignSnapshotDiff` |
| Containers | Fedora CoreOS | Sovereign Container Orchestrator | `SovereignOrchestrator` |
| Rolling updates | Arch / Solus | Incremental delta updates | `sigma-pkg update --delta` |
| Enterprise | Ubuntu | Hardware regression harness | `SovereignRegression` |
| Filesystem | Btrfs / ZFS | Sovereign CoW + journaling FS | `SovereignFS` |
| Networking | Linux TCP/IP | IPv6, mesh, VPN stack | `SovereignNet` |
| Compatibility | WSL / Wine | POSIX translation shims (opt-in) | `SovereignCompat` |

---

## Development Phases

### Phase 1: Core Stabilisation (in progress)

- [x] Bootable state on x86_64 + QEMU cross-arch validation

- [x] Sovereign Init with parallel boot and process monitoring

- [x] Ext4 read/write with superblock parsing

- [x] OmniPkg format spec and local deployment logic

- [ ] Ext4 ordered-mode JBD2 journaling

- [ ] Resolve PMM block allocator fragmentation under load

### Phase 2: Hardware & Network (planned)

- [ ] Linux DRM/KMS compatibility shim at HAL level

- [ ] VFS network abstraction (NFS/SMB mount)

- [ ] Native IPv4/IPv6 stack + drivers (e1000, ixgbe)

- [ ] USB 3.0 and NVMe controller implementation

- [ ] Wi-Fi WPA3 driver (using `drivers-dev` branch work)

### Phase 3: Desktop Environment & Tooling (planned)

- [ ] Migrate Zenith UI JS prototype to native C++ compositor

- [ ] Implement Sigma Shell robust scripting pipelines

- [ ] Implement a graphical guided installer (Calamares equivalent)

- [ ] SigmaCode IDE with Monaco editor and process spawn

- [ ] SigmaTerm PTY over WebSocket

### Phase 4: Security Hardening (planned)

- [ ] Formal verification of kernel ring isolation

- [ ] Continuous cryptographic supply chain auditing

- [ ] Signed `.spkg` registry with Dilithium3 signatures

- [ ] Hardware Security Module (HSM) integration for attestation

### Phase 5: Specialized Profiles & Distribution (planned)

- [ ] Profession-specific shard bundles (AI, Security, Gaming, Education)

- [ ] Karma-gated staged rollout (Fedora Bodhi-inspired)

- [ ] Immutable image builder for AWS/Azure/GCP

- [ ] Mobile installer for ARM64 tablets

---

## Contribution Principles

1. **Sovereignty** — minimize monolithic dependencies where possible

2. **Zero External Dependencies in Ring 0** — kernel code must compile without GNU/Linux headers; use `klib/` exclusively

3. **SPDX Headers** — every new source file needs `// SPDX-License-Identifier: GPL-2.0-or-later` as line 1

4. **Commit Message Format** — `subsystem/component: description (10–72 chars)`, e.g. `kernel/net: fix TCP conntrack overflow`

5. **Tests required** — add a corresponding test in `tests/kernel/` or `tests/userland/` for every new subsystem

6. **Pre-commit hooks** — run `pre-commit install` after cloning; hooks enforce SPDX, clang-format, go-fmt, and commit message format

---

## Where to Start

**Good first issues** (low complexity, high impact):

- Add `SPDX-License-Identifier` headers to files in `kernel/drivers/` that are missing them

- Write a regression test for an existing syscall (see `tests/kernel/` for examples)

- Fix a `TODO` or `FIXME` comment in `kernel/net/sigma_tcpip.c`

- Document a function in `klib/` with a JSDoc-style comment block

- Add a seed corpus file to `tests/kernel/fuzz_corpus/` (any valid TCP packet binary)

### Medium complexity:

- Implement `sigma_sysctl_set` for the `SYSCTL_TYPE_STRING` case in `klib/sigma_sysctl.cpp`

- Wire the Ext4 JBD2 journaling commit path in `fs/`

- Add a `--json` output flag to `sigma-sysctl` CLI tool

### High complexity:

- Implement a real e1000 NIC driver in `kernel/drivers/net/`

- Build the native C++ Zenith compositor to replace the JS prototype

- Implement the USB 3.0 xHCI controller driver

---

## Running the Full Test Suite

```bash

# Unit tests (Google Test, host-mode)

cd tests/cpp_host && cmake -B build && cmake --build build && cd build && ctest

# Kernel regression suite (OpenBSD regress-style)

make -C tests regress

# Fuzzer (libFuzzer, 30-second budget)

clang++ -fsanitize=fuzzer,address -Iinclude tests/kernel/fuzz_tcp.cpp \
        kernel/net/sigma_tcpip.c -o fuzz_tcp && ./fuzz_tcp -max_total_time=30

# Static analysis

run-clang-tidy -p build kernel/ init/ lib/
cppcheck --enable=warning,style -Iinclude kernel/ init/ lib/
```

---

*See also: [Building from Source](Building-from-Source) · [Branch Guide](Branch-Guide) · [Security Model](Security-Model)*
