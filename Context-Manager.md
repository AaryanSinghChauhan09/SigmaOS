# Context-Manager



The **Context Manager** (`/core/context/manager.cpp`) is the backbone of SigmaOS's zero-dependency, loosely-coupled architecture. It replaces all direct inter-module dependencies with a runtime-resolved API hook system.



[Module A]  --registerModule("agent.quota", this)-->  [ContextManager Registry]
[Module B]  --resolve("agent.quota")              -->  [ContextManager Registry] | returns void* to Module A




Registers a module (or sub-system) with the Context Manager by a unique string key.



SigmaOS::Kernel::Context::ContextManager::getInstance()
    .registerModule("agent.quota", this);



Dynamically resolves and returns a pointer to the registered module instance.



QuotaManager*qm = (QuotaManager*)
    SigmaOS::Kernel::Context::ContextManager::getInstance()
    .resolve("agent.quota");






