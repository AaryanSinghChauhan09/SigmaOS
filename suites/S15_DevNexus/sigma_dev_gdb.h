// SigmaOS — sigma-dev-gdb: Native Debug Stub (GDB/LLDB-inspired)
// Inspired by: GDB remote serial protocol, LLDB stub, OpenOCD
// Module: sigma-dev-gdb
// USP: No ptrace, no OS debug API — uses x86 hardware breakpoint registers
// Implements software breakpoints via INT3 (0xCC) injection + trap handler

#ifndef SIGMA_DEV_GDB_H
#define SIGMA_DEV_GDB_H

#define SIGMA_DBG_MAX_BPTS   16
#define SIGMA_DBG_INT3       0xCC

typedef enum SigmaDbgBptState {
    DBG_BPT_FREE    = 0,
    DBG_BPT_ACTIVE  = 1,
    DBG_BPT_PENDING = 2
} SigmaDbgBptState;

typedef struct SigmaDebugBreakpoint {
    unsigned long    addr;
    unsigned char    original_byte;  // saved before INT3 injection
    SigmaDbgBptState state;
    unsigned long    hit_count;
} SigmaDebugBreakpoint;

typedef struct SigmaDebugStub {
    SigmaDebugBreakpoint bpts[SIGMA_DBG_MAX_BPTS];
    unsigned int         bpt_count;
    unsigned char        attached;
    unsigned long        last_trap_addr;
} SigmaDebugStub;

static inline void dbg_init(SigmaDebugStub* d) {
    d->bpt_count      = 0;
    d->attached       = 0;
    d->last_trap_addr = 0;
    for (int i = 0; i < SIGMA_DBG_MAX_BPTS; i++)
        d->bpts[i].state = DBG_BPT_FREE;
}

// Set a software breakpoint at addr (write INT3, save original)
static inline int dbg_set_breakpoint(SigmaDebugStub* d, unsigned long addr) {
    if (d->bpt_count >= SIGMA_DBG_MAX_BPTS) return -1;
    unsigned char* p = (unsigned char*)addr;
    SigmaDebugBreakpoint* b = &d->bpts[d->bpt_count++];
    b->addr          = addr;
    b->original_byte = *p;
    b->hit_count     = 0;
    b->state         = DBG_BPT_ACTIVE;
    // Inject INT3 via inline ASM store
#if defined(__x86_64__) || defined(__i386__)
    __asm__ __volatile__(
        "movb %1, (%0)\n\t"
        :
        : "r"(p), "r"((unsigned char)SIGMA_DBG_INT3)
        : "memory"
    );
#endif
    return (int)(d->bpt_count - 1);
}

// Remove a breakpoint — restore original byte
static inline int dbg_clear_breakpoint(SigmaDebugStub* d, unsigned int bpt_id) {
    if (bpt_id >= d->bpt_count) return -1;
    SigmaDebugBreakpoint* b = &d->bpts[bpt_id];
    if (b->state == DBG_BPT_FREE) return -2;
    unsigned char* p = (unsigned char*)b->addr;
    unsigned char orig = b->original_byte;
#if defined(__x86_64__) || defined(__i386__)
    __asm__ __volatile__(
        "movb %1, (%0)\n\t"
        :
        : "r"(p), "r"(orig)
        : "memory"
    );
#endif
    b->state = DBG_BPT_FREE;
    return 0;
}

// Called from INT3 trap handler
static inline void dbg_on_trap(SigmaDebugStub* d, unsigned long trap_addr) {
    d->last_trap_addr = trap_addr;
    for (unsigned int i = 0; i < d->bpt_count; i++) {
        if (d->bpts[i].addr == trap_addr && d->bpts[i].state == DBG_BPT_ACTIVE) {
            d->bpts[i].hit_count++;
            return;
        }
    }
}

// Read a CPU register via inline ASM (x86_64: RIP, RSP, RBP)
static inline unsigned long dbg_read_rsp(void) {
#if defined(__x86_64__)
    unsigned long rsp;
    __asm__ __volatile__("mov %%rsp, %0" : "=r"(rsp));
    return rsp;
#else
    return 0;
#endif
}

static inline unsigned long dbg_read_rbp(void) {
#if defined(__x86_64__)
    unsigned long rbp;
    __asm__ __volatile__("mov %%rbp, %0" : "=r"(rbp));
    return rbp;
#else
    return 0;
#endif
}

#endif /* SIGMA_DEV_GDB_H */
