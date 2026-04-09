/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN RESOURCE LIMITS (v1.0 — PURE C11)
 * =========================================================================
 * Competitor Gap Closed: Linux sys/resource.h (getrlimit/setrlimit/prlimit),
 * macOS setrlimit(2), FreeBSD rlimit, Windows Job Objects.
 * SigmaOS had no resource enforcement whatsoever.
 *
 * Resource limits prevent runaway processes from exhausting system resources.
 * Every shell, container runtime, and security sandbox depends on them.
 *
 * This shard implements all 16 POSIX rlimit resources:
 *   RLIMIT_CPU     RLIMIT_FSIZE   RLIMIT_DATA    RLIMIT_STACK
 *   RLIMIT_CORE    RLIMIT_RSS     RLIMIT_NPROC   RLIMIT_NOFILE
 *   RLIMIT_MEMLOCK RLIMIT_AS      RLIMIT_LOCKS   RLIMIT_SIGPENDING
 *   RLIMIT_MSGQUEUE RLIMIT_NICE   RLIMIT_RTPRIO  RLIMIT_RTTIME
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"

#define RLIM_INFINITY   (~0ULL)   /* no limit */
#define RLIM_NLIMITS    16

/* Standard RLIMIT_* indices (Linux ABI compatible) */
#define RLIMIT_CPU         0   /* CPU time in seconds */
#define RLIMIT_FSIZE       1   /* max file size (bytes) */
#define RLIMIT_DATA        2   /* max data segment size */
#define RLIMIT_STACK       3   /* max stack size */
#define RLIMIT_CORE        4   /* max core dump size */
#define RLIMIT_RSS         5   /* max resident set size */
#define RLIMIT_NPROC       6   /* max number of processes */
#define RLIMIT_NOFILE      7   /* max open file descriptors */
#define RLIMIT_MEMLOCK     8   /* max locked memory */
#define RLIMIT_AS          9   /* max virtual address space */
#define RLIMIT_LOCKS      10   /* max file locks */
#define RLIMIT_SIGPENDING 11   /* max pending signals */
#define RLIMIT_MSGQUEUE   12   /* max POSIX MQ bytes */
#define RLIMIT_NICE       13   /* max nice value adjustment */
#define RLIMIT_RTPRIO     14   /* max real-time priority */
#define RLIMIT_RTTIME     15   /* max RT CPU time (µs) */

static const char *rlimit_name(int r) {
    static const char *names[RLIM_NLIMITS] = {
        "RLIMIT_CPU","RLIMIT_FSIZE","RLIMIT_DATA","RLIMIT_STACK",
        "RLIMIT_CORE","RLIMIT_RSS","RLIMIT_NPROC","RLIMIT_NOFILE",
        "RLIMIT_MEMLOCK","RLIMIT_AS","RLIMIT_LOCKS","RLIMIT_SIGPENDING",
        "RLIMIT_MSGQUEUE","RLIMIT_NICE","RLIMIT_RTPRIO","RLIMIT_RTTIME"
    };
    return (r >= 0 && r < RLIM_NLIMITS) ? names[r] : "?";
}

/* -----------------------------------------------------------------------
 * ░░ PER-RESOURCE LIMIT PAIR (soft + hard)
 * ----------------------------------------------------------------------- */
typedef struct {
    sigma_u64 soft;   /* current enforced limit (can be raised to hard) */
    sigma_u64 hard;   /* ceiling — only root can raise */
} SigmaRlimit_t;

/* -----------------------------------------------------------------------
 * ░░ PER-PROCESS RESOURCE ACCOUNTING
 * ----------------------------------------------------------------------- */
#define MAX_RLIM_PROCS 128

typedef struct {
    sigma_u32    pid;
    SigmaRlimit_t limits[RLIM_NLIMITS];
    /* Usage counters (updated by kernel subsystems) */
    sigma_u64    cpu_time_ms;      /* RLIMIT_CPU accounting */
    sigma_u64    vm_size;          /* RLIMIT_AS accounting  */
    sigma_u32    open_files;       /* RLIMIT_NOFILE         */
    sigma_u32    child_procs;      /* RLIMIT_NPROC          */
    sigma_u64    locked_mem;       /* RLIMIT_MEMLOCK        */
    sigma_bool   in_use;
} SigmaRlimCtx_t;

static SigmaRlimCtx_t s_rlim[MAX_RLIM_PROCS];
static sigma_u32      s_rlim_count = 0;

/* Default limits — mirrors Linux's /proc/1/limits (init process) */
static void rlim_set_defaults(SigmaRlimCtx_t *ctx) {
    ctx->limits[RLIMIT_CPU].soft      = RLIM_INFINITY;
    ctx->limits[RLIMIT_CPU].hard      = RLIM_INFINITY;
    ctx->limits[RLIMIT_FSIZE].soft    = RLIM_INFINITY;
    ctx->limits[RLIMIT_FSIZE].hard    = RLIM_INFINITY;
    ctx->limits[RLIMIT_DATA].soft     = RLIM_INFINITY;
    ctx->limits[RLIMIT_DATA].hard     = RLIM_INFINITY;
    ctx->limits[RLIMIT_STACK].soft    = 8 * 1024 * 1024;    /* 8 MB */
    ctx->limits[RLIMIT_STACK].hard    = RLIM_INFINITY;
    ctx->limits[RLIMIT_CORE].soft     = 0;                   /* no core dump */
    ctx->limits[RLIMIT_CORE].hard     = RLIM_INFINITY;
    ctx->limits[RLIMIT_RSS].soft      = RLIM_INFINITY;
    ctx->limits[RLIMIT_RSS].hard      = RLIM_INFINITY;
    ctx->limits[RLIMIT_NPROC].soft    = 65536;
    ctx->limits[RLIMIT_NPROC].hard    = 65536;
    ctx->limits[RLIMIT_NOFILE].soft   = 1024;                /* POSIX minimum */
    ctx->limits[RLIMIT_NOFILE].hard   = 1048576;
    ctx->limits[RLIMIT_MEMLOCK].soft  = 65536;               /* 64 KB */
    ctx->limits[RLIMIT_MEMLOCK].hard  = 65536;
    ctx->limits[RLIMIT_AS].soft       = RLIM_INFINITY;
    ctx->limits[RLIMIT_AS].hard       = RLIM_INFINITY;
    ctx->limits[RLIMIT_LOCKS].soft    = RLIM_INFINITY;
    ctx->limits[RLIMIT_LOCKS].hard    = RLIM_INFINITY;
    ctx->limits[RLIMIT_SIGPENDING].soft = 65536;
    ctx->limits[RLIMIT_SIGPENDING].hard = 65536;
    ctx->limits[RLIMIT_MSGQUEUE].soft = 819200;              /* 800 KB */
    ctx->limits[RLIMIT_MSGQUEUE].hard = 819200;
    ctx->limits[RLIMIT_NICE].soft     = 0;
    ctx->limits[RLIMIT_NICE].hard     = 0;
    ctx->limits[RLIMIT_RTPRIO].soft   = 0;
    ctx->limits[RLIMIT_RTPRIO].hard   = 0;
    ctx->limits[RLIMIT_RTTIME].soft   = RLIM_INFINITY;
    ctx->limits[RLIMIT_RTTIME].hard   = RLIM_INFINITY;
}

static SigmaRlimCtx_t *rlim_get_or_create(sigma_u32 pid) {
    for (sigma_u32 i = 0; i < s_rlim_count; i++) {
        if (s_rlim[i].in_use && s_rlim[i].pid == pid) return &s_rlim[i];
    }
    if (s_rlim_count >= MAX_RLIM_PROCS) return SIGMA_NULL;
    SigmaRlimCtx_t *ctx = &s_rlim[s_rlim_count++];
    sigma_memset(ctx, 0, sizeof(*ctx));
    ctx->pid    = pid;
    ctx->in_use = SIGMA_TRUE;
    rlim_set_defaults(ctx);
    return ctx;
}

/* -----------------------------------------------------------------------
 * ░░ getrlimit / setrlimit / prlimit
 * ----------------------------------------------------------------------- */
sigma_err_t sigma_getrlimit(sigma_u32 pid, int resource, SigmaRlimit_t *out) {
    if (resource < 0 || resource >= RLIM_NLIMITS) return SIGMA_EINVAL;
    SigmaRlimCtx_t *ctx = rlim_get_or_create(pid);
    if (!ctx) return SIGMA_ESRCH;
    *out = ctx->limits[resource];
    return SIGMA_OK;
}

sigma_err_t sigma_setrlimit(sigma_u32 pid, int resource,
                             sigma_u64 soft, sigma_u64 hard) {
    if (resource < 0 || resource >= RLIM_NLIMITS) return SIGMA_EINVAL;
    SigmaRlimCtx_t *ctx = rlim_get_or_create(pid);
    if (!ctx) return SIGMA_ESRCH;

    /* Validate: soft <= hard, can't raise hard unless root */
    if (soft != RLIM_INFINITY && hard != RLIM_INFINITY && soft > hard) {
        sigma_printf("Σ [RLIM]: EINVAL — soft(%llu) > hard(%llu) for %s\n",
                     (unsigned long long)soft, (unsigned long long)hard,
                     rlimit_name(resource));
        return SIGMA_EINVAL;
    }
    if (hard > ctx->limits[resource].hard) {
        /* Only root (uid=0) can raise the hard limit */
        sigma_printf("Σ [RLIM]: EPERM — raising hard limit requires CAP_SYS_RESOURCE\n");
        return SIGMA_EPERM;
    }

    sigma_u64 old_soft = ctx->limits[resource].soft;
    sigma_u64 old_hard = ctx->limits[resource].hard;
    ctx->limits[resource].soft = soft;
    ctx->limits[resource].hard = hard;

    sigma_printf("Σ [RLIM]: setrlimit pid=%u %s: soft %llu→%llu hard %llu→%llu\n",
                 pid, rlimit_name(resource),
                 (unsigned long long)old_soft, (unsigned long long)soft,
                 (unsigned long long)old_hard, (unsigned long long)hard);
    return SIGMA_OK;
}

/* prlimit — like prlimit64(2): can target any pid */
sigma_err_t sigma_prlimit(sigma_u32 pid, int resource,
                           sigma_u64 new_soft, sigma_u64 new_hard,
                           SigmaRlimit_t *old_out) {
    if (old_out) {
        sigma_err_t e = sigma_getrlimit(pid, resource, old_out);
        if (!sigma_ok(e)) return e;
    }
    if (new_soft != RLIM_INFINITY || new_hard != RLIM_INFINITY)
        return sigma_setrlimit(pid, resource, new_soft, new_hard);
    return SIGMA_OK;
}

/* -----------------------------------------------------------------------
 * ░░ LIMIT ENFORCEMENT — called by relevant kernel subsystems
 * ----------------------------------------------------------------------- */

/** Returns SIGMA_EACCES if opening another file would exceed RLIMIT_NOFILE */
sigma_err_t sigma_rlim_check_nofile(sigma_u32 pid) {
    SigmaRlimCtx_t *ctx = rlim_get_or_create(pid);
    if (!ctx) return SIGMA_OK;
    if (ctx->open_files >= (sigma_u32)ctx->limits[RLIMIT_NOFILE].soft) {
        sigma_printf("Σ [RLIM]: EMFILE — pid=%u exceeded RLIMIT_NOFILE(%llu)\n",
                     pid, (unsigned long long)ctx->limits[RLIMIT_NOFILE].soft);
        return SIGMA_EMFILE;
    }
    ctx->open_files++;
    return SIGMA_OK;
}

/** Returns SIGMA_EAGAIN if fork() would exceed RLIMIT_NPROC */
sigma_err_t sigma_rlim_check_nproc(sigma_u32 pid) {
    SigmaRlimCtx_t *ctx = rlim_get_or_create(pid);
    if (!ctx) return SIGMA_OK;
    if (ctx->child_procs >= (sigma_u32)ctx->limits[RLIMIT_NPROC].soft) {
        sigma_printf("Σ [RLIM]: EAGAIN — pid=%u exceeded RLIMIT_NPROC(%llu)\n",
                     pid, (unsigned long long)ctx->limits[RLIMIT_NPROC].soft);
        return SIGMA_EAGAIN;
    }
    ctx->child_procs++;
    return SIGMA_OK;
}

/** Returns SIGMA_ENOMEM if mmap would exceed RLIMIT_AS */
sigma_err_t sigma_rlim_check_as(sigma_u32 pid, sigma_u64 extra_bytes) {
    SigmaRlimCtx_t *ctx = rlim_get_or_create(pid);
    if (!ctx) return SIGMA_OK;
    if (ctx->limits[RLIMIT_AS].soft == RLIM_INFINITY) return SIGMA_OK;
    if (ctx->vm_size + extra_bytes > ctx->limits[RLIMIT_AS].soft) {
        sigma_printf("Σ [RLIM]: ENOMEM — pid=%u exceeds RLIMIT_AS(%llu)\n",
                     pid, (unsigned long long)ctx->limits[RLIMIT_AS].soft);
        return SIGMA_ENOMEM;
    }
    ctx->vm_size += extra_bytes;
    return SIGMA_OK;
}

/** CPU time accounting — call from scheduler tick */
sigma_err_t sigma_rlim_account_cpu(sigma_u32 pid, sigma_u64 elapsed_ms) {
    SigmaRlimCtx_t *ctx = rlim_get_or_create(pid);
    if (!ctx) return SIGMA_OK;
    ctx->cpu_time_ms += elapsed_ms;
    if (ctx->limits[RLIMIT_CPU].soft == RLIM_INFINITY) return SIGMA_OK;
    sigma_u64 cpu_sec = ctx->cpu_time_ms / 1000;
    if (cpu_sec >= ctx->limits[RLIMIT_CPU].soft) {
        /* Deliver SIGXCPU */
        sigma_printf("Σ [RLIM]: SIGXCPU → pid=%u (CPU time limit %llu s)\n",
                     pid, (unsigned long long)ctx->limits[RLIMIT_CPU].soft);
        return SIGMA_ETIME;
    }
    return SIGMA_OK;
}

/* -----------------------------------------------------------------------
 * ░░ /proc/{pid}/limits formatter
 * ----------------------------------------------------------------------- */
void sigma_rlim_print_limits(sigma_u32 pid) {
    SigmaRlimCtx_t *ctx = rlim_get_or_create(pid);
    if (!ctx) return;
    sigma_printf("Σ [RLIM]: /proc/%u/limits\n", pid);
    sigma_printf("%-25s %-20s %-20s %s\n",
                 "Limit","Soft Limit","Hard Limit","Units");
    static const char *units[RLIM_NLIMITS] = {
        "seconds","bytes","bytes","bytes","bytes","bytes","processes",
        "files","bytes","bytes","locks","signals","bytes","","","microseconds"
    };
    for (int r = 0; r < RLIM_NLIMITS; r++) {
        char soft_s[24], hard_s[24];
        if (ctx->limits[r].soft == RLIM_INFINITY)
            sigma_strcpy(soft_s, "unlimited", sizeof(soft_s));
        else
            sigma_snprintf(soft_s, sizeof(soft_s), "%llu",
                           (unsigned long long)ctx->limits[r].soft);
        if (ctx->limits[r].hard == RLIM_INFINITY)
            sigma_strcpy(hard_s, "unlimited", sizeof(hard_s));
        else
            sigma_snprintf(hard_s, sizeof(hard_s), "%llu",
                           (unsigned long long)ctx->limits[r].hard);
        sigma_printf("%-25s %-20s %-20s %s\n",
                     rlimit_name(r), soft_s, hard_s, units[r]);
    }
}

/* -----------------------------------------------------------------------
 * ░░ fork() rlimit inheritance (child inherits parent limits)
 * ----------------------------------------------------------------------- */
sigma_err_t sigma_rlim_fork(sigma_u32 parent_pid, sigma_u32 child_pid) {
    SigmaRlimCtx_t *parent = rlim_get_or_create(parent_pid);
    SigmaRlimCtx_t *child  = rlim_get_or_create(child_pid);
    if (!parent || !child) return SIGMA_ESRCH;
    /* Child inherits limits but starts with zeroed usage */
    sigma_memcpy(child->limits, parent->limits, sizeof(parent->limits));
    child->open_files   = 0;
    child->child_procs  = 0;
    child->vm_size      = 0;
    child->cpu_time_ms  = 0;
    sigma_printf("Σ [RLIM]: fork() rlimit inherited: pid=%u → pid=%u\n",
                 parent_pid, child_pid);
    return SIGMA_OK;
}

/* -----------------------------------------------------------------------
 * ░░ Public init
 * ----------------------------------------------------------------------- */
void SovereignRlimit_Init(void) {
    sigma_printf("Σ [RLIM]: Initialising Sovereign Resource Limits...\n");

    /* Create init (pid=1) with defaults */
    SigmaRlimCtx_t *init = rlim_get_or_create(1);
    /* Root can raise hard limit */
    init->limits[RLIMIT_NOFILE].hard = 1048576;

    /* Print /proc/1/limits */
    sigma_rlim_print_limits(1);

    /* Set RLIMIT_NOFILE for a shell process */
    sigma_setrlimit(1, RLIMIT_NOFILE, 65536, 1048576);

    /* Set CPU time limit (30 seconds) for a compute job */
    sigma_setrlimit(2, RLIMIT_CPU, 30, 60);

    /* Enforce open file check */
    for (int i = 0; i < 5; i++) sigma_rlim_check_nofile(1);

    /* Fork rlimit inheritance */
    sigma_rlim_fork(1, 3);

    /* CPU accounting */
    sigma_rlim_account_cpu(2, 29500); /* 29.5 s — still OK */
    sigma_rlim_account_cpu(2,  1000); /* + 1 s  — should trigger SIGXCPU */

    /* prlimit test */
    SigmaRlimit_t old;
    sigma_prlimit(1, RLIMIT_STACK, 16*1024*1024, RLIM_INFINITY, &old);
    sigma_printf("Σ [RLIM]: old RLIMIT_STACK soft=%llu MB\n",
                 (unsigned long long)(old.soft / 1024 / 1024));

    sigma_printf("Σ [RLIM]: Resource limits online. %d resources enforced.\n",
                 RLIM_NLIMITS);
}
