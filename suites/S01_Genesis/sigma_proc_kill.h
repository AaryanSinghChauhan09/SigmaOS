// SigmaOS — sigma-proc-kill: Process Termination & Reaping
// Modularised from: SovereignProcessManager.c
// Single responsibility: terminate, reap, and reclaim a process

#ifndef SIGMA_PROC_KILL_H
#define SIGMA_PROC_KILL_H

#include "../../include/sigma_proc_pcb.h"

typedef enum SigmaKillCode {
    KILL_OK        = 0,
    KILL_NOT_FOUND = -1,
    KILL_DENIED    = -2,
    KILL_ALREADY   = -3
} SigmaKillCode;

// Terminate a process by PID
static inline SigmaKillCode proc_kill(SigmaPCBTable* table, unsigned long pid,
                                       unsigned long caller_pid) {
    SigmaPCB* target = pcb_find(table, pid);
    if (!target)                        return KILL_NOT_FOUND;
    if (target->state == SIGMA_PROC_ZOMBIE) return KILL_ALREADY;
    // PID 1 (init) cannot be killed
    if (target->pid == 1 && caller_pid != 0) return KILL_DENIED;

    target->state = SIGMA_PROC_ZOMBIE;
    return KILL_OK;
}

// Reap zombie processes — reclaim their PCB slots
static inline unsigned int proc_reap(SigmaPCBTable* table) {
    unsigned int reaped = 0;
    for (unsigned int i = 0; i < table->count; i++) {
        if (table->entries[i].state == SIGMA_PROC_ZOMBIE) {
            // Reset PCB slot for re-use
            SigmaPCB* p = &table->entries[i];
            p->pid = p->cr3 = p->rsp = p->rip = 0;
            p->cpu_cycles = 0;
            p->state = SIGMA_PROC_READY;
            // Compact: swap with last
            table->entries[i] = table->entries[--table->count];
            reaped++;
            i--; // re-check swapped entry
        }
    }
    return reaped;
}

#endif /* SIGMA_PROC_KILL_H */
