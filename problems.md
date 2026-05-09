# SigmaOS Problem Fixes & Context Manager Migration

This document records the migration of hardcoded dependencies and unresolved references to the dynamic Context Manager API.

## Resolved Problems
1. **Context Manager Created**: Added `/core/context/manager.cpp` and `/core/context/manager.hpp` to handle dynamic dependency injection.
2. **Sigma Singleton Visibility**: Ensured `SigmaSingleton` is resolved by proper inclusion of `SigmaOOP.hpp` and normalization of path headers using the PowerShell script.
3. **Log Identifiers**: Fixed `sigma_log` undeclared identifiers by consistently prefixing and injecting the correct `#include "sigma_log.h"` headers across orchestration and quota management shards.
4. **Header Normalization**: Adjusted `#include` statements in `SovereignARM64.cpp` to correctly resolve `sigma_kernel_types.h` and the OOP framework.

## API Hooks Usage

Direct hardcoded instantiations of agents have been refactored to:
```cpp
void* module = SigmaOS::Kernel::Context::ContextManager::getInstance().resolve("problem_id_or_module_name");
```

### (This file will be migrated to the GitHub Wiki and deleted upon validation.)
