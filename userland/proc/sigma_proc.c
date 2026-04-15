/*
 * =========================================================================
 * S SIGMAOS userland/proc/sigma_proc.c
 * =========================================================================
 */

#include "sigma_proc.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"

static sigma_pcb_t s_procs[SIGMA_PROC_MAX];
static proc_u32    s_proc_count = 0;
static proc_u32    s_next_pid   = 2;  /* PID 1 = sigma_init */

static sigma_pcb_t *find_proc(proc_u32 pid) {
    for (proc_u32 i = 0; i < s_proc_count; i++)
        if (s_procs[i].pid == pid && s_procs[i].state != PROC_DEAD)
            return &s_procs[i];
    return PROC_NULL;
}

proc_i32 sigma_proc_spawn(const char *cmd, proc_u32 ppid,
                           sigma_sched_class_t sched, proc_i32 prio,
                           proc_u32 ns_flags)
{
    if (s_proc_count >= SIGMA_PROC_MAX) return PROC_ERR;

    sigma_pcb_t *p = &s_procs[s_proc_count++];
    sigma_memset(p, 0, sizeof(*p));
    sigma_strncpy(p->cmd, cmd, SIGMA_CMD_LEN - 1);
    p->pid       = s_next_pid++;
    p->ppid      = ppid;
    p->uid       = 1000;  /* default user */
    p->state     = PROC_RUNNING;
    p->sched     = sched;
    p->priority  = prio;
    p->ns_flags  = ns_flags;
    p->vm_rss_kb = 256;   /* base allocation */
    p->cpu_ticks = 0;

    sigma_printf("S [PROC] SPAWN pid=%-5u ppid=%-5u sched=%-2d prio=%-3d "
                 "ns=0x%02x cmd=%s\n",
                 p->pid, p->ppid, (int)p->sched, p->priority,
                 p->ns_flags, p->cmd);

    /* Namespace isolation — Linux unshare() gap */
    if (ns_flags & NS_PID)  sigma_printf("  ↳ PID namespace isolated\n");
    if (ns_flags & NS_NET)  sigma_printf("  ↳ NET namespace isolated\n");
    if (ns_flags & NS_MNT)  sigma_printf("  ↳ MNT namespace isolated\n");
    if (ns_flags & NS_USER) sigma_printf("  ↳ USER namespace isolated\n");

    return (proc_i32)p->pid;
}

void sigma_proc_kill(proc_u32 pid, proc_i32 signal) {
    sigma_pcb_t *p = find_proc(pid);
    if (!p) { sigma_printf("S [PROC] ERROR: pid %u not found\n", pid); return; }

    sigma_printf("S [PROC] SIGNAL: pid=%u sig=%d\n", pid, signal);

    if (signal == 9 /* SIGKILL */ || signal == 15 /* SIGTERM */) {
        p->state = PROC_ZOMBIE;
        sigma_printf("S [PROC] ZOMBIE: pid=%u awaiting reap\n", pid);
    } else if (signal == 19 /* SIGSTOP */) {
        p->state = PROC_STOPPED;
    } else if (signal == 18 /* SIGCONT */) {
        if (p->state == PROC_STOPPED) p->state = PROC_RUNNING;
    }
}

void sigma_proc_reap(proc_u32 pid) {
    sigma_pcb_t *p = find_proc(pid);
    if (!p || p->state != PROC_ZOMBIE) return;
    sigma_printf("S [PROC] REAP: pid=%u (%s) cpu_ticks=%llu\n",
                 pid, p->cmd, (unsigned long long)p->cpu_ticks);
    p->state = PROC_DEAD;
}

sigma_pcb_t *sigma_proc_get(proc_u32 pid) {
    return find_proc(pid);
}

void sigma_proc_set_sched(proc_u32 pid, sigma_sched_class_t cls, proc_i32 prio) {
    sigma_pcb_t *p = find_proc(pid);
    if (!p) return;
    p->sched    = cls;
    p->priority = prio;
    sigma_printf("S [PROC] SCHED: pid=%u cls=%d prio=%d\n", pid, (int)cls, prio);
}

void sigma_proc_enable_seccomp(proc_u32 pid) {
    sigma_pcb_t *p = find_proc(pid);
    if (!p) return;
    p->seccomp_on = PROC_TRUE;
    sigma_printf("S [PROC] SECCOMP: pid=%u filter active\n", pid);
}

void sigma_proc_top(void) {
    static const char *state_str[] = {
        "RUN","SLEEP","STOP","ZOMB","DEAD"
    };
    static const char *sched_str[] = {"NORM","RT","IDLE","BATCH"};

    sigma_printf("\nS SIGMAOS PROCESS TABLE\n");
    sigma_printf("%-6s %-6s %-4s %-6s %-5s %-8s %-8s %s\n",
                 "PID","PPID","ST","SCHED","PRI","RSS_KB","CPU","CMD");
    for (proc_u32 i = 0; i < s_proc_count; i++) {
        sigma_pcb_t *p = &s_procs[i];
        if (p->state == PROC_DEAD) continue;
        sigma_printf("  %-4u %-6u %-4s %-6s %-5d %-8llu %-8llu %s\n",
                     p->pid, p->ppid,
                     state_str[p->state],
                     sched_str[p->sched],
                     p->priority,
                     (unsigned long long)p->vm_rss_kb,
                     (unsigned long long)p->cpu_ticks,
                     p->cmd);
    }
}
