# SIGMAOS ARCHITECTURE DECISION RECORDS (ADR)

## ADR-001: Zero External Binary Dependencies Strategy
- **Status:** Accepted
- **Context:** Third-party binary dependencies introduce security risks, supply chain vulnerabilities, and platform fragmentation.
- **Decision:** All system tools, userland utilities, media codecs, ML frameworks, LLM inference engines, and scientific simulators will be natively implemented in Safe Rust (`klib` / core system shards).

## ADR-002: 12 Core System Shards Architecture
- **Status:** Accepted
- **Context:** Modular organization of operating system capabilities is required for security isolation and scalable development.
- **Decision:** Partition the system into 12 core system shards covering kernel, storage, networking, virtualization, POSIX userland, desktop compositor, media, document processing, ML/agent orchestration, LLMs/vision, scientific computing, and cyber security.

## ADR-003: Post-Quantum Cryptographic Security Standard
- **Status:** Accepted
- **Context:** Classical asymmetric cryptography faces quantum decryption risks.
- **Decision:** Implement lattice-based PQC standards (Dilithium5, Falcon-1024, Kyber) for code signing, kernel attestation, and secure IPC.
