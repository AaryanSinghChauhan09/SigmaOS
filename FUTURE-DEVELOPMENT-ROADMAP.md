# SigmaOS Future Development Roadmap

This document outlines the strategic future development roadmap for **SigmaOS** to achieve parity with, absorb, and leapfrog mainstream Linux distributions (Debian, Arch Linux, Ubuntu, Fedora, Gentoo, and NixOS).

---

## 1. Installer Ecosystem
### Minimal / Netboot Installers
- **Objective**: Develop ultra-lightweight bootable network images (< 50MB) to allow rapid deployments over PXE/HTTP.
- **Competitor Inspiration**: Debian Netinst, Arch Linux Netboot.
- **SigmaOS Implementation**: A minimal kernel + RAM disk containing the `sigpkg` package manager that fetches additional software on-demand.

### Custom Installation Profiles
- **Objective**: Implement installer scripts supporting tailored profiles during first boot.
- **Profiles**:
  - `Server / Cloud Minimal`: Headless microkernel, zero-trust network policies, only core daemons.
  - `Desktop / Workstation`: Full Smithay-based Zenith graphical environment, multimedia stack, audio drivers.
  - `RTOS / Edge`: Embedded real-time scheduler, low-overhead hardware interfaces.

---

## 2. System Services & Init Ecosystem
### Init Diversity
- **Objective**: Implement a modular supervisor daemon to support multiple init paradigms (event-driven, sequential, and parallel).
- **Competitor Inspiration**: systemd, OpenRC, runit.
- **SigmaOS Implementation**: Provide clean, sandboxed supervisors under a unified service supervisor format with backwards-compatible openrc-style scripts and systemd-style unit shims.

### Unified Service Manager
- **Objective**: Establish `sigmctl` as the unified controller for starting, stopping, auditing, and parallelizing system services.
- **Capabilities**:
  - Topological dependency resolution for ultra-fast, parallel boot times.
  - Automatic sandbox-isolation for every spawned system daemon.

---

## 3. Advanced Networking Utilities
### Advanced CLI Utilities
- **Objective**: Introduce a robust suite of native networking commands replacing legacy tools.
- **Utilities**:
  - `signet ip`: Manage interface addresses, routing tables, and interface states.
  - `signet filter`: Configure Zero-Trust policies and packet-filtering rules (equivalent to nftables).
  - `signet tunnel`: Native wireguard and proxy tunneling configuration tool.

### Integrated Wireless & VPN
- **Objective**: Built-in WiFi management with WPA3-Personal/Enterprise and automated OpenVPN/WireGuard profiles.

---

## 4. Package Ecosystem Depth
### Meta-Packages & Virtual Packages
- **Objective**: Implement package grouping and abstraction inside the `sigpkg` resolver.
- **Mechanism**:
  - **Meta-Packages**: Bundles like `sigmaos-desktop` or `sigmaos-dev-toolkit` to install large software suites with a single transaction.
  - **Virtual Packages**: Abstract dependencies like `display-server` (satisfied by either wayland-zenith or x11-shim).

### Developer Build Tooling (`sigbuild`)
- **Objective**: Provide a deterministic, reproducible build environment for packagers.
- **Features**: Cleanroom sandboxed compilation, auto-generated SBOMs (Software Bill of Materials), and GPG package signing.

---

## 5. Kernel & Module Ecosystem
### Dynamic Kernel Module Loading (`modprobe`)
- **Objective**: Support secure, dynamic loading and unloading of kernel modules (`.skm` format) at runtime.
- **Security**: Strict signature validation against the public key stored in the secure TPM chip.

### Multiple Kernel Variants
- **Objective**: Compile and maintain specialized kernel builds for distinct workloads:
  - `sigmaos-hardened`: Strict MAC enforcement, KASLR, SMAP/SMEP, and minimized syscall tables.
  - `sigmaos-rt`: Real-time scheduler with PREEMPT_RT deterministic latencies.
  - `sigmaos-lowlatency`: Tuned CPU governor and timer ticks for gaming and desktop workstations.

---

## 6. Desktop & Multimedia Stack
### Display Server Architecture (Wayland Zenith)
- **Objective**: Establish the Smithay-based Zenith compositor as the default, hardware-accelerated display protocol.
- **Features**: Zero-copy page flipping, per-monitor DPI scaling, and sandbox application containment.

### Native Audio & PipeWire Equivalence
- **Objective**: Implement a low-latency, dynamic routing audio server supporting real-time audio and video streams with full compatibility shims for ALSA and PulseAudio applications.

---

## 7. QA, Automated CI, & Certification
### Scale-Out Automated QA Pipeline
- **Objective**: Simulate boot, driver load, and application suites across thousands of hardware configurations using automated QEMU and bare-metal test matrices.

### Package CI & Community QA Lifecycle
- **Objective**: Automated package build checks with security scanners and public Release Candidate (RC) feedback cycles.
