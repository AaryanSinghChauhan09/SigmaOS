/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: PROCESS CHECKPOINTING SHARD (v1.0)
 * =============================================================================
 * Principles: Shard State Persistence & Fault-Tolerant Execution.
 * =============================================================================
 */
#include "../../include/sigma_kernel_types.h"

typedef struct Checkpoint {
    u64     task_id;
    u64     rsp;
    u64     rip;
    u8      fpu_state[512];
    bool_t  active;
} checkpoint_t;

#define MAX_CHECKPOINTS 128
static checkpoint_t checkpoint_store[MAX_CHECKPOINTS];
static u32 checkpoint_count = 0;

/* Capture the current state of a sovereign task */
void checkpoint_save(u64 task_id, u64 rsp, u64 rip) {
    if (checkpoint_count < MAX_CHECKPOINTS) {
        checkpoint_t* cp = &checkpoint_store[checkpoint_count++];
        cp->task_id = task_id;
        cp->rsp = rsp;
        cp->rip = rip;
        cp->active = TRUE;
        
        kprintf("Σ [CHECKPOINT]: Task %d persisted at RIP 0x%x\n", task_id, rip);
    }
}

/* Restore task from a persisted checkpoint */
void checkpoint_restore(u64 task_id, u64* rsp, u64* rip) {
    for (u32 i = 0; i < checkpoint_count; i++) {
        if (checkpoint_store[i].task_id == task_id && checkpoint_store[i].active) {
            *rsp = checkpoint_store[i].rsp;
            *rip = checkpoint_store[i].rip;
            return;
        }
    }
}
