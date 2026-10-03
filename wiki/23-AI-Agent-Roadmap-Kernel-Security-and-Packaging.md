# AI Agent Development Roadmap: Kernel, Security, & Universal Packaging

This roadmap outlines strategic objectives for AI Agents (**Bolt ⚡**, **Palette 🎨**, and **Sentinel 🛡️**) to enhance the core kernel, security sandboxing, zero-copy networking, and universal package management in SigmaOS.

## 1. Kernel, Security, & Package Management Parity Matrix

| Subsystem | Inspired By | SigmaOS `#![no_std]` Native Parity Engine | Responsible Agent & Objective |
| :--- | :--- | :--- | :--- |
| **Process Scheduler** | Linux EEVDF / BORE | `BoreSchedulerGovernor` | **Bolt ⚡**: Min-latency virtual runtime calculation & burst score scaling |
| **System Supervision** | Systemd / Void Runit | `SovereignInitSupervisor` | **Sentinel 🛡️**: Service dependency graphs, auto-restart, & cgroups v2 control |
| **Process Sandboxing** | OpenBSD Pledge / Unveil | `SovereignCapsicumSandbox` / Pledge | **Sentinel 🛡️**: Enforce path unveil restrict & syscall pledge promises |
| **eBPF Tracing** | Cilium Tetragon | `SovereignTetragonEbpfSecurityEngine` | **Sentinel 🛡️**: Process exec tracing, container tracking, & real-time kill actions |
| **Network Inspection** | Suricata IPS / XDP | `SovereignSuricataIpsEngine` | **Sentinel 🛡️**: Zero-copy packet flow tracking & signature alert/drop actions |
| **WASI JIT Runtime** | Wasmtime / WASI | `SovereignWasmtimeJitRuntimeEngine` | **Bolt ⚡**: WASI preopen directory grants & sandboxed WASM module execution |
| **Universal Packaging** | Apt, Pacman, Dnf, Nix, Apk | `SigmaPkg` / Universal PR Gateway | **Palette 🎨**: Auto-transpile 31 foreign formats to canonical `sovereign-*` PKGs |
| **SIMD Tensor Compiler**| Modular Mojo | `SovereignMojoTensorCompilerEngine` | **Bolt ⚡**: Vectorized MatMul/Conv2D kernel compilation & tile layout memory tuning |

## 2. Universal Packaging Architecture (`SigmaPkg`)

```
+-----------------------------------------------------------------------------------+
|                       SIGMA-PKG UNIVERSAL TRANSPILER                              |
+-----------------------------------------------------------------------------------+
|  Debian (.deb)  | Arch (.pkg.tar.zst) | Fedora (.rpm) | Alpine (.apk) | Nix/Guix |
+-----------------------------------------------------------------------------------+
|           Universal PR Interoperability Engine & SLSA v1.0 Attestations            |
|         Canonical Mapping (e.g., glibc/musl -> sovereign-libc, openssl -> sovereign-ssl) |
+-----------------------------------------------------------------------------------+
|                        Native Bare-Metal SigmaPkg Package                         |
+-----------------------------------------------------------------------------------+
```

### Key AI Agent Development Tasks:
1. **Multi-Format Autodetection**:
   - `SigmaPkg` MUST autodetect 31+ package formats (`.deb`, `.rpm`, `.pkg.tar.zst`, `.apk`, `.ebuild`, `.xbps`, `.pkg`, `.nix`, `.flatpak`, `.snap`, `.appimage`, `.hpkg`, `.ipk`, `.opkg`, etc.) and convert them into pull request (PR) diffs.

2. **SLSA Provenance v1.0 & CycloneDX SBOM Attestation**:
   - Every imported package MUST automatically generate cryptographic build provenance and CycloneDX/SPDX SBOM metadata.

3. **Zero-Trust Privilege Sandboxing**:
   - Maintainer scripts (`preinst`, `postinst`, `PKGBUILD` install hooks) MUST execute within a Landlock v5 LSM, OpenBSD unveil, and FreeBSD Capsicum sandbox matrix.

## 3. Continuous Testing & Quality Guidelines
Before submitting any changes, AI Agents MUST run:
```bash
./run_sigma_tests.sh
```
All standalone test binaries in `build/` must execute with zero failures.
