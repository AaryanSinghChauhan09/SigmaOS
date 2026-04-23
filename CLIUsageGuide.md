# SigmaOS CLI Usage Guide

The **SigmaOS Sovereign CLI** (S-CLI) is the primary interface for managing the 500-shard lattice, managing plugins, and performing system diagnostics.

## Namespaced Commands

### 🛡️ System Diagnostics
Check the health of the entire lattice and core primitives.
```bash
sigma doctor
```

### 🧩 Shard Management
Manage the 500 shards that make up the Sovereign Lattice.
```bash
sigma shard list
sigma shard add <name>
```

### ⚙️ Declarative Configuration
Apply or rollback system states using the JSON-based declarative manifests.
```bash
sigma config apply
sigma config rollback
```

### 🔌 Plugin Lifecycle
Register and manage Zenith Dashboard extensions.
```bash
sigma plugin list
sigma plugin install <path>
```

## Interactive Mode
To enter the interactive Sovereign Shell:
```bash
sigma shell
```

## Advanced Usage
For cloud-native deployments:
```bash
sigma deploy --remote
sigma sync cloud
```
