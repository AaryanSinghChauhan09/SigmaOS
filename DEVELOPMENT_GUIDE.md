# SigmaOS Development Guide

**Last Updated:** September 3, 2026

***

## Table of Contents

1.  [Getting Started](#getting-started)
2.  [Repository Structure](#repository-structure)
3.  [Build System](#build-system)
4.  [Testing](#testing)
5.  [Code Standards](#code-standards)
6.  [Contributing](#contributing)
7.  [Performance & Debugging](#performance--debugging)

***

## Getting Started

### Prerequisites

*   **Rust:** nightly toolchain with targets:
    *   `x86_64-unknown-none` (bare-metal x86\_64)
    *   `aarch64-unknown-none` (bare-metal ARM64)
    *   `riscv64gc-unknown-none-elf` (bare-metal RISC-V)

*   **Build Tools:**
    *   LLVM/Clang
    *   CMake 3.20+
    *   QEMU (for testing)
    *   GCC/G++ (for C/C++ modules)

*   **System:** Linux or macOS; Windows via WSL2

### Installation

```bash
# Install Rust nightly
rustup update nightly
rustup target add x86_64-unknown-none aarch64-unknown-none riscv64gc-unknown-none-elf

# Clone SigmaOS
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Verify build
cargo build --target x86_64-unknown-none --release
```

***

## Repository Structure

    SigmaOS/
    ├── src/                    # Main Rust source
    │   ├── klib/              # Zero-dependency kernel library
    │   │   ├── vec.rs         # Custom Vec<T>
    │   │   ├── hashmap.rs     # Custom HashMap (open-bucket hash table)
    │   │   ├── json.rs        # Zero-copy JSON parser
    │   │   ├── async_runtime.rs
    │   │   └── ...
    │   ├── kernel/            # Microkernel subsystems
    │   │   ├── task_name_cache.rs  # O(1) task-name lookups
    │   │   ├── scheduler/
    │   │   ├── memory/
    │   │   └── syscall/
    │   ├── security/          # Security modules
    │   │   ├── pledge.rs      # OpenBSD pledge/unveil
    │   │   ├── capsicum.rs    # Capability sandboxing
    │   │   └── ...
    │   ├── distro/            # Distribution parity
    │   │   ├── arch_inspirations.rs
    │   │   ├── nixos_inspirations.rs
    │   │   └── ...
    │   ├── graphics/          # Zenith desktop
    │   ├── lib.rs             # Root crate
    │   └── ...
    ├── kernel/                # C/C++ kernel components
    │   ├── syscalls/
    │   ├── core/
    │   └── CMakeLists.txt
    ├── tests/                 # Integration tests
    ├── scripts/               # Build & automation
    ├── Cargo.toml
    ├── CMakeLists.txt
    ├── Makefile
    └── README.md

***

## Build System

### Cargo

**Main binary targets:**

*   `sigma_kernel` — kernel image
*   `sigma_drivers` — driver subsystem
*   `sigma_userspace` — userland
*   `sigma_make` — build tool

**Features:**

```toml
default = []
microkernel = ["core-shards"]
desktop = []
drivers = []
ai = []
```

### Make

```bash
make all               # Build everything
make kernel           # Build kernel only
make test             # Run all tests
make test-qemu        # Run QEMU boot test
make clean            # Remove artifacts
make fmt              # Format code
make lint             # Check with clippy
```

### CMake

```bash
mkdir build
cd build
cmake ..
make
```

***

## Testing

### Unit Tests

```bash
# Run all unit tests
cargo test --lib

# Run specific test
cargo test klib::vec::tests::test_vec_push

# Test with output
cargo test -- --nocapture
```

### Integration Tests

```bash
# Run integration test suite
cargo test --test '*'

# Test hosted simulation
./run_sigma_tests.sh
```

### QEMU Boot Test

```bash
# Build ISO and boot in QEMU
make test-qemu

# Or manually
cargo build --target x86_64-unknown-none --release
./scripts/build-iso.sh
qemu-system-x86_64 -cdrom target/x86_64-unknown-none/release/sigmaos.iso \
  -m 2G -serial stdio -no-reboot -display none
```

***

## Code Standards

### Style Guide

*   **Formatting:** Use `cargo fmt` (enforced via CI)
*   **Linting:** Pass `cargo clippy -- -D warnings`
*   **Comments:** Documenting `pub` items mandatory; use `///` for public API
*   **Unsafe Blocks:** Every `unsafe` block must have a `// SAFETY: ...` comment explaining invariants

### Performance Critical Code

Mark with **⚡ Bolt** annotation:

```rust
/// ⚡ Optimized by Bolt: ...description...
pub fn optimized_function() { ... }
```

Examples: `Vec::extend_from_slice`, `SigmaString::from_str`, JSON zero-copy parsing.

### Security Code

Mark with **🔒 Security** annotation:

```rust
/// 🔒 Security: Validates user-space pointer bounds.
pub fn validate_ptr(ptr: u64, len: usize) -> bool { ... }
```

### Testing Requirements

*   All public APIs must have `#[cfg(test)] mod tests { }`
*   Unit test coverage target: 80%+
*   Security-critical code: 100% test coverage

***

## Contributing

### Workflow

1.  **Fork & Clone**
    ```bash
    git clone https://github.com/YOUR_USERNAME/SigmaOS.git
    cd SigmaOS
    ```

2.  **Create Branch**
    ```bash
    git checkout -b feature/your-feature-name
    ```

3.  **Implement & Test**
    ```bash
    cargo test --all
    cargo fmt
    cargo clippy
    ```

4.  **Commit with Message**
    ```bash
    git commit -m "feat: add your feature

    - First bullet point
    - Second bullet point"
    ```

5.  **Push & Open PR**
    ```bash
    git push origin feature/your-feature-name
    ```

### Commit Message Format

    <type>(<scope>): <subject>

    <body>

    <footer>

**Types:**

*   `feat:` New feature
*   `fix:` Bug fix
*   `perf:` Performance improvement
*   `docs:` Documentation
*   `test:` Test addition
*   `refactor:` Code refactoring
*   `security:` Security hardening

**Example:**

    perf(kernel): implement task-name cache O(1) lookups

    Add kernel/task_name_cache.rs with seqlock-protected static hash table.
    Eliminates O(n) name string scans from scheduler hot-path.

    Closes #123

***

## Performance & Debugging

### Profiling

```bash
# Build with debug symbols
cargo build --target x86_64-unknown-none

# Run with perf (if on Linux)
perf record ./target/x86_64-unknown-none/debug/sigma_kernel
perf report
```

### Debug Output

```rust
// Use console module for logging (zero-alloc)
use sigmaos::kernel::console::print;
print!("Debug: value = {}\n", value);
```

### Benchmarking

```bash
# Create benchmark in tests/benchmarks/
# Run with: cargo bench
```

### Common Issues

**Issue:** Linker error: undefined reference

*   **Solution:** Ensure bare-metal target is selected; verify `Cargo.toml` bin config

**Issue:** Memory leak in tests

*   **Solution:** Check `Vec::drop()` behavior; ensure `free_sized()` is called

**Issue:** Syscall stub returns 0 but test expects real behavior

*   **Solution:** Many syscalls are stubs (TODO markers); implement or mock for testing

***

## Resources

*   **ARCHITECTURE.md** — System design overview
*   **SECURITY.md** — Security model & vulnerabilities
*   **ROADMAP.md** — Feature roadmap
*   **GitHub Issues** — Bug reports and feature requests
*   **GitHub Discussions** — Community Q\&A

***

## License

All code contributions are licensed under [MIT License](../LICENSE).
