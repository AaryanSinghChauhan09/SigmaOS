# AGENTS_ALGORITHM_MANAGEMENT.md — AI Agent Algorithm Management Guidelines for SigmaOS

Welcome, AI Agent! This document defines the standards, mathematical models, performance constraints, SIMD routing rules, and verification protocols for managing, developing, and extending **Core Algorithms** in **SigmaOS**.

---

## 1. SigmaOS Algorithm Subsystem Overview

SigmaOS incorporates specialized algorithms spanning Machine Learning, CPU Process Scheduling, Cryptography & Hashing, and File System Data Structures, backed by a dedicated inspection test binary (`algorithm_and_components_inspection_tests`).

### Core Algorithm Domains
* **Machine Learning & AI Algorithms (`src/ai/`)**:
  - **K-Means Clustering (`src/ai/sigma_data.rs`)**: Zero-alloc centroid initialization and Euclidean distance iteration.
  - **Principal Component Analysis (`src/ai/sigma_data.rs`)**: Covariance matrix computation, power iteration eigenvalue decomposition, and dimensionality reduction.
  - **Local LLM & GGUF Inference (`src/ai/local_llm.rs`)**: llama.cpp GGUF execution, token probability sampling, and zero-copy tensor memory mapping (`AiTensorMemoryManager`).
  - **Quantization Pipeline (`src/ai/quantization.rs`)**: FP32 to FP16, INT8, and INT4 matrix weight quantization.
* **CPU Scheduling Algorithms (`src/scheduler/`)**:
  - **EEVDF (Earliest Eligible Virtual Deadline First)**: Deadline and weight-based virtual time calculations for fair thread scheduling.
  - **BORE (Burst-Oriented Response Enhancer)**: Interactive burst score tracking and dynamic priority boosting (`test_cachyos_bore_burst_algorithm_inspection`).
* **Cryptographic, Hashing & Data Structure Algorithms (`src/klib/`)**:
  - **Merkle Accumulator Trees (`src/klib/merkle.rs`)**: Cryptographic root hash calculation and leaf inclusion proof verification.
  - **Fast Zero-Alloc Hashers (`src/klib/hash.rs`)**: FNV1a, DJB2, XOR, and DJB2-based composite hashing.
  - **Buddy Allocator (`src/memory/sigma_buddy.rs`)**: Binary buddy tree page frame allocation and Order-N split/merge mechanics.

---

## 2. Algorithm Development Guidelines for AI Agents

When modifying or implementing core algorithms in SigmaOS:

### 1. Allocation & Memory Constraints
* **Zero-Allocation Hot-Paths**: Core kernel algorithms (scheduler deadlines, Merkle tree hashing, page allocation, tensor buffers) must avoid dynamic heap allocations during evaluation loops.
* **SIMD ISA Level Routing**: Use `klib::isa` to auto-detect hardware vector capabilities (AVX2, AVX-512, NEON) and route vectorized memcpy / dot-product operations to SIMD fast-paths.

### 2. Numerical Stability & Precision
* **Floating-Point Determinism**: In machine learning algorithms (K-Means, PCA), handle division-by-zero, `NaN`, and `Infinity` explicitly.
* **Quantization Saturation**: When quantizing FP32 weights to INT8/INT4, clamp values to `[-128, 127]` or `[-8, 7]` to prevent integer overflow wrapping.

### 3. Dedicated Inspection Test Requirements
* Every core algorithm must be verified by both standalone unit tests (`#[test]`) and the dedicated algorithm inspection test suite (`tests/algorithm_inspection_tests.rs`).

---

## 3. Verification & Testing Protocols

1. **Algorithm Inspection Test Binary Execution**:
   ```bash
   ./algorithm_and_components_inspection_tests
   ```
2. **Core Test Runner Execution**:
   ```bash
   ./run_sigma_tests.sh
   ```

---

## 4. Pre-Commit Checklist for Algorithm Changes

Before submitting algorithm modifications:
- [ ] Confirmed zero-allocation behavior in algorithm hot-paths.
- [ ] Handled `NaN`, `Infinity`, and zero-division edge cases in floating-point logic.
- [ ] Verified SIMD dispatch routes using `klib::isa`.
- [ ] Executed `./algorithm_and_components_inspection_tests` successfully.
- [ ] Executed `./run_sigma_tests.sh` with 100% test pass rate.
- [ ] Requested automated code review using `request_code_review`.
- [ ] Recorded algorithm learnings using `initiate_memory_recording`.
