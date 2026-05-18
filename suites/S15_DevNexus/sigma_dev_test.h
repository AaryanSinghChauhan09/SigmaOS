// SigmaOS — sigma-dev-test: Native Unit Test Framework
// Inspired by: Catch2, GoogleTest, Unity (C testing)
// Module: sigma-dev-test
// USP: Zero dynamic allocation, zero macros-as-magic — pure C static framework
// Each test is a named function pointer — no test runner binary needed

#ifndef SIGMA_DEV_TEST_H
#define SIGMA_DEV_TEST_H

#include "libc/sigma_libc.h"

#define SIGMA_TEST_MAX        128
#define SIGMA_TEST_NAME_LEN    48

typedef enum SigmaTestResult {
    TEST_PASS  = 0,
    TEST_FAIL  = 1,
    TEST_SKIP  = 2
} SigmaTestResult;

typedef SigmaTestResult (*sigma_test_fn)(void);

typedef struct SigmaTest {
    char          name[SIGMA_TEST_NAME_LEN];
    sigma_test_fn fn;
    SigmaTestResult result;
} SigmaTest;

typedef struct SigmaTestSuite {
    SigmaTest    tests[SIGMA_TEST_MAX];
    unsigned int count;
    unsigned int passed;
    unsigned int failed;
    unsigned int skipped;
} SigmaTestSuite;

// Test assertion macros — expand to inline return
#define SIGMA_ASSERT(cond) \
    do { if (!(cond)) return TEST_FAIL; } while(0)
#define SIGMA_ASSERT_EQ(a, b) \
    do { if ((a) != (b)) return TEST_FAIL; } while(0)
#define SIGMA_ASSERT_NE(a, b) \
    do { if ((a) == (b)) return TEST_FAIL; } while(0)
#define SIGMA_ASSERT_NULL(p) \
    do { if ((p) != (void*)0) return TEST_FAIL; } while(0)
#define SIGMA_ASSERT_NOT_NULL(p) \
    do { if ((p) == (void*)0) return TEST_FAIL; } while(0)
#define SIGMA_SKIP() return TEST_SKIP

static inline void test_suite_init(SigmaTestSuite* s) {
    s->count = s->passed = s->failed = s->skipped = 0;
}

static inline int test_register(SigmaTestSuite* s, const char* name, sigma_test_fn fn) {
    if (s->count >= SIGMA_TEST_MAX) return -1;
    SigmaTest* t = &s->tests[s->count++];
    for (int i = 0; i < SIGMA_TEST_NAME_LEN - 1 && name[i]; i++) t->name[i] = name[i];
    t->fn = fn; t->result = TEST_SKIP;
    return 0;
}

static inline void test_run_all(SigmaTestSuite* s) {
    for (unsigned int i = 0; i < s->count; i++) {
        SigmaTest* t = &s->tests[i];
        t->result = t->fn ? t->fn() : TEST_SKIP;
        if (t->result == TEST_PASS)      { s->passed++;  sigma_kprint("[PASS] "); }
        else if (t->result == TEST_FAIL) { s->failed++;  sigma_kprint("[FAIL] "); }
        else                             { s->skipped++; sigma_kprint("[SKIP] "); }
        sigma_kprint(t->name);
        sigma_kprint("\n");
    }
    sigma_kprint("\n--- Test Report ---\n");
    sigma_kprint("Passed: ");  sigma_kprint_int((int)s->passed);
    sigma_kprint("\nFailed: "); sigma_kprint_int((int)s->failed);
    sigma_kprint("\nSkipped: ");sigma_kprint_int((int)s->skipped);
    sigma_kprint("\n");
}

static inline int test_all_passed(SigmaTestSuite* s) {
    return s->failed == 0 && s->count > 0;
}

#endif /* SIGMA_DEV_TEST_H */
