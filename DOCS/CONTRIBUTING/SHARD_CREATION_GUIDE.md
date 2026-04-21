# 💎 SigmaOS: Sovereign Shard Creation Guide

Welcome, Architect. This guide outlines the requirements for contributing to the **Sovereign Lattice (S33)**.

## 🔳 Core Principles
To maintain the integrity of SigmaOS, all shards must adhere to the **Zero-Dependency Mandate**:
1. **No External Libraries**: Use only `sigma_libc.h` and `sigma_types.h`.
2. **Purity**: Code must be C11 compliant or raw Assembly (NASM/GAS).
3. **Isolation**: Shards must communicate via the **Sovereign Message Bus (S00)**.

## 🛠️ Step-by-Step: Building a Shard

### 1. Define the Suite
Place your shard in the appropriate suite directory within `kernel/suites/`. If creating a new suite, follow the `SX_Name` naming convention.

### 2. Implementation Template
```c
#include "sigma_libc.h"
#include "sigma_types.h"

/**
 * Sovereign [Shard Name]
 * Subsystem: S[X] ([Suite Name])
 */

void SX_Register_[ShardName](void) {
    sigma_printf("S[X]: [ShardName] Online.\n");
}
```

### 3. Registry Integration
Register your shard in the suite's registration file to ensure it is discovered by the **Sovereign Core (S00)** during the boot sequence.

## 🧪 Testing
Use the built-in `ShardOrchestrator` in the Zenith UI to verify hot-loading behavior.
1. Run `npx sigma-tool build [shard_path]`
2. Drag the resulting `.shard` file into the Zenith UI "Marketplace" window.

---
**Status:** Absolute Sovereignty Awaits your Contribution.
