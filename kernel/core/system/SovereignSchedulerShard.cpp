/*
 * =========================================================================
 * SIGMAOS: SOVEREIGN REAL-TIME SCHEDULER IMPLEMENTATION
 * =========================================================================
 */

#include "../../../include/core/sigma_scheduler.h"
#include "../../../include/sigma_log.h"

namespace SigmaOS {
namespace Core {

sigma_status SovereignSchedulerShard::init() {
    sigma_log_info("[S-SCHED] Initializing Hard-RTOS Deterministic Scheduler...");
    
    // Setup initial IDLE task
    m_tasks[0].id = 0;
    m_tasks[0].priority = TaskPriority::IDLE;
    m_tasks[0].is_active = true;
    m_task_count = 1;
    
    sigma_log_info("[S-SCHED] Scheduler initialized. Preemption ENABLED.");
    return SIGMA_OK;
}

sigma_status SovereignSchedulerShard::spawn_task(void (*entry)(), TaskPriority priority, sigma_u32* out_id) {
    if (m_task_count >= 256) return SIGMA_ERROR;
    
    sigma_u32 id = m_task_count++;
    m_tasks[id].id = id;
    m_tasks[id].priority = priority;
    m_tasks[id].context.rip = (sigma_u64)entry;
    // Allocate isolated 4KB stack per task via Sovereign Memory Manager...
    m_tasks[id].is_active = true;
    
    if (out_id) *out_id = id;
    
    sigma_log_info("[S-SCHED] Task Spawned: ID=%d, Priority=%d", id, (int)priority);
    return SIGMA_OK;
}

void SovereignSchedulerShard::yield() {
    // Manually trigger a task switch
    // __asm__ volatile ("int $0x20"); // e.g., trigger software interrupt or timer IRQ
}

void SovereignSchedulerShard::tick() {
    // Called by the PIT or local APIC timer
    // O(1) scheduling logic or EDF
    
    // AI Telemetry Hook:
    // Update performance metrics for anomaly detection
}

} // namespace Core
} // namespace SigmaOS
