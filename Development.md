# SigmaOS Development Guide

## Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add rustfmt clippy

# Clone and build
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
cargo build && cargo test
```

## Repository Structure

    src/
    ├── lib.rs              # Crate root
    ├── kernel/             # Scheduler, memory, IPC
    │   ├── scheduler.rs    # EEVDF+BORE
    │   └── sched/          # MLFQ, thermal, task types
    ├── ai/                 # S-AI orchestrator
    ├── security/           # SELinux, pledge, CVE scanner
    ├── network/            # TCP/UDP stack
    ├── container/          # OCI runtime
    ├── boot/               # UEFI, TPM, secure boot
    ├── klib/               # No-std collections
    │   ├── vec.rs          # Custom Vec
    │   ├── hashmap.rs      # Custom HashMap
    │   └── buddy_allocator.rs
    └── distro/             # Linux/BSD parity

## Module Template

```rust
//! SigmaOS My Module — brief description

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

/// Main component struct.
pub struct MyComponent { state: u64 }

impl MyComponent {
    /// Creates a new instance.
    pub fn new() -> Self { Self { state: 0 } }

    /// Processes input.
    pub fn process(&mut self, input: &str) -> Result<String, MyError> {
        Ok(String::from("result"))
    }
}

/// Error type.
#[derive(Debug)]
pub enum MyError { InvalidInput, Overflow }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        assert!(MyComponent::new().process("hello").is_ok());
    }
}
```

## Safety Rules

```rust
// All unsafe MUST have safety comment:
// SAFETY: ptr non-null by invariant X
unsafe { *ptr = 42; }

// All #[test] in #[cfg(test)] mod tests {}
// NEVER inside impl blocks
```

## Testing

```bash
cargo test                            # All tests
cargo test test_eevdf_scheduler       # Specific test
cargo test kernel::scheduler::tests   # Module tests
cargo test -- --nocapture             # With output
cargo test --test os_components_tests # Integration
```

## CI Checks

| Check | Command |
|-------|---------|
| Compile | `cargo check` |
| Tests | `cargo test` |
| Lint | `cargo clippy` |
| Format | `cargo fmt --check` |
| Security | `cargo audit` |
| ARM64 | `cargo check --target aarch64-unknown-none` |

## Contribution Workflow

1.  Fork → `git checkout -b feat/my-feature`
2.  Implement + write tests
3.  `cargo test && cargo clippy && cargo fmt`
4.  `git push origin feat/my-feature`
5.  Open PR → `main`
6.  Address reviews → merge → branch auto-deleted
