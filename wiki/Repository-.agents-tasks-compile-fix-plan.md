> Imported repository document from [`.agents/tasks/compile-fix-plan.md`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/.agents/tasks/compile-fix-plan.md). For current component status and roadmap, use the canonical component page linked from [Home](Home.md).

---

# SigmaOS Compilation Error Fix Plan

**Total Errors: 168**  
**Generated:** 2024  
**Status:** Ready for Implementation

---

## Error Categories Summary

| Category | Count | Description |
|----------|-------|-------------|
| **CATEGORY 1: Syntax** | 2 | Double `#` in derive, generic angle-bracket ambiguity |
| **CATEGORY 2: Duplicate Definitions** | 14 | Duplicate module declarations, struct/trait re-exports |
| **CATEGORY 3: Missing Imports** | 56 | Unresolved imports, missing modules |
| **CATEGORY 4: Type Mismatches** | 81 | Trait bounds, type mismatches, borrow checker errors |
| **CATEGORY 5: Other** | 15 | Visibility, unstable features, misc |

---

## CATEGORY 1: Syntax Errors (2 errors)

### 1.1 Double hash in derive macro
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/container/runtime.rs`
- **Line:** 53
- **Error Code:** E0000 (parse error)
- **Fix:** Change `##[derive(Debug, Clone)]` to `#[derive(Debug, Clone)]` (remove one `#`)

### 1.2 Generic angle-bracket ambiguity
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/kernel/dma_engine.rs`
- **Line:** 237
- **Error Code:** E0000 (parse error)
- **Fix:** Change `id as usize < self.channels.len()` to `(id as usize) < self.channels.len()` (add parentheses to disambiguate)

---

## CATEGORY 2: Duplicate Definitions (14 errors)

### 2.1 Duplicate module `usb_hid` declaration
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/drivers/mod.rs`
- **Line:** 42 (first), 92 (use statement)
- **Error Code:** E0428
- **Fix:** Remove duplicate `pub mod usb_hid;` declaration at line 42; keep the single declaration and its corresponding `pub use` statement

### 2.2 Duplicate module `ethernet` declaration
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/drivers/mod.rs`
- **Line:** 38, 98
- **Error Code:** E0428
- **Fix:** Remove duplicate `pub mod ethernet;` declaration at line 98; keep only the first declaration at line 38

### 2.3 Duplicate module `wifi_80211` declaration
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/drivers/mod.rs`
- **Line:** 39, 99
- **Error Code:** E0428
- **Fix:** Remove duplicate `pub mod wifi_80211;` declaration at line 99; keep only the first declaration at line 39

### 2.4 Duplicate module `nvme_driver` declaration
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/drivers/mod.rs`
- **Line:** 40, 119
- **Error Code:** E0428
- **Fix:** Remove duplicate `pub mod nvme_driver;` declaration at line 119; keep only the first declaration at line 40

### 2.5 Duplicate module `selinux` declaration
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/security/mod.rs`
- **Line:** 58 (appears twice)
- **Error Code:** E0428
- **Fix:** Remove one of the duplicate `pub mod selinux;` declarations

### 2.6-2.14 Duplicate type re-exports from nvme_driver
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/drivers/mod.rs`
- **Lines:** 110-122 (first pub use), 121-127 (duplicate pub use)
- **Error Codes:** E0252 (9 errors for: NvmeController, NvmeQueuePair, NvmeNamespace, WifiMode, NvmeSQEntry, NvmeCQEntry, NvmeAdminOpcode, NvmeIOOpcode, NvmeError)
- **Fix:** Remove duplicate `pub use nvme_driver::{...}` block at lines 121-127; keep only one set of re-exports

---

## CATEGORY 3: Missing Imports / Unresolved Names (56 errors)

### Process Module Missing Exports (24 items)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/lib.rs`
- **Line:** 82
- **Error Code:** E0432
- **Missing:** `AdvancedIpcHub`, `BsdRusage`, `CancellationType`, `CoreDumpMetadata`, `EventFd`, `JobControlLifecycleEngine`, `JobState`, `PosixMessage`, `PosixMessageQueue`, `ProcessCancelState`, `ProcessCancellationAndTerminationManager`, `ProcessControlError`, `ProcessJobEntry`, `ProcessVmReadWriteEngine`, `ProcessWaiterAndRusageCollector`, `SigQueuePayload`, `SovereignProcess`, `SovereignProcessManager`, `SovereignProcessState`, `WaitStatus`, `ZeroCopyIpcChannel`, `WCONTINUED`, `WNOHANG`, `WUNTRACED`
- **Fix:** Create stub implementations in `src/kernel/process.rs` or re-export from appropriate modules

### Driver Module Missing Exports (50 items)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/lib.rs`
- **Line:** 216
- **Error Code:** E0432
- **Missing:** `AudioDspStream`, `AudioSampleFormat`, `Bluetooth54LeAudioDriver`, `BusType`, `DriverCapability`, `DriverIsolationRingGuard`, and 44 more driver-related types
- **Fix:** Create stub implementations in driver modules or add re-exports from existing implementations

### Audio/Pipewire Missing (3 items)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/audio/mod.rs`
- **Line:** 16
- **Error Code:** E0432
- **Missing:** `AudioGraph`, `AudioLink`, `GraphState` from pipewire module
- **Fix:** Create stub implementations in `src/audio/pipewire.rs` or add proper exports

### IPC/Unix Socket Missing (2 items)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/ipc/mod.rs`
- **Line:** 36
- **Error Code:** E0432
- **Missing:** `UnixSocketAddress`, `UnixSocketManager` from unix_socket module
- **Fix:** Create stub implementations or add proper exports in unix socket module

### Compatibility Missing Modules (7 items)
- **Files:** `src/compatibility/mod.rs` (various lines)
- **Error Code:** E0432
- **Missing:** `fedora_domination`, `fedora_missing_components`, `cross_platform`, `cross_platform_kernel`, `community_foundation`, `antix::*`, `itsfoss_inspiration_suite::*`
- **Fix:** Create placeholder module files or remove imports if not yet implemented

### Distro Missing Components (7 items)
- **Files:** `src/distro/mod.rs`, `src/distro/omarchy.rs`, `src/distro/fedora_parity.rs`
- **Error Code:** E0432
- **Missing:** `arch_missing_components`, `future_roadmap_innovations`, `arch_parity::Constraint/VersionOp`, `sovereign_2026_distro_leap_engine`, `omarchy_inspiration`
- **Fix:** Create stub implementations or remove references

### Filesystem VFS Missing (3 items)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/filesystem/mod.rs`
- **Line:** 22
- **Error Code:** E0432
- **Missing:** `FsError`, `Inode`, `VirtualFilesystem` from `crate::filesystem::vfs`
- **Fix:** Create `src/filesystem/vfs.rs` with basic VFS types

### Kernel Innovations Missing (14 items)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/kernel/mod.rs`
- **Line:** 119
- **Error Code:** E0432
- **Missing:** Various Linux/BSD innovations like `BottomHalfKernelThread`, `CgroupResourceLimits`, `FreeBsdGeomTopology`, etc.
- **Fix:** Create stub implementations in `src/kernel/linux_bsd_innovations.rs`

### Kernel Security Missing (2 items)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/kernel/mod.rs`
- **Line:** 221
- **Error Code:** E0432
- **Missing:** `kptr_restrict::DmesgRestrictLevel`, `KernelSecurityMitigations`
- **Fix:** Create `src/kernel/kptr_restrict.rs` module

### KLib Missing Modules (4 items)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/klib/mod.rs`
- **Lines:** 34-37
- **Error Code:** E0432
- **Missing:** `ring_buffer`, `slab`, `uuid`, `zero_dependency_elimination` modules
- **Fix:** Create stub implementations in klib or remove imports

### Memory Missing Module (1 item)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/memory/mod.rs`
- **Line:** 45
- **Error Code:** E0432
- **Missing:** `huge_pages` module
- **Fix:** Create `src/memory/huge_pages.rs` stub

### Network Missing Modules (11 items)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/network/mod.rs`
- **Lines:** 40, 46, 65
- **Error Code:** E0432
- **Missing:** TCP stack (2), socket primitives (3), zero-copy networking (6)
- **Fix:** Create stub implementations in network module

### Security Missing Components (5 items)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/security/mod.rs`
- **Lines:** 58, 66, 70, 149, 299
- **Error Code:** E0432
- **Missing:** `system_audit`, `filesystem_encryption`, `selinux` re-exports, `capability` module
- **Fix:** Create stub modules or fix re-exports

### SigPkg Missing Universal Engine (6 items)
- **Files:** `src/sigpkg/mod.rs`, `src/sigpkg/universal_adapter.rs`, `src/sigpkg/arch_compat.rs`
- **Error Code:** E0432
- **Missing:** `universal_engine`, `arch_compat` exports, `store` module items, adapter types
- **Fix:** Create stub implementations or fix module structure

### Scheduler/Process Missing (6 items)
- **Files:** `src/lib.rs` (lines 237, 310)
- **Error Code:** E0432, E0433
- **Missing:** `kernel::Scheduler`, `Process`, `VirtualCpu`, `ProcessGroup`, `SimpleProcessGroup`
- **Fix:** Add proper exports in kernel/process modules

### Graphics/AI/Media Missing (3 items)
- **Files:** `src/media/sovereign_screen_recorder.rs`, `src/theming/theme_engine.rs`, `src/media/mod.rs`
- **Error Code:** E0433, E0432
- **Missing:** `crate::graphics`, `crate::ai`, `pix_image_organizer::PixCatalog`
- **Fix:** Create stub modules or remove references

### Alloc Missing in no_std Context (6 items)
- **Files:** `src/productivity/sigma_office.rs`, `src/sigpkg/mod.rs`, `src/sigpkg/arch_compat.rs`
- **Lines:** Various
- **Error Code:** E0433, E0432
- **Missing:** `alloc` crate not available in no_std context
- **Fix:** Add `extern crate alloc;` at crate root and ensure no_std compatibility

### KLib Vec Missing (1 item)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/sigpkg/arch_compat.rs`
- **Line:** 552
- **Error Code:** E0433
- **Missing:** `klib::vec`
- **Fix:** Use `alloc::vec::Vec` instead or create klib::vec wrapper

---

## CATEGORY 4: Type Mismatches & Trait Bounds (81 errors)

### 4.1 SocketAddr Visibility
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/network/mod.rs`
- **Line:** 46
- **Error Code:** E0603
- **Fix:** Make `SocketAddr` enum public in socket module

### 4.2 Conflicting Trait Implementations for SigmaString (7 errors)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/klib/custom_string.rs`
- **Line:** 42
- **Error Codes:** E0119 (Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)
- **Fix:** Remove duplicate derive attributes or manual trait implementations; keep only one set

### 4.3 Conflicting Clone for BTreeMap
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/klib/btreemap.rs`
- **Line:** 9
- **Error Code:** E0119
- **Fix:** Remove manual Clone implementation (it's likely already derived or auto-implemented)

### 4.4 Invalid Copy Implementation (2 errors)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/kernel/cgroup_v2_controller.rs`
- **Lines:** 58, 125
- **Error Code:** E0204
- **Fix:** Remove `Copy` from derive attributes for structs containing non-Copy fields

### 4.5 Enum Discriminant Collision
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/drivers/usb_audio.rs`
- **Line:** 13
- **Error Code:** E0081
- **Fix:** Ensure each enum variant has a unique discriminant value

### 4.6 Path::to_string Not Available (5 errors)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/system/shredder.rs`
- **Lines:** 69, 93, 117, 141, 182
- **Error Code:** E0599
- **Fix:** Use `path.display().to_string()` or `path.to_string_lossy().to_string()` instead

### 4.7 Unstable path_is_empty Feature
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/system/shredder.rs`
- **Line:** 181
- **Error Code:** E0658
- **Fix:** Replace with stable alternative: `path.as_os_str().is_empty()` or check metadata

### 4.8 Iterator Type Mismatch
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/container/oci_runtime.rs`
- **Line:** 270
- **Error Code:** E0277
- **Fix:** Add `.map(|(a, b)| vec![a, b])` or similar transformation to match expected Vec type

### 4.9 Type Mismatches in OCI Runtime (2 errors)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/container/oci_runtime.rs`
- **Lines:** 279, 281
- **Error Code:** E0308
- **Fix:** Ensure return types match function signature (likely needs type conversion or wrapping)

### 4.10 String Comparison Type Mismatch
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/desktop/screensaver.rs`
- **Line:** 351
- **Error Code:** E0277
- **Fix:** Change `&mut str` to `&str` or compare with `.as_str()` method

### 4.11 Missing Struct Fields in DependencyResolverEngine (5 errors)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/distro/arch_parity.rs`
- **Lines:** 758, 759, 761, 765, 773
- **Error Codes:** E0560, E0609
- **Fix:** Add `profiles: HashMap<...>` and `is_cleanroom_active: bool` fields to DependencyResolverEngine struct definition

### 4.12 Missing Struct Fields in ArchPkgctlEngine (5 errors)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/distro/arch_parity.rs`
- **Lines:** 793, 794, 799, 810, 819
- **Error Codes:** E0560, E0609
- **Fix:** Add `registered_packages: Vec<...>` and `constraints: Vec<...>` fields to ArchPkgctlEngine struct definition

### 4.13 Missing Field `articles` in PacmanDatabaseEngine
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/distro/arch_parity.rs`
- **Line:** 880
- **Error Code:** E0609
- **Fix:** Add `articles` field to PacmanDatabaseEngine struct or change field name to existing field

### 4.14 Borrow Trait Bound Issues (2 errors)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/distro/manjaro.rs`
- **Lines:** 307, 308
- **Error Code:** E0277
- **Fix:** Use `.as_str()` or borrow as `&String` to satisfy Borrow trait requirements

### 4.15 Type Mismatch in Manjaro
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/distro/manjaro.rs`
- **Line:** 313
- **Error Code:** E0308
- **Fix:** Ensure types match expected signature (add type conversion or cast)

### 4.16 Framebuffer Type Issues (2 errors)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/drivers/framebuffer.rs`
- **Line:** 158
- **Error Codes:** E0308, E0277
- **Fix:** Cast u32 to usize before arithmetic: `(value as usize) + offset`

### 4.17 PCI Bus Type Mismatch
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/drivers/pci_bus.rs`
- **Line:** 220
- **Error Code:** E0308
- **Fix:** Ensure type matches expected (likely needs cast or conversion)

### 4.18 EthernetHeader Missing Debug (2 errors)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/drivers/ethernet.rs`
- **Line:** 75
- **Error Code:** E0277
- **Fix:** Add `#[derive(Debug)]` to EthernetHeader struct definition

### 4.19 AhciCommandTable Missing Clone
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/drivers/ahci_sata.rs`
- **Line:** 182
- **Error Code:** E0277
- **Fix:** Add `#[derive(Clone)]` to AhciCommandTable or use `Arc` wrapper

### 4.20 Option<AhciPort> Missing Clone
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/drivers/ahci_sata.rs`
- **Line:** 321
- **Error Code:** E0277
- **Fix:** Ensure AhciPort implements Clone, then Option will automatically be Clone

### 4.21 JournalTransaction Missing `data` Field (3 errors)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/filesystem/sigma_fs.rs`
- **Lines:** 213, 248, 782
- **Error Codes:** E0560, E0609
- **Fix:** Add `data: Vec<u8>` field to JournalTransaction struct definition

### 4.22 TimerBucket Missing Copy
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/kernel/timer_wheel.rs`
- **Line:** 290
- **Error Code:** E0277
- **Fix:** Add `#[derive(Copy, Clone)]` to TimerBucket or remove Copy requirement

### 4.23 Atomic Types Missing Clone (7 errors)
- **Files:** Various kernel, network files
- **Lines:** Multiple locations
- **Error Code:** E0277
- **Fix:** Atomic types don't implement Clone; wrap in Arc or use raw atomic operations instead of deriving Clone on parent struct

### 4.24 IcmpHeader Missing Debug
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/network/icmp.rs`
- **Line:** 214
- **Error Code:** E0277
- **Fix:** Add `#[derive(Debug)]` to IcmpHeader struct definition

### 4.25 HashSet<CapRight> Issues (11 errors)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/security/capsicum.rs`
- **Lines:** 69, 76, 82, 87, 92, 129, 134, 145, 167, 168, 169, 222
- **Error Codes:** E0277, E0599
- **Fix:** Add `#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]` to CapRight enum definition

### 4.26 ObjectClass Missing Ord (8 errors)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/security/selinux.rs`
- **Lines:** 169, 186, 236, 268 (repeated twice each)
- **Error Code:** E0277
- **Fix:** Add `#[derive(PartialOrd, Ord)]` to ObjectClass enum definition

### 4.27 Process Priority Field Missing (4 errors)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/performance/smart_optimizer.rs`
- **Lines:** 29, 31, 36, 38
- **Error Codes:** E0308, E0609
- **Fix:** Add `priority` field to Process struct in kernel module or use correct field name

### 4.28 Borrow Checker Errors (11 errors)
- **Files:** `src/audio/pipewire.rs`, `src/filesystem/ext4.rs`, `src/memory/page_cache.rs`, `src/memory/slab_allocator.rs`
- **Lines:** Various
- **Error Codes:** E0502, E0382, E0507, E0499
- **Fix:** Refactor code to avoid multiple mutable borrows, use cloning where necessary, or restructure ownership

### 4.29 Packed Struct Unaligned References (2 errors)
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/filesystem/ext4.rs`
- **Lines:** 330, 340
- **Error Code:** E0793
- **Fix:** Use `std::ptr::addr_of!` macro or copy field value before accessing

### 4.30 Immutable Reference Assignment
- **File:** `/home/aaryansinghchauhan/SigmaOS/src/memory/page_cache.rs`
- **Line:** 120
- **Error Code:** E0594
- **Fix:** Ensure method takes `&mut self` instead of `&self` for mutation

---

## CATEGORY 5: Other (15 errors)

Most errors in this category are duplicates of those already listed above in type mismatches and are accounted for in the count.

---

## Implementation Priority Order

### Phase 1: Syntax & Duplicates (Quick Wins - 16 errors)
1. Fix syntax errors (2 errors)
2. Remove duplicate module declarations (14 errors)

### Phase 2: Missing Core Types (High Priority - 56 errors)
3. Create missing VFS types
4. Create missing process types
5. Create missing driver stubs
6. Fix alloc imports for no_std
7. Create missing security modules
8. Create missing network modules
9. Create missing klib modules

### Phase 3: Trait Implementations (Medium Priority - 30 errors)
10. Fix derive attribute conflicts
11. Add missing Debug/Clone implementations
12. Add missing Ord/Hash implementations
13. Fix enum discriminants

### Phase 4: Struct Fields (Medium Priority - 18 errors)
14. Add missing struct fields
15. Fix field access errors

### Phase 5: Type Conversions (Low Priority - 33 errors)
16. Fix type mismatches
17. Fix comparison operators
18. Fix iterator transformations

### Phase 6: Borrow Checker (Complex - 15 errors)
19. Refactor ownership issues
20. Fix multiple mutable borrows
21. Fix moved value errors

---

## Verification Command

After each phase, run:
```bash
cargo check --lib 2>&1 | grep '^error' | wc -l
```

Expected error count should decrease after each phase:
- After Phase 1: ~152 errors remaining
- After Phase 2: ~96 errors remaining
- After Phase 3: ~66 errors remaining
- After Phase 4: ~48 errors remaining
- After Phase 5: ~15 errors remaining
- After Phase 6: 0 errors (clean build)

---

## Notes

- All fixes follow AGENTS.md Algorithm A for duplicate resolution
- All missing modules should use `#![no_std]` and safe Rust
- Zero external dependencies policy maintained
- All struct definitions should use zero third-party crates
- Priority is to achieve zero compilation errors as mandated by build rules

---

*End of Fix Plan*
