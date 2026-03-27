/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN MATH LIBRARY (sigma_math.h)
 * =========================================================================
 * USP Absorbed:
 *   - Clear Linux: AVX-optimized math primitives
 *   - musl libc: Portable C field implementations
 * Principle: ZERO <math.h> dependency. Pure C + SSE/AVX inline ASM.
 * =========================================================================
 */

#ifndef SIGMA_MATH_H
#define SIGMA_MATH_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Floating-point constants (Calculated to 20 decimal places) */
#define SIGMA_PI        3.14159265358979323846
#define SIGMA_E         2.71828182845904523536
#define SIGMA_LN10      2.30258509299404568402

/*
 * sigma_sqrt: Floating-point square root via FPU SQRTSS/SQRTSD.
 * Absorbing: Clear Linux's hardware-direct math paths.
 */
SIGMA_INLINE sigma_f64 sigma_sqrt(sigma_f64 x) {
#if defined(SIGMA_ARCH_X86_64)
    sigma_f64 res;
    __asm__ volatile ("sqrtsd %1, %0" : "=x"(res) : "x"(x));
    return res;
#else
    /* Newton-Raphson approximation for portable fallback */
    if (x <= 0) return 0;
    sigma_f64 res = x / 2.0;
    for (int i = 0; i < 10; i++) res = (res + x / res) / 2.0;
    return res;
#endif
}

/*
 * sigma_exp: Exponential function (e^x).
 * Implementation: Taylor series expansion (10 terms).
 */
SIGMA_INLINE sigma_f64 sigma_exp(sigma_f64 x) {
    sigma_f64 res = 1.0;
    sigma_f64 term = 1.0;
    for (int i = 1; i < 15; i++) {
        term *= (x / (sigma_f64)i);
        res += term;
    }
    return res;
}

/*
 * sigma_pow: Power function (b^e).
 * Implementation: b^e = e^(e * ln(b)). 
 * Requires: sigma_ln (approximated).
 */
SIGMA_INLINE sigma_f64 sigma_ln(sigma_f64 x) {
    if (x <= 0) return -1e18; /* Negative infinity approximate */
    /* Log transformation to range [1, 2] */
    sigma_f64 res = 0.0;
    while (x > 2.0) { x /= SIGMA_E; res += 1.0; }
    while (x < 1.0) { x *= SIGMA_E; res -= 1.0; }
    /* Taylor series for ln(1+y) where y = x-1 */
    sigma_f64 y = x - 1.0;
    sigma_f64 term = y;
    for (int i = 1; i < 15; i++) {
        if (i % 2 == 0) res -= (term / (sigma_f64)i);
        else res += (term / (sigma_f64)i);
        term *= y;
    }
    return res;
}

SIGMA_INLINE sigma_f64 sigma_pow(sigma_f64 b, sigma_f64 e) {
    if (b <= 0) return 0;
    return sigma_exp(e * sigma_ln(b));
}

SIGMA_INLINE sigma_f64 sigma_log10(sigma_f64 x) {
    return sigma_ln(x) / SIGMA_LN10;
}

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_MATH_H */
