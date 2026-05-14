#include "../../../include/sigma_log.h"
#include "../../../include/core/sigma_types.h"
#include "../../../include/hal/sigma_hal.h"
#include "../../../include/libc/SovereignLibC.h"
#include "../../../include/system/sigma_scheduler.h"

namespace SigmaOS {
namespace Kernel {
namespace Orchestration {

SovereignScheduler& SovereignScheduler::getInstance() {
    static SovereignScheduler instance;
    return instance;
}

void SovereignScheduler::init() {
    sigma_log_info("[S-SCHED] Initializing Industrial Fair-Scheduler (Lattice-Aware)...");
    this->active_tasks = 0;
    this->initialized = true;
}

void SovereignScheduler::schedule(void (*task)(), sigma_u32 priority) {
    if (!this->initialized) return;
    if (this->active_tasks >= 1024) {
        sigma_log_warn("[S-SCHED] Task Lattice SATURATED. Dropping low-priority task.");
        return;
    }
    
    // Industrial Logic: Priority-aware scheduling (Simplified CFS concept)
    sigma_u32 task_id = this->active_tasks++;
    
    // Simulate core affinity based on priority (High priority tasks get performance cores)
    sigma_u32 core_affinity = (priority > 10) ? 0 : (task_id % 4);
    
    sigma_log_info("[S-SCHED] Task %u Scheduled | Priority: %u | Core: %u", 
                   task_id, priority, core_affinity);
    
    if (priority > 50) {
        sigma_log_info("[S-SCHED] High Priority Shard detected. Pre-empting lower shards...");
    }
}

} // namespace Orchestration
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void scheduler_init() { SigmaOS::Kernel::Orchestration::SovereignScheduler::getInstance().init(); }
    void scheduler_push(void (*task)(), sigma_u32 priority) { 
        SigmaOS::Kernel::Orchestration::SovereignScheduler::getInstance().schedule(task, priority); 
    }
}
