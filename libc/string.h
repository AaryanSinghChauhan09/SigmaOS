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
 * Σ SIGMAOS: SOVEREIGN STRING LIBRARY HEADER (Replacing <string.h>)
 * =========================================================================
 * USP Absorbed: musl libc (clean minimal API), Diet libc (zero overhead)
 * Principle: ZERO <stddef.h>, ZERO <string.h> dependency.
 *            All types sourced from our sovereign sigma_types.h.
 * OOP Principle: All string ops are pure functions (no state mutation of
 *                caller-owned data beyond the destination parameter).
 * =========================================================================
 */

#ifndef SIGMA_STRING_H
#define SIGMA_STRING_H

/*
 * Our only include: sigma_types.h provides sigma_usize, sigma_i32, etc.
 * This is our own file - NOT a standard library header.
 */
#include "sigma_types.h"
#include "sigma_libc.h"

#ifdef __cplusplus
extern "C" {
#endif

/* =========================================================================
 * MEMORY OPERATIONS (Replicate the <string.h> mem* interface)
 * All implementations are in sigma_libc.h as inline functions.
 * These are thin wrappers that maintain API compatibility.
 * ========================================================================= */

/* sigma_memcpy: Copy n bytes from src to dest (non-overlapping). */
#define memcpy(d, s, n)     sigma_memcpy((d), (s), (n))

/* sigma_memmove: Copy n bytes (safe for overlapping). */
#define memmove(d, s, n)    sigma_memmove((d), (s), (n))

/* sigma_memset: Fill n bytes at ptr with byte value c. */
#define memset(p, c, n)     sigma_memset((p), (c), (n))

/* sigma_memcmp: Compare n bytes of two memory regions. */
#define memcmp(a, b, n)     sigma_memcmp((a), (b), (n))

/* sigma_memchr: Find byte c in the first n bytes of s. */
#define memchr(s, c, n)     sigma_memchr((s), (c), (n))

/* =========================================================================
 * STRING OPERATIONS (Replicate the <string.h> str* interface)
 * ========================================================================= */

/* sigma_strlen: Compute length of null-terminated string. */
#define strlen(s)           sigma_strlen(s)

/* sigma_strnlen: Bounded string length. */
#define strnlen(s, m)       sigma_strnlen((s), (m))

/* sigma_strcmp: Compare two null-terminated strings. */
#define strcmp(a, b)        sigma_strcmp((a), (b))

/* sigma_strncmp: Bounded string comparison. */
#define strncmp(a, b, n)    sigma_strncmp((a), (b), (n))

/* sigma_strcpy: Copy null-terminated string. */
#define strcpy(d, s)        sigma_strcpy((d), (s))

/* sigma_strncpy: Bounded string copy. */
#define strncpy(d, s, n)    sigma_strncpy((d), (s), (n))

/* sigma_strcat: Append src to dest. */
#define strcat(d, s)        sigma_strcat((d), (s))

/* sigma_strncat: Bounded string append. */
#define strncat(d, s, n)    sigma_strncat((d), (s), (n))

/* sigma_strchr: Find character in string. */
#define strchr(s, c)        sigma_strchr((s), (c))

/* sigma_strrchr: Find last occurrence of character. */
#define strrchr(s, c)       sigma_strrchr((s), (c))

/* =========================================================================
 * ADDITIONAL STRING UTILITIES
 * ========================================================================= */

/*
 * sigma_str_starts_with: Check if str begins with prefix.
 */
static SIGMA_INLINE sigma_bool sigma_str_starts_with(
    const char* str,
    const char* prefix
) {
    sigma_usize plen = sigma_strlen(prefix);
    return sigma_strncmp(str, prefix, plen) == 0;
}

/*
 * sigma_str_ends_with: Check if str ends with suffix.
 */
static SIGMA_INLINE sigma_bool sigma_str_ends_with(
    const char* str,
    const char* suffix
) {
    sigma_usize slen   = sigma_strlen(str);
    sigma_usize suflen = sigma_strlen(suffix);
    if (suflen > slen) return SIGMA_FALSE;
    return sigma_strcmp(str + slen - suflen, suffix) == 0;
}

/*
 * sigma_str_to_int: Parse a decimal integer string.
 * Returns SIGMA_FALSE if the string is invalid.
 */
static SIGMA_INLINE sigma_bool sigma_str_to_int(
    const char* s,
    sigma_i64* out
) {
    if (!s || !out) return SIGMA_FALSE;
    sigma_i64 result = 0;
    sigma_bool neg = SIGMA_FALSE;
    if (*s == '-') { neg = SIGMA_TRUE; s++; }
    else if (*s == '+') { s++; }
    if (!*s) return SIGMA_FALSE;
    while (*s >= '0' && *s <= '9') {
        result = result * 10 + (*s - '0');
        s++;
    }
    if (*s != '\0') return SIGMA_FALSE; /* trailing garbage */
    *out = neg ? -result : result;
    return SIGMA_TRUE;
}

/*
 * sigma_str_to_hex: Parse a hex string (without 0x prefix).
 */
static SIGMA_INLINE sigma_bool sigma_str_to_hex(
    const char* s,
    sigma_u64* out
) {
    if (!s || !out) return SIGMA_FALSE;
    if (s[0] == '0' && (s[1] == 'x' || s[1] == 'X')) s += 2;
    sigma_u64 result = 0;
    while (*s) {
        sigma_u8 nibble;
        if (*s >= '0' && *s <= '9')      nibble = (sigma_u8)(*s - '0');
        else if (*s >= 'a' && *s <= 'f') nibble = (sigma_u8)(*s - 'a' + 10);
        else if (*s >= 'A' && *s <= 'F') nibble = (sigma_u8)(*s - 'A' + 10);
        else return SIGMA_FALSE;
        result = (result << 4) | nibble;
        s++;
    }
    *out = result;
    return SIGMA_TRUE;
}

/*
 * sigma_str_trim_left: Return pointer past leading whitespace.
 */
static SIGMA_INLINE const char* sigma_str_trim_left(const char* s) {
    while (*s == ' ' || *s == '\t' || *s == '\n' || *s == '\r') s++;
    return s;
}

/*
 * sigma_str_contains: Check if haystack contains needle.
 */
static SIGMA_INLINE sigma_bool sigma_str_contains(
    const char* haystack,
    const char* needle
) {
    sigma_usize hlen = sigma_strlen(haystack);
    sigma_usize nlen = sigma_strlen(needle);
    if (nlen > hlen) return SIGMA_FALSE;
    for (sigma_usize i = 0; i <= hlen - nlen; i++) {
        if (sigma_strncmp(haystack + i, needle, nlen) == 0) return SIGMA_TRUE;
    }
    return SIGMA_FALSE;
}

/*
 * sigma_str_copy_safe: Safe string copy that always null-terminates.
 * Absorbing: OpenBSD's strlcpy principle.
 * Returns: total length of source string.
 */
static SIGMA_INLINE sigma_usize sigma_str_copy_safe(
    char* dst,
    const char* src,
    sigma_usize dstsize
) {
    sigma_usize srclen = sigma_strlen(src);
    if (dstsize > 0) {
        sigma_usize copylen = srclen < dstsize - 1 ? srclen : dstsize - 1;
        sigma_memcpy(dst, src, copylen);
        dst[copylen] = '\0';
    }
    return srclen;
}

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_STRING_H */

