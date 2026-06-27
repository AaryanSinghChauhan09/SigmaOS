# SigmaOS Gap Analysis — Round 9

This page summarises the gap analysis performed after Rounds 1–9, comparing SigmaOS against Tier 1 (Linux distros), Tier 2 (microkernels/research OSes), and Tier 3 (cloud-native OSes). Gaps are rated by severity and assigned to a target round.

---

## How to Read This Document

| Symbol | Meaning |
|--------|---------|
| ✅ | Implemented and committed |
| 🔧 | Header/stub present — full implementation pending |
| ☐ | Not yet started |
| 🔴 | Critical gap — blocks production use |
| 🟠 | High priority — needed for v1.0 |
| 🟡 | Medium — improves quality / completeness |
| 🟢 | Low — polish / stretch goal |

---

## Tier 1 Gap Analysis — vs. Linux Distributions

Comparing SigmaOS against Alpine Linux, Debian, Fedora, Ubuntu, and Arch.

### Security

| Gap | Severity | SigmaOS Status | Reference |
|-----|----------|---------------|-----------|
| Secure Boot chain of trust | 🔴 | 🔧 `kernel/security/sigma_secboot.h` (Round 9) | shim → grub → kernel |
| TPM 2.0 seal/unseal for disk key | 🔴 | 🔧 In `sigma_secboot.h` API | systemd-cryptenroll |
| dm-verity on root partition | 🟠 | 🔧 `userland/sigma-pkg/sigma_pkg_verity.h` | Verified Boot |
| AppArmor / SELinux profiles | 🟠 | 🔧 `sigmad/mac/apparmor_gen.go` | auto-generated |
| ASLR + W^X enforcement | ✅ | `kernel/mm/sigma_aslr.cpp` | HardenedBSD |
| Stack protector (userland) | ✅ | `cmake/sigma_hardening.cmake` | `-fstack-protector-strong` |
| RELRO + BIND_NOW | ✅ | `cmake/sigma_hardening.cmake` | `-Wl,-z,relro,-z,now` |
| Hardened mount flags | ✅ | `kernel/fs/sigma_fstab.cpp` | MS_NOEXEC|MS_NOSUID|MS_NODEV |
| Capability-based security | ✅ | `kernel/security/sigma_cap.cpp` | seL4 model |
| Audit log chain | ✅ | `kernel/security/sigma_audit_backend.cpp` | SHA-256 chained |
| eBPF programmable hooks | 🟠 | 🔧 `kernel/security/sigma_ebpf.h` | kernel 5.15+ style |
| Module signing (Dilithium3) | 🟠 | 🔧 `kernel/security/sigma_module_sign.h` | replaces RSA-4096 |

### Package Management

| Gap | Severity | SigmaOS Status | Reference |
|-----|----------|---------------|-----------|
| Atomic A/B OS updates | ✅ | `sigmad/update/main.go` (Round 9) | Bottlerocket / OSTree |
| Generation rollback | ✅ | `sigmad/pkg/generations/generations.go` | rpm-ostree |
| Transactional package ops | ✅ | `userland/pkg/sigma_pkg_transaction.h` | Flatpak |
| Binary delta updates | 🟠 | 🔧 `userland/pkg/sigma_delta.h` | Clear Linux swupd |
| Pkg assertions / signatures | 🟠 | 🔧 `sigmad/pkg/assert/sigma_assert.go` | snapd SnapDeclaration |
| dm-verity per package | 🟠 | 🔧 `userland/sigma-pkg/sigma_pkg_verity.h` | snapd |
| .deb / .rpm / .apk compat | 🟡 | ☐ — format resolver needed | Planned (Phase 1) |

### Init & Services

| Gap | Severity | SigmaOS Status | Reference |
|-----|----------|---------------|-----------|
| PID 1 with signalfd loop | ✅ | `init/sigma_init_loop.c` | systemd / s6 |
| Service supervision + restart | ✅ | `userland/init/sigma_supervisor.cpp` | s6 |
| dinit service files | ✅ | `sigma-etc/services/` | dinit |
| Notification daemon | ✅ | `sigmad/notify/main.go` (Round 9) | freedesktop spec |
| Power management daemon | ✅ | `sigmad/power/main.go` | logind / UPower |
| D-Bus replacement (sigma-bus) | ✅ | `userland/ipc/sigma_bus.h` + `sigmad/busd/main.go` | custom Unix-socket IPC |
| Session manager | 🔧 | `userland/init/sigma_session.h` | logind / elogind |
| GPU driver stack | 🔴 | ☐ — `drivers-dev` branch target | Mesa / DRM / KMS |
| WiFi / BT drivers | 🔴 | ☐ — `drivers-dev` branch target | mac80211 / BlueZ |

### Developer Tooling

| Gap | Severity | SigmaOS Status | Reference |
|-----|----------|---------------|-----------|
| SDK one-liner install | ✅ | `userland/devtools/sigma-sdk/sigma-sdk-setup.sh` (Round 9) | rustup style |
| CMake toolchain + hardening | ✅ | `userland/devtools/sigma-sdk/sigma.cmake` + `cmake/sigma_hardening.cmake` | |
| sigma CLI | ✅ | `tools/sigma-cli/main.go` | |
| POSIX test suite | ✅ | `tests/posix/run_posix_tests.sh` | |
| openQA visual tests | ✅ | `tests/openqa/sigma_visual_test.py` | SUSE openQA |
| Crash reporter | 🔧 | `userland/daemons/sigma-crash/sigma_crash.h` | breakpad-style |
| Distributed tracing | 🟡 | 🔧 DTrace probes at syscall entry | illumos dtrace |

---

## Tier 2 Gap Analysis — vs. Microkernels & Research OSes

Comparing SigmaOS against seL4, MINIX 3, Genode, Redox OS, Haiku.

| Gap | Severity | SigmaOS Status | Notes |
|-----|----------|---------------|-------|
| seL4 capability space | ✅ | `kernel/security/sigma_cap.cpp` | unforgeable tokens |
| MCS hard real-time scheduler | ✅ | `kernel/sched/sigma_mcs.cpp` | budget/period per thread |
| Reincarnation Server | ✅ | `userland/rs/sigma_rs.cpp` | MINIX 3 style |
| Service discovery store | ✅ | `sigmad/ds/main.go` | MINIX 3 ds |
| Genode declarative routing | ✅ | `sigma-etc/init.xml` | |
| Redox scheme dispatcher | ✅ | `klib/sigma_scheme.cpp` | unified URL API |
| Haiku SemanticFS xattrs | 🔧 | `kernel/fs/sigma_semanticfs.h` | attribute index |
| Attribute index server | 🔧 | `sigmad/indexd/main.go` | O(log n) queries |
| Driver framework isolation | 🔧 | `kernel/drivers/core/sigma_driver_framework.h` | driver in userland |
| Formal verification | 🔴 | ☐ — no proofs yet | seL4 style; very long-term |
| IPC capability passing | 🟠 | ☐ — sigma-bus passes caps | extend sigma_bus.h |
| Kernel live patching | 🔧 | `kernel/kpatch/sigma_kpatch.h` | kpatch / livepatch |

---

## Tier 3 Gap Analysis — vs. Cloud-Native / Container OSes

Comparing SigmaOS against Talos Linux, Bottlerocket, Flatcar, CoreOS, NixOS.

| Gap | Severity | SigmaOS Status | Notes |
|-----|----------|---------------|-------|
| Immutable root filesystem | ✅ | `init/init.c` + `Makefile` | MS_RDONLY on boot |
| Atomic A/B slot updates | ✅ | `sigmad/update/main.go` | Bottlerocket style |
| gRPC management API | 🔧 | `api/sigma.proto` | Talos apid |
| OCI bundle format | ✅ | `workloads/zenith-browser/config.json` | |
| cgroup v2 resource limits | ✅ | `userland/pkg/sigma_cgroup.cpp` | OCI runc |
| Bubblewrap namespace isolation | ✅ | `kernel/security/jail/sigma_namespace.cpp` | |
| First-boot provisioner | ✅ | `userland/ignite/sigma_ignite.cpp` | Ignition-style |
| Generation rollback | ✅ | `sigmad/pkg/generations/generations.go` | NixOS style |
| Amnesic (stateless) mode | 🔧 | `kernel/core/sigma_amnesic.h` | Tails-inspired |
| Two-VM network gateway | 🔧 | `kernel/virt/sigma_netgw.h` | Whonix style |
| Declarative system config | 🟠 | ☐ — `Config.sigma` is partial | NixOS full config |
| Reproducible builds | 🟡 | `SOURCE_DATE_EPOCH` in `Makefile` | NixOS / Guix |

---

## Cross-Cutting Gaps (All Tiers)

These gaps cut across all three tiers and are tracked separately.

### Must-Fix Before v1.0

| # | Gap | File | Action |
|---|-----|------|--------|
| 1 | `sigma-cryptfs` — `derive_key()` returns 32 zero bytes | `kernel/security/sigma_cryptfs.cpp` | Implement PBKDF2-SHA512 or Argon2id |
| 2 | `kernel/core/` source files missing | `kernel/core/*.cpp` | Implement scheduler, MM, syscall dispatcher |
| 3 | GPU driver stack absent | `drivers-dev` branch | DRM/KMS minimum for Zenith DE |
| 4 | WiFi drivers missing | `drivers-dev` branch | mac80211 cfg80211 |
| 5 | Secure Boot full implementation | `sigma_secboot.h` → `.cpp` | TPM2 ESAPI calls |

### Quality & Completeness

| # | Gap | Target Round | Notes |
|---|-----|-------------|-------|
| 6 | Universal binary loader (ELF/OCI/WASM) | Round 10 | `sigma_universal_loader.h` → impl |
| 7 | eBPF hook runtime | Round 10 | `sigma_ebpf.h` → impl + verifier |
| 8 | DNS sinkhole | Round 10 | `sigma_dns_sinkhole.h` → impl |
| 9 | `sigma_locale.c` catalogues (hi_IN, zh_CN) | Round 10 | impl for header in Round 9 |
| 10 | sigma-bus capability passing | Round 10 | extend sigma_bus.h with cap token passing |
| 11 | Session manager implementation | Round 10 | `sigma_session.h` → impl |
| 12 | Crash reporter implementation | Round 10 | `sigma_crash.h` → impl |
| 13 | REST API gateway full coverage | Round 11 | all daemons exposed via HTTP |
| 14 | `.deb/.rpm/.apk` compatibility | Round 11 | format resolver in sigma-pkg |
| 15 | Formal IPC protocol spec | Round 12 | TLA+ or Alloy model of sigma-bus |

---

## Rounds 8–9 New Files Summary

All files introduced in Rounds 8 and 9 that were not present before:

| File | Category | Round |
|------|----------|-------|
| `userland/ipc/sigma_bus.h` | IPC | 9 |
| `sigmad/busd/main.go` | IPC daemon | 9 |
| `userland/audio/sigma_audio_server.h` | Audio | 9 |
| `lib/sigma-fonts/sigma_font.h` | Fonts | 9 |
| `sigmad/netd/main.go` | Networking daemon | 9 |
| `userland/init/sigma_session.h` | Session | 9 |
| `kernel/drivers/core/sigma_driver_framework.h` | Drivers | 9 |
| `klib/include/sigma_syscall.h` | Syscall table | 9 |
| `tests/posix/run_posix_tests.sh` | Testing | 9 |
| `userland/daemons/sigma-crash/sigma_crash.h` | Reliability | 9 |
| `userland/devtools/sigma-sdk/sigma.cmake` | SDK | 9 |
| `userland/devtools/sigma-sdk/sigma-sdk-setup.sh` | SDK | 9 |
| `cmake/sigma_hardening.cmake` | Build | 9 |
| `kernel/core/sigma_amnesic.h` | Security | 9 |
| `kernel/kpatch/sigma_kpatch.h` | Reliability | 9 |
| `sigmad/pkg/generations/generations.go` | Updates | 9 |
| `sigma-etc/services/sigma-apid.d` | Init | 9 |
| `sigma-etc/services/sigma-trustd.d` | Init | 9 |
| `tests/openqa/sigma_visual_test.py` | Testing | 9 |
| `userland/apps/sigma-legal/sigma_legal.h` | Compliance | 9 |
| `userland/apps/sigma-ca/sigma_ca.h` | PKI | 9 |
| `sigmad/power/main.go` | Power | 9 |
| `sigmad/notify/main.go` | Notifications | 9 |
| `sigmad/update/main.go` | A/B Updates | 9 |
| `userland/a11y/sigma-l10n/sigma_locale.h` | l10n | 9 |
| `kernel/security/sigma_secboot.h` | Secure Boot | 9 |
| `kernel/fs/sigma_semanticfs.h` | Filesystem | 8 |
| `sigmad/indexd/main.go` | Filesystem | 8 |
| `sigmad/mac/apparmor_gen.go` | Security | 8 |
| `sigmad/pkg/assert/sigma_assert.go` | Packages | 8 |
| `sigmad/pkg/sigma_pkg_txn_lock.go` | Packages | 8 |
| `kernel/virt/sigma_netgw.h` | Networking | 8 |
| `userland/sigma-pkg/sigma_pkg_verity.h` | Packages | 8 |
| `userland/sigma-pkg/sigma_pkg_journal.h` | Packages | 8 |
| `userland/display/sigma_display_protocol.h` | Display | 8 |
| `klib/include/sigma_assert.h` | Klib | 8 |
| `net/dns/sigma_dns.h` | Networking | 8 |
| `net/sigma_net.h` | Networking | 8 |
| `net/tls/sigma_tls.h` | Networking | 8 |

---

## Priority Queue — Round 10 Targets

Based on this gap analysis, Round 10 should focus on:

1. **`sigma-cryptfs`** — fix the 32-zero-byte key derivation bug (Critical)
2. **`kernel/core/` implementations** — scheduler, MM, syscall table bodies (Critical)
3. **`sigma_universal_loader.cpp`** — implement the ELF/OCI/WASM detector
4. **`sigma_ebpf.cpp`** — implement the eBPF verifier + runtime
5. **`sigma_locale_hi_IN.c` + `sigma_locale_zh_CN.c`** — translation catalogues
6. **`sigma_session.cpp`** — session manager implementation
7. **`sigma_crash.cpp`** — crash reporter implementation
8. **GPU minimum viable stack** — DRM/KMS framebuffer on `drivers-dev`

---

*See also: [Improvements Overview](Improvements-Overview) · [Feature Roadmap](Feature-Roadmap) · [Architecture Overview](Architecture-Overview) · [Security Model](Security-Model)*
