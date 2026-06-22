# Architecture

SigmaOS uses a layered, ring-based architecture where every component is isolated by cryptographic capability tokens. Nothing crosses a ring boundary without explicit authorization from the Zero-Trust Engine.

---

## Ring Hierarchy

```
Ring 0 — Zero-Trust Engine (kernel root)
Ring 1 — SovereignVMM (memory management)
Ring 2 — Driver Registry (hardware access)
Ring 3 — Userland / Daemons (Go services)
Ring 4 — Web Shell / Apps (Chromium sandbox)
```

Escalation from Ring N → Ring N-1 requires:
1. A capability token minted by Ring 0.
2. A cryptographic proof of request (Dilithium-5 signature).
3. Passing the runtime threat score threshold.

---

## Kernel (`kernel/`)

| Subsystem | Path | Description |
|-----------|------|-------------|
| Process Scheduler | `kernel/core/sigma_process.cpp` | O(1) priority scheduler with CPU affinity |
| Memory Manager | `kernel/memory/sigma_vmm.cpp` | 4-level page tables, CoW, demand paging, huge pages |
| Zero-Trust Engine | `kernel/security/sigma_zero_trust.cpp` | PAM/ACL, capability tokens, runtime threat scoring |
| Syscall Gate | `kernel/core/sigma_syscall.cpp` | Validated, auditable syscall dispatch |
| Namespace Manager | `kernel/core/sigma_namespace.cpp` | PID/Mount/Network namespace isolation |

---

## Drivers (`kernel/drivers/`)

| Driver | Path | Status |
|--------|------|--------|
| NVMe 1.4 | `kernel/drivers/storage/sigma_nvme.cpp` | ✅ Full |
| xHCI USB 3.x | `kernel/drivers/usb/sigma_xhci.cpp` | ✅ Full |
| Intel KMS/DRM | `kernel/drivers/graphics/sigma_kms.cpp` | ⚠️ Modesetting only |
| E1000 NIC | `kernel/drivers/net/sigma_e1000.cpp` | ✅ Full |
| 802.11 Wi-Fi | `kernel/drivers/net/wifi/sigma_80211.cpp` | 🔄 mac80211 state machine |
| HDA Audio | `kernel/drivers/audio/sigma_hda.cpp` | 🔄 Codec enum + mixer stub |
| ACPI | `kernel/power/sigma_power_manager.cpp` | ✅ Full |

---

## Memory Model

```
Virtual Address Space (per-process, 48-bit):
  0x0000_0000_0000 - 0x0000_7FFF_FFFF  User space
  0xFFFF_8000_0000 - 0xFFFF_FFFF_FFFF  Kernel space

Page size: 4KiB (4KiB / 2MiB huge pages supported)
Page table: 4-level (PML4 → PDPT → PD → PT)
CoW: Shared pages marked read-only; fault on write triggers private copy.
Demand paging: Lazy allocation — physical frames assigned on first access.
```

---

## Networking (`net/`)

TCP/IP stack written from scratch, RFC-793 compliant.

```
User App → sigma-sh socket() syscall
         → SovereignSocket (net/socket/sigma_socket.cpp)
         → TCP State Machine (net/tcp/sigma_tcp.cpp)
           CLOSED → LISTEN → SYN_RECV → ESTABLISHED → FIN_WAIT → TIME_WAIT
         → IP Layer (net/ip/sigma_ip.cpp)
         → ARP (net/arp/sigma_arp.cpp)
         → NIC Driver (E1000 / RTL8139)
```

---

## Filesystem (`fs/`)

| Layer | Implementation |
|-------|---------------|
| VFS | `fs/vfs/sigma_vfs.cpp` — unified inode/dentry abstraction |
| Ext4 | `fs/ext4/sigma_ext4.cpp` — full read/write + JBD2 journal |
| SemanticFS | `fs/semantic/sigma_semantic_fs.cpp` — metadata-aware overlay |
| tmpfs | `fs/tmpfs/sigma_tmpfs.cpp` — RAM-backed for `/tmp` |
