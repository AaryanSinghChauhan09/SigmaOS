# SigmaOS Hardware Support Matrix

SigmaOS implements a custom Ring 0 driver model. We currently prioritize cloud, virtualization, and modern x86_64 architectures. 

## Supported Architectures
| Architecture | Status | Notes |
|---|---|---|
| `x86_64` | **Tier 1 (Stable)** | Fully supported. Core optimizations applied. |
| `ARM64 (aarch64)` | **Tier 2 (Beta)** | Bootable. Native AI embeddings may run slower. |
| `RISC-V` | **Planned** | Cross-compilation tools available, bootloader WIP. |

## Peripherals & Drivers
| Subsystem | Status | Driver Backend |
|---|---|---|
| **Networking** | Stable | VirtIO Net, Intel E1000e, Realtek RTL8111 |
| **Storage** | Stable | VirtIO Blk, NVMe |
| **Graphics** | Stubbed / WIP | AMD/Intel KMS stubbed. UI relies on software rendering or VirtIO GPU. |
| **Audio** | Beta | High Definition Audio (HDA) basic playback (`sigma_hda`). |
| **Wi-Fi** | Alpha | `sigma_80211` exists but lacks full WPA3 supplicant. |

## Virtualization Platforms
- QEMU / KVM (Primary testing target)
- VMware ESXi
- Bare-metal (Modern UEFI systems)
