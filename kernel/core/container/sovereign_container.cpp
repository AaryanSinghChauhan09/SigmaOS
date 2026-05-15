#include "../../../include/Lattice.h"
#include "../../../include/sigma_log.h"
#include "sovereign_container.hpp"
#include "../../../include/sigma_log.h"
#include "../../../include/libc/SovereignLibC.h"
#include "../../../include/sigma_log.h"

namespace SigmaOS {
namespace Kernel {

void SovereignContainer::Launch() {
    sigma_log_info("[CONTAINER]: Igniting Bare-Metal Shard Isolation for ID: %d\n", m_container_id);
    sigma_log_info("[CONTAINER]: Image Shard: %s | Quota: %llu MB\n", m_image_shard, m_memory_limit / (1024*1024));
    m_active = SIGMA_TRUE;
}

void SovereignContainer::Terminate() {
    sigma_log_info("[CONTAINER]: Extinguishing Shard ID: %d | Reclaiming Silicon Shards...\n", m_container_id);
    m_active = SIGMA_FALSE;
}

void SovereignContainer::Audit() {
    sigma_log_info("| Container [%d]: %-15s | Status: %s | Memory: %llu MB\n", 
        m_container_id, m_image_shard, m_active ? "RUNNING" : "STOPPED", m_memory_limit / (1024*1024));
}

void ContainerManager::Deploy(const char* shard_path, sigma_size_t memory_quota) {
    if (m_count < 128) {
        m_active_containers[m_count] = new SovereignContainer(m_count, shard_path, memory_quota);
        m_active_containers[m_count]->Launch();
        m_count++;
    }
}

void ContainerManager::Audit() {
    sigma_log_info("\n--- Σ SOVEREIGN CONTAINER AUDIT ---\n");
    sigma_log_info("| Managed Containers: %d\n", m_count);
    for (sigma_u32 i = 0; i < m_count; ++i) {
        m_active_containers[i]->Audit();
    }
    sigma_log_info("-----------------------------------\n");
}

} // namespace Kernel
} // namespace SigmaOS


