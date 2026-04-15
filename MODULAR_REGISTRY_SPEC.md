# Modular Registry Specification (Sovereign v4.0)

## Overview
The Sovereign Registry is the central orchestration nexus for the 33-suite lattice. It manages the lifecycle, attestation, and dispatching of all Sovereign Shards.

## 🏗️ Structure
All registries are located in: `kernel/suites/S10_Registry/shards/`

### 1. Global Shard Registration
Shards must use the `SovereignRegistry_Register` API to bind to the lattice during stage-3 boot:
```c
sigma_err_t SovereignRegistry_Register(const char* name,
                                       shard_category_t cat,
                                       shard_init_fn init);
```

### 2. Shard Categories
- **CAT_CORE**: Genesis, Orchestration, Scheduling.
- **CAT_SECURITY**: Zero-Trust, LSM, MAC.
- **CAT_IO**: Drivers, VFS, Network.
- **CAT_USER**: Apps, UDFs, Dashboards.

### 3. Verification Protocol (GIV)
Each registration triggers a **Global Integration Verification (GIV)** sequence:
- **Hashing**: Verify shard BLAKE3 signature.
- **Purity**: Check for host-leakage (#include <stdio.h>).
- **Isolation**: Assign the shard to a Parallel-Universe Sandbox.

---
**STATUTORY RECORD. ENFORCED BY S01 GENESIS.**
