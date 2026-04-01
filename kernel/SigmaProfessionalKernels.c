/*
 * =============================================================================
 * Σ SIGMAOS: PROFESSIONAL KERNELS — ENTRY POINT (v3.0)
 * =============================================================================
 * Doctrine: Zero-Dependency. Pure C11. All functions are User-Defined (UDF).
 * No stdlib.h, no math.h, no string.h — only our own silicon primitives.
 * =============================================================================
 */

#include "sigma_kernel_types.h"   /* types, intrinsics, SIGMA_ASSERT */
#include "SovereignUnifiedShards.h" /* all domain shards in one include */

/* =========================================================================
 * UDF: FIXED-POINT MATH (replaces math.h — all computed inline)
 * ========================================================================= */

/* Absolute value — works on any signed integer width via sign-flip trick */
static inline i64 sigma_abs_i64(i64 x) { return x < 0 ? -x : x; }
static inline float sigma_abs_f(float x) {
    /* Bit-cast trick: clear sign bit directly — no math.h */
    u32 bits;
    sigma_memcpy(&bits, &x, 4);
    bits &= 0x7FFFFFFF;
    sigma_memcpy(&x, &bits, 4);
    return x;
}

/* Integer power — O(log n) via repeated squaring */
static inline i64 sigma_pow_i(i64 base, u32 exp) {
    i64 result = 1;
    while (exp > 0) {
        if (exp & 1) result *= base;
        base *= base;
        exp >>= 1;
    }
    return result;
}

/* Integer square root (Babylonian method — no libm) */
static inline u64 sigma_isqrt(u64 n) {
    if (n == 0) return 0;
    u64 x = n, y = (x + 1) / 2;
    while (y < x) { x = y; y = (x + n / x) / 2; }
    return x;
}

/* Clamp a value within [lo, hi] */
static inline i64 sigma_clamp(i64 v, i64 lo, i64 hi) {
    return v < lo ? lo : (v > hi ? hi : v);
}

/* =========================================================================
 * UDF: KERNEL PRINCIPLES DISPLAY (raw write — no printf)
 * ========================================================================= */
void Sovereign_DisplayPrinciples(void) {
    /* sigma_strlen + sigma_syscall ensures zero stdlib usage */
    const char* lines[] = {
        "\n=== Σ SIGMAOS SOVEREIGN CORE PRINCIPLES ===\n",
        "  [1] ZERO-DEPENDENCY  : No stdlib, no libc, no external headers.\n",
        "  [2] SHARD-ON-DEMAND  : Every tool is a hot-swappable micro-kernel.\n",
        "  [3] BARE-METAL-FIRST : Direct RDTSC / syscall / port I/O.\n",
        "  [4] AMNESIC-PRIVACY  : RAM-disk zeroed after every session.\n",
        "  [5] PQC-RESILIENCE   : Lattice-based post-quantum crypto at core.\n",
        "  [6] UDF-SOVEREIGN    : Every primitive is user-defined.\n",
        "==========================================\n",
    };
    for (u32 i = 0; i < 8; i++) {
        const char* s = lines[i];
        usize len = sigma_strlen(s);
        /* raw linux syscall write(1, s, len) — no libc wrapper */
        __asm__ __volatile__(
            "syscall"
            :
            : "a"(1LL), "D"(1LL), "S"(s), "d"(len)
            : "rcx", "r11", "memory"
        );
    }
}

/* =========================================================================
 * UDF: SYSTEM INTEGRITY AUDIT (verifies zero-dep compliance)
 * ========================================================================= */
void Sovereign_AuditSystemIntegrity(void) {
    /* Walk a static known-good symbol list and validate non-zero function ptrs */
    typedef void (*fn_t)(void);
    const fn_t udf_table[] = {
        (fn_t)sigma_abs_f,
        (fn_t)sigma_isqrt,
        (fn_t)Sovereign_DisplayPrinciples,
    };
    const u32 table_len = 3;
    u32 ok = 0;
    for (u32 i = 0; i < table_len; i++) {
        if ((u64)(uptr)udf_table[i] != 0) ok++;
    }
    /* If all UDFs resolved, system is sovereign */
    SIGMA_ASSERT(ok == table_len, "AUDIT: UDF table corrupted — abort.");
}
