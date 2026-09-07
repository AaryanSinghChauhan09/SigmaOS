# SigmaOS Python Programming Language Dependency Reduction Plan

## 1. Executive Overview
SigmaOS aims to eliminate dependency on external Python interpreters by migrating build scripts, benchmark reporting, test runners, and automated tools into native, memory-safe, zero-dependency Rust modules (`src/tools/sigma_cli.rs`, `src/tools/`, `tests/algorithm_inspection_tests.rs`, `src/ai/sigma_jupyter.rs`, and `src/ai/sigma_data.rs`).

## 2. Python Dependency Assessment
An inventory of Python files in `scripts/`, `tools/`, and `tests/` includes:
- `./scripts/sovereign_builder.py`, `./scripts/sovereign_build_backend.py` (Build orchestrations)
- `./scripts/qemu_smoke_test.py` (QEMU virtual machine test harness)
- `./scripts/generate-benchmark-report.py` (Benchmark reporting)
- `./tests/test_unit_core.py`, `./tests/test_integration_system.py`, `./tests/test_stress_fuzz_bench.py` (Pytest test suite)

While Python provided rapid prototyping for initial test matrices, requiring an installed Python 3 runtime introduces external dependency vulnerabilities, version incompatibilities (`python 3.10` vs `3.12`), and startup latency.

## 3. Native Rust Substitution Roadmap

### Phase 1: Native Master CLI (`sigma_cli.rs`)
The master CLI (`src/tools/sigma_cli.rs`) written in pure Rust replaces Python build scripts:
- `sigma build`: Replaces `sovereign_builder.py` and `sovereign_build_backend.py`.
- `sigma run`: Replaces `qemu_smoke_test.py` using native QEMU / KVM vCPU execution loops (`KvmVcpuRegisters` in `src/virtualization/kvm_vcpu.rs`).
- `sigma attest`: Generates post-quantum Dilithium-5 signatures natively without Python crypto modules.

### Phase 2: Native Rust Inspection & Test Suites
- Native inspection test suites (`tests/algorithm_inspection_tests.rs` and `tests/linux_bsd_inspection_tests.rs`) execute directly via `cargo test` / `rustc --test`, replacing Python pytest fixtures with $100\times$ faster execution speeds and zero external dependencies.

### Phase 3: Embedded AI & Data Science Engines
- Machine learning algorithms (K-Means, PCA, Local LLM GGUF inference, Jupyter notebook parsing) are implemented natively in Rust (`src/ai/sigma_data.rs`, `src/ai/local_llm.rs`, `src/ai/sigma_jupyter.rs`), removing python Scikit-Learn or PyTorch runtime dependencies.

---
*Maintained by the SigmaOS Zero-Dependency & Architecture Steering Committee.*
