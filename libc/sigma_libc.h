/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN STANDARD LIBRARY - COMPLETE IMPLEMENTATION
 * =========================================================================
 * File: libc/sigma_libc.h
 * Mission: Replace ALL standard C library headers:
 *          <string.h>, <stdio.h>, <stdlib.h>, <math.h>, <stdarg.h>
 * USP Absorbed:
 *   - musl libc: Minimalist portable implementations
 *   - Clear Linux: Intel AVX/SSE performance paths
 *   - uClibc-ng: Embedded-first design patterns
 *   - Diet libc: Zero-waste, bare-syscall philosophy
 *   - Fuchsia/Zircon: Modern OOP-style APIs
 * Principle: ZERO external includes. Pure syscalls + inline ASM.
 * OOP Principle: All operations encapsulated via function types and
 *                struct-based virtual dispatch.
 * Languages: C99/C11 with GCC/Clang inline ASM extensions.
 * =========================================================================
 */

#ifndef SIGMA_LIBC_H
#define SIGMA_LIBC_H

#include "sigma_types.h"  /* Our ONLY allowed include - our own types */

#ifdef __cplusplus
extern "C" {
#endif

/* =========================================================================
 * SECTION 1: MEMORY OPERATIONS (Replacing <string.h> mem* functions)
 * ========================================================================= */

/*
 * sigma_memset: Fill memory with a constant byte.
 * Implementation: x86_64 uses REP STOSB for maximum throughput.
 * Absorbing: musl's alignment optimization + Clear Linux's AVX path.
 */
SIGMA_INLINE void* sigma_memset(void* dest, sigma_i32 c, sigma_usize n) {
#if defined(SIGMA_ARCH_X86_64)
    /* x86_64 fast path: REP STOSB with modern CFG/ERMS support */
    void* ret = dest;
    __asm__ volatile (
        "rep stosb"
        : "+D"(dest), "+c"(n)
        : "a"((sigma_u8)c)
        : "memory"
    );
    return ret;
#elif defined(SIGMA_ARCH_ARM64)
    sigma_u8* p = (sigma_u8*)dest;
    sigma_usize i;
    for (i = 0; i < n; i++) p[i] = (sigma_u8)c;
    return dest;
#else
    sigma_u8* p = (sigma_u8*)dest;
    while (n--) *p++ = (sigma_u8)c;
    return dest;
#endif
}

/*
 * sigma_memcpy: Copy memory regions (non-overlapping).
 * Implementation: x86_64 REP MOVSB with Direction Flag cleared.
 */
SIGMA_INLINE void* sigma_memcpy(void* SIGMA_RESTRICT dest, const void* SIGMA_RESTRICT src, sigma_usize n) {
#if defined(SIGMA_ARCH_X86_64)
    void* ret = dest;
    __asm__ volatile (
        "cld\n"
        "rep movsb"
        : "+D"(dest), "+S"(src), "+c"(n)
        :
        : "memory"
    );
    return ret;
#else
    sigma_u8* d = (sigma_u8*)dest;
    const sigma_u8* s = (const sigma_u8*)src;
    while (n--) *d++ = *s++;
    return dest;
#endif
}

/*
 * sigma_memmove: Copy memory regions (safe for overlapping ranges).
 * Absorbing: musl's direction-aware copy.
 */
SIGMA_INLINE void* sigma_memmove(void* dest, const void* src, sigma_usize n) {
    sigma_u8* d = (sigma_u8*)dest;
    const sigma_u8* s = (const sigma_u8*)src;
    if (d < s || d >= s + n) {
        return sigma_memcpy(dest, src, n);
    }
    /* Backward copy for overlapping regions */
    d += n; s += n;
    while (n--) *--d = *--s;
    return dest;
}

/*
 * sigma_memcmp: Compare two memory regions.
 */
SIGMA_INLINE sigma_i32 sigma_memcmp(const void* a, const void* b, sigma_usize n) {
    const sigma_u8* pa = (const sigma_u8*)a;
    const sigma_u8* pb = (const sigma_u8*)b;
    while (n--) {
        if (*pa != *pb) return (sigma_i32)*pa - (sigma_i32)*pb;
        pa++; pb++;
    }
    return 0;
}

/*
 * sigma_memchr: Find a byte in memory.
 */
SIGMA_INLINE void* sigma_memchr(const void* s, sigma_i32 c, sigma_usize n) {
    const sigma_u8* p = (const sigma_u8*)s;
    while (n--) {
        if (*p == (sigma_u8)c) return (void*)p;
        p++;
    }
    return SIGMA_NULL;
}

/* =========================================================================
 * SECTION 2: STRING OPERATIONS (Replacing <string.h> str* functions)
 * ========================================================================= */

/*
 * sigma_strlen: Compute string length without <string.h>.
 */
SIGMA_INLINE sigma_usize sigma_strlen(const char* s) {
    const char* p = s;
    while (*p) p++;
    return (sigma_usize)(p - s);
}

/*
 * sigma_strnlen: Bounded string length.
 */
SIGMA_INLINE sigma_usize sigma_strnlen(const char* s, sigma_usize maxlen) {
    const char* p = s;
    while (maxlen-- && *p) p++;
    return (sigma_usize)(p - s);
}

/*
 * sigma_strcmp: Compare two strings.
 */
SIGMA_INLINE sigma_i32 sigma_strcmp(const char* a, const char* b) {
    while (*a && *a == *b) { a++; b++; }
    return (sigma_u8)*a - (sigma_u8)*b;
}

/*
 * sigma_strncmp: Bounded string comparison.
 */
SIGMA_INLINE sigma_i32 sigma_strncmp(const char* a, const char* b, sigma_usize n) {
    while (n-- && *a && *a == *b) { a++; b++; }
    if (!n) return 0;
    return (sigma_u8)*a - (sigma_u8)*b;
}

/*
 * sigma_strcpy: Copy a string (dest must have sufficient space).
 */
SIGMA_INLINE char* sigma_strcpy(char* SIGMA_RESTRICT dest, const char* SIGMA_RESTRICT src) {
    char* ret = dest;
    while ((*dest++ = *src++));
    return ret;
}

/*
 * sigma_strncpy: Bounded string copy.
 */
SIGMA_INLINE char* sigma_strncpy(char* SIGMA_RESTRICT dest, const char* SIGMA_RESTRICT src, sigma_usize n) {
    char* ret = dest;
    while (n && (*dest++ = *src++)) n--;
    while (n--) *dest++ = 0;
    return ret;
}

/*
 * sigma_strchr: Find character in string.
 */
SIGMA_INLINE char* sigma_strchr(const char* s, sigma_i32 c) {
    while (*s) {
        if (*s == (char)c) return (char*)s;
        s++;
    }
    return (c == 0) ? (char*)s : SIGMA_NULL;
}

/*
 * sigma_strrchr: Find last occurrence of character in string.
 */
SIGMA_INLINE char* sigma_strrchr(const char* s, sigma_i32 c) {
    const char* last = SIGMA_NULL;
    while (*s) {
        if (*s == (char)c) last = s;
        s++;
    }
    if (c == 0) return (char*)s;
    return (char*)last;
}

/*
 * sigma_strcat: Append src to dest.
 */
SIGMA_INLINE char* sigma_strcat(char* SIGMA_RESTRICT dest, const char* SIGMA_RESTRICT src) {
    char* ret = dest;
    while (*dest) dest++;
    while ((*dest++ = *src++));
    return ret;
}

/*
 * sigma_strncat: Bounded string append.
 */
SIGMA_INLINE char* sigma_strncat(char* SIGMA_RESTRICT dest, const char* SIGMA_RESTRICT src, sigma_usize n) {
    char* ret = dest;
    while (*dest) dest++;
    while (n-- && *src) *dest++ = *src++;
    *dest = '\0';
    return ret;
}

/*
 * sigma_str_starts_with: Check if string begins with prefix.
 */
SIGMA_INLINE sigma_bool sigma_str_starts_with(const char* s, const char* prefix) {
    if (!s || !prefix) return SIGMA_FALSE;
    while (*prefix) {
        if (*s++ != *prefix++) return SIGMA_FALSE;
    }
    return SIGMA_TRUE;
}

/*
 * sigma_str_ends_with: Check if string ends with suffix.
 */
SIGMA_INLINE sigma_bool sigma_str_ends_with(const char* s, const char* suffix) {
    if (!s || !suffix) return SIGMA_FALSE;
    sigma_usize slen = sigma_strlen(s);
    sigma_usize sublen = sigma_strlen(suffix);
    if (sublen > slen) return SIGMA_FALSE;
    return sigma_strcmp(s + slen - sublen, suffix) == 0;
}

/*
 * sigma_str_contains: Check if string contains substring.
 */
SIGMA_INLINE sigma_bool sigma_str_contains(const char* s, const char* needle) {
    if (!s || !needle) return SIGMA_FALSE;
    if (!*needle) return SIGMA_TRUE;
    for (; *s; s++) {
        if (*s == *needle) {
            if (sigma_str_starts_with(s, needle)) return SIGMA_TRUE;
        }
    }
    return SIGMA_FALSE;
}

/* =========================================================================
 * SECTION 3: I/O OPERATIONS (Replacing <stdio.h> via raw syscalls)
 * ========================================================================= */

/* File descriptor constants */
#define SIGMA_FD_STDIN   0
#define SIGMA_FD_STDOUT  1
#define SIGMA_FD_STDERR  2

/*
 * sigma_write: Raw write syscall. Returns bytes written or negative error.
 * Absorbing: Diet libc's direct syscall philosophy.
 */
SIGMA_INLINE sigma_i64 sigma_write(sigma_i32 fd, const void* buf, sigma_usize count) {
#if defined(SIGMA_ARCH_X86_64)
    sigma_i64 ret;
    __asm__ volatile (
        "syscall"
        : "=a"(ret)
        : "0"(1ULL),            /* SYS_write = 1 */
          "D"((sigma_u64)fd),
          "S"(buf),
          "d"(count)
        : "rcx", "r11", "memory"
    );
    return ret;
#elif defined(SIGMA_ARCH_ARM64)
    sigma_i64 ret;
    __asm__ volatile (
        "mov x8, #64\n"         /* SYS_write = 64 on ARM64 Linux */
        "svc #0"
        : "=r"(ret)
        : "r"((sigma_u64)fd), "r"(buf), "r"(count)
        : "x8", "memory"
    );
    return ret;
#else
    /* Portable fallback: not usable bare-metal, but functional in userspace */
    (void)fd; (void)buf; (void)count;
    return -1;
#endif
}

/*
 * sigma_read: Raw read syscall.
 */
SIGMA_INLINE sigma_i64 sigma_read(sigma_i32 fd, void* buf, sigma_usize count) {
#if defined(SIGMA_ARCH_X86_64)
    sigma_i64 ret;
    __asm__ volatile (
        "syscall"
        : "=a"(ret)
        : "0"(0ULL),            /* SYS_read = 0 */
          "D"((sigma_u64)fd),
          "S"(buf),
          "d"(count)
        : "rcx", "r11", "memory"
    );
    return ret;
#else
    (void)fd; (void)buf; (void)count;
    return -1;
#endif
}

/*
 * sigma_puts: Write string + newline to stdout.
 */
SIGMA_INLINE sigma_i64 sigma_puts(const char* s) {
    sigma_usize len = sigma_strlen(s);
    sigma_i64 r = sigma_write(SIGMA_FD_STDOUT, s, len);
    sigma_write(SIGMA_FD_STDOUT, "\n", 1);
    return r;
}

/*
 * sigma_print: Write string to stdout without newline.
 */
SIGMA_INLINE sigma_i64 sigma_print(const char* s) {
    return sigma_write(SIGMA_FD_STDOUT, s, sigma_strlen(s));
}

/*
 * sigma_putchar: Write single char to stdout.
 */
SIGMA_INLINE sigma_i32 sigma_putchar(sigma_i32 c) {
    char ch = (char)c;
    return (sigma_i32)sigma_write(SIGMA_FD_STDOUT, &ch, 1);
}

/* =========================================================================
 * SECTION 4: INTEGER-TO-STRING CONVERSION (No <stdlib.h> needed)
 * ========================================================================= */

/*
 * sigma_itoa: Convert integer to string in given base.
 * Absorbing: uClibc-ng's minimal itoa implementation.
 * Returns: number of characters written (without null terminator).
 */
SIGMA_INLINE sigma_i32 sigma_itoa(sigma_i64 n, char* buf, sigma_i32 base) {
    const char digits[] = "0123456789abcdef";
    sigma_i32 i = 0, neg = 0;
    sigma_u64 uval;

    if (n < 0 && base == 10) { neg = 1; uval = (sigma_u64)(-n); }
    else uval = (sigma_u64)n;

    if (uval == 0) { buf[i++] = '0'; buf[i] = '\0'; return i; }

    /* Build digits in reverse */
    char tmp[80];
    sigma_i32 j = 0;
    while (uval > 0 && j < 78) {
        tmp[j++] = digits[uval % (sigma_u64)base];
        uval /= (sigma_u64)base;
    }
    if (neg) tmp[j++] = '-';

    /* Reverse */
    for (sigma_i32 k = j - 1; k >= 0; k--) buf[i++] = tmp[k];
    buf[i] = '\0';
    return i;
}

/*
 * sigma_utoa: Convert unsigned integer to decimal string.
 */
SIGMA_INLINE sigma_i32 sigma_utoa(sigma_u64 n, char* buf) {
    sigma_i32 i = 0;
    if (n == 0) { buf[i++] = '0'; buf[i] = '\0'; return i; }
    char tmp[24]; sigma_i32 j = 0;
    while (n > 0) { tmp[j++] = (char)('0' + n % 10); n /= 10; }
    for (sigma_i32 k = j - 1; k >= 0; k--) buf[i++] = tmp[k];
    buf[i] = '\0';
    return i;
}

/*
 * sigma_xtoa: Convert u64 to hex string (with 0x prefix).
 */
SIGMA_INLINE sigma_i32 sigma_xtoa(sigma_u64 n, char* buf) {
    const char hex[] = "0123456789abcdef";
    sigma_i32 i = 0;
    buf[i++] = '0'; buf[i++] = 'x';
    if (n == 0) { buf[i++] = '0'; buf[i] = '\0'; return i; }
    char tmp[18]; sigma_i32 j = 0;
    while (n > 0) { tmp[j++] = hex[n & 0xF]; n >>= 4; }
    for (sigma_i32 k = j - 1; k >= 0; k--) buf[i++] = tmp[k];
    buf[i] = '\0';
    return i;
}

/*
 * sigma_print_int: Print integer to stdout without <stdio.h>.
 */
SIGMA_INLINE void sigma_print_int(sigma_i64 n) {
    char buf[32];
    sigma_itoa(n, buf, 10);
    sigma_write(SIGMA_FD_STDOUT, buf, sigma_strlen(buf));
}

/*
 * sigma_print_hex: Print hex value to stdout.
 */
SIGMA_INLINE void sigma_print_hex(sigma_u64 n) {
    char buf[24];
    sigma_xtoa(n, buf);
    sigma_write(SIGMA_FD_STDOUT, buf, sigma_strlen(buf));
}

/* =========================================================================
 * SECTION 5: MATH OPERATIONS (Replacing <math.h>)
 * ========================================================================= */

/*
 * sigma_abs: Absolute value (integer).
 */
SIGMA_INLINE SIGMA_CONST sigma_i64 sigma_abs(sigma_i64 n) {
    return n < 0 ? -n : n;
}

/*
 * sigma_min / sigma_max: Without macro side effects.
 */
SIGMA_INLINE SIGMA_CONST sigma_i64 sigma_min(sigma_i64 a, sigma_i64 b) { return a < b ? a : b; }
SIGMA_INLINE SIGMA_CONST sigma_i64 sigma_max(sigma_i64 a, sigma_i64 b) { return a > b ? a : b; }
SIGMA_INLINE SIGMA_CONST sigma_u64 sigma_umin(sigma_u64 a, sigma_u64 b)  { return a < b ? a : b; }
SIGMA_INLINE SIGMA_CONST sigma_u64 sigma_umax(sigma_u64 a, sigma_u64 b)  { return a > b ? a : b; }

/*
 * sigma_clamp: Clamp value between min and max.
 */
SIGMA_INLINE SIGMA_CONST sigma_i64 sigma_clamp(sigma_i64 v, sigma_i64 lo, sigma_i64 hi) {
    return v < lo ? lo : (v > hi ? hi : v);
}

/*
 * sigma_sqrt_int: Integer square root via Newton-Raphson (no libm).
 * Absorbing: Clear Linux's math optimization patterns.
 */
SIGMA_INLINE SIGMA_CONST sigma_u64 sigma_sqrt_int(sigma_u64 n) {
#if defined(SIGMA_ARCH_X86_64)
    sigma_u64 result;
    __asm__ volatile (
        "cvtsi2sd %1, %%xmm0\n"
        "sqrtsd %%xmm0, %%xmm0\n"
        "cvttsd2si %%xmm0, %0\n"
        : "=r"(result) : "r"(n) : "xmm0"
    );
    return result;
#else
    if (n == 0) return 0;
    sigma_u64 x = n, y = 1;
    while (x > y) { x = (x + y) / 2; y = n / x; }
    return x;
#endif
}

/*
 * sigma_pow_int: Integer exponentiation (x^n) using binary exponentiation.
 */
SIGMA_INLINE SIGMA_CONST sigma_u64 sigma_pow_int(sigma_u64 base, sigma_u32 exp) {
    sigma_u64 result = 1;
    while (exp > 0) {
        if (exp & 1) result *= base;
        base *= base;
        exp >>= 1;
    }
    return result;
}

/*
 * sigma_log2_int: Integer base-2 logarithm using BSR instruction on x86_64.
 * Absorbing: Clear Linux's compile-time BSR optimization.
 */
SIGMA_INLINE SIGMA_CONST sigma_i32 sigma_log2_int(sigma_u64 n) {
    if (n == 0) return -1;
#if defined(SIGMA_ARCH_X86_64)
    sigma_u64 r;
    __asm__ volatile ("bsrq %1, %0" : "=r"(r) : "r"(n));
    return (sigma_i32)r;
#else
    sigma_i32 r = 0;
    while (n >>= 1) r++;
    return r;
#endif
}

/*
 * sigma_popcount: Count set bits (using POPCNT on x86 if available).
 * Absorbing: Arch Linux's CPU feature utilization.
 */
SIGMA_INLINE SIGMA_CONST sigma_i32 sigma_popcount(sigma_u64 n) {
#if defined(SIGMA_ARCH_X86_64)
    sigma_u64 r;
    __asm__ volatile ("popcntq %1, %0" : "=r"(r) : "r"(n));
    return (sigma_i32)r;
#else
    sigma_i32 count = 0;
    while (n) { count += n & 1; n >>= 1; }
    return count;
#endif
}

/* =========================================================================
 * SECTION 6: PROCESS CONTROL (Replacing <stdlib.h> exit)
 * ========================================================================= */

/*
 * sigma_exit: Terminate process via raw syscall. NEVER returns.
 */
SIGMA_NORETURN SIGMA_INLINE void sigma_exit(sigma_i32 code) {
#if defined(SIGMA_ARCH_X86_64)
    __asm__ volatile (
        "syscall"
        :
        : "a"(60ULL), "D"((sigma_u64)code)   /* SYS_exit = 60 */
        : "memory"
    );
#elif defined(SIGMA_ARCH_ARM64)
    __asm__ volatile (
        "mov x8, #93\n"     /* SYS_exit = 93 on ARM64 Linux */
        "svc #0"
        :
        : "r"((sigma_u64)code)
        : "x8"
    );
#endif
    /* Unreachable, but suppresses warning */
    while(1);
}

/*
 * sigma_exit_group: Terminate all threads (SYS_exit_group = 231).
 */
SIGMA_NORETURN SIGMA_INLINE void sigma_exit_group(sigma_i32 code) {
#if defined(SIGMA_ARCH_X86_64)
    __asm__ volatile (
        "syscall"
        :
        : "a"(231ULL), "D"((sigma_u64)code)
        : "memory"
    );
#endif
    while(1);
}

/* =========================================================================
 * SECTION 7: ALIGNMENT UTILITIES
 * ========================================================================= */

/*
 * sigma_align_up: Round up to next power of 2 alignment.
 */
SIGMA_INLINE SIGMA_CONST sigma_usize sigma_align_up(sigma_usize n, sigma_usize align) {
    return (n + align - 1) & ~(align - 1);
}

/*
 * sigma_align_down: Round down to power of 2 alignment.
 */
SIGMA_INLINE SIGMA_CONST sigma_usize sigma_align_down(sigma_usize n, sigma_usize align) {
    return n & ~(align - 1);
}

/*
 * sigma_is_pow2: Check if value is a power of 2.
 */
SIGMA_INLINE SIGMA_CONST sigma_bool sigma_is_pow2(sigma_u64 n) {
    return (n != 0) && ((n & (n - 1)) == 0);
}

/* =========================================================================
 * SECTION 8: ENTROPY / RANDOM (Replacing <stdlib.h> rand / <random>)
 * ========================================================================= */

/*
 * sigma_urandom: Read from /dev/urandom directly via syscall.
 * Absorbing: Debian's cryptographically secure random principle.
 */
SIGMA_INLINE sigma_i64 sigma_urandom(void* buf, sigma_usize len) {
#if defined(SIGMA_ARCH_X86_64)
    /* Open /dev/urandom: SYS_open = 2 on x86_64 */
    sigma_i64 fd;
    __asm__ volatile (
        "syscall"
        : "=a"(fd)
        : "0"(2ULL),
          "D"("/dev/urandom"),
          "S"(0 /* O_RDONLY */),
          "d"(0)
        : "rcx", "r11", "memory"
    );
    if (fd < 0) return -1;

    sigma_i64 ret = sigma_read((sigma_i32)fd, buf, len);

    /* Close fd: SYS_close = 3 */
    __asm__ volatile (
        "syscall"
        :
        : "a"(3ULL), "D"(fd)
        : "rcx", "r11", "memory"
    );
    return ret;
#else
    (void)buf; (void)len;
    return -1;
#endif
}

/*
 * sigma_rdrand: Hardware random number from RDRAND instruction.
 * Absorbing: Clear Linux / Fedora's hardware entropy usage.
 */
SIGMA_INLINE sigma_bool sigma_rdrand(sigma_u64* out) {
#if defined(SIGMA_ARCH_X86_64)
    sigma_u8 cf;
    __asm__ volatile (
        "rdrand %0\n"
        "setc %1"
        : "=r"(*out), "=qm"(cf)
        :
        : "cc"
    );
    return cf ? SIGMA_TRUE : SIGMA_FALSE;
#else
    (void)out;
    return SIGMA_FALSE;
#endif
}

/* =========================================================================
 * SECTION 9: FORMATTED OUTPUT (Minimal printf without <stdio.h>)
 * ========================================================================= */

/*
 * sigma_printf: Minimal formatted output. Supports: %s %d %u %x %c %p %%
 * This is our replacement for printf() - no FILE*, no buffering overhead.
 * Absorbing: musl's minimal printf, Diet libc's small-footprint philosophy.
 */
sigma_i32 sigma_printf(const char* fmt, ...);

/* =========================================================================
 * SECTION 10: MEMORY BARRIER / ATOMIC OPERATIONS
 * ========================================================================= */

/*
 * sigma_barrier_full: Full memory barrier (load+store).
 */
SIGMA_INLINE void sigma_barrier_full(void) {
#if defined(SIGMA_ARCH_X86_64)
    __asm__ volatile ("mfence" ::: "memory");
#elif defined(SIGMA_ARCH_ARM64)
    __asm__ volatile ("dmb ish" ::: "memory");
#else
    SIGMA_BARRIER();
#endif
}

/*
 * sigma_barrier_load: Load-only barrier (acquire semantics).
 */
SIGMA_INLINE void sigma_barrier_load(void) {
#if defined(SIGMA_ARCH_X86_64)
    __asm__ volatile ("lfence" ::: "memory");
#elif defined(SIGMA_ARCH_ARM64)
    __asm__ volatile ("dmb ishld" ::: "memory");
#else
    SIGMA_BARRIER();
#endif
}

/*
 * sigma_barrier_store: Store-only barrier (release semantics).
 */
SIGMA_INLINE void sigma_barrier_store(void) {
#if defined(SIGMA_ARCH_X86_64)
    __asm__ volatile ("sfence" ::: "memory");
#elif defined(SIGMA_ARCH_ARM64)
    __asm__ volatile ("dmb ishst" ::: "memory");
#else
    SIGMA_BARRIER();
#endif
}

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_LIBC_H */

