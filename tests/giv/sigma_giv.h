/*
 * =========================================================================
 * S SIGMAOS tests/giv/sigma_giv.h
 * =========================================================================
 * Global Integration Verification (GIV) — SigmaOS test harness that
 * validates all 12 kernel suites interlock correctly.
 *
 * Test categories:
 *   UNIT   — single-function correctness
 *   INTEG  — cross-suite integration
 *   PERF   — throughput / latency benchmarks
 *   FUZZ   — random input robustness
 *   FORMAL — invariant assertions
 * =========================================================================
 */

#ifndef SIGMA_GIV_H
#define SIGMA_GIV_H

typedef unsigned int  gv_u32;
typedef signed   int  gv_i32;
typedef unsigned char gv_bool;
typedef unsigned long long gv_u64;
#define GV_TRUE  ((gv_bool)1)
#define GV_FALSE ((gv_bool)0)
#define GV_PASS  ((gv_i32) 0)
#define GV_FAIL  ((gv_i32)-1)

/* ── Test category ───────────────────────────────────────────────────────── */
typedef enum {
    GV_UNIT   = 0,
    GV_INTEG  = 1,
    GV_PERF   = 2,
    GV_FUZZ   = 3,
    GV_FORMAL = 4
} gv_category_t;

/* ── Test result ─────────────────────────────────────────────────────────── */
typedef enum {
    GV_RESULT_PASS    = 0,
    GV_RESULT_FAIL    = 1,
    GV_RESULT_SKIP    = 2,
    GV_RESULT_TIMEOUT = 3
} gv_result_t;

/* ── Test function signature ─────────────────────────────────────────────── */
typedef gv_result_t (*gv_test_fn)(void);

#define GV_MAX_TESTS 512
#define GV_NAME_LEN   64
#define GV_SUITE_LEN  32

/* ── Test case descriptor ───────────────────────────────────────────────── */
typedef struct {
    char          name[GV_NAME_LEN];
    char          suite[GV_SUITE_LEN];   /* e.g. "S03_Scheduler"         */
    gv_category_t category;
    gv_test_fn    fn;
    gv_result_t   result;
    gv_u64        elapsed_us;            /* execution time               */
    char          fail_msg[128];
} gv_test_t;

/* ── GIV session stats ───────────────────────────────────────────────────── */
typedef struct {
    gv_u32 total;
    gv_u32 passed;
    gv_u32 failed;
    gv_u32 skipped;
    gv_u64 total_us;
} gv_stats_t;

/* ── Assertion macros ───────────────────────────────────────────────────── */
#define GV_ASSERT(expr) \
    do { if (!(expr)) { \
        sigma_printf("S [GIV] ASSERT FAIL: %s:%d  %s\n", __FILE__, __LINE__, #expr); \
        return GV_RESULT_FAIL; \
    } } while(0)

#define GV_ASSERT_EQ(a, b) GV_ASSERT((a) == (b))
#define GV_ASSERT_NE(a, b) GV_ASSERT((a) != (b))
#define GV_ASSERT_GT(a, b) GV_ASSERT((a)  > (b))
#define GV_ASSERT_OK(expr)  GV_ASSERT((expr) == 0)

/* ── Public API ─────────────────────────────────────────────────────────── */
void sigma_giv_init(void);
void sigma_giv_register(const char *suite, const char *name,
                         gv_category_t cat, gv_test_fn fn);
void sigma_giv_run_all(void);
void sigma_giv_run_suite(const char *suite);
void sigma_giv_run_category(gv_category_t cat);
gv_stats_t sigma_giv_stats(void);
void sigma_giv_report(void);

/* Register all built-in suite tests */
void sigma_giv_register_all(void);

#endif /* SIGMA_GIV_H */
