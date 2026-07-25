# SigmaOS Kernel Internals

> Updated: July 2026 · Phase 10 · All `#![no_std]` Rust

## Boot Sequence

```
BIOS/UEFI → sigma-boot.efi (or GRUB2) → Multiboot2 header
  → sovereign_kernel_main.rs
    → serial_init() + VGA init
    → IDT + APIC init (sigma_irq_controller.rs)
    → PMM init (sigma_pmm.rs) — buddy allocator
    → slab_init (sigma_mm.rs)
    → VFS init (sigma_vfs_ext4.rs)
    → Process manager + scheduler (sigma_sched.rs)
    → Network stack (sigma_network_stack.rs)
    → Display server (sigma_display_server.rs)
    → Init process (PID 1) — sigma-init
```

## Memory Management

**Physical Memory Manager** (`sigma_pmm.rs`)
- Buddy allocator: orders 0–11 (4KB–8MB blocks)
- `sigma_pmm_alloc(order)` → physical address
- `sigma_pmm_free(phys)` → coalesces buddies
- Stats: `sigma_pmm_stats()` → (free_mb, total_mb)

**Virtual Memory** (`sigma_mm.rs`)
- 4-level x86-64 page tables (PML4 → PDPT → PD → PT)
- Slab allocator for kernel objects
- ASLR: randomize kernel/stack/heap bases
- W^X enforcement: no page both writable and executable
- Stack guard canaries (`DEAD_C0DE_BEEF_CAFE`)

## Scheduler (`sigma_sched.rs`)

Three unified policies in one scheduler:

| Policy | Algorithm | Use Case |
|--------|-----------|----------|
| `Mlfq` | 8-level MLFQ + priority boost every 1s | Interactive/general |
| `Cfs` | CFS with vruntime red-black tree | Fair CPU sharing |
| `Edf` | Earliest Deadline First | Hard real-time (RTOS) |

**MCS Spinlock** — queue-based, O(1) lock/unlock, cache-coherent.

**API:**
```rust
sigma_sched_spawn(entry: u64, sp: u64) → Option<u32>
sigma_sched_tick()       // call every 1ms from timer IRQ
sigma_sched_next()       // pick next process
sigma_sched_block(idx)   // I/O wait → priority boost on wake
sigma_sched_unblock(idx)
sigma_sched_exit(idx)
```

## IRQ Subsystem (`sigma_irq_controller.rs`)

- x86-64: IDT (256 entries), Local APIC, I/O APIC
- ARM64: GIC-400 (distributor + CPU interface)
- `register_irq_handler(vector, handler_fn)` — per-vector dispatch
- `apic_eoi()` — end-of-interrupt signaling
- `apic_send_ipi(dest, vector)` — inter-processor interrupts

## IPC (`sigma_ipc_pipe.rs`)

| Mechanism | API | Notes |
|-----------|-----|-------|
| Pipe | `pipe.write()` / `pipe.read()` | 64KB lock-free ring buffer |
| Message Queue | `msgq.send()` / `msgq.recv(type)` | 256 msgs, type-based recv |
| Shared Memory | `shm_create()` / `shm_attach()` | Physical backing pages |

## Filesystem (`sigma_vfs_ext4.rs`)

- VFS layer: inodes, dentries, open file table
- Path resolution: `vfs.path_resolve(path)` → inode
- Operations: `create`, `mkdir`, `open`, `close`, `unlink`, `rename`, `readdir`
- Ext4-compatible layout: direct blocks (12) + single/double/triple indirect
- In-memory for now; persistent ext4 driver planned Phase I

## Network Stack (`sigma_network_stack.rs`, `sigma_tcp_stack.rs`)

```
Application
  └── sigma_socket_{open,connect,send,recv,close}
        └── TCP state machine (sigma_tcp_stack.rs)
              └── IPv4 (Ipv4Header + checksum)
                    └── Ethernet (EthHeader)
                          └── NIC driver (e1000 / virtio-net)
```

Also: ARP cache, DHCP client, DNS cache with TTL aging.

## Crypto (`crypto/sigma_key_derive.rs`)

- SHA-256: sovereign, no external deps
- HMAC-SHA256
- PBKDF2-SHA256 (100K iterations for passwords)
- HKDF-SHA256 (subkey derivation)
- `derive_key(passphrase, salt)` — fixes CryptFS issue #1009
- `verify_key` — constant-time comparison

## Security

- **Kyber (ML-KEM-768)**: post-quantum key encapsulation
- **Dilithium (ML-DSA-65)**: post-quantum signatures
- **AppArmor-style MAC**: profile-based mediation
- **Pledge/Unveil**: capability reduction (OpenBSD-inspired)
- **eBPF**: kernel extension with verifier

## Container Runtime (`sigma_container_runtime.rs`)

- OCI spec compliant
- Namespaces: PID, NET, MNT, UTS, IPC, USER, CGROUP
- Cgroups: CPU quota, memory limit, PID limit
- Image layers + overlay filesystem
- CRI interface for Kubernetes
- `sigma-pod start <spec>` → < 200ms startup

## GPU/Display (`sigma_gpu_drm.rs`)

- DRM device, CRTC, connectors, framebuffers
- GEM buffer objects (physical-backed)
- Mode-setting: `drm.set_crtc(crtc_id, fb_id, mode)`
- Page-flipping: `drm.page_flip(crtc_id, fb_id)`
- VESA framebuffer fallback: `VesaFb::put_pixel()`
- Supported: 1080p, 1440p, 4K modes out-of-box

## Audio (`sigma_sound.rs`)

- Mixer: up to 32 concurrent streams, per-stream volume
- Ring buffer: lock-free, 4 × 1024-frame periods
- HDA (Intel High Definition Audio): CORB/RIRB command interface
- API: `mixer.open_stream(device)` → stream_id
- `stream.write_samples(samples)`, `mixer.mix_to_output(out)`
- Master volume + mute, soft clipping

## USB (`sigma_usb_stack.rs`)

- xHCI: MMIO register map, port probing, speed detection
- HID: keyboard (HID boot protocol), mouse
- Mass storage: BBB bulk-only transfer, SCSI READ10/WRITE10
- Keycode → ASCII decoder (US layout)
- `xhci.enumerate_all()` → discovers all connected devices
