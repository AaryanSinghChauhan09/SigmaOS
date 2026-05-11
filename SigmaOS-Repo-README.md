# Σ SigmaOS — Formal Verification

This directory contains mathematically rigorous proofs verifying the sovereignty and
correctness claims of the SigmaOS Sovereign Lattice.

## Tools

| Tool | Language | Purpose |
|------|----------|---------|
| [Kani](https://model-checking.github.io/kani/) | Rust | Bounded model checking (memory safety, reachability) |
| [Coq](https://coq.inria.fr/) | Gallina | Dependently-typed proofs of IPC isolation |
| [Isabelle/HOL](https://isabelle.in.tum.de/) | ML/HOL | Higher-order logic proofs of CRDT merge correctness |

## Proof Inventory

| File | Tool | Property Verified |
|------|------|------------------|
| `../suites/S08_Security/formal_proofs/ipc_dma_kani.rs` | Kani | DMA ∩ IPC = ∅ (non-interference) |
| `../suites/S08_Security/formal_proofs/ipc_dma_kani.rs` | Kani | Dispatch requires ownership capability |
| `../suites/S08_Security/formal_proofs/ipc_dma_kani.rs` | Kani | Sequence numbers are strictly monotonic |
| `../suites/S08_Security/formal_proofs/ipc_dma_kani.rs` | Kani | Rollback atomically purges exactly one entry |
| `../suites/S08_Security/formal_proofs/tensor_kani.rs` | Kani | NPU matmul never panics or overflows |
| `coq/ipc_isolation.v` | Coq | IPC channel isolation — Coq proof sketch |
| `isabelle/crdt_merge.thy` | Isabelle | CRDT LWW merge is idempotent and associative |

## Running Kani Proofs

```bash

# Install Kani

cargo install --locked kani-verifier
cargo kani setup

# Run all IPC/DMA proofs

cargo kani \
  --harness ipc_dma_proofs::verify_dma_ipc_non_interference \
  --harness ipc_dma_proofs::verify_dispatch_capability_ownership \
  --harness ipc_dma_proofs::verify_sequence_number_monotonicity \
  --harness ipc_dma_proofs::verify_rollback_atomic_removal \
  suites/S08_Security/formal_proofs/ipc_dma_kani.rs

```

## CI Integration

All proofs run automatically on every push via `.github/workflows/ci.yml` (`verify` job).
Results are uploaded as artifacts under `kani-verification-report`.
