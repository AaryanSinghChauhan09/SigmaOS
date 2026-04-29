/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN REAL-TIME CORE (S-REALTIME)
 * =========================================================================
 * Mission: Offer a specialized mode for robotics, industrial automation,
 * and mission-critical systems with guaranteed microsecond latency.
 * =========================================================================
 */

#ifndef SIGMA_REALTIME_H
#define SIGMA_REALTIME_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t task_id;
    uint32_t deadline_us;
    uint32_t priority;
} sigma_realtime_task_t;

/* --- Real-Time Core Primitives --- */
void realtime_init(void);
bool realtime_schedule_task(const sigma_realtime_task_t* task, void (*task_func)(void));
void realtime_execute_critical_path(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_REALTIME_H */
