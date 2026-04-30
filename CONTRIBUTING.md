# 🤝 Contributing to SigmaOS (Sovereign Standards)

Welcome to the SigmaOS development collective. To maintain the **Singularity 
integrity** of the 600-shard lattice, all contributors must adhere to the 
following industrial standards.

## 💎 The Sovereign Coding Paradigm

1. **Zero-Dependency**: No external libraries (Posix, Glibc, STL). Use only 
   `sigma_types.h` and silicon-native primitives.
2. **OOP-Isolated Singletons**: Every shard must be encapsulated as a 
   `Sovereign*Engine` struct with `extern "C"` linkage for the ignition API.
3. **Wait-Free Algorithms**: Avoid mutexes/locks. Use atomic exchange (WFAE) 
   and circular shard logging (WFCSL).
4. **Silicon-Native Telemetry**: Every core engine must expose 64-bit 
   atomic telemetry for the `SovereignAudit` CLA.

## 🛠️ Branching Strategy

- **`main`**: The stable v28.0 Singularity state.
- **`shard/[shard-id]`**: Individual shard development (e.g., `shard/S601_XYZ`).
- **`zenith/features`**: UI/UX and Cognitive layer expansions.

## 🚀 Submission Process

1. **Lint Verification**: Ensure 100% compliance with `clang-format` (Sigma 
   Standard) and Markdown lint rules (MD030/MD032).
2. **Build Test**: Run `make singularity` to verify lattice stability.
3. **PR Template**: Use the provided GitHub template to document the 
   algorithm parity (Competitor vs. Sovereign).

---

*Σ SIGMAOS: Beyond Linux. Absolute Sovereignty.*
