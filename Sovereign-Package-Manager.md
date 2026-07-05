# Sovereign Package Manager (SPM / `sigpkg`)

SPM is the cryptographically secure, deterministic package manager for SigmaOS.
Unlike `apt` or `dnf` — which rely on global shared state and binary blobs —
SPM manages isolated **shards** using a recipe-based, reproducible build system.

## Core Principles

1. **Cryptographic Verifiability:** No shard is installed without passing a
   strict Ed25519 digital signature check against the Sovereign Trust Root.
2. **Deterministic Rollbacks:** Upgrades are atomic. Any failure immediately
   reverts the state pointer via SovereignFS CoW snapshots.
3. **Dependency Isolation:** Shards do not pollute a global `/usr/lib`.
   Dependencies are strictly mapped via the Shard Manifest.
4. **Reproducible Builds:** The same `.srecipe` file + same source commit
   always produces a bit-for-bit identical shard binary.

## Components

| File | Description |
|---|---|
| `cli.py` | User-facing CLI — `sigpkg install`, `remove`, `search`, `rollback` |
| `verifier.py` | Cryptographic core — verifies Ed25519 signatures, hash chains |
| `schema/shard_manifest.json` | JSON schema for valid `.shard` package structures |

## Example Usage

```bash
# Install a shard from the Sovereign Registry
sigpkg install SovereignNet

# Verify a locally downloaded shard
sigpkg verify SovereignNet

# Atomic rollback to the previous state
sigpkg rollback

# Search the registry
sigpkg search "web server"

# Build a shard from source recipe
sigpkg build ./my_app.srecipe

# List installed shards
sigpkg list --installed
```

## Shard Manifest Format

```json
{
  "name": "SovereignNet",
  "version": "2.1.0",
  "arch": ["x86_64", "aarch64"],
  "capabilities_required": ["CAP_NET_BIND"],
  "dependencies": [],
  "sha256": "abc123...",
  "signature": "<Ed25519 over (name|version|sha256)>"
}
```

## Roadmap

- [x] CLI skeleton (`sigpkg install`, `remove`, `rollback`)
- [x] Signature verifier (`verifier.py`)
- [x] Shard manifest schema
- [ ] Registry server implementation
- [ ] Reproducible build toolchain integration
- [ ] Delta updates (only ship changed extents)
- [ ] Multi-architecture cross-build support
- [ ] GUI package browser for Zenith Desktop

## Related Modules

- [`modules/tools/loader`](../../modules/tools/loader/README.md) — Module loader
- [`modules/security/secure_boot`](../../modules/security/secure_boot/README.md) — Signature verification
