#include "core/sigma_types.h"
#include "libc/SovereignLibC.h"
#include "hal/sigma_hal.h"
#include "sigma_log.h"
#include "core/context/manager.hpp"

/**
 * SigmaOS Autonomous Agent Quota Manager
 * Encapsulation: Each agent manages its own quota.
 */

class QuotaManager {
private:
    int gpu_quota;
    int cpu_quota;
    int mem_quota;

    QuotaManager() : gpu_quota(0), cpu_quota(0), mem_quota(0) {
        // Register this module via Context Manager instead of relying on hardcoded singleton
        SigmaOS::Kernel::Context::ContextManager::getInstance().registerModule("agent.quota", this);
    }

public:
    static QuotaManager& getInstance() {
        static QuotaManager instance;
        return instance;
    }

    void setQuota(const char* resource, int percentage) {
        if (sigma_hardened_strcmp(resource, "GPU") == 0) {
            gpu_quota = percentage;
            sigma_log("[AGENT] GPU Quota set to %d%%\n", percentage);
        } else if (sigma_hardened_strcmp(resource, "CPU") == 0) {
            cpu_quota = percentage;
            sigma_log("[AGENT] CPU Quota set to %d%%\n", percentage);
        } else if (sigma_hardened_strcmp(resource, "MEM") == 0) {
            mem_quota = percentage;
            sigma_log("[AGENT] MEM Quota set to %d%%\n", percentage);
        }
    }

    int getQuota(const char* resource) {
        if (sigma_hardened_strcmp(resource, "GPU") == 0) return gpu_quota;
        if (sigma_hardened_strcmp(resource, "CPU") == 0) return cpu_quota;
        if (sigma_hardened_strcmp(resource, "MEM") == 0) return mem_quota;
        return 0;
    }
};

void agent_quota_set(const char* resource, int percentage) {
    QuotaManager* quotaManager = (QuotaManager*) SigmaOS::Kernel::Context::ContextManager::getInstance().resolve("agent.quota");
    if (!quotaManager) {
        quotaManager = &QuotaManager::getInstance();
    }
    quotaManager->setQuota(resource, percentage);
}

} // extern "C"
