# SigmaOS Installation GPG & Cryptographic Trust Keyring Subsystem - Master Development Plan

## 1. Executive Summary & Vision

`Sigma-GpgKeyring` (`pacman-key` / `apt-key` / `rpm-gpg`) is the native, zero-dependency cryptographic trust, GPG keyring initialization, and installation signature verification subsystem for **SigmaOS**. Inspired by the Web of Trust (WOT) master key initialization model of Arch Linux `pacman-key`, the modular `/etc/apt/trusted.gpg.d/` keyring hierarchy of Debian/Ubuntu, the `RPM-GPG-KEY-*` import engine of Fedora/RHEL, the lightweight `signify` public key cryptography of OpenBSD, and Post-Quantum Cryptography (PQC) hardware token attestation, `Sigma-GpgKeyring` guarantees end-to-end cryptographic integrity from live media installation through daily package updates.

---

## 2. Inspirations from Linux & BSD Ecosystems

| Ecosystem Origin | Feature & Cryptographic Keyring Capability Absorbed | Target Subsystem / Module |
| :--- | :--- | :--- |
| **Arch Linux `pacman-key`** | Master key initialization (`pacman-key --init`), distro master key population (`pacman-key --populate sigmaos`), Web of Trust (WOT) trust level assignments (*Marginal*, *Full*, *Revoked*). | `src/distro/arch_complete_parity_suite.rs` |
| **Debian & Ubuntu APT Keyring** | `/etc/apt/trusted.gpg.d/` modular keyring directory, automatic GPG key retrieval for PPAs (`gpg --keyserver hkps://keyserver.ubuntu.com`), ASCII-armored `.asc` parsing. | `src/package/debian_apt.rs` |
| **Fedora & RHEL `RPM-GPG-KEY`** | `/etc/pki/rpm-gpg/RPM-GPG-KEY-*` auto-import pipeline, RPM header OpenPGP signature packet validation, DNF repository key rotation. | `src/compatibility/fedora.rs` |
| **OpenBSD `signify` & FreeBSD `pkg`** | Lightweight Ed25519 signature verification, `signify` public key database (`/etc/signify/`), zero-dependency OpenPGP packet parser. | `src/package/bsd_linux_package_innovations.rs` |
| **PQC & TPM2 / YubiKey Hardware Security** | Dual-signature verification combining OpenPGP / Ed25519 with Dilithium-5 Post-Quantum Cryptography, TPM2 / YubiKey PKCS#11 hardware key storage. | `src/security/pki.rs` & `src/security/crypto_utils.rs` |

---

## 3. 5-Layer GPG Keyring & Installation Architecture

```
┌────────────────────────────────────────────────────────────────────────┐
│ Layer 5: Post-Quantum Cryptography (PQC) & TPM2/YubiKey Hardware Store │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 4: Automated Installer Key Injection & Repository Key Rotation  │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 3: HKPS Key Server Client & Web of Trust (WOT) Network Sync      │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 2: High-Performance OpenPGP & Signify Packet Signature Verifier  │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 1: Web of Trust Master Keyring & Distro Key Import Engine        │
└────────────────────────────────────────────────────────────────────────┘
```

### Layer 1: Web of Trust Master Keyring & Import Engine
- **Master Key Initialization (`pacman-key --init`):** Generating local entropy and initializing `/etc/sigmaos/gnupg/` trust database.
- **Distro Key Population (`pacman-key --populate`):** Auto-importing official SigmaOS developer keys, assigning WOT trust levels, and validating signature chains.
- **Modular Directory Import:** Sourcing `/etc/apt/trusted.gpg.d/*.asc` and `/etc/pki/rpm-gpg/RPM-GPG-KEY-*` keyrings seamlessly.

### Layer 2: High-Performance OpenPGP & Signify Packet Verifier
- **OpenPGP Packet Parser:** Pure Rust parser for OpenPGP (RFC 4880 / RFC 9580) public key, signature, and user ID packets.
- **Asymmetric Algorithms:** RSA-2048/4096, ECDSA (NIST P-256/P-384), Ed25519, and OpenBSD Signify Ed25519 keypairs.
- **Detached Signature Validation:** Verifying detached `.sig` / `.asc` signatures on repository metadata (`InRelease`, `repomd.xml`, `db.tar.gz`) and package archives.

### Layer 3: HKPS Key Server Client & Web of Trust Network Sync
- **HKPS Protocol Client:** Secure TLS-encrypted key retrieval over HKPS (`hkps://keyserver.ubuntu.com`, `hkps://pgp.mit.edu`).
- **PPA & COPR Key Auto-Fetch:** Automatically fetching and importing missing repository GPG keys upon adding Ubuntu PPAs or Fedora COPR repos.

### Layer 4: Automated Installer Key Injection & Repository Key Rotation
- **Installer Key Injection:** Automatically populating and trusting official release GPG keys during OS installation (`src/installer/gui_wizard.rs`).
- **Key Rotation & Revocation Lists (CRL):** Detecting expired or revoked GPG keys and updating keyring trust levels automatically.

### Layer 5: Post-Quantum Cryptography (PQC) & Hardware Security Module
- **Dual-Signature Attestation:** Validating both classical OpenPGP/Ed25519 signatures and Post-Quantum Dilithium-5 signatures on system updates.
- **TPM2 / YubiKey Hardware Protection:** Storing system release verification keys inside TPM2 NVRAM or PKCS#11 hardware security tokens.

---

## 4. Implementation Roadmap

| Milestone | Target Phase | Objectives | Status |
| :--- | :--- | :--- | :--- |
| **Milestone 1** | Keyring Import Engine | Build `pacman-key` initialization, master key import, and WOT trust level manager in `src/sigpkg/`. | Implemented |
| **Milestone 2** | OpenPGP Packet Verifier | Implement pure-Rust OpenPGP packet parser, Ed25519 / RSA signature verifier, and Signify parser. | Implemented |
| **Milestone 3** | HKPS Keyserver Sync | Implement HKPS TLS keyserver client and automated key fetching for PPA/COPR repositories. | Implemented |
| **Milestone 4** | Installer Key Injection | Integrate GPG keyring population and key verification wizard into Calamares GUI installer. | Implemented |
| **Milestone 5** | PQC & TPM2 Security | Enforce Dilithium-5 PQC dual-signatures and TPM2 hardware security module key storage. | Implemented |

---

## 5. Verification & Testing Strategy

1. **Unit Tests:** Standalone test runners in `src/distro/arch_complete_parity_suite.rs`, `src/package/debian_apt.rs`, and `src/security/pki.rs`.
2. **Signature Verification Tests:** Validating bit-exact signature verification against official Arch, Debian, Fedora, and OpenBSD release signatures.
3. **Automated Verification:** Continuous validation via `./run_sigma_tests.sh`.
