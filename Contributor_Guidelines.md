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
