# Contributing to SigmaOS

Welcome, Citizen-Developer. By contributing to SigmaOS, you are helping build a sovereign, post-quantum future for humanity.

## ⚖️ The Code of Sovereignty
1. **User First**: Always prioritize user control over system telemetry and package sources.
2. **Silicon Direct**: Avoid high-level abstractions where silicon-direct primitives are possible.
3. **PQC-Hardened**: All new shards must implement PQC-GPG attestation for inter-shard comms.
4. **Zero-Trust**: Every shard is isolated. Minimal permissions by default.

## 🛠 Development Workflow
1. **Fork the Lattice**: Fork the `main` branch.
2. **Shard Isolation**: Develop your feature in a dedicated shard directory.
3. **Build & Fuzz**: Run `sigma-build.py` and ensure the PQC-fuzzer passes.
4. **Sync**: Ensure your `.md` documentation is updated and synced via `wiki_sync.py`.

## 📝 Issue Templates
When reporting a problem, use the `@current_problems` tag so our automated tracker can prioritize it.

**Bug Report Template:**
- **Problem**: Description of the lattice anomaly.
- **Shard**: Which shard is affected (e.g., S-NET, S-VFS).
- **Environment**: Architecture (x86_64, ARM64, RISC-V).
- **Entropy Hash**: Provide your local lattice entropy hash for reproduction.

**Feature Request Template:**
- **Vision**: Description of the new sovereignty-enhancing feature.
- **Impact**: How it improves user autonomy or system resilience.

## 🤝 Community
Join the Sovereign Developer Forum or sync with the Global Lattice Mesh for real-time peer review.

**Your Silicon. Your Rules. Your Contribution.**
