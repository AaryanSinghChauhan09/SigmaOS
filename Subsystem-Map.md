# SigmaOS Subsystem Map

> Every subsystem, the file that implements it, and its language.

---

## Kernel (Rust `#![no_std]` + Zig + SPARK/Ada)

### Core

| Subsystem | File | Lang |
|---|---|---|
| Scheduler (MLFQ+EDF+CFS) | `kernel/core/sigma_sched.rs` | Rust |
| Memory (Buddy+Slab) | `kernel/core/sigma_mm.rs` | Rust |
| Syscall dispatch (32 syscalls) | `kernel/core/sigma_syscall_dispatch.rs` | Rust |
| Process management | `kernel/core/sigma_process.rs` | Rust |
| Panic handler + recovery | `kernel/core/sigma_panic.rs` | Rust |
| cgroup v2 | `kernel/core/sigma_cgroup.rs` | Rust |
| Namespaces (PID/UTS/Mount) | `kernel/core/sigma_namespace.rs` | Rust |
| HPET + APIC timer | `kernel/core/sigma_timer.zig` | Zig |
| EDF scheduler | `kernel/scheduling/sigma_edf.rs` | Rust |

### IPC

| Subsystem | File | Lang |
|---|---|---|
| sigma-bus typed IPC | `kernel/ipc/sigma_bus.rs` | Rust |
| Anonymous pipes | `kernel/ipc/sigma_pipe.rs` | Rust |
| Shared memory | `kernel/ipc/sigma_shm.rs` | Rust |

### Memory

| Subsystem | File | Lang |
|---|---|---|
| x86-64 4-level paging | `kernel/memory/sigma_vmm.zig` | Zig |
| OOM killer | `kernel/memory/sigma_oom.rs` | Rust |
| Huge pages (2MB/1GB) | `kernel/memory/sigma_hugepages.ads/.adb` | SPARK |

### Security

| Subsystem | File | Lang |
|---|---|---|
| sigma_pledge + sigma_unveil | `kernel/security/sigma_pledge.rs` | Rust |
| ASLR 42-bit entropy | `kernel/security/sigma_aslr.rs` | Rust |
| seccomp-BPF filter | `kernel/security/sigma_seccomp.rs` | Rust |
| MAC policy engine | `kernel/security/sigma_mac.rs` | Rust |
| TPM2 measured boot | `kernel/security/sigma_tpm2.rs` | Rust |
| Landlock filesystem sandbox | `kernel/security/sigma_landlock.rs` | Rust |
| SPIFFE workload identity | `kernel/security/sigma_spiffe.rs` | Rust |
| AVC O(1) cache | `security/sigma_avc.rs` | Rust |

### Crypto

| Subsystem | File | Lang |
|---|---|---|
| SHA-256 + HMAC | `kernel/crypto/sigma_sha256.rs` | Rust |
| Kyber-1024 KEM | `crypto/sigma_kyber.rs` | Rust |
| Dilithium-5 | `crypto/sigma_dilithium.ads/.adb` | SPARK |

### Networking

| Subsystem | File | Lang |
|---|---|---|
| Ethernet + IPv4 + ARP + UDP | `kernel/net/sigma_net.rs` | Rust |
| TCP state machine | `kernel/net/sigma_tcp.rs` | Rust |
| ICMP echo/reply | `kernel/net/sigma_icmp.rs` | Rust |
| DHCP client | `kernel/net/sigma_dhcp.rs` | Rust |
| DHCPv6 + SLAAC | `kernel/net/sigma_dhcpv6.rs` | Rust |
| NTP/NTS client | `kernel/net/sigma_ntp.rs` | Rust |
| mDNS/DNS-SD | `kernel/net/sigma_mdns.rs` | Rust |
| DNS/DoH resolver | `kernel/net/sigma_dns.rs` | Rust |
| TLS 1.3 + Kyber | `kernel/net/sigma_tls.rs` | Rust |
| Stateful firewall | `kernel/net/sigma_firewall.rs` | Rust |
| WireGuard VPN | `kernel/net/sigma_wireguard.rs` | Rust |

### Filesystems

| Subsystem | File | Lang |
|---|---|---|
| VFS + Tmpfs | `kernel/fs/sigma_vfs.rs` | Rust |
| FAT32 read-only | `kernel/fs/sigma_fat32.rs` | Rust |
| /proc virtual FS | `kernel/fs/sigma_procfs.rs` | Rust |
| /sys virtual FS | `kernel/fs/sigma_sysfs.rs` | Rust |
| Ext4 read-only | `kernel/fs/sigma_ext4.rs` | Rust |
| SigmaFS CoW native | `kernel/fs/sigma_sigmafs.rs` | Rust |

### Power

| Subsystem | File | Lang |
|---|---|---|
| ACPI P/C-states | `kernel/power/sigma_power.zig` | Zig |

---

## Drivers (Zig + Rust)

| Driver | File | Lang |
|---|---|---|
| Intel e1000 NIC | `drivers/net/sigma_e1000.rs` | Rust |
| VirtIO-net | `drivers/net/sigma_virtio_net.rs` | Rust |
| Wi-Fi framework + WPA2/3 | `drivers/net/sigma_wifi.rs` | Rust |
| NVMe PCIe | `drivers/storage/sigma_nvme.rs` | Rust |
| SATA AHCI | `drivers/storage/sigma_ahci.rs` | Rust |
| VESA/GOP framebuffer | `drivers/display/sigma_vesa.zig` | Zig |
| VirtIO GPU (QEMU) | `drivers/gpu/sigma_virtio_gpu.zig` | Zig |
| USB HID keyboard+mouse | `drivers/input/sigma_hid.zig` | Zig |
| xHCI USB 3.x | `drivers/usb/sigma_xhci.zig` | Zig |
| Intel HDA audio | `drivers/audio/sigma_hda.rs` | Rust |
| PCI bus + MSI-X | `drivers/hal/sigma_pci.zig` | Zig |

---

## Boot (Zig)

| Component | File | Lang |
|---|---|---|
| UEFI EFI stub | `sigma-boot/sigma_boot.zig` | Zig |
| IDT + APIC + PIC | `kernel/core/sigma_irq.zig` | Zig |

---

## Userland (Nim + Rust)

### Shell & Scripting

| Tool | File | Lang |
|---|---|---|
| sigma-sh REPL | `userland/shell/sigma_shell.nim` | Nim |
| Script runner | `userland/shell/sigma_scripting.nim` | Nim |

### Package Management

| Tool | File | Lang |
|---|---|---|
| sigma-pkg | `userland/pkg/sigma_pkg.nim` | Nim |

### Core Utilities

| Tool | File | Lang |
|---|---|---|
| sigma-coreutils (15 tools) | `userland/coreutils/sigma_coreutils.nim` | Nim |

### System Tools

| Tool | File | Lang |
|---|---|---|
| sigma-monitor | `userland/tools/sigma_monitor.rs` | Rust |
| sigma-disks | `userland/tools/sigma_disks.nim` | Nim |
| sigma-logs | `userland/tools/sigma_logs.rs` | Rust |
| sigma-doctor | `userland/tools/sigma_doctor.nim` | Nim |
| sigma-bench | `userland/tools/sigma_bench.rs` | Rust |
| sigma-update daemon | `userland/tools/sigma_update_daemon.rs` | Rust |

### Network Tools

| Tool | File | Lang |
|---|---|---|
| sigma-ssh | `userland/net/sigma_ssh.nim` | Nim |
| sigma-curl (HTTP) | `userland/net/sigma_http.nim` | Nim |
| sigma-vpn (WireGuard) | `userland/net/sigma_vpn.nim` | Nim |
| sigma-netctl | `userland/net/sigma_netctl.nim` | Nim |

### Desktop

| Component | File | Lang |
|---|---|---|
| Zenith compositor | `userland/desktop/sigma_compositor.rs` | Rust |
| Window manager (tiling/BSP/grid) | `userland/desktop/sigma_wm.rs` | Rust |
| Theme engine (3 themes) | `userland/desktop/sigma_theme.rs` | Rust |
| Notification system | `userland/desktop/sigma_notifications.rs` | Rust |
| Widget toolkit | `userland/desktop/sigma_widgets.rs` | Rust |
| App launcher (fuzzy) | `userland/desktop/sigma_launcher.rs` | Rust |

### Daemons

| Daemon | File | Lang |
|---|---|---|
| sigmad-health | `userland/daemon/sigmad_health.rs` | Rust |
| sigmad-netd | `userland/daemon/sigmad_netd.rs` | Rust |
| sigmad-vault | `userland/daemon/sigmad_vault.rs` | Rust |

### Init & Update

| Component | File | Lang |
|---|---|---|
| sigma-init (PID 1) | `userland/init/sigma_init.rs` | Rust |
| sigma-update (A/B) | `userland/update/sigma_update.rs` | Rust |

### AI & Runtime

| Component | File | Lang |
|---|---|---|
| sigma-ai (TinyLlama) | `userland/ai/sigma_ai.rs` | Rust |
| WASM/WASI parser | `runtime/wasm/sigma_wasm.rs` | Rust |
| sigma-pod OCI runtime | `runtime/containers/sigma_container.rs` | Rust |
| sigma-vault store | `userland/vault/sigma_vault.rs` | Rust |

---

## Language Summary

| Language | % | Primary Use |
|---|---|---|
| **Rust** | 60% | Kernel core, security, net, fs, userland |
| **Zig** | 20% | HAL, boot, IRQ, timers, drivers |
| **Nim** | 15% | Shell, tools, daemons, CLI apps |
| **SPARK/Ada** | 5% | Formal crypto, huge pages |
| **C/C++** | 0% | None — all converted |

### Total subsystems implemented: 90+