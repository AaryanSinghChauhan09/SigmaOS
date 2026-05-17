# Rolling Release & Delta Updates

SigmaOS achieves **Arch / Solus parity** through a rolling shard model and incremental delta updates.

## 🔄 The Rolling Lattice

SigmaOS does not have "point releases" in the traditional sense. Individual shards are updated continuously as new versions are certified.

### Key Features

- **Incremental Shard Updater**: `sigma-pkg` only downloads the binary diff (delta) between your current shard and the latest version.

- **Continuous Integration Builds**: Nightly builds for every shard target (x86, ARM, RISC-V).

- **Snapshot Manager**: Automatic FS snapshots taken before every update for risk-free rolling.

## 🚀 Stay Current

Synchronize your lattice with the rolling edge:

```bash
sigma-pkg update --rolling

```
 