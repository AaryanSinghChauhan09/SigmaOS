# Contributing to SigmaOS

Thank you for contributing! SigmaOS is a sovereign, zero-dependency operating system written in Rust. These guidelines help keep the codebase consistent, safe, and high quality.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Philosophy](#development-philosophy)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Security Guidelines](#security-guidelines)
- [Documentation Requirements](#documentation-requirements)
- [Testing Requirements](#testing-requirements)

---

## Code of Conduct

Be respectful and constructive. Harassment, personal attacks, or discriminatory language will result in removal from the project.

---

## Getting Started

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
cargo check   # verify it compiles
cargo test    # run tests
```

**Dependencies:** Rust nightly (no_std compatible toolchain). No external C libraries required.

---

## Development Philosophy

### Zero External Library Dependency
SigmaOS aims to eliminate dependency on standard libraries and external crates wherever possible:

- Use `src/klib/` collections (Vec, HashMap, BTreeMap, HashSet, VecDeque) instead of `std::collections`
- Use `src/kernel/crypto/` for cryptographic primitives instead of `openssl` or `ring`
- Implement algorithms natively rather than pulling in crates

### No Predefined Functions for Core OS Logic
Core kernel, security, and driver code must not rely on:
- `std::` (use `klib` equivalents)
- External allocators (use `src/klib/buddy_allocator.rs` or `src/kernel/slab_allocator.rs`)
- Platform libc functions (implement natively in `#[no_std]` Rust)

### OOP Architecture
Use Rust's trait system to model object-oriented designs:
- **Traits** for interfaces (polymorphism)
- **Structs** with `impl` blocks (encapsulation)
- **Enums** for type-safe state machines
- Avoid `dyn` dispatch in hot paths — prefer generics

---

## Pull Request Process

1. **Fork** the repo and create a branch: `git checkout -b feat/your-feature`
2. **Implement** your changes following the coding standards below
3. **Test**: add unit tests in the same file (`#[cfg(test)]` module)
4. **Commit**: use conventional commits — `feat:`, `fix:`, `docs:`, `refactor:`, `security:`
5. **Push** and open a PR against `main`
6. **Fill** the PR template (automatically shown)
7. **Wait** for review — typically 2-5 business days

### PR Title Format
```
<type>(<scope>): <short description>   (max 70 chars)
```
Examples:
- `feat(kernel): add NUMA-aware scheduler`
- `fix(crypto): remove hardcoded key in LUKS driver`
- `docs(wiki): add architecture overview page`

---

## Coding Standards

### Rust Style
- Run `cargo clippy` before submitting — zero warnings expected
- Run `cargo fmt` — enforce consistent formatting
- All `unsafe` blocks must have a `// SAFETY:` comment explaining the invariants

### Error Handling
- Return `Result<T, E>` — never `panic!` in library code
- Use descriptive error types (enums preferred over string errors)
- No `unwrap()` in non-test code without explicit comment

### Naming Conventions
- Types: `PascalCase`
- Functions/methods: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Modules: `snake_case`

### No Hardcoded Credentials
- **Never** hardcode passwords, keys, salts, or secrets in production code paths
- Test vectors in `#[cfg(test)]` are acceptable but must use clearly labeled test constants
- Use `const KDF_CONTEXT: &[u8] = b"sigmaos-component-v1"` style domain-separation labels

---

## Security Guidelines

- Follow `SECURITY.md` for vulnerability reporting
- All cryptographic code must be reviewed before merge
- Raw pointer dereferences in bootloader/kernel code require bounds-check comments
- New syscalls must go through the capability enforcement layer (`src/security/capability_enforcer.rs`)
- Input from userspace must be validated before use in kernel code

---

## Documentation Requirements

Every new module should include:
1. **File-level doc comment** (`//!`) explaining the module's purpose
2. **Public API doc comments** (`///`) on every `pub` struct, trait, and function
3. An entry in the relevant `docs/` file if it introduces a new subsystem

---

## Testing Requirements

- Minimum: one `#[test]` per public function that has meaningful logic
- Use `#[cfg(test)]` modules at the bottom of each file
- Integration tests go in `tests/integration_test.rs`
- No test should require network access or filesystem writes outside `target/`

---

## Areas Actively Seeking Contributions

| Area | Priority | Skills Needed |
|---|---|---|
| Bootloader UEFI pointer safety | 🔴 Critical | Rust unsafe, UEFI spec |
| JS XSS remediation in web UI | 🔴 Critical | JavaScript, DOM security |
| Native filesystem drivers (ext4, Btrfs) | 🟠 High | Rust, filesystem internals |
| GPU driver (Vulkan/Mesa concepts) | 🟠 High | Rust, GPU architecture |
| Network stack TCP/IP completeness | 🟠 High | Rust, networking protocols |
| Accessibility (screen reader daemon) | 🟡 Medium | Rust, accessibility APIs |
| Package manager SigmaPkg | 🟡 Medium | Rust, package management |
| Documentation & Wiki | 🟢 Low | Markdown, technical writing |
