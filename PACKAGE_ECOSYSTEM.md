# SigmaOS Package Ecosystem (`sigma-pkg`)

SigmaOS abandons `apt`, `dnf`, and standard ELF dynamic linking nightmares in favor of `sigma-pkg` v2: a statically-linked, universal format packaging ecosystem.

## Core Design Principles
1. **SAT-Resolved:** Dependency trees are flattened and mathematically resolved using our native SAT solver.
2. **Post-Quantum Signing:** Every package is signed with Kyber-1024. Packages without signatures, or with invalid signatures, are completely rejected by the kernel loader.
3. **Immutability & Rollbacks:** Upgrades are applied transactionally to a new partition using `sigma-update`. `sigma-rollback` guarantees 1-second reverts.

## Using `sigma-pkg`

### Install a package
```bash
sigma-pkg install <package_name>
```

### Build a package
Packages are declared using a `manifest.sigma` format (JSON-based):
```json
{
  "name": "zenith-browser",
  "version": "1.0.4",
  "dependencies": ["libzenith"],
  "signature_type": "kyber1024"
}
```
Run `sigma-build pack .` to generate the `.spk` (Sigma Package) artifact.

## Universal Formats
While SigmaOS native apps use `.spk`, we support running existing cloud-native software via our OCI container runtime integration (`sigma-ctr`). This allows you to pull Docker images directly and run them seamlessly inside a `sigma-jail` isolation shard.
