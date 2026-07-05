# SigmaOS Coding Standards
**Version:** 1.0 | Applies to: all SigmaOS source code

---

## 1. Languages & Domains

| Domain | Primary Language | Secondary |
|---|---|---|
| Kernel (`kernel/`) | Rust (`#![no_std]`) | Assembly (arch-specific) |
| Userland tools | Rust (std) | Zig |
| Security-critical crypto | Ada/SPARK | — |
| Shell (sigma-sh) | Rust (std) | — |
| Package manager (sigpkg) | Rust (std) | — |
| Drivers | Rust + C FFI stubs | C (legacy compat) |
| Build scripts | Zig build + justfile | — |
| Config files | TOML | — |
| Documentation | Markdown | — |

See [LANGUAGE_POLICY.md](../LANGUAGE_POLICY.md) for the full FFI rules.

---

## 2. Rust Style Rules

### 2.1 General
```rust
// ✅ Good: descriptive, snake_case names
fn allocate_kernel_page(size: usize) -> Result<*mut u8, KernelError> { ... }

// ❌ Bad: abbreviations, camelCase
fn allocKPg(sz: usize) -> *mut u8 { ... }
```

### 2.2 Error Handling
- **Never** use `.unwrap()` in kernel code — always propagate with `?` or explicit match
- Use `Result<T, E>` for all fallible operations
- Define domain-specific error enums (no `Box<dyn Error>` in `no_std`)

```rust
// ✅ Good
pub enum MemoryError {
    OutOfMemory,
    InvalidAlignment,
    AddressNotMapped(usize),
}

// ❌ Bad
fn alloc(size: usize) -> *mut u8 {
    some_fn().unwrap() // BANNED in kernel code
}
```

### 2.3 Unsafe
- `unsafe` blocks require a `// SAFETY: <justification>` comment immediately above
- Every `unsafe` block must be reviewed by 2 maintainers in PR
- Minimize `unsafe` surface — wrap in safe abstractions immediately

```rust
// ✅ Good
// SAFETY: ptr is guaranteed non-null and aligned by the allocator contract
let val = unsafe { ptr.read() };

// ❌ Bad  
let val = unsafe { ptr.read() }; // no safety comment
```

### 2.4 Naming
| Item | Convention | Example |
|---|---|---|
| Types, traits, enums | `UpperCamelCase` | `MemoryRegion`, `KernelError` |
| Functions, variables | `snake_case` | `init_memory()`, `page_size` |
| Constants | `SCREAMING_SNAKE_CASE` | `PAGE_SIZE`, `MAX_PROCS` |
| Modules | `snake_case` | `mod memory_manager;` |
| Files | `snake_case.rs` | `memory_manager.rs` |

### 2.5 Documentation
- All `pub` items **must** have `///` doc comments
- Include `# Examples` sections for public API functions
- Use `#[doc(hidden)]` only for true implementation details

```rust
/// Allocate `size` bytes from the kernel heap.
///
/// # Errors
/// Returns `MemoryError::OutOfMemory` if the heap is exhausted.
///
/// # Examples
/// ```
/// let ptr = sigma_malloc(1024)?;
/// ```
pub fn sigma_malloc(size: usize) -> Result<*mut u8, MemoryError> { ... }
```

---

## 3. Zig Style Rules (Userland)

- Use `comptime` for all generic code — no runtime dispatch where possible
- Error unions: `!T` for all fallible functions
- No heap allocation in hot paths without explicit `Allocator` parameter
- All public functions documented with `/// ...` comments
- Build: always go through `build.zig`, no raw `zig build-exe`

```zig
// ✅ Good
pub fn readFile(allocator: std.mem.Allocator, path: []const u8) ![]u8 {
    // ...
}

// ❌ Bad
pub fn readFile(path: []const u8) []u8 {
    // panics on error, no allocator control
}
```

---

## 4. Ada/SPARK Rules (Security-Critical Code)

- Every package spec (`.ads`) must have SPARK mode enabled: `pragma SPARK_Mode (On);`
- All subprograms must have `Pre` and `Post` contracts
- Run `gnatprove` in CI — no merge without 0 violations
- No dynamic allocation in SPARK-proved subprograms

```ada
-- ✅ Good
procedure Encrypt
  (Key  : in  Key_Type;
   Data : in  Byte_Array;
   Out  : out Byte_Array)
  with
    Pre  => Key'Length = 32 and Data'Length > 0,
    Post => Out'Length = Data'Length;
```

---

## 5. Git Commit Style

Format: `type(scope): short description`

| Type | When to use |
|---|---|
| `feat` | New feature |
| `fix` | Bug fix |
| `refactor` | Code change without behavior change |
| `docs` | Documentation only |
| `test` | Adding/fixing tests |
| `chore` | Build/CI/tooling changes |
| `security` | Security-related changes |
| `perf` | Performance improvements |

```
feat(sigma-sh): add if/else scripting support
fix(sigpkg): correct SemVer comparison for pre-release versions
security(sigma-crypto): replace placeholder SHA-256 with ring crate
docs(wiki): add Absorption Matrix page
```

---

## 6. PR Requirements

- [ ] All CI checks pass (build, test, clippy, fmt)
- [ ] `unsafe` code has `// SAFETY:` comments
- [ ] New public APIs have `///` doc comments
- [ ] Tests added for new functionality
- [ ] No `unwrap()` in kernel code
- [ ] No hardcoded paths or magic numbers (use named constants)
- [ ] SPARK proofs pass for any `security/` changes
- [ ] PR description references GitHub issue

---

## 7. Testing Standards

### Kernel Tests
- Unit tests in `#[cfg(test)]` modules within each file
- Integration tests in `kernel/tests/`
- QEMU smoke tests run in CI via `sigma_qemu.yml`

### Userland Tests
- `cargo test` for all crates
- Property-based tests with `proptest` for parser/crypto code
- `cargo bench` for performance-critical paths

### Minimum Coverage
| Component | Required Coverage |
|---|---|
| `sigma-crypto` | 95% (SPARK proofs supplement) |
| `sigma-sh` parser | 90% |
| `sigpkg` resolver | 85% |
| Kernel memory | 80% |

---

## 8. Directory Conventions

```
kernel/src/
  ├── arch/         # Arch-specific: x86_64, riscv64, aarch64
  ├── memory/       # PMM, VMM, allocator
  ├── sched/        # Scheduler (EEVDF)
  ├── drivers/      # Driver registry + device drivers
  ├── fs/           # VFS + filesystem implementations
  ├── syscall/      # Syscall dispatch table
  └── security/     # Capability engine, audit hook

userland/
  ├── shell/        # sigma-sh (Nim stubs → Rust migration in progress)
  ├── sigpkg/       # sigpkg package manager (Rust)
  ├── coreutils/    # sigma-core-utils (Rust)
  └── apps/         # Desktop apps (Zig)

security/           # Ada/SPARK security modules
docs/
  ├── wiki/         # GitHub Wiki source (synced via CI)
  └── *.md          # Project-level docs
```

---

*Maintained by SigmaOS core team. For questions, open a [discussion](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions).*
