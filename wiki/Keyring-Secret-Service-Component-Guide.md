# Keyring, Secret Service & TPM Vault Component Guide

## Component Overview
Provides FreeDesktop Secret Service API compatibility, AES-256-GCM encrypted credential storage, PAM unlock integration, and TPM 2.0 master key sealing.

## Linux Inspiration
GNOME Keyring daemon, keepassxc Secret Service provider, and Linux kernel keyring (keyctl).

## BSD Inspiration
OpenBSD authpf and blowfish credential hashing, FreeBSD secure credential vaults.

## Current SigmaOS Status
AES-256-GCM encryption primitives and secret storage tables defined in crypto engine. Secret Service D-Bus interface in progress.

## Critical Missing Features
Full org.freedesktop.secrets D-Bus wire protocol implementation, TPM 2.0 PCR-bound master key sealing, and automatic login keyring unlock.

## Implementation Priority
HIGH (Phase 1)

## Key Files to Create/Improve
src/security/keyring.rs, src/crypto/aes_ni.rs, src/security/tpm.rs

## Testing Strategy
Secret encryption/decryption round-trip tests, TPM unsealing failure tests under altered boot PCRs.

## Dependencies
CRYPTOGRAPHY_AGENT, SECURITY_AGENT, IPC_AGENT

## Success Criteria
Zero plaintext secret exposure in RAM, hardware-backed key protection, sub-millisecond password retrieval.

## Open Source Competitors Analysis
Replaces monolithic, unmaintained GNOME Keyring C code with a memory-safe, TPM-enforced Rust secret engine.

## Future Enhancements
Passkey (WebAuthn / FIDO2) credential provider embedded directly into the OS credential vault.
