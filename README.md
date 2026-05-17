# Σ SigmaOS: The Sovereign Computational Lattice

SigmaOS Zenith v15.0 is an industrial-grade, microkernel-based operating system designed for the post-quantum era. It provides absolute user sovereignty through a sharded, silicon-direct architecture.

## 🌟 Niche Parity Certification (v15.0)

SigmaOS has achieved functional parity and superiority across all major Linux industrial niches:

* **Gaming**: SteamOS-equivalent GPU scheduling.
* **Performance**: Clear Linux-equivalent auto-tuning.
* **IoT**: RPi-Distro-equivalent GPIO/Sensor management.
* **Reproducibility**: NixOS-equivalent declarative manifests.
* **Enterprise**: Ubuntu-equivalent hardware regression matrix.
* **Infrastructure**: KVM-equivalent Type-1 Hypervisor.

## 🚀 Key Release Features

* **Sovereign Choice**: Select your profile (Legacy, Modern, Cloud, RTOS, Forensic, Enterprise) at install time.
***sigma-pkg**: Professional package manager with incremental**Delta Updates**.
***sigma-cli**: Industrial CLI for shard management and**Forensic Snapshot Diffing**.
***Zenith Desktop**: Polished, glassmorphic UI with native**Adaptive UI Scaling**.

## ⚖️ Our Principles

1. **Sovereignty**: The user owns the silicon. No opaque telemetry or forced updates.
2. **Amnesic Persistence**: Zero data remanence across the lattice.
3. **Shard Autonomy**: Isolated, PQC-sealed singletons ensure fault tolerance.
4. **Transparency**: Open-source roadmap, Wiki-first documentation, and real-time health metrics.

## 🛠 OS Profiles

SigmaOS supports 8 specialized industrial profiles, including:

* **Monolithic**: High-performance workstation.
* **RTOS**: Safety-critical industrial control.
* **Forensic**: PQC-hardened audit & recovery.
* **Enterprise**: Governance & compliance certification.
* **Hypervisor**: Type-1 hardware-accelerated virtualization.

## 🛠 Getting Started

### Build Prerequisites

* `gcc` (>= 11.0) or `clang` (>= 14.0) for C++20 support
* `make` and `cmake`
* `qemu-system-x86_64` for emulation testing

### Compilation Steps

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
mkdir build && cd build
cmake .. && make -j$(nproc)
```

### Quick Demo (Run via QEMU)

```bash
# Boot the microkernel locally
qemu-system-x86_64 -kernel build/sigmaos_kernel.bin -serial stdio
```

### Supported Hardware

* **x86_64**: Standard BIOS and UEFI boot (with Secure Boot disabled for untrusted environments).
* **ARM64**: Raspberry Pi 4 and generic ARM virtual platforms.
* **RISC-V**: Experimental SiFive Unmatched support.

For detailed documentation, visit our [GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS.wiki.git) and read the [CONTRIBUTING.md](CONTRIBUTING.md) guide.

## 🎯 Strategic Roadmap

* **ALPHA (v15.0 - DONE)**: Industrial Foundation & Shard Hardening.
* **BETA (v15.1 - DONE)**: Zenith Desktop Port & Niche Parity Certification.
* **STABLE (v1.0 - CURRENT)**: Global Ecosystem & [Contributor Roadmap](CONTRIBUTOR_ROADMAP.md).

_Σ SIGMAOS: Absolute Sovereignty. Singularity Achieved._
