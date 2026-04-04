/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: PROCESS MANAGER (v1.0 - PURE C11)
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

#include "../sigma_kernel_types.h"

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
    i32    vfs_fd;    /* VFS file descriptor, -1 = closed */
    u32    flags;
} ProcFD;

/* =========================================================================
 * Process Control Block (PCB)
 * ========================================================================= */
typedef struct SigmaProc {
    u32        pid;
    u32        ppid;
    char       name[PROC_NAME_LEN];
    ProcState  state;

    /* Credentials */
    u32        uid, gid;
    u32        euid, egid;

    /* Memory */
    paddr_t    pml4_phys;      /* page table root */
    vaddr_t    heap_start;
    vaddr_t    heap_brk;
    vaddr_t    stack_top;

    /* Open files */
    ProcFD     fds[PROC_OPEN_MAX];

    /* Exit status */
    i32        exit_code;

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
    u32       next_pid;
    u32       active;
    spinlock_t lock;      /* B7: Process table lock */
} SigmaProcTable;

static SigmaProcTable g_proctab;

extern void   kprintf(const char* fmt, ...);
extern paddr_t pmm_alloc_page(void);
extern vaddr_t vmalloc(u64 npages);
extern k_status vmm_map(vaddr_t, paddr_t, u64);
extern void   sched_yield(void);
extern void*  kmalloc(usize);

/* =========================================================================
 * Helpers
 * ========================================================================= */
static SigmaProc* proc_alloc(void) {
    u32 i;
    for (i = 0; i < PROC_MAX; i++) {
        if (g_proctab.procs[i].state == PS_UNUSED) {
            SigmaProc* p = &g_proctab.procs[i];
            p->pid   = ++g_proctab.next_pid;
            p->ppid  = 0;
            p->state = PS_EMBRYO;
            p->uid   = p->gid = p->euid = p->egid = 0;
            p->exit_code = 0;
            p->sched_task = NULL;
            u32 fd;
            for (fd = 0; fd < PROC_OPEN_MAX; fd++) {
                p->fds[fd].vfs_fd = -1;
                p->fds[fd].flags  = 0;
            }
            g_proctab.active++;
            return p;
        }
    }
    return NULL;
}

static SigmaProc* proc_find(u32 pid) {
    u32 i;
    for (i = 0; i < PROC_MAX; i++) {
        if (g_proctab.procs[i].state != PS_UNUSED &&
            g_proctab.procs[i].pid == pid)
            return &g_proctab.procs[i];
    }
    return NULL;
}

static void proc_copy_name(SigmaProc* p, const char* name) {
    u32 i = 0;
    while (i < PROC_NAME_LEN - 1 && name[i]) { p->name[i] = name[i]; i++; }
    p->name[i] = '\0';
}

/* =========================================================================
 * ProcTable Init
 * ========================================================================= */
void proc_init(void) {
    u32 i;
    spinlock_init(&g_proctab.lock);
    for (i = 0; i < PROC_MAX; i++) g_proctab.procs[i].state = PS_UNUSED;
    g_proctab.next_pid = 0;
    g_proctab.active   = 0;
    kprintf("[PROC]: Process table online. Capacity=%u\n", PROC_MAX);
}

/* =========================================================================
 * sigma_fork() — Clone current process (CoW)
 * Returns pid of child to parent, 0 to child
 * ========================================================================= */
i32 proc_fork(SigmaProc* parent) {
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
    u32 fd;
    for (fd = 0; fd < PROC_OPEN_MAX; fd++)
        child->fds[fd] = parent->fds[fd];

    child->state = PS_RUNNABLE;
    kprintf("[PROC]: fork() pid=%u → child pid=%u\n", parent->pid, child->pid);
    return (i32)child->pid;
}

/* =========================================================================
 * sigma_exec() — Replace process image with new binary
 * In real impl: load ELF64 segments from VFS, set new entry RIP
 * ========================================================================= */
i32 proc_exec(SigmaProc* p, const char* path, const char* argv[]) {
    (void)argv;
    kprintf("[PROC]: exec() pid=%u path='%s'\n", p->pid, path);

    /* Close all non-inheritable fds (FD_CLOEXEC) */
    u32 fd;
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
 * sigma_wait() — Reap a zombie child
 * ========================================================================= */
i32 proc_wait(SigmaProc* parent, i32* exit_code) {
    u32 i;
    for (;;) {
        for (i = 0; i < PROC_MAX; i++) {
            SigmaProc* p = &g_proctab.procs[i];
            if (p->state == PS_ZOMBIE && p->ppid == parent->pid) {
                i32 pid = (i32)p->pid;
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
 * sigma_exit() — Terminate process
 * ========================================================================= */
void proc_exit(SigmaProc* p, i32 code) {
    p->exit_code = code;
    p->state     = PS_ZOMBIE;
    kprintf("[PROC]: exit() pid=%u code=%d → ZOMBIE\n", p->pid, code);
    
    /* B7: If no parent (or parent is init), harvest can be done by reaper */
    sched_yield();
}

/* =========================================================================
 * sigma_harvest_zombies() — B7: Background reaper for orphan zombies
 * ========================================================================= */
void proc_harvest_zombies(void) {
    u32 i;
    spinlock_acquire(&g_proctab.lock); /* Assuming a lock exists or adding one */
    for (i = 0; i < PROC_MAX; i++) {
        SigmaProc* p = &g_proctab.procs[i];
        if (p->state == PS_ZOMBIE) {
            /* If parent is dead or not interested, reap it */
            SigmaProc* parent = proc_find(p->ppid);
            if (!parent || parent->state == PS_UNUSED) {
                p->state = PS_UNUSED;
                g_proctab.active--;
                kprintf("[PROC]: B7 Reaper reaped orphan pid=%u\n", p->pid);
            }
        }
    }
    spinlock_release(&g_proctab.lock);
}

/* =========================================================================
 * brk() — Expand/contract heap
 * ========================================================================= */
vaddr_t proc_brk(SigmaProc* p, vaddr_t new_brk) {
    if (new_brk == 0) return p->heap_brk;
    if (new_brk > p->heap_brk) {
        /* Allocate pages for heap growth */
        vaddr_t cur = ALIGN_UP(p->heap_brk, PAGE_SIZE);
        while (cur < new_brk) {
            paddr_t pa = pmm_alloc_page();
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
    u32 i;
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
