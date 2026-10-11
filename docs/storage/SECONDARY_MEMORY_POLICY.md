# SigmaOS Secondary Memory Subsystem Policy

## Executive Overview & Objective

The SigmaOS Secondary Memory Subsystem governs persistent storage, block devices, filesystems, caches, and paging-backed storage as a safe, observable, and recoverable lifecycle subsystem.

> **Objective**: Maintain SigmaOS secondary memory as a safe, observable, recoverable storage subsystem: enumerate devices, manage block I/O, mount filesystems, cache data, page to disk, reclaim space, and recover from failures without corruption or undefined behavior.

---

## Storage Lifecycle Contract

SigmaOS secondary memory strictly enforces an 11-stage state machine:

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

### Stage Requirements

| Stage | Required Behavior |
|---|---|
| **Probe** | Device discovered, hardware capability bitmap reported |
| **Identify** | Geometry, sector size, and capacity limits queried & bounded |
| **Block I/O** | Requests validated, queued in bounded ring, completed **exact-once** |
| **Cache** | LRU/watermark eviction, dirty page writeback, coherence defined |
| **Filesystem** | Core transaction logging, capacity clamping, mount/unmount safety |
| **Mount** | VFS tree attachment, duplicate mount check, mount flags applied |
| **Read/Write** | Transaction-isolated data mutations with CoW or journal logging |
| **Flush** | Durable barrier semantics, hardware cache sync |
| **Unmount** | Reference count check, dirty page flush, clean detachment |
| **Detach** | Device reference removal, memory slot recycling |
| **Error/Recovery** | SCSI sense/typed error mapping, rollback without corruption |

---

## Architecture Classifications

### 1. Block Layer Class
- **Class**: Hybrid request-based with lock-free ring queue for high throughput (NVMe/virtio) and elevator scheduling for rotational drives (HDD).
- **Concurrency**: Bounded request queues with atomic sequence ID allocation and lock-free slot reclamation.
- **Limits**: Maximum 4096 in-flight requests per device; request allocation failure returns `BlockError::RequestExhaustion`.

### 2. Device Class
- **NVMe**: Multi-queue 2.0 interface with Submission/Completion rings and command submission timeouts.
- **SATA / AHCI**: Command queueing with port multiplier support and SCSI/ATA translation.
- **VirtIO Block**: Virtqueue-based asynchronous block driver (`VirtIoBlockDevice`).
- **USB Mass Storage**: BOT/UAS framing over USB xHCI host controller.

### 3. Cache Class
- **Class**: Hybrid page and buffer cache (`BlockCacheManager`).
- **Policy**: LRU eviction policy with dirty-watermark background writeback.
- **Watermarks**: High watermark 80% triggers background flush; 90% triggers aggressive sync; 100% returns `CacheError::CacheFull`.

### 4. Filesystem Class
- **Filesystems**: ZFS (ARC-inspired), OverlayFS, TmpFS, Ext4, and Sovereign SigmaFS.
- **Integrity**: Transaction record logging (`FileTransactionRecord`) and Copy-on-Write (CoW) page allocations prevent lost updates.

### 5. Mount & Flush Policy
- **Mount Safety**: Atomic mount table entry insertion; rollback on failed filesystem initialization; double-unmount protection (`MountError::NotMounted`).
- **Flush Durability**: Synchronous barrier commands flush hardware cache before returning write completion.

### 6. Reclaim & OOM Policy
- **Reclaim**: Dirty pages written back before eviction; clean pages evicted under memory pressure.
- **OOM Metric**: Exposes `CacheOomMetrics` (total pages, active pages, dirty pages, watermark triggers) for system-wide memory management monitoring.

### 7. NUMA & Hotplug Behavior
- **NUMA**: Per-node block cache allocations aligned to local CPU node domain.
- **Hotplug**: Device detachment transitions active requests to `RequestState::Failed` with `BlockError::DeviceDetached` and invalidates associated block cache pages.

---

## Periodic Task Calendar

| Frequency | Secondary-Memory Task | Output | Required PR |
|---|---|---|---|
| Every PR | Build, lint, unit, and integration tests | CI result | `feat`, `fix`, `test`, `docs` PR |
| Daily | CI/nightly triage | Root cause + fix | `fix(storage)` or `fix(filesystem)` PR |
| Weekly | Storage subsystem review | Gap list + status | `docs(secondary-memory)` PR |
| Weekly | Linux/BSD storage study | Study note | `docs(studies)` PR |
| Biweekly | Block layer audit | Leak/race findings | `fix(block)` / `test(block)` PR |
| Biweekly | Filesystem audit | Mapping findings | `fix(filesystem)` PR |
| Monthly | Boot-to-shell storage validation | Boot log + smoke report | `test(storage)` PR |
| Monthly | Storage stress/fuzz triage | Crash reports + fixes | `fix(filesystem)` PR |
| Monthly | Mount/unmount rehearsal | Mount enforcement report | `test(mount)` PR |
| Quarterly | Storage architecture review | ADR + roadmap | `docs(adr)` PR |
| Quarterly | Security review | Threat-model update | `security(storage)` PR |
| Quarterly | Release readiness review | Release checklist | `docs(release)` PR |

---

## Governance and Compliance

All storage changes must comply with `STORAGE_RULES.toml`:
1. `storage_completeness`: Must define block I/O, cache, mount, and filesystem semantics.
2. `no_leak`: Leaks are zero under stress tests.
3. `oom_metric`: Storage defines and measures OOM behavior.
4. `mount_integrity`: Mount/unmount preserves filesystem and scheduler integrity.
5. `cache_semantics`: Cache eviction defines legal eviction and error codes.
6. `no_model_only_status`: Features cannot be marked complete from model tests alone.
7. `runtime_evidence`: Boot-affecting changes require boot-to-shell evidence.
