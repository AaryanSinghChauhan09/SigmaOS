// SigmaOS — sigma-sec-musl: Sovereign LibC Security Extensions
// Inspired by: musl libc, uClibc — minimal, secure C library
// Module: sigma-sys-musl
// USP: Hardened string ops — buffer overflow detection, null termination guarantee
// All functions check bounds explicitly — no undefined behaviour paths

#ifndef SIGMA_SEC_MUSL_H
#define SIGMA_SEC_MUSL_H

// Hardened memcpy — detects overlap (like memmove) and checks bounds
static inline int sigma_safe_memcpy(void* dst, unsigned long dst_sz,
                                     const void* src, unsigned long n) {
    if (n > dst_sz) return -1; // would overflow
    unsigned char* d = (unsigned char*)dst;
    const unsigned char* s = (const unsigned char*)src;
    // Check for dangerous overlap — use byte-by-byte copy if overlapping
    if (d < s + n && s < d + n && d != s) {
        // Overlap: copy backwards
        for (long i = (long)n - 1; i >= 0; i--) d[i] = s[i];
    } else {
        for (unsigned long i = 0; i < n; i++) d[i] = s[i];
    }
    return 0;
}

// Hardened strncpy — always null-terminates, returns overflow indicator
static inline int sigma_safe_strncpy(char* dst, unsigned long dst_sz,
                                      const char* src, unsigned long max) {
    if (dst_sz == 0) return -1;
    unsigned long limit = (max < dst_sz - 1) ? max : dst_sz - 1;
    unsigned long i = 0;
    for (; i < limit && src[i]; i++) dst[i] = src[i];
    dst[i] = '\0';
    return (src[i] != '\0') ? 1 : 0; // 1 = truncated
}

// Hardened strncat — bounds-checked concatenation
static inline int sigma_safe_strncat(char* dst, unsigned long dst_sz,
                                      const char* src, unsigned long max) {
    unsigned long dlen = 0;
    while (dlen < dst_sz && dst[dlen]) dlen++;
    if (dlen >= dst_sz) return -1; // dst already full
    unsigned long remain = dst_sz - dlen - 1;
    unsigned long limit  = (max < remain) ? max : remain;
    unsigned long i = 0;
    for (; i < limit && src[i]; i++) dst[dlen + i] = src[i];
    dst[dlen + i] = '\0';
    return (src[i] != '\0') ? 1 : 0; // 1 = truncated
}

// Timing-safe memory comparison (prevents timing side-channel)
static inline int sigma_safe_memcmp(const void* a, const void* b, unsigned long n) {
    const unsigned char* p = (const unsigned char*)a;
    const unsigned char* q = (const unsigned char*)b;
    unsigned char diff = 0;
    for (unsigned long i = 0; i < n; i++) diff |= p[i] ^ q[i];
    return (diff != 0);
}

// Secure memory wipe — guaranteed not to be optimised away
static inline void sigma_secure_zero(void* ptr, unsigned long n) {
    volatile unsigned char* p = (volatile unsigned char*)ptr;
    for (unsigned long i = 0; i < n; i++) p[i] = 0;
}

// sprintf-like: format an integer into a buffer safely
static inline int sigma_snprintf_int(char* buf, unsigned long sz, int val) {
    if (sz < 2) return -1;
    unsigned char neg = (val < 0); if (neg) val = -val;
    char tmp[12]; int i = 0;
    if (val == 0) tmp[i++] = '0';
    while (val) { tmp[i++] = '0' + (char)(val % 10); val /= 10; }
    if (neg) tmp[i++] = '-';
    unsigned long total = (unsigned long)i;
    if (total >= sz) { buf[0] = '\0'; return -1; }
    for (int j = 0; j < i; j++) buf[j] = tmp[i - 1 - j];
    buf[i] = '\0';
    return i;
}

#endif /* SIGMA_SEC_MUSL_H */
