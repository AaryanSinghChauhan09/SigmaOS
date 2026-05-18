/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SCHEDULER — IMPLEMENTATION (v15.0 ZENITH)
 * =========================================================================
 * Mission: Completely Fair Scheduling (CFS) with AI telemetry hooks.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "system/sigma_scheduler.h"

namespace SigmaOS {
namespace Kernel {
namespace Orchestration {

void SovereignScheduler::init() {
    sigma_log_info("[S-SCHED] Initializing Completely Fair Scheduler (S-CFS)...");
    m_task_count    = 0u;
    m_min_vruntime  = 0u;
    sigma_log_info("[S-SCHED] CFS ready. Quantum: 4ms. Max tasks: 1024.");
}

void SovereignScheduler::schedule(void (*task)(), sigma_u32 priority) {
    if (!task || m_task_count >= 1024u) {
        sigma_log_error("[S-SCHED] Cannot schedule: %s",
                        !task ? "null task" : "task queue full");
        return;
    }

    SovereignTask& t = m_tasks[m_task_count];
    t.id       = m_task_count + 1u;
    t.func     = task;
    t.priority = priority;
    t.vruntime = m_min_vruntime + (100u / (priority + 1u)); /* CFS vruntime fairness */
    t.active   = true;
    m_task_count++;

    sigma_log_info("[S-SCHED] Task %u scheduled (priority=%u, vruntime=%llu)",
                   t.id, priority, t.vruntime);
}

} // namespace Orchestration
} // namespace Kernel
} // namespace SigmaOS

/* =========================================================================
 * C Bridge
 * ========================================================================= */
extern "C" {

void scheduler_init() {
    SigmaOS::Kernel::Orchestration::SovereignScheduler::getInstance().init();
}

void scheduler_push(void (*task)(), sigma_u32 priority) {
    SigmaOS::Kernel::Orchestration::SovereignScheduler::getInstance().schedule(task, priority);
}

} /* extern "C" */
 