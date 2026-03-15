/**
 * @file sigma_kernel_core.h
 * @brief Zero-latency kernel scheduling headers for SigmaOS.
 * 
 * Optimized for x86_64/ARM64 hybrid architectures.
 */

#ifndef SIGMA_KERNEL_CORE_H
#define SIGMA_KERNEL_CORE_H

#include <stdint.h>
#include <stdbool.h>

/**
 * Atomic context switch function.
 * Written in highly optimized assembly for zero-overhead switching.
 */
extern void sigma_context_switch(uint64_t next_task_rsp);

/**
 * Hybrid Predictive Scheduler entry point.
 */
int32_t sigma_schedule_next(uint32_t current_id, uint8_t affinity_mask);

#endif // SIGMA_KERNEL_CORE_H
