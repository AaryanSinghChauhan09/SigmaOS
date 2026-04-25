// SigmaOS — sigma-proc-pcb: Process Control Block
// Modularised from: SovereignProcessManager.c
// Single responsibility: define and manage the PCB struct only

#ifndef SIGMA_PROC_PCB_H
#define SIGMA_PROC_PCB_H

#define SIGMA_PCB_TABLE_SIZE 1024
#define SIGMA_PROC_READY     0
#define SIGMA_PROC_RUNNING   1
#define SIGMA_PROC_BLOCKED   2
#define SIGMA_PROC_ZOMBIE    3

typedef struct SigmaPCB {
    unsigned long pid;
    unsigned long cr3;          // page table root (physical)
    unsigned long rsp;          // saved stack pointer
    unsigned long rip;          // saved instruction pointer
    unsigned int  state;        // READY / RUNNING / BLOCKED / ZOMBIE
    unsigned int  priority;     // 0 = highest
    unsigned long cpu_cycles;   // total cycles consumed (RDTSC delta)
} SigmaPCB;

typedef struct SigmaPCBTable {
    SigmaPCB     entries[SIGMA_PCB_TABLE_SIZE];
    unsigned int count;
    unsigned int next_pid;
} SigmaPCBTable;

static inline void pcb_table_init(SigmaPCBTable* t) {
    t->count    = 0;
    t->next_pid = 1;
}

static inline SigmaPCB* pcb_alloc(SigmaPCBTable* t) {
    if (t->count >= SIGMA_PCB_TABLE_SIZE) return (void*)0;
    SigmaPCB* p      = &t->entries[t->count++];
    p->pid           = t->next_pid++;
    p->cr3 = p->rsp  = p->rip = 0;
    p->state         = SIGMA_PROC_READY;
    p->priority      = 128;
    p->cpu_cycles    = 0;
    return p;
}

static inline SigmaPCB* pcb_find(SigmaPCBTable* t, unsigned long pid) {
    for (unsigned int i = 0; i < t->count; i++)
        if (t->entries[i].pid == pid) return &t->entries[i];
    return (void*)0;
}

static inline void pcb_free(SigmaPCBTable* t, unsigned long pid) {
    SigmaPCB* p = pcb_find(t, pid);
    if (p) p->state = SIGMA_PROC_ZOMBIE;
}

#endif /* SIGMA_PROC_PCB_H */
