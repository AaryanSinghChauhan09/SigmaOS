/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S01_Genesis/shards/sigma_syscall_table.c
 * =========================================================================
 * Syscall dispatch engine � zero-overhead table lookup, per-call telemetry,
 * and seccomp class enforcement.
 * =========================================================================
 */

#include "sigma_syscall_table.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"

static sigma_syscall_desc_t s_table[SIGMA_SYSCALL_MAX];
static sigma_u32               s_registered = 0;

/* -- Default stub handler -------------------------------------------------- */
static sigma_i64 sc_stub(sigma_u64 a, sigma_u64 b, sigma_u64 c,
                       sigma_u64 d, sigma_u64 e, sigma_u64 f) {
    (void)a; (void)b; (void)c; (void)d; (void)e; (void)f;
    return -38; /* ENOSYS */
}

/* -- Built-in handlers (minimal stubs) ------------------------------------- */
static sigma_i64 sc_handle_getpid(sigma_u64 a,sigma_u64 b,sigma_u64 c,
                                sigma_u64 d,sigma_u64 e,sigma_u64 f) {
    (void)a;(void)b;(void)c;(void)d;(void)e;(void)f;
    return 1; /* sigma-init is always PID 1 */
}

static sigma_i64 sc_handle_write(sigma_u64 fd, sigma_u64 buf, sigma_u64 len,
                               sigma_u64 d, sigma_u64 e, sigma_u64 f) {
    (void)d;(void)e;(void)f;
    if (fd == 1 || fd == 2) {  /* stdout/stderr */
        sigma_write((int)fd, (const void*)buf, (sigma_sz_t)len);
        return (sigma_i64)len;
    }
    return -9; /* EBADF */
}

static sigma_i64 sc_handle_exit(sigma_u64 code, sigma_u64 b, sigma_u64 c,
                              sigma_u64 d, sigma_u64 e, sigma_u64 f) {
    (void)b;(void)c;(void)d;(void)e;(void)f;
    sigma_sigma_printf("S [SYSCALL] exit(%llu)\n", (unsigned long long)code);
    sigma_exit((int)code);
    return 0;
}

static sigma_i64 sc_handle_udf(sigma_u64 id, sigma_u64 args, sigma_u64 c,
                             sigma_u64 d, sigma_u64 e, sigma_u64 f) {
    (void)c;(void)d;(void)e;(void)f;
    sigma_sigma_printf("S [SYSCALL] UDF call id=%llu args=0x%llx\n",
                 (unsigned long long)id, (unsigned long long)args);
    return 0;
}

/* -- Table initialization -------------------------------------------------- */
void sigma_syscall_table_init(void) {
    /* Pre-fill with stubs */
    for (sigma_u32 i = 0; i < SIGMA_SYSCALL_MAX; i++) {
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

    sigma_sigma_printf("S [SYSCALL] Table initialized: %u entries\n", SIGMA_SYSCALL_MAX);
}

/* -- Register a handler ---------------------------------------------------- */
void sigma_syscall_register(sigma_u32 num, const char *name, sigma_u8 arity,
                             sc_security_t sec, sigma_syscall_fn handler) {
    if (num >= SIGMA_SYSCALL_MAX) return;
    sigma_strncpy(s_table[num].name, name, 31);
    s_table[num].number   = num;
    s_table[num].arity    = arity;
    s_table[num].sec_class= sec;
    s_table[num].handler  = handler ? handler : sc_stub;
}

/* -- Dispatch -------------------------------------------------------------- */
sigma_i64 sigma_syscall_dispatch(sigma_u32 num,
                               sigma_u64 a, sigma_u64 b, sigma_u64 c,
                               sigma_u64 d, sigma_u64 e, sigma_u64 f) {
    if (num >= SIGMA_SYSCALL_MAX) return -38; /* ENOSYS */

    sigma_syscall_desc_t *desc = &s_table[num];

    /* Security gate: ring-0 enforcement */
    if (desc->sec_class == SC_SEC_CRITICAL) {
        sigma_sigma_printf("S [SYSCALL] DENIED #%u '%s' � ring-0 only\n",
                     num, desc->name);
        return -1; /* EPERM */
    }

    desc->call_count++;
    return desc->handler(a, b, c, d, e, f);
}

/* -- Telemetry audit ------------------------------------------------------- */
void sigma_syscall_audit(void) {
    sigma_sigma_printf("\nS SYSCALL TELEMETRY (top 10 by call count)\n");
    sigma_sigma_printf("%-8s %-24s %s\n", "NUM", "NAME", "CALLS");
    sigma_u32 printed = 0;
    for (sigma_u32 i = 0; i < SIGMA_SYSCALL_MAX && printed < 10; i++) {
        if (s_table[i].call_count > 0) {
            sigma_sigma_printf("  %-6u %-24s %u\n",
                         s_table[i].number, s_table[i].name,
                         s_table[i].call_count);
            printed++;
        }
    }
}
