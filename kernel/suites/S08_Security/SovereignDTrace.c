/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN DTRACE + PF PACKET FILTER — IMPLEMENTATION (v1.0)
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"
#include "../../../include/SovereignDTrace.h"

/* =========================================================================
 * §1  DTRACE SUBSYSTEM
 * ====================================================================== */

static SigmaDTProbe_t    s_probes [DTRACE_PROBE_MAX];
static sigma_u32         s_probe_cnt = 0;

static SigmaDTConsumer_t s_consumers[16];
static sigma_u32         s_con_cnt   = 0;

static SigmaDTAggr_t     s_aggrs    [64];
static sigma_u32         s_aggr_cnt  = 0;

/* -------------------------------------------------------------------------
 * sigma_dt_probe_register — Add a new probe point
 * ---------------------------------------------------------------------- */
dtrace_id_t sigma_dt_probe_register(const char *provider, const char *module,
                                     const char *function, const char *name) {
    if (s_probe_cnt >= DTRACE_PROBE_MAX) return 0;
    SigmaDTProbe_t *p = &s_probes[s_probe_cnt];
    p->id = s_probe_cnt + 1;
    sigma_strcpy(p->provider, provider, DTRACE_NAME_MAX);
    sigma_strcpy(p->module,   module,   DTRACE_NAME_MAX);
    sigma_strcpy(p->function, function, DTRACE_NAME_MAX);
    sigma_strcpy(p->name,     name,     DTRACE_NAME_MAX);
    p->enabled    = SIGMA_FALSE;
    p->fire_count = 0;
    s_probe_cnt++;
    return p->id;
}

/* -------------------------------------------------------------------------
 * sigma_dt_probe_fire — Execute all enabled consumer actions for probe id
 * ---------------------------------------------------------------------- */
void sigma_dt_probe_fire(dtrace_id_t id,
                          sigma_u64 a0, sigma_u64 a1,
                          sigma_u64 a2, sigma_u64 a3) {
    if (id == 0 || id > s_probe_cnt) return;
    SigmaDTProbe_t *p = &s_probes[id - 1];
    if (!p->enabled) return;

    p->fire_count++;

    /* Run each consumer that references this probe */
    for (sigma_u32 i = 0; i < s_con_cnt; i++) {
        SigmaDTConsumer_t *c = &s_consumers[i];
        if (!c->active) continue;
        for (sigma_u32 j = 0; j < c->probe_count; j++) {
            if (c->probes[j] == id && c->action) {
                c->action(id, a0, a1, a2, a3);
            }
        }
    }
}

/* -------------------------------------------------------------------------
 * sigma_dt_enable / sigma_dt_disable
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_dt_enable(dtrace_id_t id) {
    if (!id || id > s_probe_cnt) return SIGMA_EINVAL;
    s_probes[id - 1].enabled = SIGMA_TRUE;
    return SIGMA_OK;
}

sigma_err_t sigma_dt_disable(dtrace_id_t id) {
    if (!id || id > s_probe_cnt) return SIGMA_EINVAL;
    s_probes[id - 1].enabled = SIGMA_FALSE;
    return SIGMA_OK;
}

/* -------------------------------------------------------------------------
 * sigma_dt_consumer_enable — Attach action to probe(s) matching clause
 *   clause format: "provider:module:function:name"
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_dt_consumer_enable(const char *clause, SigmaDTAction_t action) {
    if (s_con_cnt >= 16) return SIGMA_ENOSPC;
    SigmaDTConsumer_t *c = &s_consumers[s_con_cnt++];
    sigma_memset(c, 0, sizeof(*c));
    sigma_strcpy(c->clause, clause, DTRACE_NAME_MAX * 4);
    c->action = action;
    c->active = SIGMA_TRUE;

    /* Match probes against the clause (wildcard '*' supported) */
    for (sigma_u32 i = 0; i < s_probe_cnt; i++) {
        SigmaDTProbe_t *p = &s_probes[i];
        /* For simplicity: match provider substring */
        if (sigma_strstr(clause, p->provider) ||
            sigma_strstr(clause, "*")) {
            if (c->probe_count < DTRACE_PROBE_REFS_MAX) {
                c->probes[c->probe_count++] = p->id;
                p->enabled = SIGMA_TRUE;
            }
        }
    }

    sigma_printf("Σ [DTRACE]: Consumer enabled: '%s' (%u probes matched)\n",
                 clause, c->probe_count);
    return SIGMA_OK;
}

/* -------------------------------------------------------------------------
 * sigma_dt_list_probes — dtrace -l
 * ---------------------------------------------------------------------- */
void sigma_dt_list_probes(const char *filter) {
    sigma_printf("Σ [DTRACE]: Probe list (%u total):\n", s_probe_cnt);
    sigma_printf("   ID  PROVIDER      MODULE        FUNCTION          NAME\n");
    for (sigma_u32 i = 0; i < s_probe_cnt; i++) {
        SigmaDTProbe_t *p = &s_probes[i];
        if (filter && !sigma_strstr(p->provider, filter) &&
            !sigma_strstr(p->function, filter))
            continue;
        sigma_printf("  %3u  %-12s  %-12s  %-16s  %-8s  %s\n",
                     p->id, p->provider, p->module, p->function, p->name,
                     p->enabled ? "[ON]" : "");
    }
}

/* -------------------------------------------------------------------------
 * sigma_dt_aggr_print — print aggregation results
 * ---------------------------------------------------------------------- */
void sigma_dt_aggr_print(const SigmaDTAggr_t *a) {
    sigma_printf("Σ [DTRACE-AGG]: @%s: count=%llu sum=%lld min=%lld max=%lld\n",
                 a->name,
                 (unsigned long long)a->count,
                 (long long)a->sum,
                 (long long)a->min_val,
                 (long long)a->max_val);
    if (a->type == DT_AGG_AVG && a->count > 0)
        sigma_printf("  avg=%lld\n", (long long)(a->sum / (sigma_i64)a->count));
}

/* =========================================================================
 * §2  PACKET FILTER (pf)
 * ====================================================================== */

SigmaPFCtx_t g_sigma_pf;

sigma_err_t sigma_pf_enable(void) {
    g_sigma_pf.enabled = SIGMA_TRUE;
    sigma_printf("Σ [PF]: Packet filter enabled.\n");
    return SIGMA_OK;
}

sigma_err_t sigma_pf_disable(void) {
    g_sigma_pf.enabled = SIGMA_FALSE;
    sigma_printf("Σ [PF]: Packet filter disabled.\n");
    return SIGMA_OK;
}

sigma_err_t sigma_pf_add_rule(const SigmaPFRule_t *rule) {
    if (g_sigma_pf.rule_count >= SIGMA_PF_RULES_MAX) return SIGMA_ENOSPC;
    sigma_memcpy(&g_sigma_pf.rules[g_sigma_pf.rule_count++], rule, sizeof(*rule));
    return SIGMA_OK;
}

sigma_err_t sigma_pf_flush(void) {
    sigma_memset(g_sigma_pf.rules, 0,
                 sizeof(SigmaPFRule_t) * g_sigma_pf.rule_count);
    sigma_printf("Σ [PF]: All rules flushed.\n");
    g_sigma_pf.rule_count = 0;
    return SIGMA_OK;
}

sigma_err_t sigma_pf_match(const char *src, const char *dst,
                            sigma_u16 sport, sigma_u16 dport,
                            SigmaPFProto_t proto) {
    if (!g_sigma_pf.enabled) {
        g_sigma_pf.total_passed++;
        return SIGMA_OK; /* pf disabled → pass all */
    }

    for (sigma_u32 i = 0; i < g_sigma_pf.rule_count; i++) {
        SigmaPFRule_t *r = &g_sigma_pf.rules[i];

        /* Match source */
        sigma_bool src_match = sigma_streq(r->src_addr, "any") ||
                               sigma_streq(r->src_addr, src);
        /* Match dest */
        sigma_bool dst_match = sigma_streq(r->dst_addr, "any") ||
                               sigma_streq(r->dst_addr, dst);
        /* Match port */
        sigma_bool port_match = (r->dst_port == 0 || r->dst_port == dport);
        /* Match proto */
        sigma_bool proto_match = (r->proto == PF_PROTO_ANY || r->proto == proto);

        if (src_match && dst_match && port_match && proto_match) {
            r->pkts_matched++;
            if (r->log)
                sigma_printf("Σ [PF]: %s %s:%u -> %s:%u  rule#%u [label:%s]\n",
                             r->action == PF_PASS ? "PASS" : "BLOCK",
                             src, sport, dst, dport, i,
                             r->label[0] ? r->label : "—");

            if (r->action == PF_BLOCK) {
                g_sigma_pf.total_blocked++;
                return SIGMA_EPERM;
            }
            g_sigma_pf.total_passed++;
            return SIGMA_OK;
        }
    }

    /* Default policy: pass */
    g_sigma_pf.total_passed++;
    return SIGMA_OK;
}

void sigma_pf_show_rules(void) {
    sigma_printf("Σ [PF]: Rules (%u):\n", g_sigma_pf.rule_count);
    static const char *act[] = {"pass","block","nat","rdr","binat"};
    for (sigma_u32 i = 0; i < g_sigma_pf.rule_count; i++) {
        SigmaPFRule_t *r = &g_sigma_pf.rules[i];
        sigma_printf("  [%2u] %-5s  %s  %s -> %s port %u  pkts=%llu\n",
                     i, act[r->action],
                     r->interface, r->src_addr, r->dst_addr, r->dst_port,
                     (unsigned long long)r->pkts_matched);
    }
}

void sigma_pf_show_states(void) {
    sigma_printf("Σ [PF]: State table (%u entries):\n", g_sigma_pf.state_count);
    for (sigma_u32 i = 0; i < g_sigma_pf.state_count; i++) {
        SigmaPFState_t *s = &g_sigma_pf.states[i];
        sigma_printf("  %s:%u <-> %s:%u  pkts=%llu bytes=%llu\n",
                     s->src, s->sport, s->dst, s->dport,
                     (unsigned long long)s->pkts,
                     (unsigned long long)s->bytes);
    }
}

void sigma_pf_show_info(void) {
    sigma_printf("Σ [PF]: Status: %s\n"
                 "  Passed:  %llu pkts\n"
                 "  Blocked: %llu pkts\n"
                 "  Rules:   %u\n"
                 "  States:  %u\n",
                 g_sigma_pf.enabled ? "Enabled" : "Disabled",
                 (unsigned long long)g_sigma_pf.total_passed,
                 (unsigned long long)g_sigma_pf.total_blocked,
                 g_sigma_pf.rule_count,
                 g_sigma_pf.state_count);
}

/* =========================================================================
 * SovereignDTrace_Init
 * ====================================================================== */
static void trace_action(dtrace_id_t id, sigma_u64 a0, sigma_u64 a1,
                          sigma_u64 a2, sigma_u64 a3) {
    (void)a1; (void)a2; (void)a3;
    sigma_printf("Σ [DTRACE]: PROBE #%u fired: arg0=%llu\n",
                 id, (unsigned long long)a0);
}

void SovereignDTrace_Init(void) {
    sigma_printf("Σ [DTRACE]: Initialising DTrace + pf (FreeBSD parity)...\n");
    sigma_memset(g_sigma_pf.rules, 0, sizeof(g_sigma_pf.rules));

    /* ── DTrace demo ── */
    /* Register probes for every core syscall entry */
    dtrace_id_t p_open   = sigma_dt_probe_register("syscall","sigma_vfs","sigma_open",  "entry");
    dtrace_id_t p_read   = sigma_dt_probe_register("syscall","sigma_vfs","sigma_read",  "entry");
    dtrace_id_t p_write  = sigma_dt_probe_register("syscall","sigma_vfs","sigma_write", "entry");
    dtrace_id_t p_sched  = sigma_dt_probe_register("sched",  "scheduler","sigma_schedule","on-cpu");
    dtrace_id_t p_net    = sigma_dt_probe_register("io",     "network",  "sigma_sendmsg","entry");
    dtrace_id_t p_proc   = sigma_dt_probe_register("proc",   "proc",     "sigma_fork",  "return");
    (void)p_read; (void)p_write; (void)p_sched; (void)p_net; (void)p_proc;

    /* Attach consumer action to syscall provider */
    sigma_dt_consumer_enable("syscall:::entry", trace_action);

    /* Fire some probes */
    sigma_dt_probe_fire(p_open,  4, 0, 0, 0);   /* fd=4 */
    sigma_dt_probe_fire(p_open,  5, 0, 0, 0);   /* fd=5 */

    sigma_dt_list_probes("syscall");

    /* Aggregate demo */
    SigmaDTAggr_t agg;
    sigma_memset(&agg, 0, sizeof(agg));
    sigma_strcpy(agg.name, "syscall_count", DTRACE_NAME_MAX);
    agg.type    = DT_AGG_COUNT;
    agg.count   = s_probes[p_open - 1].fire_count;
    agg.sum     = (sigma_i64)agg.count;
    agg.min_val = 0;
    agg.max_val = (sigma_i64)agg.count;
    sigma_dt_aggr_print(&agg);

    /* ── pf demo ── */
    sigma_pf_enable();

    /* Allow SSH from LAN */
    SigmaPFRule_t ssh_rule = {
        .action     = PF_PASS,
        .proto      = PF_PROTO_TCP,
        .dst_port   = 22,
        .stateful   = SIGMA_TRUE,
        .log        = SIGMA_TRUE,
    };
    sigma_strcpy(ssh_rule.src_addr, "192.168.1.0/24", SIGMA_PF_ADDR_MAX);
    sigma_strcpy(ssh_rule.dst_addr, "any",            SIGMA_PF_ADDR_MAX);
    sigma_strcpy(ssh_rule.interface,"eth0",           16);
    sigma_strcpy(ssh_rule.label,    "allow-ssh-lan",  32);
    sigma_pf_add_rule(&ssh_rule);

    /* Block all inbound HTTP from internet */
    SigmaPFRule_t http_block = {
        .action   = PF_BLOCK,
        .proto    = PF_PROTO_TCP,
        .dst_port = 80,
        .log      = SIGMA_TRUE,
    };
    sigma_strcpy(http_block.src_addr, "any", SIGMA_PF_ADDR_MAX);
    sigma_strcpy(http_block.dst_addr, "any", SIGMA_PF_ADDR_MAX);
    sigma_strcpy(http_block.interface,"eth0",16);
    sigma_strcpy(http_block.label,    "block-http", 32);
    sigma_pf_add_rule(&http_block);

    /* Test matches */
    sigma_pf_match("192.168.1.50", "10.0.0.1",  54321, 22, PF_PROTO_TCP);
    sigma_pf_match("5.5.5.5",      "10.0.0.1",  12345, 80, PF_PROTO_TCP);

    sigma_pf_show_rules();
    sigma_pf_show_info();

    sigma_printf("Σ [DTRACE]: DTrace + pf Packet Filter online.\n");
}
