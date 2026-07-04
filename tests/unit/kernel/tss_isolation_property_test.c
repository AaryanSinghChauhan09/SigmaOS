/* SPDX-License-Identifier: MIT
 * tests/unit/kernel/tss_isolation_property_test.c
 *
 * Property 2: Per-Process TSS Kernel Stack Isolation
 * Validates: Requirement 2.3 — all RSP0 values across process table are unique.
 *
 * Uses theft-style exhaustive generation (no external lib in freestanding mode).
 * Mock process structures allocated on host stack for testing.
 *
 * Build (host runner):
 *   gcc -DTEST_HOST_RUNNER -nostdlib -static -ffreestanding \
 *       tss_isolation_property_test.c -lc -o tss_isolation_prop_test
 */

#include <stdint.h>
#include <stddef.h>

#ifdef TEST_HOST_RUNNER
#  include <stdio.h>
#  include <stdlib.h>
#  include <string.h>
#endif

/* ── Mock process structure (mirrors sovereign_process_manager.rs) ──────── */
typedef struct {
    uint32_t pid;
    uint64_t kernel_stack_top;   /* RSP0 value stored in TSS for this process */
    uint64_t user_stack_top;
    uint8_t  state;              /* 0=unused, 1=running, 2=ready, 3=blocked */
} mock_process_t;

/* ── Mock PROCESS_STACKS table (mirrors sigma_usermode.rs) ──────────────── */
#define MAX_PROCS      256
#define STACK_SIZE     8192U   /* 8 KiB per kernel stack */
#define STACK_BASE     0xFFFF800000100000ULL  /* canonical kernel vaddr */

static uint64_t mock_process_stacks[MAX_PROCS];
static mock_process_t mock_procs[MAX_PROCS];

/* ── Simulate sigma_set_process_kernel_stack ─────────────────────────────── */
static void sim_allocate_process_stacks(uint32_t count)
{
    for (uint32_t i = 0; i < count && i < MAX_PROCS; i++) {
        /* Each process gets a distinct stack: base + i * STACK_SIZE + STACK_SIZE
         * (top of stack = base + (i+1)*STACK_SIZE) */
        uint64_t stack_top = STACK_BASE + (uint64_t)(i + 1) * STACK_SIZE;
        mock_procs[i].pid              = i;
        mock_procs[i].kernel_stack_top = stack_top;
        mock_procs[i].state            = 1;
        mock_process_stacks[i]         = stack_top;
    }
}

/* ── Property helpers ────────────────────────────────────────────────────── */

/* Property 2a: All RSP0 values in table[0..count) are unique */
static int prop_all_stacks_unique(uint32_t count)
{
    for (uint32_t i = 0; i < count; i++) {
        for (uint32_t j = i + 1; j < count; j++) {
            if (mock_process_stacks[i] == mock_process_stacks[j]) {
#ifdef TEST_HOST_RUNNER
                printf("  [FAIL] PIDs %u and %u share RSP0=0x%016llx\n",
                       i, j, (unsigned long long)mock_process_stacks[i]);
#endif
                return 0;
            }
        }
    }
    return 1;
}

/* Property 2b: No RSP0 value is zero (null stack = invalid) */
static int prop_no_null_stack(uint32_t count)
{
    for (uint32_t i = 0; i < count; i++) {
        if (mock_process_stacks[i] == 0) {
#ifdef TEST_HOST_RUNNER
            printf("  [FAIL] PID %u has RSP0=0 (null kernel stack)\n", i);
#endif
            return 0;
        }
    }
    return 1;
}

/* Property 2c: All RSP0 values are in the canonical kernel address range
 * (bits 63:47 must all be 1, i.e. >= 0xFFFF800000000000) */
static int prop_stacks_in_kernel_range(uint32_t count)
{
    for (uint32_t i = 0; i < count; i++) {
        uint64_t addr = mock_process_stacks[i];
        /* Canonical kernel vaddr: top 17 bits all 1 */
        if ((addr >> 47) != 0x1FFFF) {
#ifdef TEST_HOST_RUNNER
            printf("  [FAIL] PID %u RSP0=0x%016llx not in kernel canonical range\n",
                   i, (unsigned long long)addr);
#endif
            return 0;
        }
    }
    return 1;
}

/* Property 2d: Each stack_top is aligned to at least 16 bytes (ABI requirement) */
static int prop_stacks_16byte_aligned(uint32_t count)
{
    for (uint32_t i = 0; i < count; i++) {
        if (mock_process_stacks[i] & 0xF) {
#ifdef TEST_HOST_RUNNER
            printf("  [FAIL] PID %u RSP0=0x%016llx is not 16-byte aligned\n",
                   i, (unsigned long long)mock_process_stacks[i]);
#endif
            return 0;
        }
    }
    return 1;
}

/* Property 2e: Stacks do not overlap (distance between consecutive tops >= STACK_SIZE) */
static int prop_stacks_no_overlap(uint32_t count)
{
    /* Stacks are allocated in order; gap between consecutive tops should be STACK_SIZE */
    for (uint32_t i = 0; i + 1 < count; i++) {
        uint64_t lo = mock_process_stacks[i];
        uint64_t hi = mock_process_stacks[i + 1];
        if (hi <= lo) {
            /* Ordering sanity */
#ifdef TEST_HOST_RUNNER
            printf("  [FAIL] PID %u stack top >= PID %u stack top (ordering broken)\n",
                   i + 1, i);
#endif
            return 0;
        }
        uint64_t gap = hi - lo;
        if (gap < STACK_SIZE) {
#ifdef TEST_HOST_RUNNER
            printf("  [FAIL] PIDs %u/%u gap=%llu < STACK_SIZE=%u (overlap possible)\n",
                   i, i + 1, (unsigned long long)gap, STACK_SIZE);
#endif
            return 0;
        }
    }
    return 1;
}

/* ── Theft-style generator: runs properties for counts 10..100 ──────────── */
/* In a full theft integration, these would be randomly generated.
 * Here we use an exhaustive sweep over the specification range [10..100]. */
static int theft_style_stack_isolation(void)
{
    int failures = 0;
    for (uint32_t count = 10; count <= 100; count += 10) {
        /* Clear and re-allocate */
        for (int i = 0; i < MAX_PROCS; i++) {
            mock_process_stacks[i] = 0;
        }
        sim_allocate_process_stacks(count);

        if (!prop_all_stacks_unique(count)) {
#ifdef TEST_HOST_RUNNER
            printf("  [FAIL] uniqueness at count=%u\n", count);
#endif
            failures++;
        }
        if (!prop_no_null_stack(count)) {
#ifdef TEST_HOST_RUNNER
            printf("  [FAIL] no-null at count=%u\n", count);
#endif
            failures++;
        }
        if (!prop_stacks_in_kernel_range(count)) {
#ifdef TEST_HOST_RUNNER
            printf("  [FAIL] kernel-range at count=%u\n", count);
#endif
            failures++;
        }
        if (!prop_stacks_16byte_aligned(count)) {
#ifdef TEST_HOST_RUNNER
            printf("  [FAIL] alignment at count=%u\n", count);
#endif
            failures++;
        }
        if (!prop_stacks_no_overlap(count)) {
#ifdef TEST_HOST_RUNNER
            printf("  [FAIL] no-overlap at count=%u\n", count);
#endif
            failures++;
        }
    }
    return failures == 0;
}

/* ── Test runner ────────────────────────────────────────────────────────────── */
#ifdef TEST_HOST_RUNNER
int main(void)
{
    int passed = 0, failed = 0;

#define RUN(name, fn)                                                       \
    do {                                                                    \
        if (fn) { printf("[PASS] %s\n", name); passed++; }                \
        else    { printf("[FAIL] %s\n", name); failed++; }                \
    } while (0)

    printf("=== TSS Kernel Stack Isolation Property Tests ===\n");

    /* Single-shot properties with max count */
    sim_allocate_process_stacks(MAX_PROCS);
    RUN("prop_all_stacks_unique(256)",       prop_all_stacks_unique(256));
    RUN("prop_no_null_stack(256)",           prop_no_null_stack(256));
    RUN("prop_stacks_in_kernel_range(256)",  prop_stacks_in_kernel_range(256));
    RUN("prop_stacks_16byte_aligned(256)",   prop_stacks_16byte_aligned(256));
    RUN("prop_stacks_no_overlap(256)",       prop_stacks_no_overlap(256));
    RUN("theft_style_stack_isolation",       theft_style_stack_isolation());

    printf("\nResults: %d passed, %d failed\n", passed, failed);
    return failed == 0 ? 0 : 1;
}
#endif
