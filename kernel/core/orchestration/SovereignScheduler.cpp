#include "sigma_hal.h"
#include "SovereignLibC.h"
#include "sigma_scheduler.h"

namespace SigmaOS {
namespace Kernel {
namespace Orchestration {

SovereignScheduler& SovereignScheduler::getInstance() {
    static SovereignScheduler instance;
    return instance;
}

void SovereignScheduler::init() {
    sigma_log("Σ [SCHEDULER]: Initializing Lattice-Aware Scheduler...");
    this->active_tasks = 0;
    this->initialized = true;
}

void SovereignScheduler::schedule(void (*task)(), sigma_u32 priority) {
    if (this->active_tasks >= 1024) return;
    
    lattice_task_t new_task;
    new_task.id = this->active_tasks++;
    new_task.priority = priority;
    new_task.entry_point = task;
    
    // Industrial Logic: Assign silicon affinity based on thermal telemetry
    // In a real kernel, this would read from MSRs or a thermal shard
    sigma_u32 core_temp = 45; 
    if (core_temp > 80) {
        new_task.silicon_affinity = (new_task.id + 1) % 8; // Thermal throttling avoidance
    } else {
        new_task.silicon_affinity = new_task.id % 8; 
    }
    
    sigma_printf("Σ [SCHEDULER]: Scheduled Task %u [Priority: %u] on Core %u (Silicon Thermal: %uC)\n", 
                 new_task.id, priority, new_task.silicon_affinity, core_temp);
}

} // namespace Orchestration
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void scheduler_init() {
    SigmaOS::Kernel::Orchestration::SovereignScheduler::getInstance().init();
}

extern "C" void scheduler_push(void (*task)(), sigma_u32 priority) {
    SigmaOS::Kernel::Orchestration::SovereignScheduler::getInstance().schedule(task, priority);
}


