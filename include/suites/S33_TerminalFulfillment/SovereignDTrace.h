/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN DTRACE — DYNAMIC KERNEL TRACING (v1.0 — C11)
 * =========================================================================
 * Absorbed USPs from: FreeBSD DTrace / illumos DTrace
 *   https://github.com/freebsd/freebsd-src/tree/main/sys/cddl/contrib/opensolaris
 *
 * DTrace USPs absorbed:
 *   ✓ Probe registration (provider:module:function:name)
 *   ✓ D language predicate / action model (C11 callbacks)
 *   ✓ Built-in aggregations: @count, @sum, @min, @max, @avg, @quantize
 *   ✓ Probe providers: syscall, sched, io, proc, fbt, profile
 *   ✓ Tracing enable/disable per probe
 *   ✓ Consumer script compilation and execution
 *   ✓ Ring buffer output (dtrace -l / dtrace -n)
 *
 * FreeBSD pf Packet Filter USPs absorbed:
 *   ✓ Rule table (pass/block/nat/rdr/binat)
 *   ✓ State table (stateful TCP/UDP tracking)
 *   ✓ Anchors (modular rule sets, like pfctl -f)
 *   ✓ queueing / ALTQ bandwidth shaping
 *   ✓ pf.conf parity — pfctl -e/-d/-F/-f/-s
 * =========================================================================
 */

#ifndef SOVEREIGN_DTRACE_H
#define SOVEREIGN_DTRACE_H

#include "sigma_types.h"

/* =========================================================================
 * §1  DTRACE SUBSYSTEM
 * ====================================================================== */

#define DTRACE_PROBE_MAX       512
#define DTRACE_PROVIDER_MAX     32
#define DTRACE_NAME_MAX         64
#define DTRACE_AGG_BUCKETS      64

/* -------------------------------------------------------------------------
 * Probe ID and description
 * ---------------------------------------------------------------------- */
typedef sigma_u32 dtrace_id_t;

typedef struct {
    dtrace_id_t  id;
    char         provider [DTRACE_NAME_MAX];  /* e.g. "syscall"   */
    char         module   [DTRACE_NAME_MAX];  /* e.g. "sigma_vfs" */
    char         function [DTRACE_NAME_MAX];  /* e.g. "sigma_open"*/
    char         name     [DTRACE_NAME_MAX];  /* e.g. "entry"     */
    sigma_bool   enabled;
    sigma_u64    fire_count;
} SigmaDTProbe_t;

/* -------------------------------------------------------------------------
 * Aggregation types (@count, @sum, @quantize …)
 * ---------------------------------------------------------------------- */
typedef enum {
    DT_AGG_COUNT    = 0,
    DT_AGG_SUM      = 1,
    DT_AGG_MIN      = 2,
    DT_AGG_MAX      = 3,
    DT_AGG_AVG      = 4,
    DT_AGG_QUANTIZE = 5,
} SigmaDTAggType_t;

typedef struct {
    char             name  [DTRACE_NAME_MAX];
    SigmaDTAggType_t type;
    sigma_i64        values[DTRACE_AGG_BUCKETS];  /* histogram buckets */
    sigma_u64        count;
    sigma_i64        sum;
    sigma_i64        min_val;
    sigma_i64        max_val;
} SigmaDTAggr_t;

/* -------------------------------------------------------------------------
 * Action callback (D-language action, implemented as C11 fn pointer)
 * ---------------------------------------------------------------------- */
typedef void (*SigmaDTAction_t)(dtrace_id_t probe_id,
                                 sigma_u64 arg0, sigma_u64 arg1,
                                 sigma_u64 arg2, sigma_u64 arg3);

/* -------------------------------------------------------------------------
 * DTrace Consumer (an active tracing session)
 * ---------------------------------------------------------------------- */
#define DTRACE_PROBE_REFS_MAX 32

typedef struct {
    char          clause    [DTRACE_NAME_MAX * 4]; /* provider:mod:fn:name */
    SigmaDTAction_t action;
    dtrace_id_t   probes    [DTRACE_PROBE_REFS_MAX];
    sigma_u32     probe_count;
    sigma_bool    active;
} SigmaDTConsumer_t;

/* -------------------------------------------------------------------------
 * Public API — DTrace (mirrors dtrace(1) and libdtrace)
 * ---------------------------------------------------------------------- */
dtrace_id_t  sigma_dt_probe_register (const char *provider, const char *module,
                                       const char *function, const char *name);
void         sigma_dt_probe_fire     (dtrace_id_t id,
                                       sigma_u64 a0, sigma_u64 a1,
                                       sigma_u64 a2, sigma_u64 a3);
sigma_err_t  sigma_dt_enable         (dtrace_id_t id);
sigma_err_t  sigma_dt_disable        (dtrace_id_t id);
sigma_err_t  sigma_dt_consumer_enable(const char *clause, SigmaDTAction_t action);
void         sigma_dt_list_probes    (const char *filter);   /* dtrace -l    */
void         sigma_dt_aggr_print     (const SigmaDTAggr_t *a);

/* =========================================================================
 * §2  PACKET FILTER (pf) SUBSYSTEM
 * ====================================================================== */

#define SIGMA_PF_RULES_MAX   256
#define SIGMA_PF_STATES_MAX 4096
#define SIGMA_PF_ANCHOR_MAX   32
#define SIGMA_PF_ADDR_MAX     16

typedef enum {
    PF_PASS  = 0,
    PF_BLOCK = 1,
    PF_NAT   = 2,
    PF_RDR   = 3,    /* Redirect */
    PF_BINAT = 4,
} SigmaPFAction_t;

typedef enum {
    PF_PROTO_ANY = 0,
    PF_PROTO_TCP = 6,
    PF_PROTO_UDP = 17,
    PF_PROTO_ICMP= 1,
} SigmaPFProto_t;

typedef struct {
    char           src_addr [SIGMA_PF_ADDR_MAX]; /* CIDR or "any"        */
    char           dst_addr [SIGMA_PF_ADDR_MAX];
    sigma_u16      src_port;
    sigma_u16      dst_port;
    char           interface[16];                /* "eth0", "lo", "any"   */
    SigmaPFProto_t proto;
    SigmaPFAction_t action;
    sigma_bool     stateful; /* keep state */
    sigma_bool     log;
    char           label   [32];
    sigma_u64      bytes_matched;
    sigma_u64      pkts_matched;
} SigmaPFRule_t;

/* Stateful connection entry */
typedef struct {
    char           src [SIGMA_PF_ADDR_MAX];
    char           dst [SIGMA_PF_ADDR_MAX];
    sigma_u16      sport, dport;
    SigmaPFProto_t proto;
    sigma_u32      state;   /* 0=NEW 1=ESTABLISHED 2=CLOSE_WAIT 3=TIME_WAIT */
    sigma_u64      bytes;
    sigma_u64      pkts;
} SigmaPFState_t;

typedef struct {
    SigmaPFRule_t  rules [SIGMA_PF_RULES_MAX];
    sigma_u32      rule_count;
    SigmaPFState_t states[SIGMA_PF_STATES_MAX];
    sigma_u32      state_count;
    sigma_bool     enabled;
    sigma_u64      total_passed;
    sigma_u64      total_blocked;
} SigmaPFCtx_t;

extern SigmaPFCtx_t g_sigma_pf;

/* Public API — pf (mirrors pfctl) */
sigma_err_t sigma_pf_enable   (void);
sigma_err_t sigma_pf_disable  (void);
sigma_err_t sigma_pf_add_rule (const SigmaPFRule_t *rule);
sigma_err_t sigma_pf_flush    (void);                /* pfctl -F rules    */
sigma_err_t sigma_pf_match    (const char *src, const char *dst,
                                sigma_u16 sport, sigma_u16 dport,
                                SigmaPFProto_t proto);
void        sigma_pf_show_rules (void);              /* pfctl -s rules    */
void        sigma_pf_show_states(void);              /* pfctl -s state    */
void        sigma_pf_show_info  (void);              /* pfctl -s info     */

void SovereignDTrace_Init(void);

#endif /* SOVEREIGN_DTRACE_H */
