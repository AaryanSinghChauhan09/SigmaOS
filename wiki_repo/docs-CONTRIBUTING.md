# ðŸ¤ Contributing to SigmaOS

Thank you for your interest in contributing to **SigmaOS Sovereign Lattice**. We are building a high-performance, AI-native ecosystem and we welcome contributions that align with our vision of absolute digital sovereignty.

---

## ðŸ›ï¸ Contribution Philosophy

SigmaOS is built on a **600-Shard Modular Architecture**. Every contribution should be:

1. **Atomic**: Focus on a single shard or functional cluster.

2. **Zero-Dependency**: Do not introduce external libraries unless absolutely necessary.

3. **PQC-Ready**: Consider security and attestation in every line of code.

---

## ðŸ› ï¸ How to Contribute

1. **Fork the Repository**: Create your own fork and clone it locally.

2. **Create a Shard Branch**:
   ```bash
   git checkout -b shard/your-feature-name
   ```

1. **Implement & Document**:
   - Add your logic to the appropriate directory (`/kernel`, `/drivers`, `/ui`).
   - Update the corresponding `.md` file in the wiki if the architecture changes.

2. **Validation**:
   - Run the build system: `make all`.
   - Test in QEMU: `./qemu-boot.sh`.
   - Ensure `sigma-heal` reports no technical debt.

3. **Submit a PR**: Provide a clear description of the shard's purpose and any capability requirements.

---

## ðŸŽ¨ Code Style

- Use **OOP-Isolated Singletons** for core engines.

- Follow the `sigma_` naming convention for kernel-level primitives.

- Maintain strict **C++11/14** standards for hardware compatibility.

---

## ðŸ›¡ï¸ Security First

If you find a security vulnerability, please follow our **[Security Policy](SECURITY.md)**. Do NOT open a public issue for security bugs.

---

### Build the lattice. Command the future.
