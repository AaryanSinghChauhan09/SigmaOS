# Getting Started with SigmaOS

## Prerequisites

- Linux, macOS, or WSL2 on Windows
- Rust nightly toolchain
- QEMU (for running SigmaOS)
- Git

## Quick Start

```bash
# 1. Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# 2. Install Rust nightly
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly
rustup target add x86_64-unknown-none
rustup component add rust-src llvm-tools-preview

# 3. Check compilation
cargo +nightly check

# 4. Build
cargo +nightly build

# 5. Run tests
cargo +nightly test --all
```

## Running in QEMU

```bash
# Build and run
make iso
make run

# Run with more RAM
make run RAM=4G
```

## Project Structure

```
SigmaOS/
├── src/          ← Rust source code (1,600+ files)
├── kernel/       ← C/C++ kernel layer
├── drivers/      ← C++ hardware drivers
├── tests/        ← Test suite
├── docs/         ← Documentation
└── .github/      ← CI/CD workflows
```

## Next Steps

- Read the [[Architecture-Overview]]
- Learn how to [[Building-from-Source|build from source]]
- Check the [[Kernel-Development|kernel dev guide]]
- See [[Contributing]] to contribute
