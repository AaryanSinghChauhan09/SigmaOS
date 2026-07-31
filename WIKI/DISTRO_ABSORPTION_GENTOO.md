# SigmaOS Distro Absorption: Gentoo (Hardware Autotuning & USE Flags Engine)

## 1. Overview
SigmaOS absorbs Gentoo's compiler flag flexibility and source-based optimization engine, automating runtime hardware capability discovery and dynamic JIT kernel module compilation.

## 2. Architecture & Mechanisms
- **Hardware Probing Engine (`sigprobe`)**: Detects precise CPU capabilities (AVX2, AVX-512, AMX, Neon, RISC-V Vector Extensions).
- **USE Flag Profile Engine (`/etc/sigmaos/useflags.conf`)**: Enables modular compilation options per target package.
- **Dynamic Compiler JIT (`sigjit`)**: Compiles optimized native binaries utilizing machine-specific instructions without pre-compiled compromise.

## 3. System USE Flags Specification
```ini
[global]
use = "+avx512 +amx +pqc +ebpf +vulkan +wayland -legacy"

[packages]
sys-kernel/sigma-kernel = "realtime preempt numa bpf"
net-misc/sigwire = "post-quantum dilution aes-gcm-vector"
gui-wm/zenith-desktop = "wayland direct-rendering glassmorphism"
```
