# Architecture Expansion

SigmaOS multi-architecture support overview.

## Supported Architectures

| Architecture | Target | Purpose |
|-------------|--------|---------|
| x86_64 | `x86_64-unknown-linux-gnu` | Native development |
| AArch64 | `aarch64-unknown-linux-gnu` | ARM 64-bit |
| RISC-V | `riscv64gc-unknown-linux-gnu` | RISC-V embedded |

## Building

```bash
# Native build
cargo check

# Cross-compile
cargo check --target aarch64-unknown-linux-gnu
```

## CI Pipeline

GitHub Actions builds all targets on every push.

## Roadmap

Future: Raspberry Pi support, embedded targets.
