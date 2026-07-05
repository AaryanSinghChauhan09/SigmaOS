# SigmaOS Roadmap: Formal Verification of Kernel Modules
Use Kani/Creusot to formally verify correctness of critical kernel paths.
## Goals
- Memory safety proofs for IPC ring buffer
- Capability token non-forgeability proof
## Key Milestones
- [ ] Kani harnesses for ipc.rs
- [ ] Creusot contracts for cap.rs
- [ ] CI verification gate (fail on unsound proof)