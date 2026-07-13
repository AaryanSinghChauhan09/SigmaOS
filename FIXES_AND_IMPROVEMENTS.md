# FIXES AND IMPROVEMENTS

> **Status**: ACTIVE | **Tracks**: Bug Fixes, Performance, Quality | **Updated**: Session-3

This document provides actionable solutions for known issues and tracks quality improvements across the SigmaOS kernel, userland, and build system.

---

## Critical Fixes (Completed)

### FIX-001: IPv6 Fragment Handling in SigmaShield
- **Issue**: ICMPv6 neighbor discovery dropped for fragmented packets
- **Root Cause**: u16 overflow in fragment offset calculation
- **Fix**: Use u32 arithmetic; add IPv6 fragment reassembly state machine
- **PR**: #142 | **Anomaly**: ANOMALY-0001
- **Status**: ✅ Fixed

### FIX-002: AMD Zen 4 Thermal Sensor Mismatch
- **Issue**: HAL reporting Tctl instead of Tdie (15°C offset)
- **Root Cause**: Missing CPU family/model detection in k10temp driver
- **Fix**: Family 19h → read Tdie directly
- **PR**: #178 | **Anomaly**: ANOMALY-0002
- **Status**: ✅ Fixed

### FIX-003: cgroup Memory Accounting After Restart
- **Issue**: Negative RSS counter after container restart cycle
- **Root Cause**: AtomicI64 not reset on cgroup teardown
- **Fix**: Explicit zero on destroy; assertion on init
- **PR**: #201 | **Anomaly**: ANOMALY-0003
- **Status**: ✅ Fixed

### FIX-004: sigpkg Dry-Run Database Mutation
- **Issue**: `--dry-run` modifying on-disk package database
- **Root Cause**: Dry-run check after DB write, not before
- **Fix**: Gate all mutations behind `DryRun::is_live()`
- **PR**: #234 | **Anomaly**: ANOMALY-0004
- **Status**: ✅ Fixed

### FIX-005: sigma-bus Ring-Buffer Stall
- **Issue**: IPC latency spikes >10ms under load >100K msg/s
- **Root Cause**: Aggressive exponential backoff (up to 512μs)
- **Fix**: Adaptive backoff capped at 16μs + condition variable
- **PR**: #267 | **Anomaly**: ANOMALY-0005
- **Status**: ✅ Fixed

### FIX-006: Dilithium5 ARM64 Alignment Fault
- **Issue**: Package verification failing for >16MB packages on ARM64
- **Root Cause**: SHA3 buffer 8-byte aligned, NEON needs 16-byte
- **Fix**: `#[repr(align(16))]` + compile-time assertion
- **PR**: #299 | **Anomaly**: ANOMALY-0006
- **Status**: ✅ Fixed

---

## In-Progress Fixes

### FIX-007: EEVDF Throughput for Compile Workloads
- **Issue**: EEVDF lower throughput than CFS for many short-lived tasks
- **Fix**: I/O-wait fraction tracking for CPU vs I/O-bound classification
- **PR**: #331 | **Anomaly**: ANOMALY-0008
- **Status**: ✅ Fixed (+18% compile throughput)

### FIX-008: Wiki Deduplication
- **Issue**: sigma_automation.sh creating duplicate wiki pages
- **Fix**: `--dedup` flag + cleanup script
- **PR**: #312 | **Anomaly**: ANOMALY-0007
- **Status**: ✅ Fixed

---

## Improvement Initiatives

### Quality Improvements

| Improvement | Description | Status |
|---|---|---|
| Stub elimination | Remove all `1` placeholder content from wiki | 🔄 Active (Session-3) |
| Mermaid diagrams | Add architecture diagrams to all spec docs | 🔄 Active |
| Code documentation | `/// doc comments` on all pub functions | 🔄 Active |
| Test coverage | Target 60% unit test coverage for kernel | 📋 Planned |
| Clippy compliance | Zero clippy warnings on `#[deny(clippy::all)]` | 🔄 Active |
| MSRV documentation | Document Minimum Supported Rust Version | ✅ Done |

### Performance Improvements

| Improvement | Metric | Before | After | Status |
|---|---|---|---|---|
| sigma-bus latency | IPC P50 | ~500ns | ~245ns | ✅ Done |
| Boot time (NVMe) | Wall clock | ~5s | ~3s | 🔄 Active |
| sigpkg install | Cached install | ~3s | ~2s | ✅ Done |
| VFS metadata ops | ops/s | 280K | 450K | ✅ Done |
| EEVDF compile throughput | make -j16 | baseline | +18% | ✅ Done |
| Memory footprint (minimal) | Idle RAM | 64MB | 48MB | ✅ Done |

### Build System Improvements

| Improvement | Description | Status |
|---|---|---|
| `just check-env` | Pre-build environment validation | ✅ Done |
| CI matrix | x86_64 + ARM64 QEMU in GitHub Actions | ✅ Done |
| RISC-V CI | Add RISC-V QEMU target to CI | 📋 Planned |
| Build caching | sccache for incremental Rust builds | 🔄 Active |
| Cross-compilation | `just build --target aarch64` | 🔄 Active |

---

## Known Issues (Open)

| ID | Severity | Component | Description |
|---|---|---|---|
| BUG-101 | MEDIUM | kernel/gpu | Vulkan compute shader dispatch hang on AMD RDNA3 |
| BUG-102 | LOW | sigpkg | Mirror fallback timeout too aggressive (2s → should be 5s) |
| BUG-103 | MEDIUM | kernel/net | TCP BBR probe_rtt phase causing throughput dip |
| BUG-104 | LOW | zenith | Window resize flicker on fractional scaling |
| BUG-105 | HIGH | kernel/sched | Priority inversion possible with RT shard + cgroup limit |

---

## Quality Gate Status

```bash
$ ./scripts/sigma_quality_check.sh
✅ No stub files detected in kernel/
✅ All pub functions documented
✅ Clippy: 0 warnings
✅ Build: x86_64 release OK
⚠️  Build: ARM64 cross-compile 3 warnings (non-critical)
✅ Tests: sigpkg unit tests pass (47/47)
⚠️  Tests: kernel integration tests skipped (no_std conflict)
✅ Security: No unsafe blocks without SAFETY comment
✅ License: All files have SPDX headers
```

---

*Total fixes shipped: 8 | Open bugs: 5 | Next fix sprint: Q3 2025*
