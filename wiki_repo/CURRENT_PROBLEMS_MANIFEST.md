# SigmaOS — Active Problems & Resolution Status
> Updated: July 2026 · Phase 10

This is the canonical issue tracker for kernel/driver/subsystem problems.

## ✅ Phase G–H: All Resolved This Session

| ID | Problem | Fix | File |
|----|---------|-----|------|
| G-01 | TCP stack had no full RFC 793 state machine | Full FSM implementation | `sigma_tcp_stack.rs` |
| G-02 | No Wi-Fi stack | IEEE 802.11ax + WPA3-SAE | `sigma_wifi_stack.rs` |
| G-03 | No Bluetooth | HCI/L2CAP/GATT BT 5.3 | `sigma_bluetooth.rs` |
| G-04 | PMM had no buddy coalescing | Full coalesce on free | `sigma_pmm.rs` |
| G-05 | Scheduler lacked priority boost (starvation) | 1s period boost | `sigma_mlfq_sched.rs` |
| G-06 | IRQ had no I/O APIC support | Full IOAPIC mapping | `sigma_irq_controller.rs` |
| G-07 | CryptFS `derive_key()` returned 32 zero bytes | PBKDF2-SHA256 100K iter | `sigma_key_derive.rs` |
| H-01 | No IPC beyond channels | Pipe + MsgQ + SHM | `sigma_ipc_pipe.rs` |
| H-02 | VFS had no path resolver | Full `path_resolve()` | `sigma_vfs_ext4.rs` |
| H-03 | No audio subsystem | HDA + PipeWire-style mixer | `sigma_sound.rs` |
| H-04 | USB had no device enumeration | xHCI port probing | `sigma_usb_stack.rs` |
| H-05 | GPU had no mode-setting | Full DRM/KMS + page-flip | `sigma_gpu_drm.rs` |
| H-06 | Network stack missing ARP/DHCP/DNS | Full stack | `sigma_network_stack.rs` |
| H-07 | No container runtime | OCI + CRI complete | `sigma_container_runtime.rs` |
| H-08 | No DVFS or battery management | Full power manager | `sigma_power_mgmt.rs` |
| H-09 | sigma-ai daemon was Python stub only | Rust inference engine | `sigma_local_llm.rs` |

## 🔴 Open — Phase I (Next Sprint)

| ID | Problem | Impact |
|----|---------|--------|
| I-01 | No UEFI `sigma-boot.efi` | Cannot boot without GRUB |
| I-02 | `make iso` fails / produces non-bootable image | No easy install |
| I-03 | NVMe uses MMIO polling (no MSI-X interrupts) | 4× less throughput |
| I-04 | No SATA AHCI driver | Many systems have only SATA |
| I-05 | virtio-GPU missing (QEMU can't display) | CI video test fails |
| I-06 | Multi-monitor KMS not implemented | One display only |
| I-07 | sigma-pkg has no real package repository server | Can't install packages |
| I-08 | Zenith crashes if display server exits | No recovery |
| I-09 | No Indian language IME | Non-English users blocked |
| I-10 | Wi-Fi 6E 6 GHz band not supported | New hardware incompatible |

## 🟠 Open — Phase J

| ID | Problem |
|----|---------|
| J-01 | ARM64 port incomplete (no BSP for Pi 5) |
| J-02 | RISC-V port not started |
| J-03 | eBPF uses interpreter (no JIT) — 10× slower |
| J-04 | No formal proofs (Coq) for scheduler + PMM |
| J-05 | Linux binaries don't run (no binfmt_misc) |
| J-06 | Wayland protocol not implemented client-side |
| J-07 | sigma-pod rootless containers not implemented |
| J-08 | TPM 2.0 commands not implemented |

## Low-Priority / Known Limitations

- Shell globbing (`*`, `?`) in sigma-sh incomplete
- Recovery GUI not implemented (serial-only recovery)
- Package delta updates not implemented
- Multi-architecture CI (ARM64/RISC-V) not yet running
- ZFS driver is a stub only

---
*See [FUTURE_ROADMAP.md](FUTURE_ROADMAP.md) for planned resolutions and timelines.*
