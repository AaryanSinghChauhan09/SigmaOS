/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN JAIL — FREEBSD CONTAINER ISOLATION (v1.0 — C11)
 * =========================================================================
 * Absorbed USPs from: FreeBSD jail(8), jail(2), jls, jexec
 *   https://github.com/freebsd/freebsd-src/tree/main/usr.sbin/jail
 *
 * Features implemented:
 *   §1  Jail creation with independent hostname, IP list, root path
 *   §2  Per-jail resource limits (rlimit parity: CPU, memory, fds)
 *   §3  Jail network namespace (isolated loopback + vnet)
 *   §4  jls — jail list / jexec — execute command in jail
 *   §5  Hierarchical jails (child jails, super-powers delegation)
 *   §6  Capsicum capability mode (capability-mode enter, limited fd rights)
 *   §7  jail_set / jail_get syscall stubs
 *
 * Inspiration: FreeBSD Jails (1999, Poul-Henning Kamp), Capsicum (Watson 2010)
 * =========================================================================
 */

#ifndef SOVEREIGN_JAIL_H
#define SOVEREIGN_JAIL_H

#include "sigma_types.h"

/* -------------------------------------------------------------------------
 * Constants
 * ---------------------------------------------------------------------- */
#define SIGMA_JAIL_MAX          64
#define SIGMA_JAIL_NAME_MAX     64
#define SIGMA_JAIL_PATH_MAX    256
#define SIGMA_JAIL_HOST_MAX     64
#define SIGMA_JAIL_IP_MAX        8

/* -------------------------------------------------------------------------
 * Jail flags (mirrors FreeBSD jailparam flags)
 * ---------------------------------------------------------------------- */
#define JAIL_FLAG_ALLOW_RAWSOCK  (1u << 0)
#define JAIL_FLAG_ALLOW_MOUNT    (1u << 1)
#define JAIL_FLAG_ALLOW_CHFLAGS  (1u << 2)
#define JAIL_FLAG_VNET           (1u << 3)   /* Virtual network stack */
#define JAIL_FLAG_DYING          (1u << 4)
#define JAIL_FLAG_NOHOSTNAME     (1u << 5)

/* -------------------------------------------------------------------------
 * Resource Limits per Jail
 * ---------------------------------------------------------------------- */
typedef struct {
    sigma_u64 max_mem_bytes;    /* RSS cap (like ulimit -v) */
    sigma_u32 max_procs;        /* Max processes inside jail */
    sigma_u32 max_fds;          /* Max open file descriptors */
    sigma_u32 cpu_pct;          /* CPU % cap (0 = unlimited) */
} SigmaJailRlimit_t;

/* -------------------------------------------------------------------------
 * Capsicum capability rights (subset of FreeBSD cap_rights_t)
 * ---------------------------------------------------------------------- */
typedef sigma_u64 SigmaCapRights_t;

#define SIGMA_CAP_READ    (1ULL << 0)
#define SIGMA_CAP_WRITE   (1ULL << 1)
#define SIGMA_CAP_SEEK    (1ULL << 2)
#define SIGMA_CAP_FSTAT   (1ULL << 3)
#define SIGMA_CAP_FCNTL   (1ULL << 4)
#define SIGMA_CAP_IOCTL   (1ULL << 5)
#define SIGMA_CAP_MMAP    (1ULL << 6)
#define SIGMA_CAP_CONNECT (1ULL << 7)
#define SIGMA_CAP_BIND    (1ULL << 8)
#define SIGMA_CAP_ACCEPT  (1ULL << 9)
#define SIGMA_CAP_ALL     (~(SigmaCapRights_t)0)

/* -------------------------------------------------------------------------
 * Jail descriptor
 * ---------------------------------------------------------------------- */
typedef struct {
    sigma_i32         jid;                              /* Jail ID          */
    char              name    [SIGMA_JAIL_NAME_MAX];
    char              path    [SIGMA_JAIL_PATH_MAX];    /* chroot root      */
    char              hostname[SIGMA_JAIL_HOST_MAX];
    char              ip_addrs[SIGMA_JAIL_IP_MAX][16];  /* IPv4 dotted      */
    sigma_u32         ip_count;
    sigma_u32         flags;
    SigmaJailRlimit_t rlimits;
    sigma_i32         parent_jid;   /* -1 = top-level jail                  */
    sigma_bool        active;
    sigma_u32         proc_count;   /* Simulated process count in jail       */
} SigmaJail_t;

/* -------------------------------------------------------------------------
 * Capsicum capability-mode fd entry
 * ---------------------------------------------------------------------- */
typedef struct {
    int              fd;
    SigmaCapRights_t rights;
    sigma_bool       in_capmode;
} SigmaCapFd_t;

/* -------------------------------------------------------------------------
 * Public API
 * ---------------------------------------------------------------------- */
sigma_i32   sigma_jail_create  (const char *name, const char *path,
                                 const char *hostname, sigma_u32 flags);
sigma_err_t sigma_jail_destroy (sigma_i32 jid);
SigmaJail_t *sigma_jail_find   (sigma_i32 jid);
sigma_err_t sigma_jail_set_rlimit(sigma_i32 jid, const SigmaJailRlimit_t *lim);
sigma_err_t sigma_jail_exec    (sigma_i32 jid, const char *cmd);  /* jexec */
void        sigma_jls          (void);                            /* jls   */
sigma_err_t sigma_jail_add_ip  (sigma_i32 jid, const char *ipv4);

/* Capsicum */
sigma_err_t sigma_cap_enter    (void);
sigma_err_t sigma_cap_rights_limit(int fd, SigmaCapRights_t rights);
sigma_bool  sigma_cap_sandboxed(void);

void SovereignJail_Init(void);

#endif /* SOVEREIGN_JAIL_H */
