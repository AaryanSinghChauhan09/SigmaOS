# SigmaOS Security Model

SigmaOS adopts a zero-trust, natively isolated approach to security, fundamentally differing from monolithic designs like Linux or Windows.

## Capability-Based Access Control

Every resource (IPC channels, file descriptors, PCI config space, memory maps) is gated by 64-bit unforgeable **capability tokens**.
- **No Global root:** There is no concept of a "root" user possessing omnipotent access.
- **VFS Enforcement:** `kernel/fs/vfs.rs` enforces capability checks at the descriptor open/truncate layer.
- **IPC Enforcement:** `kernel/ipc/ring_channel.rs` strictly drops messages that do not provide the correct capability token for the target channel.

## Cryptography

The kernel ships with native, zero-dependency implementations of essential cryptography for disk encryption and network protocols.
- **ChaCha20 (`kernel/crypto/chacha20.rs`)**: A highly optimized, constant-time stream cipher used natively by the kernel. No dynamic memory allocations are used.

## Package Security (`sigpkg`)

Supply chain security is a first-class citizen:
- **SBOM Verification (`sigma-pkg/sbom.rs`)**: Software Bill of Materials (SBOM) manifests are verified against incoming packages to ensure zero drift from signed source.
- **ED25519 Signatures (`sigma-pkg/ed25519_verify.rs`)**: Package manifests must be signed using ED25519 public key infrastructure. Untrusted binaries cannot execute on the host.

## Memory Safety

- **100% Rust No-Std**: Memory safety vulnerabilities (buffer overflows, use-after-free) are structurally prevented by Rust's ownership model in the kernel core.
- **Custom Memory Manager (`kernel/mm/buddy_slab_vmm.rs`)**: Avoids fragmentation and isolates kernel objects using dedicated SLAB caches.
