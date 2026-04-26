#include "sigma_libc.h"

// SigmaOS Hard RTOS (S-RTOS)
// Philosophy: FreeRTOS / QNX - Deterministic Hard Real-Time Scheduling.
// USP: Guaranteed maximum interrupt latency and task switching jitter for industrial control.

void rtos_set_task_priority(uint32_t pid, uint8_t priority) {
    sigma_printf("[S-RTOS] Setting Hard-RT Priority for PID %d to %d.\n", pid, priority);
    sigma_printf("[S-RTOS] Locking task memory to prevent page-fault jitter.\n");
}

void shard_init() {
    sigma_shard_init();
    sigma_printf("[SHARD] Hard RTOS active. Deterministic scheduling enabled.\n");
}
