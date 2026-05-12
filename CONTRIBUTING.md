# Σ SigmaOS Contributing Guidelines

Welcome to the **Sovereign Lattice™** development ecosystem. We are building the world's most stable, profession-aware operating system, and we value industrial-grade contributions.

## 🛠 Development Philosophy
- **Zero-Dependency**: Every shard must be self-contained or use strictly audited Lattice bridges.
- **PQC-First**: All cryptographic routines must use Post-Quantum algorithms (Dilithium-5, Kyber).
- **Industrial Stability**: Code must be deterministic, deadlock-free, and pass S-REGRESS testing.

## 📦 Shard Development
1. **Define the Shard**: Create a new `.cpp` file in the appropriate layer.
2. **Implement SigmaSingleton**: Follow the `SigmaSingleton<T>` pattern for kernel shards.
3. **Attestation**: Every shard must be signed with a PQC-attested GPG key before merging.
4. **Registration**: Add the shard to `SHARDS.manifest`.

## 🧪 Testing Requirements
- Every pull request must pass the **QEMU Regression Suite**.
- Fuzz testing is required for new memory allocators or security shards.
- Benchmark your code against industrial baselines (BusyBox, Linux-Minimal).

## 🏛 Governance
SigmaOS follows the **Sovereign Industrial Governance** model. Maintainers act as "Shard Wardens" to ensure the integrity of the Lattice.

---
*Stay Sovereign.*
