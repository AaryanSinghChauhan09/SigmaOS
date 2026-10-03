# FreeBSD Capsicum Implementation Plan for SigmaOS

## Executive Summary

This plan implements a complete FreeBSD Capsicum capability-based security model for SigmaOS, including per-FD capability rights, process capability mode, syscall handlers, and process descriptor support. The implementation follows SigmaOS guidelines: `#![no_std]` with `extern crate alloc`, safe Rust, zero external dependencies, and full compatibility with existing code.

## Context & Constraints

**Existing code to preserve:**
- `src/security/bsd_hardening.rs` exports `CapsicumCapability` (enum) and `CapsicumManager` (struct)
- `src/security/capsicum.rs` exports `CapEntry`, `CapMode`, `CapRight` (enum), `CapabilitySandbox`
- `src/security/mod.rs` re-exports both sets

**Key constraint:** The new types must NOT conflict with existing names. We'll use distinct names for new components:
- `CapRightsMask` (new 64-bit bitmask struct) vs. `CapRight` (existing enum)
- `CapabilityMode` (new enum) vs. `CapMode` (existing enum)
- `FdCapTable` (new per-FD rights table) vs. `CapsicumManager` (existing)
- `CapsicumSandbox` (new full implementation) vs. `CapabilitySandbox` (existing)

**Build verification:** `cargo check --lib 2>&1 | grep '^error' | wc -l` must return 0

**Test verification:** Comprehensive `#[cfg(test)]` unit tests in each file

---

## Implementation Steps

### Step 1: Create `src/security/cap_rights.rs` — Capability Rights Bitmask

**What:** Implement a 64-bit capability rights bitmask with 30+ FreeBSD CAP_* constants, rights operations (union, intersection, contains), and inheritance rules.

**Files:**
- Create: `src/security/cap_rights.rs`

**Implementation details:**
- Define `CapRightsMask` struct wrapping `u64` with bitfield operations
- Define 30+ CAP_* constants as `u64` (CAP_READ = 0x0000000000000001, CAP_WRITE = 0x0000000000000002, CAP_SEEK = 0x0000000000000004, etc.)
- Implement methods:
  - `new(rights: u64) -> Self`
  - `contains(&self, rights: u64) -> bool` — check if all rights present
  - `union(&self, other: &Self) -> Self` — bitwise OR
  - `intersection(&self, other: &Self) -> Self` — bitwise AND
  - `limit(&mut self, rights: u64)` — restrict to subset (bitwise AND)
  - `is_subset_of(&self, other: &Self) -> bool` — check if all our rights are in other
- Use `#![cfg_attr(not(any(feature = "standalone_test", test)), no_std)]` like existing `capsicum.rs`
- `extern crate alloc;` at top
- Include comprehensive tests covering all operations

**Verification:**
```bash
cargo check --lib 2>&1 | grep '^error' | wc -l
# Expected: 0
cargo test --lib security::cap_rights --features standalone_test
# Expected: All tests pass
```

---

### Step 2: Update `src/security/mod.rs` — Add cap_rights Module

**What:** Add `pub mod cap_rights;` and re-export the new types without conflicting with existing exports.

**Files:**
- Modify: `src/security/mod.rs`

**Implementation details:**
- Add `pub mod cap_rights;` after `pub mod capsicum;` (line ~8)
- Add re-exports after existing capsicum exports (around line 155):
  ```rust
  pub use cap_rights::{
      CapRightsMask, CAP_READ, CAP_WRITE, CAP_SEEK, CAP_FSTAT, CAP_FCNTL,
      CAP_FCHDIR, CAP_FCHFLAGS, CAP_FCHMOD, CAP_FCHOWN, CAP_FLOCK,
      CAP_FSYNC, CAP_FTRUNCATE, CAP_MMAP, CAP_MMAP_R, CAP_MMAP_W,
      CAP_MMAP_X, CAP_MMAP_RW, CAP_MMAP_RX, CAP_MMAP_WX, CAP_MMAP_RWX,
      CAP_CREATE, CAP_DELETE, CAP_UNLINK, CAP_MKDIR, CAP_RMDIR,
      CAP_LOOKUP, CAP_ACCEPT, CAP_BIND, CAP_CONNECT, CAP_LISTEN,
      CAP_RECV, CAP_SEND, CAP_IOCTL, CAP_EVENT, CAP_PDWAIT, CAP_ALL,
  };
  ```
- Preserve all existing exports unchanged

**Verification:**
```bash
cargo check --lib 2>&1 | grep '^error' | wc -l
# Expected: 0
```

---

### Step 3: Replace `src/security/capsicum.rs` — Full Implementation

**What:** Replace the existing thin stub with a complete FreeBSD Capsicum implementation including per-FD capability rights table, process capability mode state, rights-checking on syscall entry, capability mode restrictions, process descriptor support, and audit logging hooks.

**Files:**
- Modify: `src/security/capsicum.rs` (REPLACE most content, preserve header comments and existing type names where needed)

**Implementation details:**

**New structures to add:**
1. `CapabilityMode` enum:
   - `Normal` — not in capability mode
   - `Capability` — restricted mode (no global namespace operations)

2. `FdCapTable` struct — per-file-descriptor capability rights table:
   - `rights: HashMap<i32, CapRightsMask>` (fd -> rights)
   - Methods:
     - `new() -> Self`
     - `set_rights(fd: i32, rights: CapRightsMask)`
     - `get_rights(fd: i32) -> Option<CapRightsMask>`
     - `limit_rights(fd: i32, rights: CapRightsMask) -> Result<(), i32>` — can only narrow, never expand
     - `check_rights(fd: i32, required: u64) -> bool`
     - `inherit_rights(old_fd: i32, new_fd: i32)` — for dup/dup2

3. `CapsicumSandbox` struct — main sandbox state:
   - `mode: CapabilityMode`
   - `fd_table: FdCapTable`
   - `in_capability_mode: bool` (cached flag)
   - Methods:
     - `new() -> Self`
     - `enter_capability_mode(&mut self) -> Result<(), i32>` — irreversible, sets flag
     - `is_in_capability_mode(&self) -> bool`
     - `check_syscall_allowed(&self, syscall_name: &str) -> Result<(), i32>` — deny open/openat in cap mode
     - `check_fd_operation(&self, fd: i32, required_rights: u64) -> Result<(), i32>`
     - `limit_fd_rights(&mut self, fd: i32, rights: CapRightsMask) -> Result<(), i32>`
     - `get_fd_rights(&self, fd: i32) -> Option<CapRightsMask>`

4. Keep existing `CapRight` enum, `CapMode` enum, `CapEntry` struct, `CapabilitySandbox` struct for backward compatibility (mark with doc comment "Legacy interface, prefer CapsicumSandbox")

**Syscall deny list in capability mode:**
- `open`, `openat`, `creat`, `mknodat`, `linkat`, `symlinkat`, `unlinkat`, `renameat`, `chdir`, `fchdir` (without CAP_FCHDIR), `chroot`, `mount`, `umount`

**Error codes:**
- `ECAPMODE = 94` (Not permitted in capability mode)
- `ENOTCAPABLE = 93` (Capabilities insufficient)
- `EINVAL = 22` (Invalid argument)

**Audit logging hooks:**
- `log_capability_violation(fd: i32, required_rights: u64, actual_rights: u64)`
- `log_syscall_denied(syscall_name: &str)`

**Tests to include:**
- `test_enter_capability_mode()` — verify irreversible
- `test_capability_mode_denies_open()`
- `test_fd_rights_limit_cannot_expand()`
- `test_fd_rights_inheritance_on_dup()`
- `test_check_fd_operation_with_sufficient_rights()`
- `test_check_fd_operation_with_insufficient_rights()`

**Verification:**
```bash
cargo check --lib 2>&1 | grep '^error' | wc -l
# Expected: 0
cargo test --lib security::capsicum --features standalone_test
# Expected: All tests pass
```

---

### Step 4: Create `src/syscall/capsicum_syscalls.rs` — Syscall Handlers

**What:** Implement syscall handler stubs for FreeBSD Capsicum syscalls: `cap_enter`, `cap_getmode`, `cap_rights_limit`, `cap_rights_get`, `cap_ioctls_limit`, `cap_fcntls_limit`, `pdfork`, `pdkill`, `pdwait4`.

**Files:**
- Create: `src/syscall/capsicum_syscalls.rs`

**Implementation details:**
- Use `#![no_std]` compatible style like other syscall modules
- Import `crate::security::{CapsicumSandbox, CapRightsMask, CAP_*}`
- Define syscall handler functions:
  1. `sys_cap_enter() -> Result<(), i32>` — enter capability mode (irreversible)
  2. `sys_cap_getmode(mode_out: *mut u32) -> Result<(), i32>` — query capability mode status
  3. `sys_cap_rights_limit(fd: i32, rights: u64) -> Result<(), i32>` — limit FD rights
  4. `sys_cap_rights_get(fd: i32, rights_out: *mut u64) -> Result<(), i32>` — query FD rights
  5. `sys_cap_ioctls_limit(fd: i32, cmds: *const u64, ncmds: usize) -> Result<(), i32>` — stub (returns Ok for now)
  6. `sys_cap_fcntls_limit(fd: i32, fcntls: u32) -> Result<(), i32>` — stub (returns Ok for now)
  7. `sys_pdfork(fdp: *mut i32, flags: i32) -> Result<i32, i32>` — stub returns child PID
  8. `sys_pdkill(fd: i32, signal: i32) -> Result<(), i32>` — stub returns Ok
  9. `sys_pdwait4(fd: i32, status: *mut i32, options: i32) -> Result<i32, i32>` — stub returns 0

- Each function takes raw syscall arguments and returns `Result<T, i32>` (Ok(value) or Err(errno))
- Use `unsafe` only at pointer dereference boundaries (validate pointers first)
- Include doc comments with FreeBSD man page references

**Tests to include:**
- `test_cap_enter_succeeds()`
- `test_cap_getmode_returns_correct_state()`
- `test_cap_rights_limit_restricts_fd()`
- `test_cap_rights_get_retrieves_fd_rights()`

**Verification:**
```bash
cargo check --lib 2>&1 | grep '^error' | wc -l
# Expected: 0
cargo test --lib syscall::capsicum_syscalls --features standalone_test
# Expected: All tests pass
```

---

### Step 5: Update `src/syscall/mod.rs` — Add capsicum_syscalls Module

**What:** Add `pub mod capsicum_syscalls;` to the syscall module registry.

**Files:**
- Modify: `src/syscall/mod.rs`

**Implementation details:**
- Add `pub mod capsicum_syscalls;` after `pub mod bpf_syscalls;` (around line 16)
- No re-exports needed initially (syscalls accessed via fully qualified path)

**Verification:**
```bash
cargo check --lib 2>&1 | grep '^error' | wc -l
# Expected: 0
```

---

### Step 6: Resolve Potential Naming Conflicts

**What:** Audit all new type names against existing exports and resolve any duplicate definition errors found during compilation.

**Files:**
- Check: `src/security/mod.rs`, `src/security/bsd_hardening.rs`, `src/security/capsicum.rs`

**Implementation details:**
- Run `cargo check --lib 2>&1 | grep "error\[E0428\]"` to find duplicate definitions
- If conflicts found:
  - **Option A:** Rename new types with `New` prefix (e.g., `NewCapabilityMode`)
  - **Option B:** Deprecate old types with `#[deprecated]` attribute and doc comment redirecting to new types
  - **Option C:** Use module-qualified paths in re-exports to distinguish (e.g., `pub use capsicum::CapabilityMode as CapsicumCapabilityMode`)
- Preferred approach: Option A — keep backward compatibility, new types coexist
- Update all internal references to use correct type names

**Verification:**
```bash
cargo check --lib 2>&1 | grep '^error' | wc -l
# Expected: 0
```

---

### Step 7: Integration Testing and Documentation

**What:** Create integration tests demonstrating full Capsicum workflow and update inline documentation.

**Files:**
- Modify: `src/security/capsicum.rs` (add module-level doc comments)
- Create: `tests/capsicum_integration_test.rs` (optional, if tests/ directory supports it)

**Implementation details:**
- Add comprehensive module-level doc comment in `capsicum.rs`:
  ```rust
  //! # FreeBSD Capsicum Capability Model
  //!
  //! Complete implementation of FreeBSD's Capsicum capability-based security model.
  //!
  //! ## Core Concepts
  //!
  //! 1. **Capability Mode**: Process enters a restricted sandbox via `cap_enter()` (irreversible)
  //! 2. **File Descriptor Rights**: Each FD has a capability rights mask that can only be narrowed
  //! 3. **Global Namespace Denial**: In capability mode, syscalls like `open()` are denied
  //!
  //! ## Usage Example
  //!
  //! ```no_run
  //! use sigmaos::security::{CapsicumSandbox, CapRightsMask, CAP_READ, CAP_WRITE};
  //!
  //! let mut sandbox = CapsicumSandbox::new();
  //! sandbox.limit_fd_rights(3, CapRightsMask::new(CAP_READ)).unwrap();
  //! sandbox.enter_capability_mode().unwrap();
  //! // Now process cannot open new files, only use existing FDs with limited rights
  //! ```
  //!
  //! ## References
  //!
  //! - FreeBSD `cap_enter(2)`, `cap_rights_limit(2)` man pages
  //! - Capsicum: practical capabilities for UNIX (USENIX Security 2010)
  ```

- Integration test demonstrating:
  1. Open file, get FD
  2. Limit FD rights to CAP_READ only
  3. Enter capability mode
  4. Verify read succeeds
  5. Verify write fails with ENOTCAPABLE
  6. Verify open() fails with ECAPMODE

**Verification:**
```bash
cargo check --lib 2>&1 | grep '^error' | wc -l
# Expected: 0
cargo test --lib security::capsicum --features standalone_test
# Expected: All tests pass including new integration tests
```

---

## Final Verification Protocol

After all steps complete:

1. **Compilation check:**
   ```bash
   cargo check --lib 2>&1 | grep '^error' | wc -l
   ```
   Must return: **0**

2. **Unit test execution:**
   ```bash
   cargo test --lib security::cap_rights --features standalone_test
   cargo test --lib security::capsicum --features standalone_test
   cargo test --lib syscall::capsicum_syscalls --features standalone_test
   ```
   Expected: **All tests pass**

3. **Full test suite:**
   ```bash
   ./run_sigma_tests.sh
   ```
   Expected: **100% pass rate** (no regressions)

4. **Documentation check:**
   ```bash
   cargo doc --lib --no-deps
   ```
   Expected: **No warnings, docs build successfully**

---

## Potential Issues and Resolutions

### Issue 1: Name conflicts with existing types

**Symptom:** `error[E0428]: the name 'X' is defined multiple times`

**Resolution:** Follow Step 6 — rename new types with distinguishing suffixes or prefixes. Update `mod.rs` re-exports to use module-qualified paths if needed.

### Issue 2: HashMap not available in no_std context

**Symptom:** `error[E0433]: failed to resolve: use of undeclared type 'HashMap'`

**Resolution:** Use `crate::klib::HashMap` instead of `std::collections::HashMap`. Check `src/klib/mod.rs` for available collections. If not present, use `alloc::collections::BTreeMap` as fallback.

### Issue 3: Pointer validation in syscall handlers

**Symptom:** Unsafe pointer dereferences without validation

**Resolution:** Add safety checks before dereferencing:
```rust
if mode_out.is_null() {
    return Err(EFAULT); // Bad address
}
unsafe { *mode_out = mode_value; }
```

### Issue 4: Test failures due to missing std features

**Symptom:** Tests fail to compile in no_std mode

**Resolution:** Already using conditional compilation pattern from existing code:
```rust
#![cfg_attr(not(any(feature = "standalone_test", test)), no_std)]
```

---

## Dependencies on External Code

- `crate::klib::HashMap` or `alloc::collections::BTreeMap` for FD -> rights table
- `alloc::string::String` for audit logging messages
- `alloc::vec::Vec` for ioctl/fcntl command lists
- Existing `src/kernel/pidfd.rs` for process descriptor integration (optional, can stub initially)

---

## Success Criteria

1. ✅ All new files compile without errors
2. ✅ Zero naming conflicts with existing code
3. ✅ All unit tests pass
4. ✅ `./run_sigma_tests.sh` shows 100% pass rate
5. ✅ Documentation builds without warnings
6. ✅ New types properly exported from `src/security/mod.rs`
7. ✅ Syscall handlers accessible from `src/syscall/capsicum_syscalls`
8. ✅ Full capability mode workflow demonstrated in tests

---

## Estimated Complexity

- **Step 1 (cap_rights.rs):** ~150 lines (30 constants + struct + methods + tests)
- **Step 2 (mod.rs update):** ~5 lines
- **Step 3 (capsicum.rs replacement):** ~400 lines (3 structs + methods + extensive tests)
- **Step 4 (capsicum_syscalls.rs):** ~250 lines (9 syscall handlers + tests)
- **Step 5 (mod.rs update):** ~2 lines
- **Step 6 (conflict resolution):** Variable, likely minimal with careful naming
- **Step 7 (documentation):** ~100 lines of doc comments + integration test

**Total estimated:** ~900 lines of new/modified code

---

## References

- FreeBSD man pages: `cap_enter(2)`, `cap_rights_limit(2)`, `cap_ioctls_limit(2)`, `pdfork(2)`, `pdkill(2)`, `pdwait4(2)`
- Capsicum: practical capabilities for UNIX (USENIX Security 2010)
- FreeBSD src: `sys/kern/kern_capsicum.c`, `sys/sys/capsicum.h`
- Existing SigmaOS code: `src/security/pledge_unveil.rs` (pattern reference), `src/kernel/pidfd.rs` (process descriptor pattern)

---

**Plan Status:** Ready for implementation
**Verification Command:** `cargo check --lib 2>&1 | grep '^error' | wc -l` (must be 0 after each step)
