# SigmaOS Improvements Overview (Rounds 1–16)

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

## Round 10 — SMP · ACPI · THP · zram · CET+KASLR · RT Scheduler · Journal

| # | Improvement | Inspired By | File |
|---|---|---|---|
| 1 | SMP support — LAPIC/IPI/per-CPU data structures | Linux SMP | `kernel/arch/sigma_smp.h` |
| 2 | ACPI parser — full DSDT/SSDT table walking | ACPICA | `kernel/arch/sigma_acpi.h` |
| 3 | Transparent Huge Pages + HugeTLB | Linux THP | `kernel/mm/sigma_hugepage.h` |
| 4 | zram — compressed RAM-backed swap | Linux zram | `kernel/mm/sigma_zram.h` |
| 5 | CET shadow stack + KASLR kernel randomisation | Intel CET / PAX | `kernel/arch/sigma_cet.h` |
| 6 | PREEMPT_RT real-time scheduler | PREEMPT_RT patch | `kernel/sched/sigma_rt.h` |
| 7 | NTP daemon with leap-second handling | chrony / ntpd | `sigmad/ntpd/main.go` |
| 8 | Structured binary journal (indexed, queryable) | systemd journald | `sigmad/journal/main.go` |
| 9 | DVFS + thermal management (cpufreq governor) | Linux cpufreq | `sigmad/thermald/main.go` |
| 10 | Reproducible build verification | Debian repro builds | `cmake/sigma_repro.cmake` |
| 11 | ARM64 + RISC-V boot stubs | Linux arch/ | `arch/arm64/` + `arch/riscv/` |
| 12 | Formal verification contracts (Frama-C style) | seL4 proofs | `kernel/formal/sigma_contracts.h` |
| 13 | sigma-ide — AI-assisted editor integration | Zed / VS Code | `userland/apps/sigma-ide/` |

---

## Round 11 — Driver Shards · VFS · SigmaFS · RAID · DRM · Audio · Namespaces

| # | Improvement | Inspired By | File |
|---|---|---|---|
| 1 | Shard-based driver architecture | Genode components | `kernel/drivers/sigma_shard_arch.h` |
| 2 | Virtual File System (VFS) layer | Linux VFS | `kernel/fs/sigma_vfs.h` |
| 3 | SigmaFS — native copy-on-write filesystem | Btrfs / ZFS | `kernel/fs/sigmafs/sigma_sigmafs.h` |
| 4 | Software RAID 0/1/5/6/10 | Linux md RAID | `kernel/fs/sigma_raid.h` |
| 5 | Full-disk AES-256-GCM encryption (dm-crypt level) | dm-crypt / LUKS2 | `kernel/crypto/sigma_cryptfs_real.h` |
| 6 | DRM/KMS GPU driver model | Linux DRM | `kernel/drivers/gpu/sigma_drm.h` |
| 7 | PCM audio pipeline | ALSA PCM | `kernel/drivers/audio/sigma_pcm.h` |
| 8 | MAC policy enforcement engine (replaces stub) | SELinux policy.33 | `kernel/security/sigma_mac_engine.h` |
| 9 | Container namespace orchestrator | Linux namespaces | `kernel/ns/sigma_ns_orch.h` |
| 10 | NVMe driver shard | Linux nvme | `kernel/drivers/nvme/sigma_nvme.h` |
| 11 | Init shard — s6-style supervised process tree | s6 / runit | `kernel/drivers/init/sigma_init_shard.h` |
| 12 | WASM browser runtime port | wasmtime / v86 | `browser/sigma_wasm_bridge.c` |
| 13 | sigma-web Progressive Web App shell | Electron / Tauri | `userland/apps/sigma-web/` |
| 14 | Network relay daemon | socat / chisel | `sigmad/netrelay/main.go` |

---

## Round 12 — eBPF VM · IPC Tracer · ACPI Power · AHCI · LVM · Professional Apps

| # | Improvement | Inspired By | File |
|---|---|---|---|
| 1 | eBPF VM + verifier (safe kernel programmability) | Linux eBPF | `kernel/ebpf/sigma_ebpf_vm.h` |
| 2 | IPC tracer — strace-equivalent for sigma-bus | strace / bpftrace | `tools/sigma-trace/sigma_ipc_tracer.h` |
| 3 | ACPI power state daemon | acpid | `sigmad/acpid/main.go` |
| 4 | AHCI SATA shard | Linux libata | `kernel/drivers/ahci/sigma_ahci.h` |
| 5 | Driver hot-plug manager (udev equivalent) | udev / mdev | `sigmad/hotplug/sigma_hotplug_mgr.h` |
| 6 | LVM — logical volume management | Linux LVM2 | `kernel/fs/sigma_lvm.h` |
| 7 | CodeExplorer React app + AST data backend | LSP / Tree-sitter | `userland/apps/code-explorer/` |
| 8 | sigma-gov — India e-governance portal | DigitalIndia APIs | `userland/apps/sigma-gov/` |
| 9 | sigma-realty — property management app | India real estate | `userland/apps/sigma-realty/` |
| 10 | sigma-agri / sigma-edu / sigma-bank / sigma-labour | India sector apps | `userland/apps/sigma-{agri,edu,bank,labour}/` |

---

## Round 13 — Compositor · CryptFS Fix · IPC SHM · A11y · Cloud Sync · Bench

| # | Improvement | Inspired By | File |
|---|---|---|---|
| 1 | Zenith Wayland compositor (VRR, HDR, multi-monitor, animations) | Sway / KWin / Mutter | `userland/compositor/sigma_compositor.h` |
| 2 | **CryptFS real AES-256-GCM (fixes Issue #44 zero-key stub)** | dm-crypt / fscrypt / LUKS2 | `kernel/crypto/sigma_cryptfs_real.cpp` |
| 3 | Shared memory, named pipes, message queues | POSIX IPC / Fuchsia VMO | `kernel/ipc/sigma_shm.h` |
| 4 | Transparent Huge Page API (2 MiB + 1 GiB, khugepaged) | Linux THP / FreeBSD superpages | `kernel/mm/sigma_hugepage.h` |
| 5 | Privacy-respecting opt-in telemetry daemon (PII scrub, TLS 1.3) | Ubuntu apport / Fedora ABRT | `sigmad/telemetry/main.go` |
| 6 | Accessibility framework — AT-SPI2, TTS, magnifier, WCAG 2.2 AA | AT-SPI2 / UIAutomation | `userland/accessibility/sigma_a11y.h` |
| 7 | E2E encrypted cloud sync (Argon2id key, AES-256-GCM chunks) | Nextcloud / Syncthing | `sigmad/cloudsync/main.go` |
| 8 | System benchmark suite (CPU/mem/disk/net/boot/kernel) | phoronix / sysbench / fio | `tools/sigma-bench/sigma_bench.sh` |

## Round 14 — Interactive Roadmap SPA · WPA3 Impl · Net Stack Tests · CMakeLists

| # | Improvement | Inspired By | File |
|---|---|---|---|
| 1 | **Interactive Utilities Roadmap SPA** — 42 utilities, 5 phases, live filter/search, dependency tracking | GitHub Projects / Linear | `userland/roadmap/index.html` |
| 2 | WPA3/SAE full implementation (hunting-and-pecking, dragonfly key exchange) | IEEE 802.11-2020 | `net/wifi/sigma_wpa3.cpp` |
| 3 | Network stack unified init (`sigma_net_init`, config defaults, error strings) | lwIP / gVisor netstack | `net/sigma_net.cpp` |
| 4 | 22-test network stack unit test suite (TLS, DNS, DHCP, WPA3, net init) | Google Test / CTest | `tests/net/test_net_stack.cpp` |
| 5 | CMakeLists for network stack library + test executable | CMake best practices | `net/CMakeLists.txt` |
| 6 | `package-lock.json` pinning Electron 42.5.0, ESLint 10.5.0, Vite 8.1.0, Vitest 4.1.9 | npm lockfile | `package-lock.json` |

## Round 15 — Firewall · Watchdog · Metrics · Shell · Rollback · OOM · MM Tests

| # | Improvement | Inspired By | File |
|---|---|---|---|
| 1 | Stateful packet filter + NAT engine | OpenBSD pf / nftables | `kernel/net/sigma_firewall.h` |
| 2 | Hardware + software watchdog daemon (WDT pet + daemon health) | Linux watchdog / systemd-watchdog | `sigmad/watchdog/main.go` |
| 3 | Prometheus-compatible metrics exporter (CPU/mem/load) | node_exporter / collectd | `sigmad/metrics/main.go` |
| 4 | POSIX shell (sigma_sh) — pipelines, I/O redir, built-ins, history | dash / busybox sh | `userland/shell/sigma_sh.c` |
| 5 | Package + OS generation rollback engine (NixOS-style) | NixOS generations / rpm-ostree | `userland/pkg/sigma_rollback.cpp` |
| 6 | Out-of-Memory killer with per-process score + cgroup integration | Linux OOM / Android LMKD | `kernel/mm/sigma_oom.h` |
| 7 | Memory management regression tests (mmap, mprotect, THP, OOM) | Linux kselftest/mm | `tests/kernel/test_mm.sh` |

---

## Round 16 — Full TLS 1.3 · DNS Resolver · DHCP Client Implementations

| # | Improvement | Inspired By | File |
|---|---|---|---|
| 1 | TLS 1.3 with HKDF extract/expand/expand-label, Kyber hybrid handshake | RFC 8446 + draft-ietf-tls-hybrid | `net/tls/sigma_tls.cpp` |
| 2 | DNS resolver: UDP/TCP/DoH, DNSSEC, LRU cache, name encode/decode | RFC 1035 / 8484 (DoH) | `net/dns/sigma_dns.cpp` |
| 3 | DHCP client: RFC 2131 state machine, option builder, IP helpers | RFC 2131 / 2132 | `net/dhcp/sigma_dhcp.cpp` |

---

*See also: [Gap Analysis](Gap-Analysis) · [Feature Roadmap](Feature-Roadmap) · [Architecture Overview](Architecture-Overview)*
