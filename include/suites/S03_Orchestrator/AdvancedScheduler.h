#ifndef SIGMA_SCHEDULER_H
#define SIGMA_SCHEDULER_H

// SigmaOS Advanced Task Scheduler
// Absorbing CFS (Linux) and GCD (macOS) paradigms
#include "sigma_types.h"

void sched_init_appropriate_scheduling();
void sched_enqueue_task(void(*task_ptr)(void), uint8_t priority);
void sched_optimize_paging();

#endif // SIGMA_SCHEDULER_H

