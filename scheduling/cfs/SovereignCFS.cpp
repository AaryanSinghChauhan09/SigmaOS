/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN COMPLETELY FAIR SCHEDULER (S-CFS) IMPLEMENTATION
 * =========================================================================
 */
#include "SovereignCFS.h"
#include "../../include/sigma_log.h"

namespace SigmaOS {
namespace Scheduling {

void SovereignCFS::init() {
    sigma_log_info("[SCHED-CFS] Initializing Sovereign Completely Fair Scheduler...");
    m_task_count = 0;
    m_current_task = nullptr;
    for (int i = 0; i < MAX_TASKS; ++i) {
        m_runqueue[i] = nullptr;
    }
}

void SovereignCFS::enqueue_task(ShardTask* task) {
    if (m_task_count >= MAX_TASKS) {
        sigma_log_error("[SCHED-CFS] Runqueue overflow! Cannot enqueue Task %d", task->task_id);
        return;
    }
    
    task->is_runnable = true;
    m_runqueue[m_task_count++] = task;
    sigma_log_info("[SCHED-CFS] Enqueued Task %d (Shard %d). Total Tasks: %d", task->task_id, task->shard_id, m_task_count);
    
    sort_runqueue();
}

ShardTask* SovereignCFS::pick_next_task() {
    if (m_task_count == 0) return nullptr;
    
    // The runqueue is sorted, so index 0 has the lowest vruntime
    ShardTask* next = m_runqueue[0];
    
    if (next != m_current_task) {
        sigma_log_info("[SCHED-CFS] Context Switch -> Task %d (vruntime: %llu)", next->task_id, next->vruntime);
    }
    
    m_current_task = next;
    return m_current_task;
}

void SovereignCFS::tick_current_task(sigma_u64 elapsed_time) {
    if (!m_current_task) return;
    
    // Update virtual runtime (in a real CFS, this is scaled by priority weight)
    sigma_u64 time_slice_cost = elapsed_time * (100 / m_current_task->priority);
    m_current_task->vruntime += time_slice_cost;
    
    // Re-sort to maintain CFS fairness invariant
    sort_runqueue();
}

// Simple insertion sort to maintain ascending vruntime order
void SovereignCFS::sort_runqueue() {
    for (sigma_u32 i = 1; i < m_task_count; ++i) {
        ShardTask* key = m_runqueue[i];
        int j = i - 1;
        while (j >= 0 && m_runqueue[j]->vruntime > key->vruntime) {
            m_runqueue[j + 1] = m_runqueue[j];
            j = j - 1;
        }
        m_runqueue[j + 1] = key;
    }
}

} // namespace Scheduling
} // namespace SigmaOS
