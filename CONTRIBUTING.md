# 🛠️ Contributing to the SigmaOS Sovereign Lattice

Thank you for your interest in expanding the SigmaOS ecosystem! To maintain our status as a **Silicon Sovereign** entity, all contributions must adhere to the following standards.

---

## 💎 Sovereign Coding Standards

### 1. Zero-Dependency Mandate
- **Strictly `no_std`**: Shards must not link against standard libraries (`libc`, `libstdc++`).
- **Sovereign Primitives**: Use only the primitives defined in `libsigma.h` or `sigma_libc.h`.
- **Static Memory**: Prefer static or pool-based allocation over dynamic heap usage in kernel shards.

### 2. Shard Atomic Modularity
- Every new feature MUST be its own shard in the `suites/` directory.
- Each shard must have a `module.json` manifest defining its dependencies.
- Shards must communicate exclusively via **Capability-Based IPC**.

### 3. Portability via UAL
- Never include environment-specific assembly (e.g., `asm volatile`) directly in a generic shard.
- Use the **Universal Abstraction Layer (UAL)** to switch between hardware implementations.

---

## 🚀 Shard Development Workflow

1.  **Scaffold**: Use `./s-cli scaffold <shard_name>` to generate the shard template.
2.  **Implement**: Write your logic in `shard_init.c`.
3.  **Verify**:
    - Run `./s-cli test --shard <name>` for atomic verification.
    - Run `./s-cli build` to ensure the lattice synchronizes correctly.
4.  **Audit**: Ensure no external headers are leaked.

---

## 📬 Pull Request Process

1.  **Atomicity**: One feature per PR.
2.  **Documentation**: Update the Wiki if your shard introduces new system capabilities.
3.  **Verification**: Ensure all CI/CD jobs (Matrix Build, Kani Proofs, Fuzzing) pass.

*By contributing to SigmaOS, you help build the foundation of a sovereign digital future.*
