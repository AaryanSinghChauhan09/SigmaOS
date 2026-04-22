/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S10_Registry/shards/sigma_container.h
 * =========================================================================
 * Sovereign Container Runtime — gap-closes:
 *   Linux  : LXC/Docker namespaces, cgroups v2, seccomp-BPF, overlayfs
 *   macOS  : Virtualization.framework, App Sandbox (per-app containers)
 *   Windows: WSL2/HCS, Windows Containers (WCOW), Hyper-V isolation
 *   Android: Isolated processes, SELinux domains per-app
 *   WASM   : WASI runtime isolation model
 * =========================================================================
 */

#ifndef SIGMA_CONTAINER_H
#define SIGMA_CONTAINER_H

typedef unsigned long long ct_u64;
typedef unsigned int       ct_u32;
typedef signed   int       ct_i32;
typedef unsigned char      ct_bool;
#define CT_TRUE  ((ct_bool)1)
#define CT_FALSE ((ct_bool)0)
#define CT_NULL  ((void*)0)
#define CT_OK    ((ct_i32) 0)
#define CT_ERR   ((ct_i32)-1)

/* ── Isolation level ─────────────────────────────────────────────────────── */
typedef enum {
    ISOLATE_NONE      = 0,  /* no isolation — bare process               */
    ISOLATE_PROCESS   = 1,  /* PID + IPC namespace only (Android model)  */
    ISOLATE_CONTAINER = 2,  /* full: PID+NET+MNT+UTS+IPC (Docker model)  */
    ISOLATE_VM_LITE   = 3,  /* user-mode kernel (gVisor/microVM)         */
    ISOLATE_WASM      = 4   /* WASI capability model                     */
} sigma_isolation_t;

/* ── Container state ─────────────────────────────────────────────────────── */
typedef enum {
    CT_CREATED   = 0,
    CT_RUNNING   = 1,
    CT_PAUSED    = 2,
    CT_STOPPED   = 3,
    CT_DEAD      = 4
} sigma_ct_state_t;

/* ── Resource limits (cgroup v2 parity) ─────────────────────────────────── */
typedef struct {
    ct_u64 cpu_quota_us;     /* cpu.max: quota per period              */
    ct_u64 cpu_period_us;
    ct_u64 mem_limit_kb;     /* memory.max                             */
    ct_u64 mem_swap_kb;      /* memory.swap.max                        */
    ct_u64 pids_max;         /* pids.max                               */
    ct_u64 net_rx_bps;       /* net_cls bandwidth limit                */
    ct_u64 net_tx_bps;
    ct_u64 io_rbps;          /* blkio throttle                         */
    ct_u64 io_wbps;
} sigma_ct_limits_t;

#define CT_NAME_LEN  64
#define CT_IMG_LEN  128
#define CT_MAX       64

/* ── Container descriptor ───────────────────────────────────────────────── */
typedef struct {
    char               id[CT_NAME_LEN];   /* 12-char hex like Docker    */
    char               name[CT_NAME_LEN];
    char               image[CT_IMG_LEN];
    sigma_isolation_t  isolation;
    sigma_ct_state_t   state;
    sigma_ct_limits_t  limits;
    ct_u32             root_pid;         /* init process inside CT      */
    ct_u32             ns_flags;         /* namespace bitmask           */
    ct_u64             created_ns;       /* creation timestamp          */
    ct_u64             cpu_used_us;
    ct_u64             mem_used_kb;
    ct_bool            readonly_rootfs;  /* overlayfs upper=tmpfs       */
    ct_bool            network_disabled;
} sigma_container_t;

/* ── Public API ─────────────────────────────────────────────────────────── */
void     sigma_ct_init(void);
ct_i32   sigma_ct_create(const char *name, const char *image,
                          sigma_isolation_t level,
                          sigma_ct_limits_t *limits);
ct_i32   sigma_ct_start(const char *name);
ct_i32   sigma_ct_pause(const char *name);
ct_i32   sigma_ct_resume(const char *name);
ct_i32   sigma_ct_stop(const char *name);
ct_i32   sigma_ct_destroy(const char *name);
void     sigma_ct_exec(const char *name, const char *cmd);
void     sigma_ct_stats(const char *name);
void     sigma_ct_ps(void);            /* docker ps equivalent          */

#endif /* SIGMA_CONTAINER_H */
