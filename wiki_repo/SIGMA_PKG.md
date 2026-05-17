# Σ SIGMA-PKG: INDUSTRIAL SHARD DISTRIBUTION

## 📦 The Package Nexus

`sigma-pkg` is the primary industrial distribution engine for SigmaOS shards. It ensures that every component added to the **Sovereign Lattice™** is cryptographically verified and dependencies are resolved with zero-dependency engineering principles.

## 🏛 Core USPs

- **Dilithium-5 Verification**: Every shard package must be signed with a PQC-attested key.

- **Dependency Graph Resolution**: Automatically identifies and pulls required kernel or userland shards.

- **Atomic Rollback**: Uses `S-WATCHDOG` and `S-DIAG` to revert the Lattice state if an installation fails integrity checks.

- **Lattice-Native**: Directly manipulates the shard registry (`SHARDS.manifest`).

## 🛠 Command Reference

- `sigma-pkg install <shard>`: Installs and registers a new industrial shard.

- `sigma-pkg remove <shard>`: Safely unlinks and purges a shard from the Lattice.

- `sigma-pkg update`: Synchronizes local shard manifests with the Sovereign Registry.

- `sigma-pkg rollback`: Emergency recovery to the last stable Lattice snapshot.

## 🚀 Industrial Workflow

1. **Verification**: PQC-GPG check of the shard binary.

2. **Resolution**: Mapping dependencies to the existing Lattice topology.

3. **Commit**: Updating `SHARDS.manifest` and re-linking the kernel bridge.

4. **Audit**: Running `S-AUDIT` to ensure zero-vulnerability integration.

---

### Stay Sovereign
 