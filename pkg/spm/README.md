# Sovereign Package Manager (SPM)

SPM is the cryptographically secure, deterministic package manager for SigmaOS. Unlike apt or dnf, which rely on legacy POSIX architectures and assume global shared states, SPM manages isolated **shards**.

## Core Principles
1. **Cryptographic Verifiability:** No shard is installed without passing a strict digital signature verification check against the Sovereign Trust Root.
2. **Deterministic Rollbacks:** Upgrades are atomic. Any failure immediately reverts the state pointer.
3. **Dependency Isolation:** Shards do not pollute a global `/usr/lib`. Dependencies are strictly mapped via the Shard Manifest.

## Components
* `cli.py`: The user interface for managing packages.
* `verifier.py`: The cryptographic core that enforces the Zero-Trust execution module.
* `schema/shard_manifest.json`: The JSON schema dictating valid package structures.

## Example Usage
```bash
spm install SovereignNet
spm verify SovereignNet
spm rollback
```text
