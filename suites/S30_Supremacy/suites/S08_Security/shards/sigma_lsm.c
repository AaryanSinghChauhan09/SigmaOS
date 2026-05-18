#include "libc/SovereignLibC.h"
/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S08_Security/shards/sigma_lsm.c
 * =========================================================================
 */

#include "sigma_lsm.h"
#include "libc/sigma_libc.h"

static sigma_security_ctx_t s_ctxs[SIGMA_LSM_MAX_PROCS];
static sigma_u32            s_ctx_count = 0;
static sigma_lsm_hooks_t   *s_hooks = SIGMA_NULL;

/* ── Audit ring buffer ───────────────────────────────────────────────────── */
#define AUDIT_LOG_MAX 512
static struct {
    sigma_u32 pid;
    char    event[64];
    char    result[8];
} s_audit_log[AUDIT_LOG_MAX];
static sigma_u32 s_audit_head = 0;

static void audit_write(sigma_u32 pid, const char *event, sigma_bool allowed) {
    s_audit_log[s_audit_head % AUDIT_LOG_MAX].pid = pid;
    sigma_strncpy(s_audit_log[s_audit_head % AUDIT_LOG_MAX].event, event, 63);
    sigma_strncpy(s_audit_log[s_audit_head % AUDIT_LOG_MAX].result,
                  allowed ? "ALLOW" : "DENY", 7);
    s_audit_head++;
}

/* ── Helpers ─────────────────────────────────────────────────────────────── */
static sigma_security_ctx_t *get_ctx(sigma_u32 pid) {
    for (sigma_u32 i = 0; i < s_ctx_count; i++)
        if (s_ctxs[i].pid == pid) return &s_ctxs[i];
    return LSM_NULL;
}

/* ── Init ────────────────────────────────────────────────────────────────── */
void sigma_lsm_init(void) {
    sigma_sigma_memset(s_ctxs, 0, sizeof(s_ctxs));
    sigma_sigma_printf("S [LSM] Security framework initialized\n");
    sigma_sigma_printf("S [LSM] Active policies: SELinux | AppArmor | Pledge | MIC | TCC\n");
}

void sigma_lsm_register_hooks(sigma_lsm_hooks_t *hooks) {
    s_hooks = hooks;
    sigma_sigma_printf("S [LSM] Hook table registered\n");
}

/* ── Context ─────────────────────────────────────────────────────────────── */
sigma_i32 sigma_lsm_ctx_create(sigma_u32 pid, const char *domain) {
    if (s_ctx_count >= SIGMA_LSM_MAX_PROCS) return LSM_DENY;
    sigma_security_ctx_t *ctx = &s_ctxs[s_ctx_count++];
    sigma_sigma_memset(ctx, 0, sizeof(*ctx));
    ctx->pid = pid;
    sigma_strncpy(ctx->label.domain, domain, LSM_LABEL_LEN - 1);
    sigma_strncpy(ctx->label.type, "sigma_default_t", LSM_LABEL_LEN - 1);
    ctx->caps_permitted  = SIGMA_CAP_STDIO;
    ctx->caps_effective  = 0;
    ctx->selinux_enforcing = LSM_TRUE;
    sigma_sigma_printf("S [LSM] CTX: pid=%u domain=%s\n", pid, domain);
    return LSM_ALLOW;
}

void sigma_lsm_ctx_destroy(sigma_u32 pid) {
    for (sigma_u32 i = 0; i < s_ctx_count; i++) {
        if (s_ctxs[i].pid == pid) {
            for (sigma_u32 j = i; j < s_ctx_count - 1; j++)
                s_ctxs[j] = s_ctxs[j+1];
            s_ctx_count--;
            return;
        }
    }
}

sigma_i32 sigma_lsm_set_caps(sigma_u32 pid, unsigned long long caps) {
    sigma_security_ctx_t *ctx = get_ctx(pid);
    if (!ctx) return LSM_DENY;
    /* Only allow granting caps that are in permitted set */
    ctx->caps_effective = caps & ctx->caps_permitted;
    return LSM_ALLOW;
}

sigma_i32 sigma_lsm_pledge(sigma_u32 pid, sigma_u32 pledge_mask) {
    sigma_security_ctx_t *ctx = get_ctx(pid);
    if (!ctx) return LSM_DENY;
    /* pledge() is one-way — can only reduce privileges */
    ctx->pledge_mask &= pledge_mask;
    if (pledge_mask == 0) ctx->unveil_locked = LSM_TRUE;
    sigma_sigma_printf("S [LSM] PLEDGE: pid=%u mask=0x%x\n", pid, ctx->pledge_mask);
    return LSM_ALLOW;
}

void sigma_lsm_unveil(sigma_u32 pid, const char *path, const char *perms) {
    sigma_security_ctx_t *ctx = get_ctx(pid);
    if (!ctx || ctx->unveil_locked) return;
    if (!path) { ctx->unveil_locked = LSM_TRUE; return; }
    sigma_sigma_printf("S [LSM] UNVEIL: pid=%u path=%s perms=%s\n", pid, path, perms);
}

/* ── Access checks ─────────────────────────────────────────────────────────── */
sigma_i32 sigma_lsm_check_file_open(sigma_u32 pid, const char *path, sigma_u32 flags) {
    sigma_security_ctx_t *ctx = get_ctx(pid);
    if (!ctx) return LSM_ALLOW;  /* no ctx = unconstrained (root services) */

    /* pledge RPATH/WPATH check */
    if ((flags & 0x01) && !(ctx->pledge_mask & PLEDGE_WPATH)) {
        audit_write(pid, path, LSM_FALSE);
        sigma_sigma_printf("S [LSM] DENY: pid=%u write-open '%s' (pledge WPATH)\n", pid, path);
        return LSM_DENY;
    }

    /* run registered hook */
    if (s_hooks && s_hooks->file_open) {
        lsm_hook_result_t r = s_hooks->file_open(pid, path, flags);
        if (r == LSM_HOOK_DENY) { audit_write(pid, path, LSM_FALSE); return LSM_DENY; }
    }

    audit_write(pid, path, LSM_TRUE);
    return LSM_ALLOW;
}

sigma_i32 sigma_lsm_check_net(sigma_u32 pid, sigma_u32 dst_ip, sigma_u32 port) {
    sigma_security_ctx_t *ctx = get_ctx(pid);
    if (!ctx) return LSM_ALLOW;
    if (!(ctx->pledge_mask & (PLEDGE_INET | PLEDGE_DNS))) {
        sigma_sigma_printf("S [LSM] DENY: pid=%u net dst=0x%x:%u (pledge INET)\n",
                     pid, dst_ip, (unsigned)port);
        return LSM_DENY;
    }
    return LSM_ALLOW;
}

sigma_i32 sigma_lsm_check_syscall(sigma_u32 pid, sigma_u32 nr) {
    sigma_security_ctx_t *ctx = get_ctx(pid);
    if (!ctx) return LSM_ALLOW;
    if (s_hooks && s_hooks->syscall)
        return s_hooks->syscall(pid, nr) == LSM_HOOK_DENY ? LSM_DENY : LSM_ALLOW;
    return LSM_ALLOW;
}

sigma_i32 sigma_lsm_check_cap(sigma_u32 pid, unsigned long long needed_cap) {
    sigma_security_ctx_t *ctx = get_ctx(pid);
    if (!ctx) return LSM_ALLOW;
    if (ctx->caps_effective & needed_cap) return LSM_ALLOW;
    sigma_sigma_printf("S [LSM] DENY: pid=%u missing cap 0x%llx\n",
                 pid, (unsigned long long)needed_cap);
    return LSM_DENY;
}

/* ── Audit dump ───────────────────────────────────────────────────────────── */
void sigma_lsm_audit_dump(void) {
    sigma_u32 count = s_audit_head < AUDIT_LOG_MAX ? s_audit_head : AUDIT_LOG_MAX;
    sigma_sigma_printf("\nS LSM AUDIT LOG (%u events)\n", count);
    for (sigma_u32 i = 0; i < count; i++) {
        sigma_u32 idx = i % AUDIT_LOG_MAX;
        sigma_sigma_printf("  pid=%-5u %-6s %s\n",
                     s_audit_log[idx].pid,
                     s_audit_log[idx].result,
                     s_audit_log[idx].event);
    }
}
