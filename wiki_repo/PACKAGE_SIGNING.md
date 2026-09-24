# SigmaOS Cryptographic Package Signing Specification

## 1. Overview

To guarantee supply-chain integrity and defend against repository tampering, man-in-the-middle attacks, and unauthorized software distribution, SigmaOS enforces mandatory cryptographic signature verification on all `.sigpkg` packages and repository index databases.

## 2. Cryptographic Key Architecture

SigmaOS implements a hybrid signing strategy featuring standard elliptic-curve cryptography alongside post-quantum signatures:

```
+-----------------------------------------------------------------+
|               Cryptographic Signature Header Layout             |
+-----------------------------------------------------------------+
| 1. Primary Algorithm: Ed25519 (High performance 256-bit ECC)   |
| 2. Post-Quantum Backup: ML-DSA / Dilithium-3 (NIST Standard)    |
| 3. Key Identifier (64-bit Fingerprint)                          |
| 4. Detached Signature Payload (Signed over BLAKE3 Merkle Root)  |
+-----------------------------------------------------------------+
```

### 2.1 Web of Trust & Keyrings
- **System Keyring (`/etc/sigpkg/keys/`)**: Holds trusted public keys of official SigmaOS core maintainers and automated build farm release keys.
- **Key Revocation Lists (KRL)**: Automatically fetched during repository updates to instantly invalidate compromised signing keys.

## 3. Package Verification Workflow (`sigkeyring`)

When installing or upgrading packages via `sigpkg`:

```
+-----------------------------------------------------------+
| 1. Download Package Header & Detached Signature           |
+-----------------------------+-----------------------------+
                              |
                     sigkeyring verify
                              |
+-----------------------------v-----------------------------+
| 2. Verify Key Fingerprint against Trusted System Keyring  |
|    (Reject if key is missing or revoked)                  |
+-----------------------------+-----------------------------+
                              |
                   BLAKE3 Hash Tree Verification
                              |
+-----------------------------v-----------------------------+
| 3. Compute BLAKE3 Hash over Package Payload               |
|    (Ensure payload hash matches signed header root)       |
+-----------------------------+-----------------------------+
                              |
                  Unpack & Install Payload
                              |
+-----------------------------v-----------------------------+
| 4. Commit to Atomic Merkle Store                          |
+-----------------------------------------------------------+
```

## 4. Key Management CLI Utilities

- **Generate Maintainer Keypair**: `sigkeyring gen-key --name "Alice Maintainer <alice@sigmaos.org>"`
- **Sign Package**: `sigkeyring sign-pkg --key ~/.sigpkg/secring.gpg zenith-terminal-1.2.0-1-x86_64.sigpkg`
- **Verify Package**: `sigkeyring verify-pkg zenith-terminal-1.2.0-1-x86_64.sigpkg`
- **Import Public Key**: `sigkeyring import-key official-release-2026.pub`
