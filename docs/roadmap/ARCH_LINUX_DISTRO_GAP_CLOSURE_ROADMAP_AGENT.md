# PR Specification: Arch Linux Distro Gap Closure Roadmap for AI Agents

## Metadata
- **PR Title:** `docs(roadmap): Arch Linux Distro Gap Closure Roadmap for AI Agents`
- **PR Branch:** `feat/arch-linux-gap-closure-roadmap-agent`
- **Status:** Approved / Implementation Ready
- **Target Subsystem:** Arch Linux Parity & Distro Ecosystem (`src/distro/`, `src/sigpkg/`, `docs/roadmap/`)

---

## 1. Overview & Architectural Scope

This specification defines the AI Agent development roadmap for filling gaps between **SigmaOS** and **Arch Linux**. It provides a structured, PR-formatted roadmap for AI agents to implement, verify, and maintain Arch Linux ecosystem parity across package management, build systems, initramfs generators, bootloaders, linter tooling, and automated installation profiles.

---

## 2. Core Arch Linux Components & PR Roadmap

### 2.1 Pacman 6 Package Manager Engine (`Pacman6Engine`)
- **Key Features:**
  - Parallel package downloads with HTTP/2 multiplexing.
  - SAT dependency solver for complex multi-repo graph resolution.
  - Web-of-Trust GPG/PQC digital signature verification (`pacman-key`).
  - `.pkg.tar.zst` multi-threaded zstd archive unpacking.
- **Inspiration:** Arch Linux Pacman 6, ALPM library.

### 2.2 AUR v5 RPC & Helper Engine (`AurV5RpcEngine`)
- **Key Features:**
  - Arch User Repository (AUR) v5 RPC JSON API interface (`/rpc/?v=5&type=search`).
  - Subuid/subgid unprivileged containerized build sandboxing.
  - Automated dependency tree expansion for AUR PKGBUILDs.
  - Integration with `yay`, `paru`, `pikaur`, and `aurman` CLI syntax.
- **Inspiration:** Arch Linux AUR v5, `yay`, `paru`.

### 2.3 Makepkg PKGBUILD Engine (`MakepkgPkgbuildEngine`)
- **Key Features:**
  - Complete `PKGBUILD` bash syntax parser (`pkgname`, `pkgver`, `pkgrel`, `arch`, `depends`, `makedepends`).
  - Standard function lifecycle execution: `prepare()`, `build()`, `check()`, `package()`.
  - Automated `.SRCINFO` manifest generator.
  - Multi-threaded reproducible packaging with `makepkg.conf` optimization flags.
- **Inspiration:** Arch Linux `makepkg`, `pacman-contrib`.

### 2.4 Mkinitcpio Initramfs Generator Engine (`MkinitcpioInitramfsEngine`)
- **Key Features:**
  - Early-boot CPIO archive builder with zstd compression.
  - Modular hook resolver (`base`, `udev`, `autodetect`, `modconf`, `block`, `filesystems`, `fsck`).
  - Microcode injection (`intel-ucode.img`, `amd-ucode.img`).
  - Encrypted LUKS2 root volume hook generation.
- **Inspiration:** Arch Linux `mkinitcpio`, `dracut`.

### 2.5 systemd-boot & Unified Kernel Image (UKI) Engine (`SystemdBootUkiEngine`)
- **Key Features:**
  - EFI system partition (ESP) `loader.conf` configuration generator.
  - Unified Kernel Image (UKI) creation bundling stub, kernel, initramfs, and cmdline into a single PE binary.
  - Secure Boot PCR signing and enrolment.
- **Inspiration:** Arch Linux `systemd-boot`, `ukify`.

### 2.6 Archinstall Automated Installer Profile Engine (`ArchinstallProfileEngine`)
- **Key Features:**
  - Declarative JSON profile generator for automated system disk provisioning.
  - Btrfs subvolume layout (`@`, `@home`, `@log`, `@pkg`) with zstd transparent compression.
  - Automated network configuration (`NetworkManager`, `systemd-networkd`).
- **Inspiration:** Arch Linux `archinstall`.

### 2.7 Namcap Linter & Quality Assurance Engine (`NamcapLinterEngine`)
- **Key Features:**
  - Automated linter for `PKGBUILD` files and built `.pkg.tar.zst` packages.
  - Detection of missing runtime dependencies, redundant linkage, and FHS directory violations.
- **Inspiration:** Arch Linux `namcap`.

### 2.8 Reflector Mirrorlist Optimization Engine (`ReflectorMirrorlistEngine`)
- **Key Features:**
  - GeoIP-based Arch Linux mirror discovery.
  - Dynamic mirror ranking based on latency, completion rate, and download throughput.
  - Automated `/etc/pacman.d/mirrorlist` generation.
- **Inspiration:** Arch Linux `reflector`.

---

## 3. Evidence Status Vocabulary for AI Agents

When implementing features from this roadmap, AI agents must classify implementation status using:
- `unimplemented`: No credible implementation found in codebase.
- `model-only`: Unit-tested model exists, but no connected runtime path.
- `integration-pending`: Code exists but is not connected end-to-end to kernel boot/runtime.
- `qemu-verified`: Passes automated QEMU boot and runtime execution tests.
- `hardware-verified`: Tested on named physical hardware with serial/kernel logs on record.

---

## 4. Agent Verification Pipeline

Every AI agent PR closing Arch Linux gaps must pass:
```bash
rustc --test --edition=2021 src/distro/arch_linux_advancements_v43_pr.rs -o build/test_arch_v43 && ./build/test_arch_v43
rustc --test --edition=2021 src/sigpkg/arch_pacman_engine.rs -o build/test_arch_engine && ./build/test_arch_engine
./run_sigma_tests.sh
```
