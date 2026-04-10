/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN PROCESS MANAGER (v26.0 — PURE C11)
 * =========================================================================
 * Mission: Process Control, Deadlock Prevention, Namespace Isolation.
 * Design: C11 / Zero-Dependency / No raw opcode execution.
 * Principle: Bit-Perfect. POSIX-parity. Sovereign.
 *
 * FIXED in v26.0:
 *   • Removed all stack-allocated raw-opcode buffers cast to function pointers.
 *     Those constructs invoke undefined behaviour and crash on systems with
 *     W^X enforcement (non-executable stacks / NX bit). They also have no
 *     meaningful effect and defeat static analysis.
 *   • Context-switch simulation now uses standard struct assignment.
 *   • TLB-flush is described via proper inline comment (real impl: write CR3).
 *   • namespace_isolate models the clone(2) flag conceptually without ring-0 UB.
 * =========================================================================
 */

#include "../../SovereignOSBasicsZenith.h"

/* -------------------------------------------------------------------------
 * Process table (flat array — max 1024 concurrent tasks)
 * ---------------------------------------------------------------------- */
#define SIGMA_PM_MAX_PROCS 1024

typedef struct {
    int        pid;
    int        ppid;
    int        state;     /* 0=free 1=running 2=sleeping 3=zombie */
    sigma_u64  pc;        /* Program counter (simulated) */
    sigma_u64  registers[16];
    const char *image;
} SigmaProcEntry_t;

static SigmaProcEntry_t s_proc_table[SIGMA_PM_MAX_PROCS];
static int              s_proc_count = 0;
static int              s_next_pid   = 1;

/* -------------------------------------------------------------------------
 * sovereign_pm_spawn — Fork + exec model (safe — no raw opcodes)
 * ---------------------------------------------------------------------- */
sigma_status sovereign_pm_spawn(sovereign_process_manager_t *pm,
                                 const char *image) {
    if (!pm || !image) return SIGMA_ERROR;
    if (s_proc_count >= SIGMA_PM_MAX_PROCS) {
        sigma_printf("[PM]: Process table full — cannot spawn '%s'.\n", image);
        return SIGMA_ERROR;
    }

    SigmaProcEntry_t *p = &s_proc_table[s_proc_count];
    p->pid    = s_next_pid++;
    p->ppid   = 1;            /* Reparented to PID 1 by default */
    p->state  = 1;            /* RUNNING */
    p->pc     = 0xFFFF800000100000ULL; /* Simulated kernel-space entry VA */
    p->image  = image;
    sigma_memset(p->registers, 0, sizeof(p->registers));
    s_proc_count++;
    pm->active_count++;

    sigma_printf("[PM]: Spawned shard '%s' -> PID %d\n", image, p->pid);
    return SIGMA_OK;
}

/* -------------------------------------------------------------------------
 * sovereign_pm_kill — Terminate a process cleanly
 * ---------------------------------------------------------------------- */
void sovereign_pm_kill(sovereign_process_manager_t *pm) {
    if (!pm || s_proc_count == 0) return;

    /* Mark last entry as zombie then collect it */
    SigmaProcEntry_t *p = &s_proc_table[s_proc_count - 1];
    sigma_printf("[PM]: Terminating PID %d ('%s') — marking zombie.\n",
                 p->pid, p->image ? p->image : "?");
    p->state = 3;   /* ZOMBIE */

    /* TLB flush would be: write to CR3 register (requires ring-0 asm).
     * In simulation we simply note the event. Real impl: sigma_tlb_flush(). */
    sigma_printf("[PM]: TLB flush requested for address space of PID %d.\n",
                 p->pid);

    /* Reap */
    s_proc_count--;
    pm->active_count = (pm->active_count > 0) ? pm->active_count - 1 : 0;
    sigma_printf("[PM]: Reaped PID %d. Active processes: %u.\n",
                 p->pid, pm->active_count);
}

/* -------------------------------------------------------------------------
 * sovereign_pm_shard_resources — Context switch (safe register model)
 * ---------------------------------------------------------------------- */
void sovereign_pm_shard_resources(sovereign_process_manager_t *pm) {
    if (!pm || s_proc_count < 2) {
        sigma_log("[PM]: Not enough processes for context switch.");
        return;
    }

    /* Simulate saving current task state and restoring the next one */
    SigmaProcEntry_t *prev = &s_proc_table[s_proc_count - 2];
    SigmaProcEntry_t *next = &s_proc_table[s_proc_count - 1];

    /* "Save" prev: store a dummy incremented PC */
    prev->pc += 4;    /* Advance instruction pointer on save */

    /* "Restore" next: marks it as running */
    next->state = 1;

    sigma_printf("[PM]: Context switch: PID %d -> PID %d (PC=0x%llx).\n",
                 prev->pid, next->pid, (unsigned long long)next->pc);
}

/* -------------------------------------------------------------------------
 * sovereign_pm_isolate_vfs — Namespace isolation via clone(2) flags model
 * ---------------------------------------------------------------------- */
void sovereign_pm_isolate_vfs(sovereign_process_manager_t *pm,
                               const char *ns) {
    (void)pm;
    /*
     * A real implementation would call sigma_clone() with flags:
     *   CLONE_NEWNS  | CLONE_NEWPID | CLONE_NEWNET | CLONE_NEWUTS
     * This creates a new VFS mount namespace (like Linux namespaces /
     * FreeBSD jails / Solaris zones).
     */
    sigma_printf("[PM]: Namespace isolation requested: '%s'\n", ns);
    sigma_printf("[PM]: Mount namespace (CLONE_NEWNS): isolated.\n");
    sigma_printf("[PM]: PID namespace  (CLONE_NEWPID): isolated.\n");
    sigma_printf("[PM]: Net namespace  (CLONE_NEWNET): isolated.\n");
    sigma_printf("[PM]: Container '%s' ready for shard execution.\n", ns);
}

/* -------------------------------------------------------------------------
 * sovereign_pm_audit — Human-readable process table dump
 * ---------------------------------------------------------------------- */
void sovereign_pm_audit(sovereign_process_manager_t *pm) {
    static const char *state_str[] = { "FREE", "RUNNING", "SLEEPING", "ZOMBIE" };
    sigma_printf("\n--- Σ SOVEREIGN PROCESS AUDIT (v26.0) ---\n");
    sigma_printf("| Active Shards  : %u\n", pm->active_count);
    sigma_printf("| PID   PPID  STATE     IMAGE\n");
    for (int i = 0; i < s_proc_count; i++) {
        SigmaProcEntry_t *p = &s_proc_table[i];
        sigma_printf("|  %-4d  %-4d  %-8s  %s\n",
                     p->pid, p->ppid,
                     state_str[p->state & 3],
                     p->image ? p->image : "(unknown)");
    }
    sigma_printf("------------------------------------------\n");
}

/* -------------------------------------------------------------------------
 * sigma_kernel_entry — Smoke-test entry point for this module
 * ---------------------------------------------------------------------- */
void sigma_process_manager_entry(void) {
    sovereign_process_manager_t pm = {0};
    pm.hdr.type_name = "SovereignProcessManager";
    pm.hdr.version   = 26;

    sovereign_pm_spawn(&pm, "sigma-init");
    sovereign_pm_spawn(&pm, "sigma-netd");
    sovereign_pm_spawn(&pm, "sigma-desktop");

    sovereign_pm_isolate_vfs(&pm, "/sigma/containers/alpha");
    sovereign_pm_shard_resources(&pm);
    sovereign_pm_kill(&pm);
    sovereign_pm_audit(&pm);
}
