/*
 * =========================================================================
 * Σ SIGMAOS kernel/suites/S10_Containers/shards/sigma_container.c
 * =========================================================================
 */

#include "sigma_container.h"
#include "../../../../include/sigma_libc.h"

static sigma_container_t s_containers[CT_MAX];
static ct_u32            s_ct_count = 0;

static sigma_container_t *find_ct(const char *name) {
    for (ct_u32 i = 0; i < s_ct_count; i++)
        if (sigma_streq(s_containers[i].name, name))
            return &s_containers[i];
    return CT_NULL;
}

static void gen_id(char *out, const char *name) {
    /* Simple deterministic 12-char hex from name hash */
    ct_u64 h = 0xDEAD0000ULL;
    for (const char *p = name; *p; p++) h = h * 31 + (unsigned char)*p;
    sigma_snprintf(out, CT_NAME_LEN, "%012llx", (unsigned long long)h);
}

/* ── Namespace flags from isolation level ────────────────────────────────── */
#define CT_NS_PID  (1<<0)
#define CT_NS_NET  (1<<1)
#define CT_NS_MNT  (1<<2)
#define CT_NS_UTS  (1<<3)
#define CT_NS_IPC  (1<<4)
#define CT_NS_USER (1<<5)

static ct_u32 isolation_to_ns(sigma_isolation_t lvl) {
    switch (lvl) {
        case ISOLATE_PROCESS:   return CT_NS_PID | CT_NS_IPC;
        case ISOLATE_CONTAINER: return CT_NS_PID | CT_NS_NET | CT_NS_MNT |
                                        CT_NS_UTS | CT_NS_IPC | CT_NS_USER;
        case ISOLATE_VM_LITE:   return 0xFF;
        case ISOLATE_WASM:      return 0;
        default:                return 0;
    }
}

void sigma_ct_init(void) {
    sigma_memset(s_containers, 0, sizeof(s_containers));
    sigma_printf("Σ [CT] Container Runtime initialized. Max slots: %u\n", CT_MAX);
}

ct_i32 sigma_ct_create(const char *name, const char *image,
                        sigma_isolation_t level, sigma_ct_limits_t *limits) {
    if (s_ct_count >= CT_MAX || find_ct(name)) return CT_ERR;

    sigma_container_t *ct = &s_containers[s_ct_count++];
    sigma_memset(ct, 0, sizeof(*ct));
    sigma_strncpy(ct->name,  name,  CT_NAME_LEN - 1);
    sigma_strncpy(ct->image, image, CT_IMG_LEN  - 1);
    gen_id(ct->id, name);
    ct->isolation      = level;
    ct->ns_flags       = isolation_to_ns(level);
    ct->state          = CT_CREATED;
    ct->readonly_rootfs= CT_TRUE;
    if (limits) ct->limits = *limits;

    sigma_printf("Σ [CT] CREATE: %s (%s) image=%s isolation=%d ns=0x%x\n",
                 ct->name, ct->id, ct->image, (int)level, ct->ns_flags);
    return CT_OK;
}

ct_i32 sigma_ct_start(const char *name) {
    sigma_container_t *ct = find_ct(name);
    if (!ct || ct->state == CT_RUNNING) return CT_ERR;
    ct->state    = CT_RUNNING;
    ct->root_pid = 1000 + s_ct_count;  /* simulated container init PID */

    sigma_printf("Σ [CT] START: %s (root_pid=%u)\n", ct->name, ct->root_pid);

    /* Apply cgroup v2 limits */
    if (ct->limits.cpu_quota_us)
        sigma_printf("  ↳ cpu.max: %llu/%llu us\n",
                     (unsigned long long)ct->limits.cpu_quota_us,
                     (unsigned long long)ct->limits.cpu_period_us);
    if (ct->limits.mem_limit_kb)
        sigma_printf("  ↳ memory.max: %llu KB\n",
                     (unsigned long long)ct->limits.mem_limit_kb);
    if (ct->limits.pids_max)
        sigma_printf("  ↳ pids.max: %llu\n",
                     (unsigned long long)ct->limits.pids_max);

    /* Overlayfs mount for readonly rootfs */
    sigma_printf("  ↳ overlayfs: upper=tmpfs lower=%s\n", ct->image);
    return CT_OK;
}

ct_i32 sigma_ct_pause(const char *name) {
    sigma_container_t *ct = find_ct(name);
    if (!ct || ct->state != CT_RUNNING) return CT_ERR;
    ct->state = CT_PAUSED;
    sigma_printf("Σ [CT] PAUSE: %s (SIGSTOP all procs)\n", ct->name);
    return CT_OK;
}

ct_i32 sigma_ct_resume(const char *name) {
    sigma_container_t *ct = find_ct(name);
    if (!ct || ct->state != CT_PAUSED) return CT_ERR;
    ct->state = CT_RUNNING;
    sigma_printf("Σ [CT] RESUME: %s (SIGCONT all procs)\n", ct->name);
    return CT_OK;
}

ct_i32 sigma_ct_stop(const char *name) {
    sigma_container_t *ct = find_ct(name);
    if (!ct) return CT_ERR;
    sigma_printf("Σ [CT] STOP: %s (SIGTERM -> SIGKILL)\n", ct->name);
    ct->state    = CT_STOPPED;
    ct->root_pid = 0;
    return CT_OK;
}

ct_i32 sigma_ct_destroy(const char *name) {
    for (ct_u32 i = 0; i < s_ct_count; i++) {
        if (sigma_streq(s_containers[i].name, name)) {
            sigma_printf("Σ [CT] DESTROY: %s (%s)\n",
                         s_containers[i].name, s_containers[i].id);
            for (ct_u32 j = i; j < s_ct_count - 1; j++)
                s_containers[j] = s_containers[j+1];
            s_ct_count--;
            return CT_OK;
        }
    }
    return CT_ERR;
}

void sigma_ct_exec(const char *name, const char *cmd) {
    sigma_container_t *ct = find_ct(name);
    if (!ct || ct->state != CT_RUNNING) {
        sigma_printf("Σ [CT] EXEC FAIL: %s not running\n", name);
        return;
    }
    sigma_printf("Σ [CT] EXEC: [%s] $ %s\n", ct->id, cmd);
}

void sigma_ct_stats(const char *name) {
    sigma_container_t *ct = find_ct(name);
    if (!ct) return;
    sigma_printf("\nΣ CT STATS: %s (%s)\n", ct->name, ct->id);
    sigma_printf("  state:    %d   root_pid: %u\n", (int)ct->state, ct->root_pid);
    sigma_printf("  cpu_used: %llu us   mem_used: %llu KB\n",
                 (unsigned long long)ct->cpu_used_us,
                 (unsigned long long)ct->mem_used_kb);
}

void sigma_ct_ps(void) {
    static const char *st[] = {"CREATED","RUNNING","PAUSED","STOPPED","DEAD"};
    sigma_printf("\nΣ CONTAINER TABLE (%u)\n", s_ct_count);
    sigma_printf("%-12s %-16s %-8s %-24s\n", "ID", "NAME", "STATE", "IMAGE");
    for (ct_u32 i = 0; i < s_ct_count; i++) {
        sigma_container_t *ct = &s_containers[i];
        sigma_printf("  %-10s %-16s %-8s %-24s\n",
                     ct->id, ct->name, st[ct->state], ct->image);
    }
}
