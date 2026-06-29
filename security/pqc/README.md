# Quantum-Safe Cryptography Toolkit

Prepares SigmaOS for the post-quantum era by integrating NIST-selected PQC
algorithms at the kernel level.

## Algorithms Integrated
| Purpose | Algorithm | NIST Status | 
| --- | --- | --- | 
| Key Encapsulation | Kyber-768 / ML-KEM | ✅ Standard | 
| Digital Signatures | Dilithium3 / ML-DSA | ✅ Standard | 
| Hashing | BLAKE3 | 🔧 Best-in-class | 

## Integration Points
- VPN key exchange (Kyber hybrid with X25519)
- Shard identity tokens (Dilithium signatures)
- Shard manifest signatures (SPM verifier)

## Roadmap
- [ ] Kyber KEM integration in VPN
- [ ] Dilithium signature in Identity Manager
- [ ] Side-channel hardening (constant-time implementations)
