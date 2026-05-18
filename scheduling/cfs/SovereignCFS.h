/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN COMPLETELY FAIR SCHEDULER (S-CFS) HEADER
 * =========================================================================
 */
#ifndef SIGMA_SOVEREIGN_CFS_H
#define SIGMA_SOVEREIGN_CFS_H

#include "sigma_kernel_types.h"

namespace SigmaOS {
namespace Scheduling {

struct ShardTask {
    sigma_u32 task_id;
    sigma_u32 shard_id;
    sigma_u64 vruntime;      // Virtual runtime for CFS ordering
    sigma_u32 priority;      // Base priority (1-100)
    bool      is_runnable;
};

class SovereignCFS {
public:
    void init();
    void enqueue_task(ShardTask* task);
    ShardTask* pick_next_task();
    void tick_current_task(sigma_u64 elapsed_time);

private:
    // Simple priority queue abstraction (Zero-dependency fixed array for now)
    static const int MAX_TASKS = 256;
    ShardTask* m_runqueue[MAX_TASKS];
    sigma_u32 m_task_count;
    ShardTask* m_current_task;

    void sort_runqueue();
};

} // namespace Scheduling
} // namespace SigmaOS

#endif // SIGMA_SOVEREIGN_CFS_H
