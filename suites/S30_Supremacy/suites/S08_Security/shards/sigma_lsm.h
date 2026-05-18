/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S08_Security/shards/sigma_lsm.h
 * =========================================================================
 * Sovereign Linux Security Module (LSM) framework — gap-closes:
 *   Linux  : LSM hooks, SELinux, AppArmor, seccomp-BPF
 *   macOS  : TCC (Transparency Consent Control), Gatekeeper, SIP
 *   Windows: Mandatory Integrity Control (MIC), Windows Defender ASG
 *   Android: SELinux enforcing, scoped storage, runtime permissions
 *   OpenBSD: pledge/unveil
 * =========================================================================
 */

#ifndef SIGMA_LSM_H
#define SIGMA_LSM_H

#include "core/sigma_types.h"

#define LSM_TRUE  ((sigma_bool)1)
#define LSM_FALSE ((sigma_bool)0)
#define LSM_ALLOW ((sigma_i32)0)
#define LSM_DENY  ((sigma_i32)-1)

/* ── Security labels ─────────────────────────────────────────────────────── */
#define LSM_LABEL_LEN 128

typedef struct {
    char  domain[LSM_LABEL_LEN];   /* e.g. "sigma_browser_t"            */
    char  type[LSM_LABEL_LEN];     /* e.g. "sigma_config_t"             */
    sigma_u32 sensitivity;           /* MLS level (0=unclassified)        */
    sigma_u32 integrity;             /* Biba-style integrity level        */
} sigma_label_t;

/* ── Capability bits (Linux CAP_* parity) ───────────────────────────────── */
#define SIGMA_CAP_CHOWN       (1ULL <<  0)
#define SIGMA_CAP_NET_BIND    (1ULL <<  1)
#define SIGMA_CAP_NET_ADMIN   (1ULL <<  2)
#define SIGMA_CAP_SYS_ADMIN   (1ULL <<  3)
#define SIGMA_CAP_SYS_PTRACE  (1ULL <<  4)
#define SIGMA_CAP_SYS_MODULE  (1ULL <<  5)
#define SIGMA_CAP_SETUID      (1ULL <<  6)
#define SIGMA_CAP_SETGID      (1ULL <<  7)
#define SIGMA_CAP_KILL        (1ULL <<  8)
#define SIGMA_CAP_AUDIT       (1ULL <<  9)
#define SIGMA_CAP_MLOCK       (1ULL << 10)

/* ── pledge-style syscall restrictions (OpenBSD gap) ────────────────────── */
#define PLEDGE_STDIO   (1 << 0)   /* read/write/seek on FDs             */
#define PLEDGE_RPATH   (1 << 1)   /* read-only filesystem access        */
#define PLEDGE_WPATH   (1 << 2)   /* write access to filesystem         */
#define PLEDGE_EXEC    (1 << 3)   /* fork+exec privilege                */
#define PLEDGE_INET    (1 << 4)   /* AF_INET sockets                    */
#define PLEDGE_DNS     (1 << 5)   /* DNS resolution only                */
#define PLEDGE_PROC    (1 << 6)   /* process control                    */

/* ── Permission context per process ─────────────────────────────────────── */
typedef struct {
    sigma_u32       pid;
    sigma_label_t label;
    unsigned long long caps_effective;  /* effective capability set      */
    unsigned long long caps_permitted;  /* permitted capability set      */
    sigma_u32       pledge_mask;          /* OpenBSD pledge flags          */
    sigma_bool      selinux_enforcing;
    sigma_bool      apparmor_confined;
    sigma_bool      unveil_locked;        /* unveil() called with NULL     */
} sigma_security_ctx_t;

/* ── LSM hook return codes ───────────────────────────────────────────────── */
typedef enum {
    LSM_HOOK_ALLOW  = 0,
    LSM_HOOK_DENY   = 1,
    LSM_HOOK_AUDIT  = 2   /* allow but log                             */
} lsm_hook_result_t;

/* ── Hook table (Linux LSM hook structure parity) ─────────────────────────── */
typedef struct {
    lsm_hook_result_t (*process_create)(sigma_u32 parent_pid, const char *cmd);
    lsm_hook_result_t (*file_open)(sigma_u32 pid, const char *path, sigma_u32 flags);
    lsm_hook_result_t (*file_write)(sigma_u32 pid, const char *path);
    lsm_hook_result_t (*net_connect)(sigma_u32 pid, sigma_u32 dst_ip, sigma_u32 port);
    lsm_hook_result_t (*syscall)(sigma_u32 pid, sigma_u32 syscall_num);
    lsm_hook_result_t (*capability)(sigma_u32 pid, unsigned long long cap);
} sigma_lsm_hooks_t;

#define SIGMA_LSM_MAX_PROCS 512

/* ── Public API ─────────────────────────────────────────────────────────── */
void       sigma_lsm_init(void);
void       sigma_lsm_register_hooks(sigma_lsm_hooks_t *hooks);

/* Context management */
sigma_i32    sigma_lsm_ctx_create(sigma_u32 pid, const char *domain);
void       sigma_lsm_ctx_destroy(sigma_u32 pid);
sigma_i32    sigma_lsm_set_caps(sigma_u32 pid, unsigned long long caps);
sigma_i32    sigma_lsm_pledge(sigma_u32 pid, sigma_u32 pledge_mask);
void       sigma_lsm_unveil(sigma_u32 pid, const char *path, const char *perms);

/* Access checks (called from syscall dispatcher) */
sigma_i32    sigma_lsm_check_file_open(sigma_u32 pid, const char *path, sigma_u32 flags);
sigma_i32    sigma_lsm_check_net(sigma_u32 pid, sigma_u32 dst_ip, sigma_u32 port);
sigma_i32    sigma_lsm_check_syscall(sigma_u32 pid, sigma_u32 nr);
sigma_i32    sigma_lsm_check_cap(sigma_u32 pid, unsigned long long needed_cap);

/* Audit log */
void       sigma_lsm_audit_dump(void);

#endif /* SIGMA_LSM_H */
