# Contributing to Σ SigmaOS

Welcome to the Sovereign Lattice. SigmaOS is a modular, 600-shard operating system designed for absolute digital sovereignty. We welcome contributions from developers, security researchers, and UI/UX designers.

## 🏛️ Contribution Principles
1.  **Zero-STL**: All kernel shards must be zero-dependency (no standard library).
2.  **Shard Isolation**: Subsystems must be isolated with strict, versioned APIs.
3.  **Post-Quantum Ready**: Security modules must adhere to NIST-standard PQC algorithms.
4.  **Silicon-Native**: Optimize for direct hardware orchestration where possible.

## 🚀 Getting Started
1.  **Fork the Lattice**: Fork the repository and create your shard branch.
2.  **Build Locally**: Use `make all` to validate the lattice.
3.  **Test Shards**: Ensure your changes pass the QEMU boot tests.

## 🧪 Testing Guidelines
- All new shards must include a `run_stress_test()` hook.
- Regression tests are mandatory for core system services (IPC, Sched, VMM).

## 🛡️ Code of Conduct
We maintain a professional, inclusive, and mission-oriented environment. Please refer to `CODE_OF_CONDUCT.md` for details.

---
*Join us in achieving Digital Sovereignty.*
