# Package Signing Agent

## Role
`SigmaPkgSign` implements cryptographic package signing and verification for SigmaOS packages — ensuring integrity and authenticity of all software distributed via SigmaPkg.

## Status
🔴 Not Started — Critical security gap. Required before any public package distribution.

## Source File
`src/package/signing.rs` (planned)

## Linux/BSD Inspiration
| Concept | Linux Distro | FreeBSD | SigmaOS |
|---|---|---|---|
| Package signing | GPG (`apt`/`pacman`) | pkg pubkey | SigmaPkgSign |
| Keyring management | `/etc/apt/keyrings/` | `/usr/share/keys/pkg/` | SigmaKeyring |
| Signature format | OpenPGP / minisign | RSA-PSS | SigmaSig (Ed25519) |
| Key revocation | CRL / WKD | — | SigmaRevoke |
| Repo metadata | InRelease (GPG clear-sign) | `digests` | SigmaRepoSig |
| Build reproducibility | `reproducible-builds.org` | — | SigmaRepro |

## Responsibilities
- Generate Ed25519 signing keypairs for package maintainers
- Sign package archives: `SigmaSig { pubkey_id, signature, timestamp }`
- Verify package signature before installation
- Manage trusted keyring: add/remove/revoke keys
- Sign repository metadata (package index, checksums)
- Detached signature files (`.sig`) alongside `.sigma` packages
- Hardware Security Module (HSM) / YubiKey signing support
- Build reproducibility verification

## Implementation Notes
- `#![no_std]` — Ed25519 via `sigma-crypto` (no OpenSSL)
- Signature: `Ed25519Sign(private_key, SHA512(package_bytes))`
- Keyring: `/etc/sigma/trusted-keys/` directory of Ed25519 pubkeys
- Integration: `SigmaPkg install` → verify sig → proceed
- Revocation list at `/etc/sigma/revoked-keys.list`

## Testing
```bash
rustc --test src/package/signing.rs --edition 2021 -o /tmp/test_sign && /tmp/test_sign --test-threads=1
```

## Future Development
1. **Sigstore integration** — Keyless signing via OIDC (like cosign)
2. **Hardware signing** — YubiKey/HSM for official SigmaOS keys
3. **Threshold signing** — M-of-N signatures for release keys
4. **SBOM integration** — Sign Software Bill of Materials
5. **OCI image signing** — Sigstore/Notary for container images
6. **Policy enforcement** — Only install packages signed by trusted keys
