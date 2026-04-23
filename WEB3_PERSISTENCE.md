# Web3 Persistence Layer

SigmaOS treats local storage as ephemeral and disposable. The true system state is securely backed up to a decentralised mesh network natively at the kernel level.

Located in `modules/core/fs/web3_persistence.c`.

## Competitive Advantages (USPs)

1. **Bare-Metal IPFS Integration**:
   - Standard operating systems require heavy user-space daemons (like `ipfs` or `syncthing`) to backup data.
   - SigmaOS integrates directly with the `SigmaFS` Copy-on-Write engine. The kernel periodically creates a zero-cost snapshot, chunks the data, and broadcasts it to the Sovereign Mesh Network via `mesh_net.c`.

2. **Hardware Resilience**:
   - If the physical NVMe drive dies, the SigmaOS node can be rebooted from a USB stick. The UEFI Bootloader authenticates the user's Ed25519 hardware key, queries the mesh network for the latest `CID` (Content Identifier), and restores the exact system state automatically.

3. **Zero-Trust Encryption**:
   - Before any block leaves the local machine, it is encrypted via `libsovereign_crypto` using ChaCha20. The mesh network stores the data but cannot read it.
