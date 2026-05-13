# Security Policy

## Supported Versions

| Version | Supported               |
| ------- | ----------------------- |
| v29.x   | ✅ Actively supported   |
| v28.x   | ⚠️  Security fixes only |
| < v28   | ❌ Not supported        |

## Reporting a Vulnerability

If you discover a security vulnerability in the SigmaOS Sovereign Lattice, **do not open a public issue**. Instead:

1. Email the SigmaOS Security Council at the contact email on the repository profile.
2. Include: affected shard name, reproduction steps, and potential impact.

3. You will receive a response within **72 hours**.

## Security Architecture

SigmaOS employs multiple layers of defense:

* **`SovereignSEL`** — Mandatory Access Control enforced in Ring-0.
* **`SovereignPQC`** — Post-Quantum Cryptography for all key material.

* **`SovereignEnclave`** — Hardware-level isolation for cryptographic state.
* **`SovereignSandbox`** — Zero-trust container isolation for all userland processes.

* **CodeQL** — Automated vulnerability scanning on every pull request.
* **Dependabot** — Weekly automated supply-chain patch management.

