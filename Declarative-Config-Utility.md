# Declarative Config Utility (`sigma-config`)

The `sigma-config` utility provides NixOS-inspired absolute state definition and bit-for-bit reproducibility across all Sovereign shards.

## Core Capabilities
* **Declarative States:** All system settings—from package dependencies to firewall rules—are defined in a strictly declarative `SigmaConf` definition.
* **Generation Snapshots:** Upon applying a configuration, SigmaOS generates a new, cryptographically signed system Generation.
* **Atomic Rollbacks:** If an applied state causes instability, you can atomically revert the entire OS state back to a previous Generation instantly.

## Commands
```bash
# Apply a target configuration
sigma-config apply /etc/sigma/system.json

# View active generation and state drift
sigma-config status

# Rollback globally to generation 41
sigma-config rollback 41
```
