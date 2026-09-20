# 🎯 SIGMAOS TRUTHFUL PRODUCT BASELINE & STRATEGIC EXECUTION ROADMAP

## Executive Summary & Strategic Shift

SigmaOS has established an ambitious architectural vision and rich subsystem abstractions. However, to evolve from a prototype into a usable, reproducible, hardware-tested operating system distribution comparable to Linux or BSD, SigmaOS is executing a pivotal strategic shift:

> **Core Strategic Directivity:** Stop optimizing for "supporting every distro concept simultaneously." First deliver a reliable, minimal, x86_64 reference system with a stable ABI, real installer, bare-metal hardware drivers, signed package repository, transactional update mechanism, and daily-use desktop applications.

---

## PART 1: PRIORITY 0 — TRUTHFUL PRODUCT BASELINE & CAPABILITY MATRIX

To eliminate discrepancy between subsystem interface claims and bare-metal functionality, SigmaOS adopts a 5-tier machine-readable capability status taxonomy:

1. `implemented` — Full production implementation verified by automated hardware or QEMU tests.
2. `partial` — Core logic exists with test coverage, but depends on stubs or limited hardware backends.
3. `prototype` — Interface and basic scaffold present; non-functional stub or simulated return values.
4. `design_proposal` — Architectural specification and type definitions drafted, no execution logic.
5. `not_started` — Planned milestone feature without codebase representation.

### TOML Machine-Readable Capability Matrix (`capability_matrix.toml`)

```toml
[feature.uefi_boot]
status = "partial"
tested_on = ["qemu-x86_64-ovmf"]
required_for_release = true
blocker_description = "Initramfs parsing requires bare-metal memory handoff verification."

[feature.wayland_compositor]
status = "prototype"
tested_on = ["qemu-virtio-gpu"]
required_for_release = true
blocker_description = "Wayland protocol engine uses software frame stubs; DRM/KMS backend initialization returns mock success."

[feature.sigma_pkg]
status = "partial"
tested_on = ["qemu-x86_64"]
required_for_release = true
blocker_description = "CLI path fallback accepts synthetic demo metadata when remote mirror is unreachable."

[feature.installer_partitioning]
status = "prototype"
tested_on = ["qemu-x86_64"]
required_for_release = true
blocker_description = "Installer disk partitioning operates in dry-run mode without invoking real fdisk/parted/mkfs."

[feature.nvme_driver]
status = "partial"
tested_on = ["qemu-nvme"]
required_for_release = true
blocker_description = "PCI MMIO queue setup functional; hardware controller error recovery loop pending."

[feature.wifi_networking]
status = "not_started"
tested_on = []
required_for_release = true
blocker_description = "802.11 mac80211 driver framework and WPA3 supplicant not implemented."

[feature.atomic_updates]
status = "partial"
tested_on = ["qemu-x86_64"]
required_for_release = true
blocker_description = "Generation pointer rollback implemented; post-reboot health check handshake pending."
```

---

## PART 2: THE 12 DOMAIN BLUEPRINTS & RESOLUTION ROADMAPS

### 1. Boot and Kernel Foundations
* **Gap Analysis:** Current kernel entry initializes hard-coded address scaffold (`0x8000`, `0x9000`).
* **Execution Plan:**
  1. Implement complete boot-to-userspace path: UEFI boot protocol → ACPI table parsing → SMP CPU AP startup → LAPIC/IO-APIC interrupt routing → PML4 page tables with SMEP/SMAP.
  2. Implement core syscall ABI (`exec`, `exit`, `wait`, `read`, `write`, `open`, `close`, `mmap`, `poll`).
  3. Launch first statically-linked init process (`/sbin/init`) from initramfs.

### 2. Userspace and POSIX Compatibility (Hybrid Strategy C)
* **Gap Analysis:** Missing complete dynamic linking, pthread ABI, and libc symbol coverage.
* **Execution Plan:**
  1. Adopt **Strategy C (Hybrid Userspace Compatibility Architecture)**: Maintain `sigma-musl` POSIX compatibility shim for Linux dynamic ELF binaries while natively supporting WASM/WASI, Flatpak sandboxes, and OCI microVMs.
  2. Provide complete termios, process groups, job control, signals, and pseudo-terminals (PTYs).

### 3. Real Hardware Enablement Strategy
* **Gap Analysis:** Relies primarily on QEMU VirtIO drivers; physical GPUs, Wi-Fi, audio, and laptop power zones scheduled.
* **Execution Plan:**
  1. Certify two reference target platforms: 1 Laptop (e.g., ThinkPad X1 / Dell XPS x86_64) and 1 Desktop (AMD Ryzen + Radeon GPU).
  2. Implement native PCI hotplug, USB 3.x xHCI, DRM/KMS atomic modesetting, and ACPI thermal/battery zones.
  3. Driver isolation: Host legacy Linux drivers inside lightweight microVM compatibility wrappers until native Rust drivers mature.

### 4. Graphics and Zenith Desktop Integration
* **Gap Analysis:** Compositor methods present success stubs without real Wayland DRM/KMS frame rendering.
* **Execution Plan:**
  1. Build Zenith Compositor MVP: Minimal wlroots-inspired Wayland protocol engine → 1 DRM output → software fallback renderer + VirtIO-GPU driver → evdev/libinput input routing.
  2. Integrate desktop portals, XWayland compatibility bridge, clipboard, screenshots, and font shaping (FreeType/HarfBuzz shims).

### 5. Init, Service Management, and System Integration
* **Gap Analysis:** Exposes stubs for systemd, runit, OpenRC, and s6 without a single canonical supervisor.
* **Execution Plan:**
  1. Establish `sigmainit` as the single canonical service supervisor built on cgroups v2 resource slices (`system.slice`, `user.slice`).
  2. Implement declarative service unit manifests with dependency graph ordering, socket activation, automatic restart policies, and binary structured logging.

### 6. Package Management and Software Distribution
* **Gap Analysis:** `sigma-pkg` contains synthetic metadata fallbacks and demo signatures.
* **Execution Plan:**
  1. Enforce **Fail-Closed Verification**: Reject any unsigned package or unresolved checksum. Eliminate demo fallback metadata.
  2. Build TUF (The Update Framework) repository client with threshold signing, key rotation, and CAS (Content-Addressed Storage) package store.
  3. Import foreign package formats (.deb, .rpm, Arch .pkg.tar.zst) via sandboxed cleanroom conversion into native Sigma packages.

### 7. Atomic Updates, A/B Deployment, and Recovery
* **Gap Analysis:** Update mechanism handles generation switching but lacks bare-metal boot success verification.
* **Execution Plan:**
  1. Implement A/B slot transaction lifecycle: `Download → Verify TUF → Stage → Fsync → Toggle Boot Slot → Reboot`.
  2. Post-boot health check agent: If init fails or health check times out, hardware watchdog triggers automatic fallback to prior generation.
  3. Power-loss resilience testing: Validate atomic journal survival under forced power interruption during update staging.

### 8. Storage and Filesystem Architecture
* **Gap Analysis:** VFS abstractions present, but POSIX file locks, ACLs, and power-loss recovery require verification.
* **Execution Plan:**
  1. Select **ext4** as default rootfs for baseline reliability and **Btrfs** as primary choice for native CoW snapshots and subvolumes.
  2. Implement POSIX extended attributes (xattrs), ACLs, mandatory locking, and TRIM/discard for NVMe/SSD wear leveling.

### 9. Security Model & Real OS Enforcement
* **Gap Analysis:** Contains abstractions for Linux Landlock, OpenBSD pledge/unveil, and FreeBSD Capsicum simultaneously.
* **Execution Plan:**
  1. Define unified `SandboxBackend` Rust trait mapping generic filesystem/network policies to host primitives.
  2. Enforce real user/group DAC permission gates, PAM-equivalent authentication, PQC Dilithium-5 kernel module signing, and memory protection (W^X, SMEP, SMAP).

### 10. Networking Stack & Infrastructure
* **Gap Analysis:** High-level abstractions for XDP and WireGuard exist alongside basic socket stubs.
* **Execution Plan:**
  1. Deploy production-grade dual-stack IPv4/IPv6 TCP/IP network stack with DHCP client, DNSSEC resolver, and OpenBSD PF-inspired firewall rule parser.
  2. Implement native WireGuard VPN tunnel driver and Netlink-equivalent network configuration IPC interface.

### 11. Developer Ecosystem & SDK
* **Gap Analysis:** SDK and developer toolchains remain in planning stage.
* **Execution Plan:**
  1. Release `sigma-sdk`: C/Rust cross-compilation toolchain, `sigma-build` recipe engine, and LLDB debugger support.
  2. Provide pre-packaged development container images and VSCode/Neovim Language Server Protocol (LSP) integrations.

### 12. Observability, Diagnostics, and Release Engineering
* **Gap Analysis:** Standalone unit tests pass on data structures, but end-to-end QEMU integration tests are needed.
* **Execution Plan:**
  1. Deploy `sigma-diagnose` tool generating sanitized system state, loaded driver maps, kernel ring buffer logs, and panic trace bundles.
  2. Establish 15-stage CI Matrix including QEMU headless boot, installer execution, package update, forced power-loss, and reproducible ISO build verification.

---

## PART 3: RECOMMENDED 5-PHASE EXECUTION ORDER

```
==================================================================================================
PHASE 1: "BOOTABLE AND HONEST" (Months 1-3)
- UEFI/QEMU boot to serial console
- Statically-linked init process & shell
- Fail-closed package verifier (zero demo metadata)
- Reproducible x86_64 ISO build pipeline

PHASE 2: "INSTALLABLE" (Months 4-6)
- Bare-metal disk partitioning & ext4/Btrfs formatting
- Bootloader configuration & system rootfs deployment
- User account creation & network setup
- Recovery environment & rescue shell

PHASE 3: "USABLE DESKTOP" (Months 7-9)
- Zenith Compositor MVP (1 DRM output, evdev input, terminal)
- Audio routing (PipeWire compatible) & Wi-Fi supplicant
- Default applications (browser, file manager, text editor)

PHASE 4: "SAFE TO UPDATE" (Months 10-12)
- TUF signed package repositories
- A/B transactional updates with automatic boot rollback
- Power-loss resilience verification
- Offline installation & recovery ISO

PHASE 5: "COMPETITIVE" (Months 13+)
- Hybrid Linux/WASM compatibility layers
- Multi-arch hardware expansion (ARM64, RISC-V)
- Enterprise security compliance (FIPS 140-3)
- Ecosystem SDK & developer portal
==================================================================================================
```

---
*End of Truthful Product Baseline & Strategic Execution Roadmap.*
