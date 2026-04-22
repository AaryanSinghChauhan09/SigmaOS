/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN KERNEL BASICS (v94.0 ZENITH SUPREME)
 * =========================================================================
 * Mission: Low-level definitions for PCB, Memory, and Resource Management.
 * Standard: Pure C11 (ISO/IEC 9899:2011)
 * =========================================================================
 */

#ifndef SIGMA_BASICS_H
#define SIGMA_BASICS_H

#include "../libc/sigma_types.h"

/* --- PROCESS CONTROL BLOCK (PCB) & STATES --- */
typedef enum {
    SIGMA_PROC_READY = 0,
    SIGMA_PROC_RUNNING,
    SIGMA_PROC_WAITING,
    SIGMA_PROC_TERMINATED
} sigma_proc_state_t;

typedef struct sigma_pcb {
    sigma_u64 pid;
    sigma_proc_state_t state;
    sigma_u64 pc;
    sigma_u64 registers[16];
    sigma_u64 cr3;
    sigma_u64 rsp;
} sigma_pcb_t;

/* --- RESOURCE MANAGEMENT (BANKER'S ALGORITHM SHARD) --- */
typedef struct sigma_resource_manager {
    int max[5][3];
    int allocation[5][3];
    int available[3];
} sigma_resource_manager_t;

sigma_bool sigma_is_in_safe_state(const sigma_resource_manager_t* rm);

/* --- MEMORY MANAGEMENT (SHARDED PAGING) --- */
void sigma_handle_paging(sigma_u64 logical_addr);
void sigma_handle_segmentation(sigma_u64 segment_id, sigma_u64 offset);
void sigma_handle_page_fault(sigma_u64 faulting_page);

#endif /* SIGMA_BASICS_H */
