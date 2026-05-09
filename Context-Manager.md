# SigmaOS Context Manager

## Overview
The **Context Manager** (`/core/context/manager.cpp`) is the backbone of SigmaOS's zero-dependency, loosely-coupled architecture. It replaces all direct inter-module dependencies with a runtime-resolved API hook system.

## Architecture
```
[Module A]  --registerModule("agent.quota", this)-->  [ContextManager Registry]
[Module B]  --resolve("agent.quota")              -->  [ContextManager Registry]
                                                          |
                                                    returns void* to Module A
```

## API Reference

### `void registerModule(const char* module_id, void* instance)`
Registers a module (or sub-system) with the Context Manager by a unique string key.

**Example:**
```cpp
SigmaOS::Kernel::Context::ContextManager::getInstance()
    .registerModule("agent.quota", this);
```

### `void* resolve(const char* problem_id)`
Dynamically resolves and returns a pointer to the registered module instance.

**Example:**
```cpp
QuotaManager* qm = (QuotaManager*)
    SigmaOS::Kernel::Context::ContextManager::getInstance()
    .resolve("agent.quota");
```

## Module Registry (Sovereign Map)

| Module ID        | Registered By              | Consumers                     |
|-----------------|---------------------------|-------------------------------|
| `agent.quota`   | `QuotaManager`            | `CommandInterpreter`          |
| `agent.policy`  | `GovernanceRules`         | `CommandInterpreter`, Sandbox |
| `cashier`       | Profile loader            | `CommandInterpreter`          |
| `accountant`    | Profile loader            | `CommandInterpreter`          |
| `doctor`        | Profile loader            | `CommandInterpreter`          |
| `engineer`      | Profile loader            | `CommandInterpreter`          |
| `lawyer`        | Profile loader            | `CommandInterpreter`          |
| `farmer`        | Profile loader            | `CommandInterpreter`          |

## Design Principles
- **Zero-coupling**: No module imports another module directly. All communication is via the registry.
- **No stdlib**: Uses `SigmaString` and `SigmaMap` from `SigmaOOP.hpp`.
- **Singleton**: The `ContextManager` itself is a `SigmaSingleton` — one instance per kernel boot.
- **Fail-safe**: `resolve()` returns `nullptr` on miss; callers must handle gracefully.

## Implementation Files
- Header: `/include/core/context/manager.hpp`
- Source:  `/kernel/core/context/manager.cpp`
