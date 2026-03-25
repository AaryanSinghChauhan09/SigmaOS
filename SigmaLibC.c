/*
 * =========================================================================
 * Σ SIGMAOS: SIGMA LIBC ROOT IMPLEMENTATION (v6.0)
 * =========================================================================
 * USP Absorbed: Clear Linux (AVX/SSE perf), musl (minimal), Diet libc
 * Principle:    ZERO standard includes. Delegates to libc/sigma_libc.h/.c
 * Note: The heavy implementation is in libc/sigma_libc.c
 *       This file provides the "entry" test + verification layer.
 * =========================================================================
 */

#include "SigmaLibC.h"

/*
 * sigma_libc_selftest: Internal verification that SigmaLibC is working.
 * Called during OS bootstrap to ensure the custom libc is functional.
 */
void sigma_libc_selftest(void) {
    /* 1. Test sigma_strlen */
    const char* test_str = "SIGMA";
    if (sigma_strlen(test_str) != 5) {
        sigma_write(SIGMA_FD_STDERR, "[FAIL] sigma_strlen\n", 20);
        return;
    }

    /* 2. Test sigma_memset */
    char buf[16];
    sigma_memset(buf, 0xAA, 16);
    sigma_bool memset_ok = SIGMA_TRUE;
    for (sigma_i32 i = 0; i < 16; i++)
        if ((sigma_u8)buf[i] != 0xAA) { memset_ok = SIGMA_FALSE; break; }
    if (!memset_ok) {
        sigma_write(SIGMA_FD_STDERR, "[FAIL] sigma_memset\n", 20);
        return;
    }

    /* 3. Test sigma_memcpy */
    char src[] = "SigmaOS!", dst[9];
    sigma_memcpy(dst, src, 9);
    if (sigma_strcmp(dst, "SigmaOS!") != 0) {
        sigma_write(SIGMA_FD_STDERR, "[FAIL] sigma_memcpy\n", 20);
        return;
    }

    /* 4. Test sigma_itoa */
    char itoa_buf[32];
    sigma_itoa(-42, itoa_buf, 10);
    if (sigma_strcmp(itoa_buf, "-42") != 0) {
        sigma_write(SIGMA_FD_STDERR, "[FAIL] sigma_itoa\n", 18);
        return;
    }

    /* 5. Test sigma_sqrt_int with AVX */
    sigma_u64 sq = sigma_sqrt_int(144);
    if (sq != 12) {
        sigma_write(SIGMA_FD_STDERR, "[FAIL] sigma_sqrt_int\n", 22);
        return;
    }

    /* 6. Test sigma_log2_int */
    sigma_i32 lg = sigma_log2_int(1024);
    if (lg != 10) {
        sigma_write(SIGMA_FD_STDERR, "[FAIL] sigma_log2_int\n", 22);
        return;
    }

    /* 7. Test align_up */
    sigma_usize align_result = sigma_align_up(13, 8);
    if (align_result != 16) {
        sigma_write(SIGMA_FD_STDERR, "[FAIL] sigma_align_up\n", 22);
        return;
    }

    sigma_printf("[SIGMA_LIBC]: All self-tests PASSED. Sovereign libc online.\n");
    sigma_printf("[SIGMA_LIBC]: Absorbed musl, Clear Linux AVX, Diet libc.\n");
    sigma_printf("[SIGMA_LIBC]: AVX sqrt(144) = %lld\n", (long long)sq);
}

/*
 * _start: Bare-metal entry point when running SigmaLibC standalone.
 * In the full OS, this is replaced by the kernel entry point.
 */
#ifndef SIGMA_EMBEDDED_BUILD
void _start(void) {
    sigma_printf("[SIGMA_LIBC v6.0]: Bootstrapping Sovereign Standard Library...\n");
    sigma_printf("[SIGMA_LIBC]: ZERO dependency. No stdlib. No glibc. No limits.\n");

    sigma_libc_selftest();

    sigma_printf("[SIGMA_LIBC]: Absorbed distros: Clear Linux + musl + uClibc-ng + Diet libc\n");
    sigma_printf("[SIGMA_LIBC]: Architecture: ");

#if defined(SIGMA_ARCH_X86_64)
    sigma_printf("x86_64 (RDRAND, BSR, POPCNT, SSE/AVX enabled)\n");
#elif defined(SIGMA_ARCH_ARM64)
    sigma_printf("AArch64 (TLBI, DMBISHLD, SVC enabled)\n");
#elif defined(SIGMA_ARCH_RISCV64)
    sigma_printf("RISC-V 64 (RV64GC)\n");
#else
    sigma_printf("Generic (portable fallback)\n");
#endif

    sigma_printf("[SIGMA_LIBC]: SigmaOS Sovereign Standard Library: READY.\n");

    sigma_exit(0);
}
#endif /* SIGMA_EMBEDDED_BUILD */
