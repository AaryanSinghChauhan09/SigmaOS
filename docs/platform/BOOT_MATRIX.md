# SigmaOS Boot Matrix

## Overview
The Boot Matrix records end-to-end boot validation results across target platform architectures and execution environments.

## Boot Target Matrix

| Architecture | Environment | Image Type | Stage Reached | Boot Duration | Verification Status | Artifact Log |
|---|---|---|---|---|---|---|
| `x86_64` | QEMU / KVM | ISO (`sigmaos.iso`) | `shell_ready` | 0.082s | **PASS** | `build/boot/boot_smoke_result.json` |
| `x86_64` | Bare Metal | USB ISO | `shell_ready` | 1.120s | **PASS** | Log link |
| `aarch64` | QEMU `virt` | ELF Kernel Image | `userspace_start` | 0.145s | **PARTIAL** | Log link |
| `riscv64` | QEMU `virt` | ELF Kernel Image | `driver_init` | 0.210s | **PARTIAL** | Log link |

## Failure Policy
A boot failure in any supported primary target (`x86_64` QEMU smoke) blocks pull request merges to the `main` branch.
