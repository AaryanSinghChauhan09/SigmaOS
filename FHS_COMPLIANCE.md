# SigmaOS Filesystem Hierarchy Standard (FHS) Compliance

To provide "shared identity" and familiarity for Linux users, SigmaOS maps its decentralized Sovereign Lattice to the traditional **Filesystem Hierarchy Standard (FHS)**. [**STATUS: OPERATIONAL**]

## 📂 Mapping the Lattice to FHS

While SigmaOS is amnesic and shard-based, it provides a Virtual File System (VFS) layer that presents a standard Linux directory structure: | FHS Path | SigmaOS Lattice Shard / Role | | :--- | :--- | | `/bin` / `/sbin` | **Core Shard Binaries**: Immutable system-level execution shards. | | `/etc` | **SovereignPersonalization**: Configuration shards persisted via `LatticeFS`. | | `/dev` | **HAL Shards**: Direct hardware abstraction layer device mappings. | | `/lib` / `/lib64` | **SovereignLibC** / **LinuxDriverCompat**: Shared shims and compatibility ABIs. | | `/mnt` | **LatticeMount**: Dynamic attachment points for external storage shards (NVMe, USB). | | `/proc` / `/sys` | **SovereignTelemetry**: Real-time kernel state and hardware metrics. | | `/var` | **SovereignLog**: Persistent logging and ephemeral shard state. | | `/home` | **User Workspaces**: Cryptographically isolated per-user sandboxes. | ## 📦 Convention Over Configuration

By adhering to these paths, SigmaOS ensures that:

1. **Tooling Parity**: Standard tools (`ls`, `cd`, `grep`, `vim`) work as expected.
2. **User Onboarding**: Users from Ubuntu, Fedora, or Arch find a recognizable environment.
3. **Application Portability**: Linux applications can locate resources using standard FHS assumptions.
