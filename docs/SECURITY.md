# Security Provider Boundaries

Security-facing APIs must fail closed when the trusted provider they need is unavailable. A model or caller-supplied boolean does not prove biometric identity, hardware-backed sealing, encryption, or a remote password-breach lookup.

## Vault and password manager status

- Biometric vault unlock is unavailable until an authenticated biometric provider is integrated. A caller-provided match flag must never unlock the vault.
- TPM-backed password sealing is unavailable until a trusted TPM provider is integrated. Passwords must not be stored with a descriptive prefix or other reversible placeholder.
- Password-breach lookup is unavailable until the remote provider and its privacy-preserving protocol are integrated. A small local list cannot establish that a password is safe.
- Cryptodev encryption is unavailable until an audited cryptographic provider is integrated. Reversible XOR or other home-grown transformations must never be exposed as encryption.

The current APIs return provider-unavailable errors for these operations and preserve locked or empty state.

## Maintenance instructions for AI agents

When changing these components, keep provider boundaries explicit, avoid logging or retaining plaintext credentials, and preserve the fail-closed behavior when provider initialization or verification fails. Do not describe an operation as hardware-backed or authenticated until its trusted provider is wired into the runtime and the integration path is reviewed. Update this document and the GitHub Wiki security topic when the verified runtime behavior changes.

Relevant code: `src/wiki_unimplemented_ideas.rs` and `tests/test_md_wiki_ideas_verification.rs`.
The unintegrated cryptodev prototype is in `src/crypto/advanced_encryption_standard.rs`.
