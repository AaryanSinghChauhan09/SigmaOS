# SigmaOS Improvements Overview (Rounds 1–7)

A consolidated reference of all OS improvements implemented across seven rounds of development, inspired by production operating systems.

---

## Round 1 — Critical Bug Fixes

| # | Fix | File | Severity |
|---|---|---|---|
| 1 | PID 1 exits after 5 loops → kernel panic | `init/sigma_init_loop.c` | 🔴 Critical |
| 2 | `sprintf`/`strcpy` buffer overflows in ZeroTrust | `kernel/security/sigma_zerotrust.cpp` | 🔴 Critical |
| 3 | Revoked workloads still pass policy checks | `kernel/security/sigma_zerotrust.cpp` | 🔴 Critical |
| 4 | Extension Promise never resolves (chrome.storage function drop) | `background.js` | 🔴 Critical |
| 5 | Kernel links against host glibc (`-nostdlib` commented out) | `Makefile` | 🔴 Critical |
| 6 | Freestanding kernel includes `<stdio.h>` / `<string.h>` | Multiple | 🔴 Critical |
| 7 | `svc_count` array overflow, no `MAX_SERVICES` guard | `sigma_init.cpp` | 🟠 High |
| 8 | CI tests permanently commented out | `sigma_build.yml` | 🟠 High |
| 9 | Firewall always evaluates mocked packet | `sigma_shield.cpp` | 🟠 High |
| 10 | Hardcoded fake timestamp in audit log | `sigma_zerotrust.cpp` | 🟠 High |
| 11 | Go daemon eject always returns success | `sigmad-hotplug/main.go` | 🟠 High |
| 12 | WiFi + BT compiled into same binary | `CMakeLists.txt` | 🟠 High |
| 13 | XSS via unsanitized innerHTML | `web-shell/index.html` | 🟡 Medium |

---

## Round 2 — OpenBSD · FreeBSD · Gentoo · Debian · Fedora

| # | Improvement | Inspired By | File |
|---|---|---|---|
| 1 | `sigma_pledge()` — per-process syscall restriction | OpenBSD | `kernel/security/jail/sigma_pledge.cpp` |
| 2 | `sigma_unveil()` — per-process filesystem restriction | OpenBSD | `kernel/security/mac/sigma_unveil.cpp` |
| 3 | USE flags / compile-time feature toggles | Gentoo portage | `Makefile`, `profiles/` |
| 4 | Multi-queue parallel downloads + SHA-256+BLAKE2b verification | Debian apt | `userland/pkg/sigma_acquire.cpp` |
| 5 | Diff-cover CI gate (80% coverage on changed lines) | Fedora Bodhi | `.github/workflows/sigma-build.yml` |
| 6 | Karma-gated staged rollout + auto-revert | Fedora Bodhi | `userland/pkg/sigma_staged_update.cpp` |
| 7 | `sigma_sysctl` — runtime kernel parameter tuning | FreeBSD / OpenBSD | `klib/sigma_sysctl.cpp` |
| 8 | `.pre-commit-config.yaml` with SPDX + commit format | Gentoo / Fedora | `.pre-commit-config.yaml` |
| 9 | OpenBSD `regress/`-style regression test suite | OpenBSD | `tests/kernel/` |
| 10 | Mirror fallback chain + dual hash verification | Gentoo portage | `userland/pkg/sigma_pkg_fetch.cpp` |

---

## Round 3 — OSTree · Bottlerocket · Talos · Buildroot · Bubblewrap

| # | Improvement | Inspired By | File |
|---|---|---|---|
| 1 | Atomic OS updates via content-addressed object store | OSTree | `userland/pkg/sigma_ostree.cpp` |
| 2 | Real Linux namespace isolation (unshare + pivot_root + seccomp) | Bubblewrap | `kernel/security/jail/sigma_namespace.cpp` |
| 3 | Immutable read-only root in production builds | Bottlerocket | `init/init.c`, `Makefile` |
| 4 | gRPC management API (sigma-apid) for all daemons | Talos Linux | `api/sigma.proto` |
| 5 | Commit conformance: DCO, conventional format, license | Talos Linux | `.conform.yaml` |
| 6 | BR2_BROKEN stub tracker — warns + blocks release on stubs | Buildroot | `Makefile` (check-stubs target) |
| 7 | `sigma_pkg_fetch` mirror fallback chain | Gentoo portage | `userland/pkg/sigma_pkg_fetch.cpp` |

---

## Round 4 — HardenedBSD · illumos · SELinux · OCI runc · CoreOS

| # | Improvement | Inspired By | File |
|---|---|---|---|
| 1 | ASLR 42-bit per-region entropy + W^X enforcement | HardenedBSD | `kernel/mm/sigma_aslr.cpp` |
| 2 | DTrace-style zero-cost SDT probe framework | illumos DTrace | `klib/sigma_trace.cpp` |
| 3 | Access Vector Cache (O(1) MAC decisions) | SELinux | `kernel/security/mac/sigma_avc.cpp` |
| 4 | cgroup v2 resource limits on every workload | OCI runc fs2 | `userland/pkg/sigma_cgroup.cpp` |
| 5 | Structured health daemon (`sigma-healthd`) | CoreOS + Flatpak | `sigmad/healthd/main.go` |
| 6 | OCI bundle format with `sigmaExtensions` | OCI Runtime Spec | `workloads/zenith-browser/config.json` |
| 7 | `sigma_secure_join` — symlink jail-escape prevention | OCI filepath-securejoin | `kernel/security/jail/sigma_securepath.cpp` |
| 8 | DHCP client — full RFC 2131/2132 implementation | RFC standards | `net/dhcp/sigma_dhcp.h` |

---

## Round 5 — HardenedBSD · MINIX 3 · Zephyr RTOS · Redox OS · illumos

| # | Improvement | Inspired By | File |
|---|---|---|---|
| 1 | Trust label matrix — real MAC policy (replaces always-GRANTED stub) | Qubes OS | `kernel/security/sigma_trust_labels.h` |
| 2 | seL4-inspired capability space per workload | seL4 | `kernel/security/sigma_cap.cpp` |
| 3 | `sigma_usercopy` — type-safe kernel↔user memory API | Redox UserSlice | `klib/sigma_usercopy.cpp` |
| 4 | `sigma_build_assert.h` — compile-time struct size contracts | Zephyr RTOS | `klib/include/sigma_build_assert.h` |
| 5 | s6-style supervision state machine + exponential backoff | s6-supervise | `userland/init/sigma_supervisor.cpp` |
| 6 | sigma-rs Reincarnation Server — crash-restart for all daemons | MINIX 3 | `userland/rs/sigma_rs.cpp` |
| 7 | sigma-ds Service Discovery Data Store | MINIX 3 ds | `sigmad/ds/main.go` |
| 8 | sigma-ignite first-boot provisioner (idempotent stamp) | CoreOS Ignition | `userland/ignite/sigma_ignite.cpp` |

---

## Round 6 — seL4 · Plan 9 · Unikraft · Genode OS · Yocto

| # | Improvement | Inspired By | File |
|---|---|---|---|
| 1 | `sigma_scheme` — unified resource URL dispatcher | Redox OS | `klib/sigma_scheme.cpp` |
| 2 | Hardened `sigma_fstab` with `MS_NOEXEC\|MS_NOSUID\|MS_NODEV` | OCI runc | `kernel/fs/sigma_fstab.cpp` |
| 3 | MCS scheduler — budget/period per thread, high-crit preempts low | seL4 MCS | `kernel/sched/sigma_mcs.cpp` |
| 4 | Genode-style declarative service routing policy | Genode OS | `sigma-etc/init.xml` |
| 5 | `Config.sigma` — Unikraft/Kconfig component selection | Unikraft | `Config.sigma` |
| 6 | sigma CLI tool — init, sign (Ed25519), verify, run, health, sysctl | — | `tools/sigma-cli/main.go` |
| 7 | sigma-heartbeat — detects STUCK services (not just crashed) | Genode | `sigmad/heartbeat/sigma_heartbeat.cpp` |
| 8 | Transactional package ops (resolve → execute → error-stop) | Flatpak | `userland/pkg/sigma_pkg_transaction.h` |
| 9 | Binary delta updates (download only changed bytes) | Clear Linux swupd | `userland/pkg/sigma_delta.h` |

---

## Round 7 — Fix Critical Cryptographic Bug

**Kyber-1024 is NOT a signature scheme.** It is a KEM (Key Encapsulation Mechanism). The hypervisor was calling `sigma_crypto_verify_kyber1024()` which cannot exist — Kyber has no signing operation.

**Correct usage:**
- `Dilithium3` (ML-DSA-65) — for **signatures** (VM image verification, package signing)
- `Kyber-1024` — for **key exchange only** (hypervisor↔guest encrypted channel)

The `sigma-trustd` daemon (in `sigmad/trustd/`) enforces this separation: it issues Dilithium3-signed certificates and uses Kyber-1024 for the key exchange establishing the mTLS session.

---

## Current Stub Status

Run `make check-stubs` to see the current list at build time.  
Run `sigmactl health` to see the list at runtime (requires sigma-healthd).

| Subsystem | Status | Tracking |
|---|---|---|
| `sigma-cryptfs` derive_key | **STUB** — 32 zero bytes, no real encryption | [Issue #44](https://github.com/AaryanSinghChauhan09/SigmaOS/issues/44) |
| `kernel/core/*.cpp` | **MISSING** — scheduler/mm/syscall source files not committed | [Issue #47](https://github.com/AaryanSinghChauhan09/SigmaOS/issues/47) |
| `sigma-mac` policy engine | Partially fixed — AVC + trust label matrix present, policy loading still basic | — |
| `sigma-jail` | Fixed — real namespace isolation via `sigma_namespace.cpp` | — |

---

## Security Model Quick Reference

```
Capability system (manifest.json)     → enforced by Chrome extension
sigma_pledge()                        → syscall restriction per process
sigma_unveil()                        → filesystem restriction per process
sigma_namespace.cpp (unshare)         → PID/net/mount/user isolation
sigma_avc.cpp (O(1) MAC cache)        → mandatory access control
sigma_trust_labels.h (Qubes matrix)   → information flow policy
sigma_cap.cpp (seL4 capabilities)     → unforgeable object tokens
sigma_cgroup.cpp (cgroup v2)          → CPU/memory/PID limits
sigma_fstab.cpp (MS_NOEXEC|NOSUID)    → hardened mount flags
sigma_aslr.cpp (42-bit entropy)       → ASLR + W^X enforcement
sigma_securepath.cpp                  → symlink jail-escape prevention
sigma_usercopy.cpp                    → safe kernel↔user memory API
```

---

*See also: [Architecture Overview](Architecture-Overview) · [Security Model](Security-Model) · [Kernel Architecture](Kernel) · [Building from Source](Building-from-Source) · [Contributor Roadmap](Contributor-Roadmap)*

---

## Round 8 — dm-verity · SemanticFS · snapd · Haiku · rpm-ostree · Whonix

| # | Improvement | Inspired By | File |
|---|---|---|---|
| 1 | dm-verity per-package hash verification | Android Verified Boot / snapd | `userland/sigma-pkg/sigma_pkg_verity.h` |
| 2 | Package assertions with anti-replay chain | snapd SnapDeclaration | `sigmad/pkg/assert/sigma_assert.go` |
| 3 | SemanticFS xattrs — SIGMA:TRUST, SIGMA:CLASS, SIGMA:SIGNER | Haiku BFS | `kernel/fs/sigma_semanticfs.h` |
| 4 | Attribute index server (O(log n) queries) | Haiku index_server | `sigmad/indexd/main.go` |
| 5 | Sysroot exclusive lock — no concurrent pkg ops | rpm-ostree | `sigmad/pkg/sigma_pkg_txn_lock.go` |
| 6 | Two-VM network gateway isolation | Whonix | `kernel/virt/sigma_netgw.h` |
| 7 | Auto-generate AppArmor profiles (deny-all + plug exceptions) | snapd | `sigmad/mac/apparmor_gen.go` |
| 8 | HMAC-sealed package journal | rpm-ostree | `userland/sigma-pkg/sigma_pkg_journal.h` |
| 9 | Display server protocol (browser off framebuffer) | Haiku app_server | `userland/display/sigma_display_protocol.h` |
| 10 | SIGMA_ASSERT — zero-cost in release, full diagnostics in debug | Unikraft UK_ASSERT | `klib/include/sigma_assert.h` |
| 11 | DNS resolver + TLS 1.3 + Kyber hybrid stack | RFC 8446 / IETF PQ TLS | `net/dns/sigma_dns.h`, `net/tls/sigma_tls.h`, `net/sigma_net.h` |

---

## Round 9 — IPC · Audio · Fonts · Session · Drivers · Syscalls · SDK · Updates · l10n · SecBoot

| # | Improvement | Inspired By | File |
|---|---|---|---|
| 1 | sigma-bus IPC subsystem — typed Unix-socket message bus | D-Bus / Mach ports | `userland/ipc/sigma_bus.h` + `sigmad/busd/main.go` |
| 2 | Audio server protocol | PipeWire / PulseAudio | `userland/audio/sigma_audio_server.h` |
| 3 | Font subsystem with bitmap + vector support | FreeType / Haiku font_server | `lib/sigma-fonts/sigma_font.h` |
| 4 | Network daemon (netd) — interface + route + firewall management | Android netd | `sigmad/netd/main.go` |
| 5 | Session manager header — login, seat, PAM hooks | logind / elogind | `userland/init/sigma_session.h` |
| 6 | Driver framework — userland driver isolation with capability channels | Fuchsia DDK / Genode | `kernel/drivers/core/sigma_driver_framework.h` |
| 7 | Syscall table — complete ABI surface with capability checks | Linux + seL4 | `klib/include/sigma_syscall.h` |
| 8 | POSIX conformance test suite runner | OpenBSD regress | `tests/posix/run_posix_tests.sh` |
| 9 | Crash reporter — minidump + structured report | Breakpad / Sentry | `userland/daemons/sigma-crash/sigma_crash.h` |
| 10 | SDK CMake toolchain with hardening flags | Yocto SDK / rustup | `userland/devtools/sigma-sdk/sigma.cmake` + `cmake/sigma_hardening.cmake` |
| 11 | One-script SDK installer | rustup / Homebrew | `userland/devtools/sigma-sdk/sigma-sdk-setup.sh` |
| 12 | Amnesic (stateless) mode — tmpfs-only runtime | Tails / Qubes Disposable | `kernel/core/sigma_amnesic.h` |
| 13 | Live kernel patching | kpatch / livepatch | `kernel/kpatch/sigma_kpatch.h` |
| 14 | Generation rollback manager | NixOS / rpm-ostree | `sigmad/pkg/generations/generations.go` |
| 15 | dinit service files for sigma-apid + sigma-trustd | dinit | `sigma-etc/services/sigma-apid.d` + `sigma-etc/services/sigma-trustd.d` |
| 16 | openQA visual regression tests | SUSE openQA | `tests/openqa/sigma_visual_test.py` |
| 17 | Legal compliance tool | REUSE / SPDX | `userland/apps/sigma-legal/sigma_legal.h` |
| 18 | Certificate Authority tool — Dilithium3 PQ certs | CFSSL / step-ca | `userland/apps/sigma-ca/sigma_ca.h` |
| 19 | Power management daemon — battery, lid, suspend | logind / UPower | `sigmad/power/main.go` |
| 20 | Notification daemon — urgency levels, auto-expire | freedesktop.org spec | `sigmad/notify/main.go` |
| 21 | Atomic A/B system updater | Bottlerocket / OSTree | `sigmad/update/main.go` |
| 22 | Localisation subsystem — en_US, hi_IN, zh_CN | GNU gettext / Haiku l10n | `userland/a11y/sigma-l10n/sigma_locale.h` |
| 23 | Secure Boot + TPM 2.0 subsystem | UEFI SB / ChromeOS vboot | `kernel/security/sigma_secboot.h` |

---

*See also: [Gap Analysis](Gap-Analysis) · [Feature Roadmap](Feature-Roadmap) · [Architecture Overview](Architecture-Overview)*
