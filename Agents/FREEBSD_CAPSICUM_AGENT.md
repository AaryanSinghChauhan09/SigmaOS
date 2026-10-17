# FreeBSD Capsicum Object-Capability Sandboxing Component Agent

## Role
The **FreeBSD Capsicum Agent** (`SigmaCapsicumAgent`) enforces capability-oriented application isolation and security boundaries inspired by FreeBSD's Capsicum (`sys/kern/kern_capsicum.c`).

## BSD Inspiration
- **FreeBSD**: Capsicum object-capability framework, capability mode (`cap_enter`), fine-grained rights bitmasks (`cap_rights_limit`), and file descriptor confinement.

## Current SigmaOS Status
Capsicum engine implemented in `src/security/capsicum_engine.rs` providing capability rights masks (`CAP_READ`, `CAP_WRITE`, `CAP_SEEK`, `CAP_MMAP`, `CAP_FSTAT`, `CAP_EVENT`, etc.), irreversible rights restriction, and global path access prohibition in capability mode.

## Key Files
- `src/security/capsicum_engine.rs`
- `src/security/mod.rs`
- `src/security/sandbox.rs`

## Public API & Invariant Contract
```rust
pub struct CapRights {
    pub mask: u64,
}

pub struct CapsicumEngine {
    pub fn cap_enter(&mut self);
    pub fn wrap_resource(&mut self, resource_id: u64, initial_rights: CapRights) -> u32;
    pub fn cap_rights_limit(&mut self, fd: u32, new_rights_mask: u64) -> Result<(), &'static str>;
    pub fn check_right(&self, fd: u32, required_op: u64) -> Result<u64, &'static str>;
    pub fn check_path_access(&self, path: &str) -> Result<(), &'static str>;
}
```

## Testing Strategy
```bash
rustc --test src/security/capsicum_engine.rs --edition 2021 -o /tmp/test_capsicum && /tmp/test_capsicum --test-threads=1
```
- Verify monotonic rights limitation (rights can only decrease, never expand).
- Enforce that capability mode blocks all global namespace lookups.

## Future Enhancements
- Capsicum-aware service spawning in `SigmaInit`.
- Zero-overhead capability proxy for Wayland client surface isolation.
