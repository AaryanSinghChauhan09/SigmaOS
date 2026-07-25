# 🗺️ SigmaOS Open-Source Absorption Roadmap & Strategy

## Executive Summary
SigmaOS systematically absorbs critical open-source software to eliminate external dependencies and create a fully sovereign, capability-gated, `#![no_std]` Rust/Zig/Nim operating system. By replacing monolithic, legacy C utilities with memory-safe, zero-allocation alternatives, SigmaOS delivers superior security, performance, and operational continuity.

---

## 🏛️ Definitive Map of Open-Source Projects → SigmaOS Sovereign Replacements

| External Tool | SigmaOS Sovereign Replacement | Status | Priority | Inspired By |
| :--- | :--- | :--- | :--- | :--- |
| **GNU Coreutils** | `sigma-core-utils` (Rust) | 🔄 In Progress | P0 | BusyBox, uutils/coreutils |
| **BusyBox** | `sigma-core-utils` (Rust) | 🔄 In Progress | P0 | BusyBox |
| **Bash / Zsh** | `sigma-sh` (Rust) | 🔄 In Progress | P0 | Fish shell, elvish |
| **systemd** | `sigma-init` (Rust) | 🎯 Planned | P1 | OpenRC, s6, dinit |
| **OpenRC** | `sigma-init` (Rust) | 🎯 Planned | P1 | runit |
| **syslog / journald** | `sigma-log` (Rust) | 🎯 Planned | P1 | — |
| **cron** | `sigma-cron` (Rust) | 🎯 Planned | P2 | — |
| **sudo** | `sigma-priv` (capability-based) | 🎯 Planned | P1 | doas |
| **man pages** | `sigma-doc` | 🎯 Planned | P2 | tldr, tealdeer |
| **ext4** | `SovereignFS` (journaling, POSIX) | 🎯 Planned | P0 | xv6, Minoca OS |
| **btrfs** | `SovereignFS` (snapshots, CoW) | 🎯 Planned | P1 | btrfs, ZFS |
| **ZFS** | `sigma-zfs` integration | 🎯 Planned | P2 | OpenZFS |
| **LVM** | `sigma-volume` | 🎯 Planned | P2 | — |
| **mdadm (RAID)** | `sigma-raid` | 🎯 Planned | P2 | — |
| **LUKS** | `sigma-crypt` (dm-crypt sovereign) | 🎯 Planned | P1 | LUKS2 |
| **VirtIO drivers** | `sigma-virtio` | 🎯 Planned | P1 | Hermit-rs, Unikraft |
| **NVMe driver** | `sigma-nvme` | 🎯 Planned | P0 | Linux NVMe |
| **USB/HID stack** | `sigma-usb` | 🎯 Planned | P1 | — |
| **GCC / Clang** | `sigma-cc` (Rust/Zig frontend) | 🎯 Planned | P1 | LLVM, zig cc |
| **CMake / Meson** | `sigpkg` build (Rust) | 🔄 In Progress | P1 | Zig build system |
| **Make / Ninja** | `sigma-make` (Rust) | 🎯 Planned | P2 | just, ninja |
| **Git** | `SigmaVCS` | 🎯 Planned | P1 | jj (Jujutsu), fossil |
| **GDB** | `sigma-debug` | 🎯 Planned | P2 | — |
| **Valgrind** | `sigma-memcheck` | 🎯 Planned | P2 | — |
| **strace / perf** | `sigma-trace` | ✅ Implemented | P0 | eBPF |
| **Docker** | `sigma-container` | 🎯 Planned | P1 | nanos, gvisor |
| **Kubernetes** | `sigma-orchestrator` | 🔄 In Progress | P1 | Unikraft, nomad |
| **QEMU / KVM** | `sigma-hypervisor` | 🎯 Planned | P2 | — |
| **Vagrant** | `sigma-vm` | 🎯 Planned | P3 | — |
| **OpenSSH** | `sigma-ssh` (Rust) | 🎯 Planned | P0 | russh, Dropbear |
| **curl / wget** | `sigma-fetch` (Rust) | 🎯 Planned | P0 | — |
| **Firefox / Chromium** | `sigma-browse` | 🔄 In Progress | P1 | Ladybird, NetSurf |
| **Tor Browser** | `sigma-anon` | 🎯 Planned | P2 | Whonix |
| **WireGuard** | `sigma-vpn` (native) | 🔄 In Progress | P0 | WireGuard-rs |
| **OpenVPN** | `sigma-vpn` | 🎯 Planned | P2 | — |
| **nmap** | `sigma-scan` | 🎯 Planned | P2 | — |
| **Wireshark** | `sigma-capture` | 🎯 Planned | P3 | — |
| **iptables / nftables** | `sigma-shield` (BPF) | ✅ Implemented | P0 | eBPF, XDP |
| **dnsmasq** | `sigma-dns` (DoH) | ✅ Implemented | P0 | — |
| **apt / dpkg** | `sigpkg` (Rust) | 🔄 In Progress | P0 | Wolfi OS, apk |
| **rpm / yum** | `sigpkg` (Rust) | 🔄 In Progress | P0 | — |
| **pacman** | `sigpkg` (Rust) | 🔄 In Progress | P0 | — |
| **Snap / Flatpak** | `sigma-sandbox` | 🎯 Planned | P1 | Nanos, gVisor |
| **Nix** | `sigpkg --reproducible` | 🎯 Planned | P1 | NixOS, Wolfi OS |
| **Cargo** | `sigpkg` (natively wraps) | ✅ Implemented | P0 | — |
| **npm / pip** | `sigpkg` plugin:lang | 🎯 Planned | P2 | — |
| **SELinux** | `sigma-sandbox` (capability) | 🎯 Planned | P0 | Capsicum |
| **AppArmor** | `sigma-sandbox` | 🎯 Planned | P0 | — |
| **OpenSSL** | `sigma-crypto` (Ada/SPARK) | 🔄 In Progress | P0 | libsodium, rustls |
| **GnuTLS** | `sigma-crypto` | 🔄 In Progress | P0 | — |
| **libsodium** | `sigma-crypto` | 🔄 In Progress | P0 | libsodium |
| **KeePass** | `sigma-vault` | 🎯 Planned | P1 | — |
| **Bitwarden** | `sigma-vault` | 🎯 Planned | P1 | — |
| **Auditd** | `sigma-audit` | ✅ Implemented | P0 | BPF audit |
| **Fail2ban** | `sigma-guard` | 🎯 Planned | P2 | — |
| **ClamAV** | `sigma-scan` (behavioral) | 🎯 Planned | P3 | — |
| **TPM tools** | `sigma-tpm` | 🎯 Planned | P1 | tpm2-tools |
| **LibreOffice** | `sigma-write` / `sigma-calc` / `sigma-present` | 🎯 Planned | P2 | — |
| **VLC / MPV** | `sigma-play` | 🎯 Planned | P2 | MPV |
| **GIMP** | `sigma-paint` | 🎯 Planned | P3 | — |
| **Inkscape** | `sigma-draw` | 🎯 Planned | P3 | — |
| **Evince / Okular** | `sigma-view` (PDF) | 🎯 Planned | P2 | — |
| **Thunderbird** | `sigma-mail` | 🎯 Planned | P2 | — |
| **Signal desktop** | `sigma-chat` | 🎯 Planned | P2 | Signal protocol |
| **Matrix client** | `sigma-matrix` | 🎯 Planned | P2 | Matrix.org |
| **Obsidian** | `sigma-notes` | 🎯 Planned | P3 | — |
| **Terminal emulator** | `sigma-term` (Zenith native) | ✅ Implemented | P0 | — |

---

## 📈 Roadmap Phases (12-Month Plan)

### Phase 1: Foundation (Months 1-3)
*   **Weeks 1-4 (Core Infrastructure):** Port `smoltcp`, `libsodium`, `SQLite`, and `Tokio` primitives to `#![no_std]` Rust and integrate into SigmaOS core.
*   **Weeks 5-8 (WASM Foundation):** Establish `Wasmer`, `Wasmtime`, `wasm3`, and `wasi-common` boundaries for isolated third-party app execution.
*   **Weeks 9-12 (Desktop & Security):** Embed `smithay`, `wlroots`, and `egui` to formulate our Wayland window compositor (`Zenith`). Incorporate basic out-of-band `tpm2-tools` for trusted platform integrity attestation.

### Phase 2: Expansion (Months 4-6)
*   **Month 4 (Desktop Expansion):** Enhance keyboard gesture handling, custom rendering layers, and high-DPI scaling configurations.
*   **Month 5 (Services & Storage):** Introduce native zero-copy HTTPS micro-servers (`Caddy`-style integration) and transactional journaling in `SovereignFS`.
*   **Month 6 (Observability):** Deploy metrics collection interfaces and distributed tracing spans (`OpenTelemetry` parity) inside the security audit logging pipeline.

### Phase 3: Optimization (Months 7-9)
*   **Month 7 (Kernel Performance):** Refine buddy allocators, preemption latencies, and minimize memory footprints.
*   **Month 8 (Advanced Networking):** Deploy zero-trust packet filters and kernel-space WireGuard-rs tunnels.
*   **Month 9 (Package Management):** Upgrade `sigpkg` to natively process declarative `SigmaRecipes` with post-quantum NIST Dilithium-5 signatures.

### Phase 4: Innovation (Months 10-12)
*   **Month 10 (AI/ML & Edge):** Integrate background inference state machines and local AI priority scheduling optimization hooks.
*   **Month 11 (Cloud & Storage):** Establish highly secure micro-VM boundary models (`Firecracker`-style) executing custom sandboxed services.

---

## 📊 Technical Performance Targets

### Boot Performance
| Metric | Phase 1 | Phase 2 | Phase 3 | Phase 4 / Target |
| :--- | :--- | :--- | :--- | :--- |
| **Cold boot (NVMe)** | 5s | 3s | 2.5s | **< 2s** |
| **Resume from suspend** | 2s | 1s | 750ms | **< 500ms** |
| **Service startup** | 500ms | 300ms | 200ms | **< 100ms** |

### Memory Efficiency
| Metric | Phase 1 | Phase 2 | Phase 3 | Phase 4 / Target |
| :--- | :--- | :--- | :--- | :--- |
| **Idle memory (desktop)** | 300MB | 250MB | 200MB | **< 150MB** |
| **Idle memory (server)** | 150MB | 120MB | 100MB | **< 64MB** |
| **Per-process overhead** | 5MB | 4MB | 3MB | **< 2MB** |

---

## ⚖️ Legal & Three-Tier Licensing Strategy

To protect the sovereign core of SigmaOS from license infection while utilizing open-source heritage, we apply a strict **Three-Tier Absorption Strategy**:

### 🟢 Tier 1: Direct Integration (Permissive Licenses)
*   **License Type:** MIT, BSD, Apache-2.0, ISC.
*   **Action:** Direct compilation into the codebase with appropriate cargo features and attribution notices.
*   **Examples:** `libsodium` (ISC), `smoltcp` (MIT), `wlroots` (MIT).

### 🟡 Tier 2: Reference-Based Reimplementation (Copyleft Licenses)
*   **License Type:** GPLv2, GPLv3, AGPL.
*   **Action:** Reimplement all core concepts in a "clean-room" environment under `#![no_std]` Rust/Zig/Nim. No copyleft code is integrated directly.
*   **Examples:** `WireGuard` (reimplemented in Rust), Linux driver trees (reimplemented as modular OOP driver traits).

### 🔵 Tier 3: Hybrid Approach (Mixed-License Projects)
*   **License Type:** LGPL, EPL, Mozilla Public License (MPL).
*   **Action:** Wrap the interfaces in isolated, dynamically linked user-space sandboxes, or write a clean-room compatibility shim around the permissive parts.
*   **Examples:** `seL4` microkernel libraries, FUSE userspace layers.
