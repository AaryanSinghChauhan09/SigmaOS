# 🛡️ Post-Quantum Security

> "Securing the Sovereign OS against future quantum threats."

SigmaOS integrates NIST-standardized Post-Quantum Cryptography directly into the kernel to verify executable shards before they run.

## 1. Dilithium-5
We utilize Dilithium-5, a lattice-based digital signature scheme. 

## 2. Shard Attestation
When a new shard (e.g., a driver or application) requests execution, `sigma_pqc_verify_shard()` validates its digital signature against the known public keys in the bootloader chain.

If a shard's signature is invalid or tampered with, the system refuses to execute it, guaranteeing uncompromised execution environments for high-security applications (defense, aerospace).
