/**
 * Σ SIGMAOS PROFESSIONAL KERNELS (v160.0)
 * Low-Level, Zero-Dependency, User-Defined Functions for Cyber Security.
 * ACHIEVES PURE PERFORMANCE WITHOUT STANDARD LIBRARIES.
 */

#include "../SovereignOSBasicsZenith.h"

/**
 * SIGMA_PATTERN_AUDIT
 * Manual string pattern matching without string.h.
 */
int sigma_str_contains(const char* str, const char* pattern) {
    if (!*pattern) return 1;
    for (const char* p = str; *p; p++) {
        const char* p1 = p;
        const char* p2 = pattern;
        while (*p1 && *p2 && *p1 == *p2) {
            p1++;
            p2++;
        }
        if (!*p2) return 1;
    }
    return 0;
}
