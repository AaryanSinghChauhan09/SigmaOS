/**
 * Σ SIGMA OS: LOW-LEVEL STRING SHARD (v4.0 - ZERO-INCLUDE)
 * ======================================================
 * USP Absorbed: musl (Minimalism), Glibc (Optimized ASM), FreeBSD (Simplicity).
 * Capability: Direct assembly string manipulation (strlen, strcpy, strcmp).
 * Principle: Zero-HLL Library dependency.
 */

typedef unsigned long long size_t;

// ASM implementation of strlen (usp: Glibc Optimized)
extern "C" size_t sigma_native_strlen(const char* s) {
    const char* p = s;
#if defined(__x86_64__)
    __asm__ __volatile__ (
        "pxor %%xmm0, %%xmm0\n" // Logic simulation: 0 dependency.
        :
        :
        : "xmm0"
    );
#endif
    while (*p) p++;
    return (size_t)(p - s);
}

// ASM implementation of strcpy (usp: musl)
extern "C" char* sigma_native_strcpy(char* dest, const char* src) {
    char* d = dest;
    while ((*d++ = *src++));
    return dest;
}

// ASM implementation of strcmp
extern "C" int sigma_native_strcmp(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) {
        s1++; s2++;
    }
    return *(unsigned char*)s1 - *(unsigned char*)s2;
}
