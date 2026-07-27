# 🤝 SigmaOS Contributor Guidelines & Coding Standards

Thank you for contributing to SigmaOS! As a zero-dependency, capability-based microkernel written in Rust under strict `#![no_std]` constraints, we adhere to rigorous guidelines to ensure system reliability, safety, and performance.

---

## 📜 Coding Standards & Principles

To maintain absolute system safety, every component in SigmaOS must obey the following coding rules:

### 1. Pure Rust and Zero-Dependency Constraints
- Avoid external crates from `crates.io` unless they are strictly `#![no_std]` compatible and have been pre-approved by the core security team.
- Do not introduce dependencies that require `std` under any circumstances for core drivers.

### 2. Strict Memory Safety & Drop Semantics
- Memory allocated inside raw pointers (`*mut T`) must be wrapped within clear RAII structures.
- Custom Collections and Vector implementations must strictly implement the `Drop` trait to recursively drop internal items and deallocate their backing raw pointers to prevent severe memory leaks.

### 3. Capability-Based API Design
- Every API exposing hardware, network, or file access must accept a valid `CapabilityToken` as its first parameter:
```rust
pub fn read_secure_file(token: &CapabilityToken, path: &str) -> Result<Vec<u8>, SecurityError> {
    if !token.has_permission(Permission::FileRead) {
        return Err(SecurityError::PermissionDenied);
    }
    // ... logic ...
}
```

---

## 🧪 Unit Test Templates

All new contributions must include a testing module. Follow this template to structure your unit tests correctly.

### Rust Unit Test Template
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_functionality() {
        // Initialize state
        let mut resource = CoreResource::new();

        // Assert correct default states
        assert_eq!(resource.get_status(), Status::Idle);

        // Execute operation
        let result = resource.process_request(0x100);

        // Verify output and state transitions
        assert!(result.is_ok());
        assert_eq!(resource.get_status(), Status::Active);
    }

    #[test]
    fn test_permission_failure() {
        let resource = CoreResource::new();
        let invalid_token = CapabilityToken::new_restricted();

        let result = resource.secure_access(&invalid_token);
        assert!(result.is_err());
    }
}
```

---

## 🚀 Proposing a New Built-in App

We welcome new built-in apps (e.g., system utilities, terminal programs, or desktop widgets). To propose and add a new built-in app, follow this workflow:

1. **Design Proposal (.md):**
   - Create a Markdown design proposal inside the `WIKI/` directory outlining the app's purpose, storage layout, capability token requirements, and integration hooks.
2. **Implement FFI/Capabilities:**
   - Define custom system calls inside `src/syscall/` and map permissions clearly inside `src/security/capability_enforcer.rs`.
3. **Register CLI Multi-Call Parser:**
   - Map your app's CLI name in `src/shell/multicall.rs` to allow space-efficient execution via BusyBox-style symbolic links.
4. **Desktop Integration:**
   - Register any GUI components or graphical launches under the Zenith Desktop configuration files.
<<<<<<< HEAD
=======


---
## Merged from Contributor-Guidelines.md
# SigmaOS Contributor Guidelines

Welcome to SigmaOS! Every contributor must follow these rules. They act like a constitution for the repo, ensuring disciplined development, clear documentation, and smooth collaboration.

## 🛠 General Task Rules

### Consistency First

- Every task must follow the same naming conventions, formatting, and documentation style.

- **No shortcuts**: even small fixes require tests and documentation updates.

### Traceability & Commit Messages

- Each task must be linked to an issue or roadmap item.

*Commit messages must use the**imperative style** (e.g., "Add memory allocator," "Fix IRQ handler").

- Reference issue IDs for traceability to ensure every change maps back to the roadmap.

### Atomic Changes

- One task = one logical change.

- Avoid mixing bug fixes, new features, and documentation in a single PR.

---

## 📂 Repo Rules

### .MD Files

- Every `.md` file must be fully implemented (no placeholders).

- Once complete, its content should be migrated into the Wiki.

*After migration,**delete the `.md` file** from the main repository to avoid duplication (except for core files like `README.md` and `CONTRIBUTING.md`).

### Code Contributions

- Must include unit tests and CI validation.

- Kernel shards must be modular and documented in the Wiki.

- Security primitives must pass regression tests before merging.

### Branching & PRs

- Use `main` for stable releases only.

- Feature branches must follow the `feature/<name>` format.

- PRs require at least one reviewer approval.

---

## 📚 Wiki Rules

### Structure

- Each subsystem (kernel, memory, scheduler, drivers, security, CI/CD) gets its own page.

- Add diagrams, flowcharts, and examples where possible.

- Maintain a glossary for technical terms.

### Updates

*Any new feature or module must be documented in the Wiki**before** merging.

- Wiki pages must be kept in sync with repo changes.

- Changelogs and release notes must be mirrored in the Wiki.

### Contributor Guidelines

- Clearly state coding standards, testing requirements, and review process (this file).

- Provide templates for bug reports, feature requests, and PRs.

---

## ⚙️ Automation Rules

### CI/CD

- Every commit triggers automated builds, QEMU boot tests, and security scans.

- Cross-architecture builds (x86, ARM, RISC-V) must be validated.

### Docs Automation

- Auto-generate API docs from code comments using Doxygen or Sphinx.

- Push generated docs into the Wiki automatically.

### Release Automation

- Tagged releases must package ISO images and binaries.

- Publish changelogs both in the repo and Wiki.

---

## ✅ Enforcement

- These rules are enforced via GitHub Actions checks (`lint`, `test`, `docs`).

- **No merge is allowed if rules are violated.**

- Every contributor or AI tool knows exactly what to do: `implement` → `document` → `automate` → `enforce`.
>>>>>>> wiki/master
