# libsovereign_crypto

The `libsovereign_crypto` library provides the core cryptographic primitives required by SigmaOS to establish sovereign trust at the lowest levels of the hardware stack.

Located in `modules/security/crypto/`.

## Implemented Algorithms

### 1. SHA-256 (`sha256.c`)
- **Use Case:** Used by SigmaFS (`modules/core/fs/sigmafs.c`) for block hashing and Merkle tree root generation, and by the Tamper-Proof Audit Chain (`modules/security/access_control/audit_chain.c`) for linking log entries.
- **Design:** Highly optimized for bare-metal execution without OS dependencies.

### 2. ChaCha20 (`chacha20.c`)
- **Use Case:** The backbone of the Encrypted Sovereign Network Stack (`modules/core/net/sovereign_net.c`).
- **Design:** Chosen over AES because it provides superior software performance on minimal RISC-V and ARM architectures without hardware AES acceleration.

### 3. Ed25519 (`ed25519_stub.c`)
- **Use Case:** Used by Zero-Trust IPC (`modules/core/kernel/ipc.c`) to sign and verify every inter-process message, and by Mesh Networking (`modules/core/net/mesh_net.c`) for node identity.
- **Design:** Currently a kernel-interface stub representing the public-key signature system. Future work will integrate a full curve implementation.

## Future Roadmap
- Poly1305 integration for AEAD (Authenticated Encryption with Associated Data) to pair with ChaCha20.
- Hardware cryptographic accelerator hooks (mapping to `accel_hal.c`).
