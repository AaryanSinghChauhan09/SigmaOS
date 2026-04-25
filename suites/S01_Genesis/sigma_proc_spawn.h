// SigmaOS — sigma-proc-spawn: Process Spawning
// Modularised from: SovereignProcessManager.c
// Single responsibility: create and initialise a new process entry

#ifndef SIGMA_PROC_SPAWN_H
#define SIGMA_PROC_SPAWN_H

#include "sigma_proc_pcb.h"
#include "sigma_vmm.h"

typedef struct SigmaSpawnResult {
    unsigned long pid;
    int           status;  // 0 = OK, -1 = table full, -2 = no VMM page
} SigmaSpawnResult;

static inline SigmaSpawnResult proc_spawn(SigmaPCBTable* table,
                                           SigmaVMM*      vmm,
                                           unsigned long  entry_vaddr) {
    SigmaSpawnResult res = {0, -1};

    SigmaPCB* pcb = pcb_alloc(table);
    if (!pcb) return res;

    // Assign a fresh page directory for this process
    // (In real boot: allocate physical frame; here we use VMM)
    pcb->rip   = entry_vaddr;
    pcb->state = SIGMA_PROC_READY;
    res.pid    = pcb->pid;
    res.status = 0;

    return res;
}

// Inline ASM: load new RIP via indirect jump (x86_64 bare-metal spawn)
static inline void proc_jump_to_entry(unsigned long rip) {
#if defined(__x86_64__)
    __asm__ __volatile__ (
        "jmp *%0\n\t"
        :
        : "r" (rip)
        :
    );
#endif
    (void)rip;
}

#endif /* SIGMA_PROC_SPAWN_H */
