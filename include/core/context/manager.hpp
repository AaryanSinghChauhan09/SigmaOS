#ifndef SIGMA_CORE_CONTEXT_MANAGER_HPP
#define SIGMA_CORE_CONTEXT_MANAGER_HPP

#include "include/SigmaOOP.hpp"
#include "../../../include/sigma_log.h"

namespace SigmaOS {
namespace Kernel {
namespace Context {

class ContextManager : public SigmaSingleton<ContextManager>, public SigmaObject {
public:
    const char* type_name() const noexcept override { return "ContextManager"; }

    // Resolves a dependency or problem ID to a module or hook.
    void* resolve(const char* problem_id);

    // Registers a dependency
    void registerModule(const char* module_id, void* instance);

private:
    SigmaMap<SigmaString, void*> m_registry;
};

} // namespace Context
} // namespace Kernel
} // namespace SigmaOS

#endif
