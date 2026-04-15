/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S01_Genesis/shards/sigma_syscall_table.c
 * =========================================================================
 * Syscall dispatch engine — zero-overhead table lookup, per-call telemetry,
 * and seccomp class enforcement.
 * =========================================================================
 */

#include "sigma_syscall_table.h"
#include "../../../../include/sigma_libc.h"

static sigma_syscall_desc_t s_table[SIGMA_SYSCALL_MAX];
static sc_u32               s_registered = 0;

/* ── Default stub handler ────────────────────────────────────────────────── */
static sc_i64 sc_stub(sc_u64 a, sc_u64 b, sc_u64 c,
                       sc_u64 d, sc_u64 e, sc_u64 f) {
    (void)a; (void)b; (void)c; (void)d; (void)e; (void)f;
    return -38; /* ENOSYS */
}

/* ── Built-in handlers (minimal stubs) ───────────────────────────────────── */
static sc_i64 sc_handle_getpid(sc_u64 a,sc_u64 b,sc_u64 c,
                                sc_u64 d,sc_u64 e,sc_u64 f) {
    (void)a;(void)b;(void)c;(void)d;(void)e;(void)f;
    return 1; /* sigma-init is always PID 1 */
}

static sc_i64 sc_handle_write(sc_u64 fd, sc_u64 buf, sc_u64 len,
                               sc_u64 d, sc_u64 e, sc_u64 f) {
    (void)d;(void)e;(void)f;
    if (fd == 1 || fd == 2) {  /* stdout/stderr */
        sigma_write((int)fd, (const void*)buf, (sigma_sz_t)len);
        return (sc_i64)len;
    }
    return -9; /* EBADF */
}

static sc_i64 sc_handle_exit(sc_u64 code, sc_u64 b, sc_u64 c,
                              sc_u64 d, sc_u64 e, sc_u64 f) {
    (void)b;(void)c;(void)d;(void)e;(void)f;
    sigma_printf("S [SYSCALL] exit(%llu)\n", (unsigned long long)code);
    sigma_exit((int)code);
    return 0;
}

static sc_i64 sc_handle_udf(sc_u64 id, sc_u64 args, sc_u64 c,
                             sc_u64 d, sc_u64 e, sc_u64 f) {
    (void)c;(void)d;(void)e;(void)f;
    sigma_printf("S [SYSCALL] UDF call id=%llu args=0x%llx\n",
                 (unsigned long long)id, (unsigned long long)args);
    return 0;
}

/* ── Table initialization ────────────────────────────────────────────────── */
void sigma_syscall_table_init(void) {
    /* Pre-fill with stubs */
    for (sc_u32 i = 0; i < SIGMA_SYSCALL_MAX; i++) {
        s_table[i].number   = i;
        s_table[i].arity    = 0;
        s_table[i].sec_class= SC_SEC_SAFE;
        s_table[i].handler  = sc_stub;
        s_table[i].call_count = 0;
        sigma_strncpy(s_table[i].name, "unimplemented", 31);
    }
    s_registered = SIGMA_SYSCALL_MAX;

    /* Register core handlers */
    sigma_syscall_register(SC_WRITE,   "write",   3, SC_SEC_MODERATE, sc_handle_write);
    sigma_syscall_register(SC_GETPID,  "getpid",  0, SC_SEC_SAFE,     sc_handle_getpid);
    sigma_syscall_register(SC_EXIT,    "exit",    1, SC_SEC_MODERATE,  sc_handle_exit);
    sigma_syscall_register(SC_SIGMA_UDF_CALL, "sigma_udf", 2, SC_SEC_MODERATE, sc_handle_udf);

    sigma_printf("S [SYSCALL] Table initialized: %u entries\n", SIGMA_SYSCALL_MAX);
}

/* ── Register a handler ──────────────────────────────────────────────────── */
void sigma_syscall_register(sc_u32 num, const char *name, sc_u8 arity,
                             sc_security_t sec, sigma_syscall_fn handler) {
    if (num >= SIGMA_SYSCALL_MAX) return;
    sigma_strncpy(s_table[num].name, name, 31);
    s_table[num].number   = num;
    s_table[num].arity    = arity;
    s_table[num].sec_class= sec;
    s_table[num].handler  = handler ? handler : sc_stub;
}

/* ── Dispatch ────────────────────────────────────────────────────────────── */
sc_i64 sigma_syscall_dispatch(sc_u32 num,
                               sc_u64 a, sc_u64 b, sc_u64 c,
                               sc_u64 d, sc_u64 e, sc_u64 f) {
    if (num >= SIGMA_SYSCALL_MAX) return -38; /* ENOSYS */

    sigma_syscall_desc_t *desc = &s_table[num];

    /* Security gate: ring-0 enforcement */
    if (desc->sec_class == SC_SEC_CRITICAL) {
        sigma_printf("S [SYSCALL] DENIED #%u '%s' — ring-0 only\n",
                     num, desc->name);
        return -1; /* EPERM */
    }

    desc->call_count++;
    return desc->handler(a, b, c, d, e, f);
}

/* ── Telemetry audit ─────────────────────────────────────────────────────── */
void sigma_syscall_audit(void) {
    sigma_printf("\nS SYSCALL TELEMETRY (top 10 by call count)\n");
    sigma_printf("%-8s %-24s %s\n", "NUM", "NAME", "CALLS");
    sc_u32 printed = 0;
    for (sc_u32 i = 0; i < SIGMA_SYSCALL_MAX && printed < 10; i++) {
        if (s_table[i].call_count > 0) {
            sigma_printf("  %-6u %-24s %u\n",
                         s_table[i].number, s_table[i].name,
                         s_table[i].call_count);
            printed++;
        }
    }
}
