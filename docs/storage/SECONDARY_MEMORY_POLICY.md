# SigmaOS Secondary Memory Subsystem Policy

## Objective

Maintain SigmaOS secondary memory as a safe, observable, recoverable storage subsystem: enumerate devices, manage block I/O, mount filesystems, cache data, page to disk, reclaim space, and recover from failures without corruption or undefined behavior.

---

## Storage Lifecycle Contract

SigmaOS secondary memory operates under one authoritative lifecycle contract:

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

### Stage Requirements

| Stage | Required Behavior | Enforcement / Mechanisms |
|---|---|---|
| **Probe** | Device discovered, capabilities and descriptors reported | PCI/VirtIO/NVMe/SATA controller enumeration |
| **Identify** | Geometry, sector size (512B/4096B), and capacity limits known | Block device capability discovery |
| **Block I/O** | Requests validated, queued, completed exactly once | `BoundedBlockDeviceManager`, queue limits |
| **Cache** | Invalidation, LRU eviction, and dirty writeback coherence defined | `BlockCacheManager`, dirty page writeback |
| **Filesystem** | Mount/unmount lifecycle safe, rollback on initialization failure | `FilesystemCoreManager`, `MountManager` |
| **Flush** | Data durability semantics explicit with sync/barrier primitives | Explicit block flush operations, write barriers |
| **Unmount** | Resources, locks, and active file references released exactly once | Ref counting, double-unmount prevention |
| **Recovery** | Failure or unexpected detachment does not corrupt filesystem state | Transaction sequence logging, atomic commits |

---

## Architecture Specifications

### 1. Block Layer Class
- **Type**: Hybrid Request-Based and Bio-Based Block Layer.
- **Queue Engine**: `BoundedBlockDeviceManager` with `AtomicU64` request identifier generation, submission queues, and completion callbacks.
- **Concurrency**: Lock-free atomic state transition (`Pending` -> `Dispatched` -> `Completed`/`Failed`) preventing race conditions during concurrent enqueue/dequeue.

### 2. Device Classes
- **VirtIO Block**: `VirtIoBlockDevice` providing virtualized queue handling for QEMU/KVM environments.
- **NVMe 2.0**: Multi-queue submission/completion queue manager (`NvmeBlockDevice`) with ZNS (Zone Namespaces) support.
- **SSD / Flash**: `SsdBlockDevice` with integrated FTL (Flash Translation Level) wear leveling and block write protection.
- **HDD / SATA**: Legacy sector-based queuing with elevator request sorting and bad-block mapping.

### 3. Cache Class
- **Type**: Hybrid LRU Page & Buffer Cache Manager (`BlockCacheManager`).
- **Coherence**: Lock-free page status monitoring, atomic dirty page tagging, and writeback synchronization.
- **Metadata Footprint**: Bounded metadata per cache page; maximum total page bounds enforced dynamically.

### 4. Filesystem Classes
- **Custom Native**: `SigmaFS` / `FilesystemCoreManager` providing volume capacity bounds, inode management, and transaction logging.
- **Virtual / Memory-Backed**: `tmpfs`, `sysfs`, `procfs`, `devtmpfs`.
- **Advanced / CoW**: ZFS ARC / HAMMER2 inspired Copy-on-Write snapshots with checksum verification.

### 5. Mount Policy
- **Reference Counting**: Mount entry tracks active file descriptors and open handle references (`open_ref_count`). Unmounting while references are active returns `MountError::ResourceBusy`.
- **Rollback Protection**: `mount_with_rollback` enforces transaction rollback on initialization errors, restoring the mount table state cleanly.
- **Double-Unmount Protection**: Attempting to unmount an already unmounted target returns `MountError::NotMounted`.

### 6. Flush & Durability Policy
- **Semantics**: Explicit `BlockOpCode::Flush` and `BlockOpCode::Barrier` commands propagate durability guarantees down to physical hardware.
- **Writeback Timing**: High-watermark cache thresholds trigger background dirty page flush sweeps before page allocation or eviction.

### 7. Reclaim & OOM Policy
- **Watermark Levels**: `Low` (<50%), `Medium` (50-80%), `High` (80-95%), `OomCritical` (>=95%).
- **Reclaim Mechanism**: Under `High` or `OomCritical` watermarks, clean LRU pages are evicted immediately and dirty pages are queued for batch writeback.

### 8. Error Policy
- **Typed Errors**: Storage operations return typed errors (`BlockError`, `CacheError`, `MountError`, `FsCoreError`). Panic or undefined behavior on I/O error is strictly prohibited.
- **Loss Prevention**: Lost completions are detected via timestamp timeout sweeps (`detect_lost_completions`), marking timed-out requests as `BlockError::LostCompletion`.

### 9. Limits & Allocations
- **Queue Bounds**: Bounded queue depth configured per block device (default 256 active requests).
- **Cache Limits**: Max page capacity explicitly bounded to prevent kernel memory exhaustion.

### 10. NUMA & Storage Hotplug Behavior
- **NUMA Alignment**: Buffer allocations align to local NUMA memory nodes to minimize cross-socket DMA latencies.
- **Hotplug Detachment**: Detached block devices update state immediately; queued I/O fails gracefully with `BlockError::DeviceDetached`.

---

## Periodic Task Calendar

| Frequency | Secondary-Memory Task | Output | Required PR |
|---|---|---|---|
| Every PR | Build, lint, unit, and integration tests | CI result | `feat`, `fix`, `test`, or `docs` PR |
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

## First 12 Storage PRs Roadmap

| # | PR | Purpose |
|---:|---|---|
| 1 | `docs(storage): add lifecycle state machine` | Establish authoritative storage states |
| 2 | `docs(storage): add frame and page-table policy` | Define allocation, reuse, and limits |
| 3 | `feat(storage): add bounded block manager` | Prevent unbounded kernel growth |
| 4 | `feat(storage): add race-free request allocator` | Ensure safe request lifecycle |
| 5 | `test(storage): add mount/recovery conformance tests` | Validate core lifecycle |
| 6 | `feat(storage): define recovery and OOM policy` | Eliminate corruption leaks |
| 7 | `feat(storage): add cache contract and integrity metric` | Make paging observable |
| 8 | `test(mm): add preemption and starvation tests` | Validate scheduling behavior |
| 9 | `feat(swap): define swap-out/swap-in policy` | Make signals deterministic |
| 10 | `test(swap): add CPU, memory, fd, and process-limit tests` | Enforce resource limits |
| 11 | `test(namespaces): add address-space isolation tests` | Validate isolation boundaries |
| 12 | `test(boot): add boot-to-shell storage smoke test` | Connect storage management to runtime |

---

## Storage Health Metrics

| Metric | Target |
|---|---|
| Storage CI pass rate | 100% |
| Boot-to-shell pass rate | 100% |
| Orphan requests after stress test | 0 |
| Double-completion incidents | 0 |
| Request reuse races | 0 |
| Storage corruption incidents | 0 |
| Declared limits with enforcement tests | 100% |
| Storage claims linked to evidence | 100% |
| Security-sensitive PRs reviewed | 100% |
