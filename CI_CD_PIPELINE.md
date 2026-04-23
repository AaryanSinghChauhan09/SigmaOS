# SigmaOS CI/CD Pipeline

The Continuous Integration pipeline is implemented using GitHub Actions. It is located in `.github/workflows/sigmaos-ci.yml`.

## Architecture
Every push to the `main` branch triggers the following workflow:
1. **Toolchain Setup**: Installs `gcc-aarch64-linux-gnu`, `gcc-riscv64-linux-gnu`, and QEMU emulators.
2. **Build Orchestration**: Invokes the custom `sovereign_builder.py` script across all three target architectures (`x86_64`, `aarch64`, `riscv64`).
3. **Smoke Testing**: Simulates booting the generated `sigmaos_x86_64.bin` inside a headless QEMU instance to verify that capability bindings and the memory contracts initialize without crashing.

This ensures that the Sovereign Lattice remains mathematically stable. No broken capsule or driver can ever break the master build.
