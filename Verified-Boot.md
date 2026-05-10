# Verified & Secure Boot in SigmaOS

To ensure the **Sovereignty** of the lattice, SigmaOS implements a chain of trust from the hardware root up to the userland Zenith UI.

## 🔑 Chain of Trust

1. **S-ROM**: Immutable silicon-level public key hash.
2. **S-BOOT**: The bootloader verified against the S-ROM key.
3. **S-KERNEL**: The core lattice shards verified by the bootloader.
4. **S-APP**: Userland shards signed by a trusted identity.

## 🛠️ Implementation Details

* **Signature Algorithm**: Dilithium-based Post-Quantum Cryptography (PQC).
* **TPM Integration**: Measurements of each shard are stored in TPM PCRs via the `SovereignTPM` shard.
* **PCR Management**: PCR[0-7] track the core lattice state, while PCR[17-23] are available for userland-signed shards.
* **Attestation**: Real-time verification of shard integrity via the `SovereignAttestation` shard, which cross-references TPM signatures.

## 🖋️ Signing Shards

Developers can sign their custom shards using the `sigma-sign` tool:

```bash
sigma-sign --key my_identity.key --shard custom_driver.cpp
```

## 🔒 Policy Enforcement

The `SovereignInit` shard enforces the boot policy:

* `STRICT`: Only officially signed shards can boot.
* `USER_TRUST`: Allows shards signed by user-added keys.
* `OPEN`: (Not recommended) Any shard can boot (development mode).

---

### For key management instructions, see the [Security Admin Guide](Security-Admin-Guide.md)
