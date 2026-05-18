# Reproducibility & Declarative Shards

SigmaOS achieves **NixOS parity** through declarative YAML manifests and immutable system profiles.

## 📜 Declarative Sovereignty

Every SigmaOS lattice state is defined by a single manifest. Changing the manifest and running `sigma-pkg sync` reconfigures the entire system.

### Key Concepts

- **Immutable Shards**: Shards are read-only and PQC-sealed. Updates happen via atomic symlink swaps.

- **Rollback Utilities**: Every update creates a snapshot. Run `sigma-cli rollback` to instantly revert to a previous lattice state.

- **Reproducible Builds**: `sigma-pkg` ensures that the same manifest always produces a bit-identical lattice.

## 🛠 Usage

Edit your `lattice.yaml` and apply:

```bash
sigma-pkg apply ./lattice.yaml


```
