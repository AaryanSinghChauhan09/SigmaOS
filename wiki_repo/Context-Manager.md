# Context-Manager

1

1

The **Context Manager** (`/core/context/manager.cpp`) is the backbone of SigmaOS's zero-dependency, loosely-coupled architecture. It replaces all direct inter-module dependencies with a runtime-resolved API hook system.

1

1

[Module A]  --registerModule("agent.quota", this)-->  [ContextManager Registry]
[Module B]  --resolve("agent.quota")              -->  [ContextManager Registry] | returns void* to Module A

1

1

1

Registers a module (or sub-system) with the Context Manager by a unique string key.

1

1

SigmaOS::Kernel::Context::ContextManager::getInstance()
    .registerModule("agent.quota", this);

1

1

Dynamically resolves and returns a pointer to the registered module instance.

1

1

QuotaManager*qm = (QuotaManager*)
    SigmaOS::Kernel::Context::ContextManager::getInstance()
    .resolve("agent.quota");

1

1

1

1

1

1
 