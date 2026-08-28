/* SPDX-License-Identifier: MIT */
/*
 * =========================================================================
 * Σ SIGMAOS: SCHEDULER & REAL-TIME INTERFACE (S-SCHED)
 * =========================================================================
 * CachyOS BORE interactivity scores, RTLane real-time preemption,
 * CPU affinity, and task scheduling policy definitions.
 * =========================================================================
 */

#ifndef SIGMA_SCHED_H
#define SIGMA_SCHED_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Scheduling Policies --- */
#define SIGMA_SCHED_NORMAL   0
#define SIGMA_SCHED_FIFO     1
#define SIGMA_SCHED_RR       2
#define SIGMA_SCHED_BATCH    3
#define SIGMA_SCHED_IDLE     5
#define SIGMA_SCHED_DEADLINE 6
#define SIGMA_SCHED_BORE     7
#define SIGMA_SCHED_RTLANE   8

/* --- RTLane Real-Time Priority Levels --- */
#define SIGMA_RTLANE_PRIO_MAX 99
#define SIGMA_RTLANE_PRIO_MIN 1

/* --- BORE Scheduler Parameter Defaults --- */
#define SIGMA_BORE_MAX_BURST_MS    100
#define SIGMA_BORE_SCORE_DEFAULT   50

/* --- CPU Affinity Bitmask Structure --- */
typedef struct {
    sigma_u64 bits[16]; // Supports up to 1024 CPU cores
} sigma_cpu_set_t;

#define SIGMA_CPU_ZERO(set)           do { for(int i=0; i<16; i++) (set)->bits[i] = 0; } while(0)
#define SIGMA_CPU_SET(cpu, set)       ((set)->bits[(cpu)/64] |= (1ULL << ((cpu)%64)))
#define SIGMA_CPU_CLR(cpu, set)       ((set)->bits[(cpu)/64] &= ~(1ULL << ((cpu)%64)))
#define SIGMA_CPU_ISSET(cpu, set)     (((set)->bits[(cpu)/64] & (1ULL << ((cpu)%64))) != 0)

/* --- Scheduler Param Structure --- */
struct sigma_sched_param {
    sigma_i32 sched_priority;
    sigma_u32 bore_interactivity_score;
    sigma_u64 rtlane_deadline_ns;
};

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SCHED_H */
