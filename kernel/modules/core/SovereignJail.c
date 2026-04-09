/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN JAIL — IMPLEMENTATION (v1.0 — PURE C11)
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"
#include "../../../include/SovereignJail.h"

/* -------------------------------------------------------------------------
 * Global jail table
 * ---------------------------------------------------------------------- */
static SigmaJail_t s_jails   [SIGMA_JAIL_MAX];
static sigma_u32   s_jail_cnt = 0;
static sigma_i32   s_next_jid = 1;

/* Capsicum state */
static sigma_bool  s_capmode  = SIGMA_FALSE;

/* -------------------------------------------------------------------------
 * sigma_jail_create — jail(2) parity
 * ---------------------------------------------------------------------- */
sigma_i32 sigma_jail_create(const char *name, const char *path,
                              const char *hostname, sigma_u32 flags) {
    if (s_jail_cnt >= SIGMA_JAIL_MAX) {
        sigma_printf("Σ [JAIL]: Jail table full.\n");
        return -1;
    }
    SigmaJail_t *j = &s_jails[s_jail_cnt++];
    sigma_memset(j, 0, sizeof(*j));

    j->jid        = s_next_jid++;
    j->parent_jid = -1;
    j->flags      = flags;
    j->active     = SIGMA_TRUE;

    sigma_strcpy(j->name,     name,     SIGMA_JAIL_NAME_MAX);
    sigma_strcpy(j->path,     path,     SIGMA_JAIL_PATH_MAX);
    sigma_strcpy(j->hostname, hostname, SIGMA_JAIL_HOST_MAX);

    /* Default resource limits */
    j->rlimits.max_mem_bytes = 512ULL * 1024 * 1024;  /* 512 MB */
    j->rlimits.max_procs     = 128;
    j->rlimits.max_fds       = 1024;
    j->rlimits.cpu_pct       = 0;  /* unlimited */

    sigma_printf("Σ [JAIL]: Created jail %d '%s' root=%s hostname=%s\n",
                 j->jid, j->name, j->path, j->hostname);
    return j->jid;
}

/* -------------------------------------------------------------------------
 * sigma_jail_destroy — jail_remove(2) parity
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_jail_destroy(sigma_i32 jid) {
    SigmaJail_t *j = sigma_jail_find(jid);
    if (!j) return SIGMA_ENOENT;

    sigma_printf("Σ [JAIL]: Destroying jail %d '%s'...\n", j->jid, j->name);
    j->active = SIGMA_FALSE;

    /* Terminate all simulated processes in jail */
    sigma_printf("Σ [JAIL]: %u process(es) in jail terminated.\n", j->proc_count);
    j->proc_count = 0;
    return SIGMA_OK;
}

/* -------------------------------------------------------------------------
 * sigma_jail_find
 * ---------------------------------------------------------------------- */
SigmaJail_t *sigma_jail_find(sigma_i32 jid) {
    for (sigma_u32 i = 0; i < s_jail_cnt; i++) {
        if (s_jails[i].jid == jid && s_jails[i].active)
            return &s_jails[i];
    }
    return SIGMA_NULL;
}

/* -------------------------------------------------------------------------
 * sigma_jail_set_rlimit
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_jail_set_rlimit(sigma_i32 jid, const SigmaJailRlimit_t *lim) {
    SigmaJail_t *j = sigma_jail_find(jid);
    if (!j) return SIGMA_ENOENT;
    j->rlimits = *lim;
    sigma_printf("Σ [JAIL]: rlimits updated for jail %d (mem=%lluMB procs=%u)\n",
                 jid,
                 (unsigned long long)(lim->max_mem_bytes / (1024*1024)),
                 lim->max_procs);
    return SIGMA_OK;
}

/* -------------------------------------------------------------------------
 * sigma_jail_add_ip — add IPv4 address to jail's allowed address list
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_jail_add_ip(sigma_i32 jid, const char *ipv4) {
    SigmaJail_t *j = sigma_jail_find(jid);
    if (!j) return SIGMA_ENOENT;
    if (j->ip_count >= SIGMA_JAIL_IP_MAX) return SIGMA_ENOSPC;
    sigma_strcpy(j->ip_addrs[j->ip_count++], ipv4, 16);
    sigma_printf("Σ [JAIL]: Added IP %s to jail %d.\n", ipv4, jid);
    return SIGMA_OK;
}

/* -------------------------------------------------------------------------
 * sigma_jail_exec — jexec: run a command inside the jail
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_jail_exec(sigma_i32 jid, const char *cmd) {
    SigmaJail_t *j = sigma_jail_find(jid);
    if (!j) return SIGMA_ENOENT;

    sigma_printf("Σ [JAIL]: jexec %d: chroot(%s) exec(%s)\n",
                 jid, j->path, cmd);
    /*
     * Live kernel: sigma_fork() then:
     *   chroot(j->path)
     *   setuid/setgid to jail's UID
     *   sigma_execve(cmd, ...)
     */
    j->proc_count++;
    sigma_printf("Σ [JAIL]: Process spawned inside jail %d (count=%u)\n",
                 jid, j->proc_count);
    return SIGMA_OK;
}

/* -------------------------------------------------------------------------
 * sigma_jls — list all active jails (jls(8) parity)
 * ---------------------------------------------------------------------- */
void sigma_jls(void) {
    sigma_printf("Σ [JLS]:   JID  Hostname           IPv4\n");
    sigma_printf("Σ [JLS]:   ---  ─────────────────  ───────────────\n");
    for (sigma_u32 i = 0; i < s_jail_cnt; i++) {
        SigmaJail_t *j = &s_jails[i];
        if (!j->active) continue;
        char ip_list[64] = "(none)";
        if (j->ip_count > 0) {
            sigma_strcpy(ip_list, j->ip_addrs[0], 64);
        }
        sigma_printf("Σ [JLS]:  %4d  %-18s %s  procs=%u\n",
                     j->jid, j->hostname, ip_list, j->proc_count);
    }
}

/* -------------------------------------------------------------------------
 * Capsicum capability mode (cap_enter / cap_rights_limit)
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_cap_enter(void) {
    if (s_capmode) {
        sigma_printf("Σ [CAP]: Already in capability mode.\n");
        return SIGMA_OK;
    }
    s_capmode = SIGMA_TRUE;
    sigma_printf("Σ [CAP]: Process entered capability mode. "
                 "No new fds/syscalls outside capabilities.\n");
    return SIGMA_OK;
}

sigma_err_t sigma_cap_rights_limit(int fd, SigmaCapRights_t rights) {
    sigma_printf("Σ [CAP]: fd %d rights limited to 0x%llx\n",
                 fd, (unsigned long long)rights);
    /* Live: store rights in fdtable[fd], enforce on every syscall */
    return SIGMA_OK;
}

sigma_bool sigma_cap_sandboxed(void) {
    return s_capmode;
}

/* -------------------------------------------------------------------------
 * SovereignJail_Init — demo
 * ---------------------------------------------------------------------- */
void SovereignJail_Init(void) {
    sigma_printf("Σ [JAIL]: Initialising Sovereign Jail Subsystem (FreeBSD parity)...\n");

    /* Create a Web server jail */
    sigma_i32 web_jid = sigma_jail_create("sigma-web",
                                           "/jail/sigma-web",
                                           "web.sigma.local",
                                           JAIL_FLAG_ALLOW_RAWSOCK);
    sigma_jail_add_ip(web_jid, "192.168.10.10");
    sigma_jail_exec(web_jid, "/usr/sbin/sigma-httpd");

    /* Create a DB jail */
    sigma_i32 db_jid = sigma_jail_create("sigma-db",
                                          "/jail/sigma-db",
                                          "db.sigma.local",
                                          JAIL_FLAG_VNET);
    sigma_jail_add_ip(db_jid, "192.168.10.20");

    SigmaJailRlimit_t db_lim = {
        .max_mem_bytes = 2ULL * 1024 * 1024 * 1024,  /* 2 GB */
        .max_procs     = 64,
        .max_fds       = 512,
        .cpu_pct       = 80,
    };
    sigma_jail_set_rlimit(db_jid, &db_lim);
    sigma_jail_exec(db_jid, "/usr/sbin/sigma-postgres");

    sigma_jls();

    /* Capsicum demo */
    sigma_cap_rights_limit(3, SIGMA_CAP_READ | SIGMA_CAP_SEEK | SIGMA_CAP_FSTAT);
    sigma_cap_enter();
    sigma_printf("Σ [CAP]: Sandboxed: %s\n",
                 sigma_cap_sandboxed() ? "yes" : "no");

    sigma_printf("Σ [JAIL]: Sovereign Jail + Capsicum online.\n");
}
