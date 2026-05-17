#include "../../../include/libc/SovereignLibC.h"
#include "../../../include/core/context/manager.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Context {

void* ContextManager::resolve(const char* problem_id) {
    sigma_log("[ContextManager] Resolving dependency for %s\n", problem_id);
    SigmaString key(problem_id);
    for (sigma_size_t i = 0; i < m_registry.size(); ++i) {
        // Simple string comparison for map
        const char* reg_key = m_registry.key_at(i).c_str();
        sigma_size_t len1 = sigma_strlen(reg_key);
        sigma_size_t len2 = sigma_strlen(problem_id);
        if (len1 == len2) {
            bool match = true;
            for (sigma_size_t j = 0; j < len1; ++j) {
                if (reg_key[j] != problem_id[j]) {
                    match = false;
                    break;
                }
            }
            if (match) {
                return *m_registry.at_index(i);
            }
        }
    }
    sigma_log("[ContextManager] Failed to resolve dependency %s\n", problem_id);
    return nullptr;
}

void ContextManager::registerModule(const char* module_id, void* instance) {
    sigma_log("[ContextManager] Registering module %s\n", module_id);
    m_registry.insert(SigmaString(module_id), instance);
}

} // namespace Context
} // namespace Kernel
} // namespace SigmaOS

 