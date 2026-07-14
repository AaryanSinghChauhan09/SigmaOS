# Σ tools/verification — Formal Verification Suite

The `tools/verification` module contains formal methods, property-based tests, and static analysis tooling used to mathematically verify critical invariants in the SigmaOS kernel.

---

## Overview

Unlike conventional testing that samples inputs, **formal verification** exhaustively proves properties hold for all possible program states. SigmaOS applies formal methods to:

- Memory safety invariants in the kernel allocator
- Capability integrity (no capability forging or escalation)
- Scheduling correctness (no priority inversion, no starvation)
- Cryptographic protocol correctness (Kyber-1024, Dilithium-5)
- IPC message safety (no buffer overflows across shard boundaries)


---

## Verification Approaches Used

### 1. Bounded Model Checking (BMC)

Used for: Kernel scheduler, IPC bus, memory allocator.

Tool: [CBMC](https://www.cprover.org/cbmc/) for C components, `kani` for Rust.

```bash

# Run Kani verification on the buddy allocator

cargo kani --package sigma-mm --harness verify_buddy_alloc
```

### 2. Property-Based Testing (PBT)

Used for: Package resolver, configuration parser, network stack.

Tool: `proptest` (Rust), `hypothesis` (Python).

```rust
proptest! {
    #[test]
    fn pkg_resolver_no_cycle(deps in arb_dep_graph()) {
        let result = resolve(deps);
        assert!(result.is_ok() || matches!(result, Err(ResolveError::CyclicDependency)));
    }
}
```

### 3. Type-Level Verification

Used for: Capability tokens, system call interface, IPC protocol.

SigmaOS uses Rust's type system as a lightweight verification layer — impossible states are unrepresentable.

```rust
// A CapabilityToken<Read> can never be used where CapabilityToken<Write> is expected
fn read_file<P: ReadPerm>(cap: CapabilityToken<P>, path: &Path) -> Result<Vec<u8>, SigmaError>
```

### 4. Static Analysis

Used for: Entire codebase.

| Tool | Language | Checks |
| ---- | -------- | ------ |
| `clippy` | Rust | Correctness lints, unsoundness patterns |
| `codeql` | All | Security vulnerabilities, taint analysis |
| `cppcheck` | C | Undefined behavior, memory safety |
| `nimlint` | Nim | Unsafe patterns, unreachable code |

---

## Running the Verification Suite

```bash

# Full verification suite (slow, thorough)

make verify-all

# Quick property tests only

cargo test --features proptest

# Kani formal proofs (requires kani installed)

cargo kani --package sigma-mm
cargo kani --package sigma-crypto

# CodeQL (requires GitHub Advanced Security or local runner)

make codeql-analyze
```

---

## Key Properties Verified

| Property | Component | Method | Status |
| -------- | --------- | ------ | ------ |
| No double-free | Buddy allocator | Kani BMC | ✅ Verified |
| No capability forging | S-SEC shard | Type-level | ✅ Verified |
| No priority inversion | MLFQ scheduler | CBMC | 🔄 In progress |
| Dilithium-5 constant-time | PQC module | Manual + timing tests | ✅ Verified |
| TCP no-deadlock | Network stack | TLA+ spec | 📋 Planned |
| IPC no-overflow | IPC bus | Kani BMC | 🔄 In progress |

---

## Adding a New Verification Target

1. Identify the critical invariant to verify
2. Choose the appropriate tool (Kani, proptest, CBMC, TLA+)
3. Write the harness/property in `modules/tools/verification/`
4. Add it to the `verify-all` Makefile target
5. Document it in the table above


---

## Roadmap

- [x] Kani harness for buddy allocator (no double-free, no OOB)
- [x] proptest suite for dependency resolver
- [ ] TLA+ specification for IPC bus protocol
- [ ] Kani harness for capability token lifecycle
- [ ] Automated verification CI gate on every PR
- [ ] seL4-inspired formal proof of scheduler isolation
- [ ] Tamarin prover for PQC protocol analysis


---

## References

- [Kani Rust Verifier](https://model-checking.github.io/kani/)
- [CBMC for C](https://www.cprover.org/cbmc/)
- [proptest crate](https://docs.rs/proptest/)
- [TLA+ Tools](https://lamport.azurewebsites.net/tla/tools.html)
- [seL4 Verification Story](https://sel4.systems/Info/Docs/seL4-whitepaper.pdf)
