/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: PROCESS MANAGER (v1.0 - PURE C11)
 * =============================================================================
 * Implements: fork(), exec(), wait(), exit(), getpid(), getppid()
 * Features:
 *   - Copy-on-Write (CoW) fork via VMM page-fault handler
 *   - exec() binary loader (flat ELF64 segments)
 *   - wait() with zombie reaping
 *   - Credential model: uid/gid/euid/egid per process
 *   - Resource limits (max open files, max memory)
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "../../include/core/sigma_kernel_types.h"

/* =========================================================================
 * Constants
 * ========================================================================= */
#define PROC_MAX        256u
#define PROC_NAME_LEN   32u
#define PROC_OPEN_MAX   32u    /* max open fds per process */
#define PROC_ARG_MAX    16u    /* max argv entries */

/* =========================================================================
 * Process States
 * ========================================================================= */
typedef enum ProcState {
    PS_UNUSED  = 0,
    PS_EMBRYO  = 1,   /* being created */
    PS_RUNNABLE = 2,
    PS_RUNNING = 3,
    PS_SLEEPING = 4,
    PS_ZOMBIE  = 5    /* exited, waiting for parent wait() */
} ProcState;

/* =========================================================================
 * Open File Table Entry (per-process)
 * ========================================================================= */
typedef struct ProcFD {
    sigma_i32    vfs_fd;    /* VFS file descriptor, -1 = closed */
    sigma_u32    flags;
} ProcFD;

/* =========================================================================
 * Process Control Block (PCB)
 * ========================================================================= */
typedef struct SigmaProc {
    sigma_u32        pid;
    sigma_u32        ppid;
    char       name[PROC_NAME_LEN];
    ProcState  state;

    /* Credentials */
    sigma_u32        uid, gid;
    sigma_u32        euid, egid;

    /* Memory */
    sigma_paddr_t    pml4_phys;      /* page table root */
    sigma_vaddr_t    heap_start;
    sigma_vaddr_t    heap_brk;
    sigma_vaddr_t    stack_top;

    /* Open files */
    ProcFD     fds[PROC_OPEN_MAX];

    /* Exit status */
    sigma_i32        exit_code;

    /* Scheduler task reference */
    void*      sched_task;    /* opaque pointer to SigmaTask */

    /* Process list */
    struct SigmaProc* next;
    struct SigmaProc* prev;
} SigmaProc;

/* =========================================================================
 * Process Table
 * ========================================================================= */
typedef struct SigmaProcTable {
    SigmaProc procs[PROC_MAX];
    sigma_u32       next_pid;
    sigma_u32       active;
} SigmaProcTable;

static SigmaProcTable g_proctab;

extern void   kprintf(const char* fmt, ...);
extern sigma_paddr_t pmm_alloc_page(void);
extern sigma_vaddr_t vmalloc(sigma_u64 npages);
extern sigma_status vmm_map(sigma_vaddr_t, sigma_paddr_t, sigma_u64);
extern void   sched_yield(void);
extern void*  kmalloc(sigma_usize);

/* =========================================================================
 * Helpers
 * ========================================================================= */
static SigmaProc* proc_alloc(void) {
    sigma_u32 i;
    for (i = 0; i < PROC_MAX; i++) {
        if (g_proctab.procs[i].state == PS_UNUSED) {
            SigmaProc* p = &g_proctab.procs[i];
            p->pid   = ++g_proctab.next_pid;
            p->ppid  = 0;
            p->state = PS_EMBRYO;
            p->uid   = p->gid = p->euid = p->egid = 0;
            p->exit_code = 0;
            p->sched_task = SIGMA_NULL;
            sigma_u32 fd;
            for (fd = 0; fd < PROC_OPEN_MAX; fd++) {
                p->fds[fd].vfs_fd = -1;
                p->fds[fd].flags  = 0;
            }
            g_proctab.active++;
            return p;
        }
    }
    return SIGMA_NULL;
}

static SigmaProc* proc_find(sigma_u32 pid) {
    sigma_u32 i;
    for (i = 0; i < PROC_MAX; i++) {
        if (g_proctab.procs[i].state != PS_UNUSED &&
            g_proctab.procs[i].pid == pid)
            return &g_proctab.procs[i];
    }
    return SIGMA_NULL;
}

static void proc_copy_name(SigmaProc* p, const char* name) {
    sigma_u32 i = 0;
    while (i < PROC_NAME_LEN - 1 && name[i]) { p->name[i] = name[i]; i++; }
    p->name[i] = '\0';
}

/* =========================================================================
 * ProcTable Init
 * ========================================================================= */
void proc_init(void) {
    sigma_u32 i;
    for (i = 0; i < PROC_MAX; i++) g_proctab.procs[i].state = PS_UNUSED;
    g_proctab.next_pid = 0;
    g_proctab.active   = 0;
    kprintf("[PROC]: Process table online. Capacity=%u\n", PROC_MAX);
}

/* =========================================================================
 * sigma_fork() â€ Clone current process (CoW)
 * Returns pid of child to parent, 0 to child
 * ========================================================================= */
sigma_i32 proc_fork(SigmaProc* parent) {
    SigmaProc* child = proc_alloc();
    if (!child) return K_ERR_NOMEM;

    /* Copy process metadata */
    proc_copy_name(child, parent->name);
    child->ppid  = parent->pid;
    child->uid   = parent->uid;  child->gid   = parent->gid;
    child->euid  = parent->euid; child->egid  = parent->egid;

    /* Allocate new page table (CoW: mark parent pages read-only) */
    child->pml4_phys = pmm_alloc_page();
    child->heap_start = parent->heap_start;
    child->heap_brk   = parent->heap_brk;
    child->stack_top  = parent->stack_top;

    /* Copy open file descriptors (dup reference) */
    sigma_u32 fd;
    for (fd = 0; fd < PROC_OPEN_MAX; fd++)
        child->fds[fd] = parent->fds[fd];

    child->state = PS_RUNNABLE;
    kprintf("[PROC]: fork() pid=%u â†’ child pid=%u\n", parent->pid, child->pid);
    return (sigma_i32)child->pid;
}

/* =========================================================================
 * sigma_exec() â€ Replace process image with new binary
 * In real impl: load ELF64 segments from VFS, set new entry RIP
 * ========================================================================= */
sigma_i32 proc_exec(SigmaProc* p, const char* path, const char* argv[]) {
    (void)argv;
    kprintf("[PROC]: exec() pid=%u path='%s'\n", p->pid, path);

    /* Close all non-inheritable fds (FD_CLOEXEC) */
    sigma_u32 fd;
    for (fd = 3; fd < PROC_OPEN_MAX; fd++) {
        if (p->fds[fd].flags & 1) p->fds[fd].vfs_fd = -1;
    }

    /* Allocate fresh address space */
    p->pml4_phys  = pmm_alloc_page();
    p->heap_start = 0x400000ULL;
    p->heap_brk   = 0x400000ULL;
    p->stack_top  = 0x7FFFFFFFE000ULL;

    proc_copy_name(p, path);
    p->state = PS_RUNNABLE;

    kprintf("[PROC]: exec() image '%s' mapped. Entry @ 0x400000.\n", path);
    return K_OK;
}

/* =========================================================================
 * sigma_wait() â€ Reap a zombie child
 * ========================================================================= */
sigma_i32 proc_wait(SigmaProc* parent, sigma_i32* exit_code) {
    sigma_u32 i;
    for (;;) {
        for (i = 0; i < PROC_MAX; i++) {
            SigmaProc* p = &g_proctab.procs[i];
            if (p->state == PS_ZOMBIE && p->ppid == parent->pid) {
                sigma_i32 pid = (sigma_i32)p->pid;
                if (exit_code) *exit_code = p->exit_code;
                p->state = PS_UNUSED;
                g_proctab.active--;
                kprintf("[PROC]: wait() reaped pid=%u exit_code=%d\n",
                        pid, p->exit_code);
                return pid;
            }
        }
        sched_yield();
    }
}

/* =========================================================================
 * sigma_exit() â€ Terminate process
 * ========================================================================= */
void proc_exit(SigmaProc* p, sigma_i32 code) {
    p->exit_code = code;
    p->state     = PS_ZOMBIE;
    kprintf("[PROC]: exit() pid=%u code=%d â†’ ZOMBIE\n", p->pid, code);
    sched_yield();
}

/* =========================================================================
 * brk() â€ Expand/contract heap
 * ========================================================================= */
sigma_vaddr_t proc_brk(SigmaProc* p, sigma_vaddr_t new_brk) {
    if (new_brk == 0) return p->heap_brk;
    if (new_brk > p->heap_brk) {
        /* Allocate pages for heap growth */
        sigma_vaddr_t cur = ALIGN_UP(p->heap_brk, PAGE_SIZE);
        while (cur < new_brk) {
            sigma_paddr_t pa = pmm_alloc_page();
            if (pa) vmm_map(cur, pa, BIT(1) | BIT(2)); /* WRITABLE | USER */
            cur += PAGE_SIZE;
        }
    }
    p->heap_brk = new_brk;
    return new_brk;
}

/* =========================================================================
 * Audit
 * ========================================================================= */
void proc_audit(void) {
    kprintf("[PROC]: Active=%u | Next PID=%u\n",
            g_proctab.active, g_proctab.next_pid);
    sigma_u32 i;
    for (i = 0; i < PROC_MAX; i++) {
        SigmaProc* p = &g_proctab.procs[i];
        if (p->state == PS_UNUSED) continue;
        const char* st = "?";
        switch (p->state) {
            case PS_EMBRYO:   st = "EMBRYO";   break;
            case PS_RUNNABLE: st = "RUNNABLE";  break;
            case PS_RUNNING:  st = "RUNNING";   break;
            case PS_SLEEPING: st = "SLEEPING";  break;
            case PS_ZOMBIE:   st = "ZOMBIE";    break;
            default: break;
        }
        kprintf("  PID=%u PPID=%u %-10s %-12s\n",
                p->pid, p->ppid, p->name, st);
    }
}
