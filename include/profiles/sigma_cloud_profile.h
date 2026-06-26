/*
 * =============================================================================
 * Σ SIGMAOS: SOVEREIGN CLOUD PROFILE
 * =============================================================================
 * Flagship Niche: High-Performance Computing (HPC) & Cloud-Native Workloads.
 * Replaces Docker/K8s nodes with hyper-optimized bare-metal execution.
 * =============================================================================
 */

#ifndef SIGMA_CLOUD_PROFILE_H
#define SIGMA_CLOUD_PROFILE_H

/* Disable UI Compositor completely */
#define SIGMA_FEATURE_GUI_ENABLED 0

/* Optimize Scheduler for throughput (longer timeslices, less preemption) */
#define SCHEDULER_MODE_THROUGHPUT 1
#define SCHEDULER_TIME_SLICE_MS   50

/* Maximize Network Buffer Sizes for heavy I/O */
#define NET_MAX_SOCKETS     4096
#define NET_RX_BUFFER_SIZE  65536

#endif /* SIGMA_CLOUD_PROFILE_H */
