#include "core/sigma_types.h"
#include "libc/SovereignLibC.h"
#include "hal/sigma_hal.h"

/**
 * SigmaOS Autonomous Agent Quota Manager
 * Encapsulation: Each agent manages its own quota.
 */

class QuotaManager {
private:
    int gpu_quota;
    int cpu_quota;
    int mem_quota;

    QuotaManager() : gpu_quota(0), cpu_quota(0), mem_quota(0) {}

public:
    static QuotaManager& getInstance() {
        static QuotaManager instance;
        return instance;
    }

    void setQuota(const char* resource, int percentage) {
        if (sigma_hardened_strcmp(resource, "GPU") == 0) {
            gpu_quota = percentage;
            sigma_log("[AGENT] GPU Quota set.");
        } else if (sigma_hardened_strcmp(resource, "CPU") == 0) {
            cpu_quota = percentage;
            sigma_log("[AGENT] CPU Quota set.");
        }
    }

    int getQuota(const char* resource) {
        if (sigma_hardened_strcmp(resource, "GPU") == 0) return gpu_quota;
        if (sigma_hardened_strcmp(resource, "CPU") == 0) return cpu_quota;
        return 0;
    }
};

extern "C" void agent_quota_set(const char* resource, int percentage) {
    QuotaManager::getInstance().setQuota(resource, percentage);
}
