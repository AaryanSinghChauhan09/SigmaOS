# SigmaOS — Current Problems Manifest

> SigmaOS v15.1 "Zenith" — Last Updated: 2026-07-08

This document tracks known issues, limitations, and technical debt across the codebase.

---

## Critical (P0) — Blocks Boot/Functionality

| ID | Component | Description | Workaround |
| --- | --- | --- | --- |
| BUG-001 | Buddy Allocator | `alloc_pages` / `free_pages` not fully wired to VMM | Use stack-allocated arrays for early-boot code |
| BUG-002 | Scheduler | Work-stealing uses O(n) scan; may cause latency spikes on >8 CPUs | Limit to single-CPU in current builds |
| BUG-003 | UEFI Boot | `sigma_efi_entry.c` ELF segment loading is a stub — kernel not actually mapped from ELF | Use flat binary load for testing |

---

## High (P1) — Significant Limitations

| ID | Component | Description | Target Release |
| --- | --- | --- | --- |
| BUG-004 | Btrfs | `create_snapshot` / `rollback` are stubs; actual CoW tree operations not implemented | v15.1 |
| BUG-005 | sigpkg | Dilithium-5 signature verification is stubbed; packages not cryptographically verified | v15.1 |
| BUG-006 | AMD GPU | `sigma_amdgpu.rs` probe is stubbed; display not functional on AMD hardware | v15.1 |
| BUG-007 | Bluetooth | `sigma_hci_usb.rs` USB endpoint setup is stubbed | v15.1 |
| BUG-008 | RTL Wi-Fi | `sigma_rtl8xxxu.rs` USB control transfer not implemented | v15.1 |
| ~~BUG-009~~ | ~~Init System~~ | ~~`sigma_init.rs` service dependency graph not built; all services start sequentially~~ | **RESOLVED v15.1** — Kahn's BFS topological sort implemented via `SigmaDepGraph` |
| BUG-010 | AI Agent | Inference is a heuristic stub; no real LLM model loaded | v15.2 |

---

## Medium (P2) — Functional Gaps

| ID | Component | Description | Target Release |
| --- | --- | --- | --- |
| BUG-011 | IDS | Rule parser not implemented; only packet pass-through | v15.2 |
| BUG-012 | Fail2Ban | IP blocklist not connected to network filter tables | v15.2 |
| BUG-013 | UBC | `allocate_page` references `buddy_allocator::alloc_pages` which is not yet wired | v15.1 |
| BUG-014 | MAC | `sigma_mac.rs` label enforcement not wired into VFS call sites | v15.2 |
| BUG-015 | Seccomp | `sigma_seccomp.rs` BPF JIT not implemented; filters not applied | v15.2 |
| BUG-016 | Locale | Locale packs are TOML stubs; not loaded by the display system | v15.2 |

---

## Low (P3) — Polish / Optimization

| ID | Component | Description | Status |
| --- | --- | --- | --- |
| ~~BUG-017~~ | ~~CFS Scheduler~~ | ~~Red-black tree not implemented; O(n) sorted array used instead~~ | **RESOLVED v15.1** — Array-backed RB-tree in `sigma_sched.rs` |
| ~~BUG-018~~ | ~~EDF Scheduler~~ | ~~Binary heap not implemented; O(n) linear scan used instead~~ | **RESOLVED v15.1** — Array-backed Binary Min-Heap in `sigma_sched.rs` |
| BUG-019 | VFS | Mount table limited to 16 entries (increase to 256 in next release) | Open |
| BUG-020 | sigpkg GUI | GUI runs as stub; not connected to `sigpkgd` daemon | Open |
| BUG-021 | SigmaFS | Indirect block pointers for large files not implemented | Open |
| BUG-022 | i915 | Display mode setting relies on GOP pre-configuration; custom resolution not supported | Open |

---

## Technical Debt

| Area | Debt Description |
| --- | --- |
| `no_std` violations | Some modules use `alloc::string` which requires a global allocator not yet wired |
| Static `mut` usage | Many driver singletons use `static mut` without proper spinlock protection |
| Test coverage | Unit test files are stubs; no actual assertions implemented |
| Error propagation | Most functions return `bool` or `Option`; should migrate to `Result<T, SigmaError>` |
| Documentation | Inline doc comments missing from most public functions |

---

## Reporting New Issues

Open an issue at: `https://github.com/AaryanSinghChauhan09/SigmaOS/issues`

Use the template:

```text
**Component**:
**Severity**: P0 / P1 / P2 / P3
**Description**:
**Reproduction Steps**:
**Expected Behavior**:
**Actual Behavior**:
**Logs**: (attach sigma-journal output)
```
