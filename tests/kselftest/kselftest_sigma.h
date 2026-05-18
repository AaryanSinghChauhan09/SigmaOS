/*
 * =========================================================================
 * Σ SIGMAOS: KSELFTEST REGRESSION TESTING FRAMEWORK
 * =========================================================================
 * Inspired by Linux's kselftest framework.
 * Zero-dependency, bare-metal diagnostic validation suites.
 * =========================================================================
 */

#ifndef KSELFTEST_SIGMA_H
#define KSELFTEST_SIGMA_H

#include <stdio.h>
#include <stdarg.h>

#define KSFT_PASS 0
#define KSFT_FAIL 1
#define KSFT_XFAIL 2
#define KSFT_XPASS 3
#define KSFT_SKIP 4

static int ksft_pass_cnt = 0;
static int ksft_fail_cnt = 0;

inline void ksft_print_header() {
    printf("====================================================\n");
    printf("Σ SIGMAOS KSELFTEST REGRESSION TEST SUITE\n");
    printf("====================================================\n");
}

inline void ksft_test_result(int condition, const char* fmt, ...) {
    va_list args;
    va_start(args, fmt);
    if (condition) {
        printf("[PASS] ");
        vprintf(fmt, args);
        printf("\n");
        ksft_pass_cnt++;
    } else {
        printf("[FAIL] ");
        vprintf(fmt, args);
        printf("\n");
        ksft_fail_cnt++;
    }
    va_end(args);
}

inline void ksft_print_summary() {
    printf("\n====================================================\n");
    printf("KSELFTEST SUMMARY:\n");
    printf("  Total Passed: %d\n", ksft_pass_cnt);
    printf("  Total Failed: %d\n", ksft_fail_cnt);
    printf("====================================================\n");
}

#endif // KSELFTEST_SIGMA_H
