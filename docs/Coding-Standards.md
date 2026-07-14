# SigmaOS Coding Standards

> **Core Principle**: "Sovereignty is the ultimate efficiency."  
> All code in SigmaOS must be self-contained, first-principles, and demonstrably correct.

---

## Table of Contents

1. [Language Policy](#language-policy)
2. [No-Stdlib / No-External-Libs Constraint](#no-stdlib-constraint)
3. [OOP Principles](#oop-principles)
4. [Rust Standards](#rust-standards)
5. [Zig Standards](#zig-standards)
6. [Nim Standards](#nim-standards)
7. [Naming Conventions](#naming-conventions)
8. [Error Handling](#error-handling)
9. [Testing Requirements](#testing-requirements)
10. [Documentation Standards](#documentation-standards)

---

## Language Policy

| Layer | Preferred Language | Rationale |
| :--- | :--- | :--- |
| Kernel core | **Rust** | Memory safety, `#![no_std]`, zero-cost abstractions |
| Driver framework | **Zig** | Comptime generics, direct hardware access, C interop |
| Userland daemons | **Nim** | Expressive OOP, lightweight, compile-time dispatch |
| AI/inference | **Rust** | Quantized math, deterministic behaviour |
| Desktop GUI | **Nim** | Widget hierarchy via method dispatch |
| Package manager | **Rust** | Correctness, trait-based extensibility |

**Prohibited**: Python, JavaScript, Go, Java, C++ in the kernel or drivers.  
C is permitted *only* as FFI glue with explicit `unsafe` documentation.

---

## No-Stdlib Constraint

### Rust (`#![no_std]`)

```rust
// MANDATORY for all kernel crates
#![no_std]
#![no_main]

// MANDATORY if heap is used (only with sigma's own allocator)
extern crate alloc;
```

- **No `std::` imports** anywhere in `kernel/` or `userland/agent/`.
- **No external crates** without explicit approval in `LICENSES.md`.
- Every `[dependencies]` entry must have a `# JUSTIFICATION:` comment.
- Implement all data structures from scratch (ring buffer, hash map, etc.).

### Zig (freestanding)

```zig
// Target flag required for ALL kernel drivers:
// zig build-exe -target x86_64-freestanding-none

// FORBIDDEN:
const std = @import("std");   // ❌ Never in kernel/drivers/

// ALLOWED (comptime only):
const builtin = @import("builtin");  // ✅ Target/arch detection only
```

### Nim (`--mm:none`)

```nim
# REQUIRED compiler flags for all daemons:
# nim compile --mm:none --verbosity:0

# FORBIDDEN:
import os        # ❌ No OS-level imports
import strutils  # ❌ No standard library imports

# REQUIRED:
{.push raises: [].}  # ✅ Force explicit error types
```

---

## OOP Principles

SigmaOS mandates the four core OOP pillars across all languages:

### 1. Encapsulation

- All internal state is `private` or `pub(crate)`.
- Public API is defined via **trait** (Rust), **vtable struct** (Zig), or **method** (Nim).

```rust
// ✅ CORRECT: encapsulated internal state
pub struct SigmaAllocator {
    heap_start: usize,  // private — internal implementation detail
    allocated:  usize,
}
impl SigmaAllocator {
    pub fn alloc(&mut self, size: usize) -> *mut u8 { /* ... */ }
}
```

### 2. Abstraction

- Abstract interfaces must be defined before concrete types.
- Rust: use `trait`, Zig: use vtable `struct`, Nim: use `ref object of RootObj`.

```zig
// ✅ CORRECT: abstract vtable defined before concrete impl
pub const DriverVtable = struct {
    initFn:  *const fn(ctx: *anyopaque) void,
    readFn:  *const fn(ctx: *anyopaque, buf: []u8) usize,
};
```

### 3. Inheritance / Composition

- **Prefer composition over inheritance** (except for Nim `ref object` hierarchies).
- Rust: embed structs as fields; Zig: embed pointers to sub-systems.

### 4. Polymorphism

- Rust: `dyn Trait` for runtime dispatch; generics for compile-time.
- Zig: function pointer vtables for runtime dispatch; `comptime` for static.
- Nim: `method` + `procCall` for virtual dispatch.

```nim
# ✅ CORRECT: Nim polymorphic method dispatch
type
  BaseWidget* = ref object of RootObj
    id*: uint32

method paint*(self: BaseWidget) {.base.} = discard

type
  ButtonWidget* = ref object of BaseWidget
    label*: array[32, char]

method paint*(self: ButtonWidget) =
  procCall self.BaseWidget.paint()
  # draw button pixels
```

---

## Rust Standards

### File header (mandatory)

```rust
// <filename>.rs — <Short description>
// Language: Rust (#![no_std], no external crates)
// OOP: <TraitName> (abstract), <ConcreteType> (impl), <Compositor> (composition)
// Specification: <wiki or docs link>
#![no_std]
```

### Safety

- Mark all `unsafe` blocks with a `// SAFETY:` comment explaining why it's safe.
- Never use `mem::transmute` without explicit justification.
- Prefer `*const` over `*mut` wherever possible.

### Memory

- Use `BumpAllocator` for kernel heap (from `kernel/src/custom_allocators.rs`).
- Never call `Box::new`, `Vec::new`, or other `alloc` APIs without registering the global allocator.

---

## Zig Standards

### File header (mandatory)

```zig
// <filename>.zig — <Short description>
// Language: Zig (no stdlib, freestanding)
// OOP: <abstract vtable> → <ConcreteType>
// Specification: <wiki or docs link>
```

### Const generics

- Use `comptime` constants and generic functions instead of runtime dispatch where performance is critical.
- Mark compile-time-only values with `comptime` keyword.

### Error handling

- Use `error{}` union types for all fallible operations.
- Never use `unreachable` in production paths without a `// PROOF:` comment.

---

## Nim Standards

### File header (mandatory)

```nim
## <filename>.nim — <Short description>
## Language: Nim (freestanding — no stdlib, no third-party packages)
## OOP: <BaseType> (abstract), <DerivedType> (derived)
## Specification: <wiki or docs link>
{.push raises: [].}
```

### Type system

- All primitive types must be defined explicitly (e.g., `type SigmaU8* = uint8`).
- Never use Nim's `string`, `seq`, or `Table` types in kernel-facing code.
- Use fixed-size `array` types only.

### Method dispatch

- Use `method` for virtual dispatch; `proc` for non-virtual.
- Always call `procCall self.BaseType.method()` in overriding methods.

---

## Naming Conventions

| Element | Rust | Zig | Nim |
| :--- | :--- | :--- | :--- |
| Types | `PascalCase` | `PascalCase` | `PascalCase` |
| Functions | `snake_case` | `camelCase` | `camelCase` |
| Constants | `SCREAMING_SNAKE` | `SCREAMING_SNAKE` | `SCREAMING_SNAKE` |
| Traits/interfaces | `PascalCase` | `PascalCaseVtable` | `PascalCase` |
| Files | `snake_case.rs` | `snake_case.zig` | `snake_case.nim` |
| Modules | `snake_case` | `snake_case` | `snake_case` |

---

## Error Handling

- **Never panic** in kernel code. Use `Result<T, E>` (Rust), `!ErrorType` (Zig), or explicit error enums (Nim).
- **Never use `unwrap()`** in production paths. Always match or propagate.
- Map errors from lower layers to higher-level domain errors at subsystem boundaries.

```rust
// ❌ FORBIDDEN in kernel paths:
let v = result.unwrap();

// ✅ REQUIRED:
let v = result.map_err(|e| KernelError::from(e))?;
```

---

## Testing Requirements

Every module **must** include unit tests:

| Language | Test location | Command |
| :--- | :--- | :--- |
| Rust | `#[cfg(test)]` mod at bottom of file | `cargo test` |
| Zig | `test "..."` blocks at bottom of file | `zig test <file>` |
| Nim | `proc test*()` returning `bool` at bottom | `nim compile --run` |

Minimum coverage requirements:
- Happy path: **required**.
- At least one error path: **required**.
- Boundary conditions (e.g., buffer full, empty input): **strongly recommended**.

---

## Documentation Standards

Every public function/type must have:

```rust
/// Short one-line summary.
///
/// # Arguments
/// - `param`: Description.
///
/// # Returns
/// What the return value means.
///
/// # Safety
/// (If `unsafe`) Why this is sound.
pub fn my_function(param: u32) -> Result<u32, KernelError> { ... }
```

For Zig:
```zig
/// Short summary.
/// Returns the number of bytes written, or FsError on failure.
pub fn write(self: *SigmaExt4, inode: Inode, ...) FsError!Usize { ... }
```

For Nim:
```nim
## Short summary.
## Returns true if operation succeeded.
proc doThing*(self: BaseWidget): bool = ...
```

---

*For the full engineering philosophy, see [Engineering-Principles](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Engineering-Principles-Roadmap) on the Wiki.*
