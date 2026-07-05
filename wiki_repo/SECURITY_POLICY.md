# SigmaOS Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| v15.0.x Zenith (current) | ✅ Yes |
| v14.x and earlier | ❌ No |

## Reporting a Vulnerability

### Do not open a public GitHub issue for security vulnerabilities.

Report security issues privately:

1. Email: security@sigmaos.dev (or open a private GitHub Security Advisory)

2. Include: affected component, reproduction steps, potential impact

3. Response SLA: acknowledge within 48 hours, patch within 14 days for critical

## Security Architecture Overview

### Post-Quantum Cryptography

- **KEM:** Kyber-1024 (FIPS 203 / CRYSTALS-Kyber)

- **Signatures:** Dilithium-5 (FIPS 204 / CRYSTALS-Dilithium)

- **Hash:** BLAKE3 for package integrity, BLAKE2b for audit trails

- **TLS:** 1.3 with X25519/Kyber-1024 hybrid key exchange

### Kernel Hardening

- W^X (Write XOR Execute) enforcement on all memory regions

- ASLR 42-bit per-region randomisation

- sigma_pledge: per-process syscall allowlist

- sigma_unveil: per-process filesystem path restriction

- AVC (Access Vector Cache): O(1) MAC policy enforcement

- Zero-trust SPIFFE workload identities

- Namespace isolation (unshare/pivot_root/seccomp)

### Boot Security

- TPM2 attestation + key unsealing (CryptFS)

- Immutable audit trail (append-only, cryptographically chained)

- Verified boot pipeline (planned: sigma-boot.efi with signed stages)

### Package Security

- All `.spkg` packages signed with Dilithium-5

- BLAKE2b content hashes verified before installation

- Reproducible builds enforced via `sigma-repro-build`

## Known Open Issues

| ID | Component | Severity | Status |
|----|-----------|----------|--------|
| #1009 | CryptFS key derivation returns zero bytes | Critical | Phase G — `derive_key()` fix required |
| #851-WLAN | Wi-Fi stack not yet implemented | High | Phase G planned |
| #1007 | sigma-boot.efi does not exist | High | Phase G planned |

## Security Contacts

- Maintainer: @AaryanSinghChauhan09

- Security label: `security` on GitHub Issues

- CVE tracking: see `wiki_repo/CVE_TRIAGE.md`

---

*See also: [SECURITY.md](SECURITY.md) · [Wiki: Security Model](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Security-Model)*
