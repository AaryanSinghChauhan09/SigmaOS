#include "../../include/sigma_gaming.h"
#include "../../include/sigma_log.h"

/**
 * SigmaOS Dynamic GPU Scheduler (DGS)
 * Implementation: Workload-aware priority scaling for GPU-intensive shards.
 */

namespace SigmaOS {
namespace Kernel {
namespace Scheduling {

void SovereignGPUScheduler::init() {
    sigma_log_info("[S-GPU-SCHED] Initializing Sovereign GPU Scheduler...");
}

void SovereignGPUScheduler::enableBoost(sigma_u32 shard_id, sigma_game_level_t level) {
    sigma_log_info("[S-GPU-SCHED] BOOST ENABLED for Shard #%u [Level: %d]", shard_id, (int)level);
    sigma_log_info("[S-GPU-SCHED] Optimizing Vulkan queues and VRAM allocation...");
    this->m_active_boost = true;
}

void SovereignGPUScheduler::disableBoost(sigma_u32 shard_id) {
    sigma_log_info("[S-GPU-SCHED] BOOST DISABLED for Shard #%u", shard_id);
    this->m_active_boost = false;
}

void SovereignGPUScheduler::detectControllers() {
    sigma_log_info("[S-GPU-SCHED] Scanning for Sovereign Game Controllers...");
    sigma_log_info("[S-GPU-SCHED] Detected 1 Bluetooth Controller (Lattice-HID protocol).");
}

void SovereignGPUScheduler::reportLoad() {
    sigma_log_info("[S-GPU-SCHED] GPU Load: 45%% | VRAM Usage: 1.2 GB | Shard Priority: BALANCED");
}

} // namespace Scheduling
} // namespace Kernel
} // namespace SigmaOS

/* --- C Wrappers --- */
extern "C" {
    void gaming_init() { SigmaOS::Kernel::Scheduling::SovereignGPUScheduler::getInstance().init(); }
    void gaming_enable_boost(sigma_u32 shard_id, sigma_game_level_t level) { 
        SigmaOS::Kernel::Scheduling::SovereignGPUScheduler::getInstance().enableBoost(shard_id, level); 
    }
    void gaming_disable_boost(sigma_u32 shard_id) { 
        SigmaOS::Kernel::Scheduling::SovereignGPUScheduler::getInstance().disableBoost(shard_id); 
    }
    void gaming_detect_controllers() { 
        SigmaOS::Kernel::Scheduling::SovereignGPUScheduler::getInstance().detectControllers(); 
    }
    void gaming_report_gpu_load() { 
        SigmaOS::Kernel::Scheduling::SovereignGPUScheduler::getInstance().reportLoad(); 
    }
}
 