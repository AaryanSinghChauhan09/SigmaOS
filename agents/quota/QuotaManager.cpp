#include "../../include/sigma_types.h"
#include "../../include/SigmaOOP.hpp"
#include "../../include/libc/sigma_libc.h"
#include "../../include/sigma_log.h"

/**
 * SigmaOS Autonomous Agent Quota Manager (S-QUOTA)
 * Mission: AI-native resource orchestration and neural workload balancing.
 * Principle: Modular. Encapsulated. Sovereign.
 */

namespace SigmaOS {
namespace Kernel {
namespace Agents {

class QuotaManager : public SigmaObject {
private:
    sigma_u32 m_gpu_quota;
    sigma_u32 m_cpu_quota;
    sigma_u32 m_mem_quota;
    sigma_u32 m_neural_quota;
    sigma_u32 m_lattice_quota;

    QuotaManager() : m_gpu_quota(0), m_cpu_quota(0), m_mem_quota(0), m_neural_quota(0), m_lattice_quota(0) {}

public:
    static QuotaManager& getInstance() {
        static QuotaManager instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "QuotaManager"; }

    void setQuota(const char* resource, sigma_u32 percentage) {
        if (sigma_strcmp(resource, "GPU") == 0) {
            m_gpu_quota = percentage;
        } else if (sigma_strcmp(resource, "CPU") == 0) {
            m_cpu_quota = percentage;
        } else if (sigma_strcmp(resource, "MEM") == 0) {
            m_mem_quota = percentage;
        } else if (sigma_strcmp(resource, "NEURAL") == 0) {
            m_neural_quota = percentage;
        } else if (sigma_strcmp(resource, "LATTICE") == 0) {
            m_lattice_quota = percentage;
        }
        sigma_log_info("[S-QUOTA] %s Quota recalibrated to %u%%", resource, percentage);
    }

    sigma_u32 getQuota(const char* resource) {
        if (sigma_strcmp(resource, "GPU") == 0) return m_gpu_quota;
        if (sigma_strcmp(resource, "CPU") == 0) return m_cpu_quota;
        if (sigma_strcmp(resource, "MEM") == 0) return m_mem_quota;
        if (sigma_strcmp(resource, "NEURAL") == 0) return m_neural_quota;
        if (sigma_strcmp(resource, "LATTICE") == 0) return m_lattice_quota;
        return 0;
    }
};

} // namespace Agents
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void agent_quota_set(const char* resource, sigma_u32 percentage) {
    SigmaOS::Kernel::Agents::QuotaManager::getInstance().setQuota(resource, percentage);
}

sigma_u32 agent_quota_get(const char* resource) {
    return SigmaOS::Kernel::Agents::QuotaManager::getInstance().getQuota(resource);
}

} // extern "C"

