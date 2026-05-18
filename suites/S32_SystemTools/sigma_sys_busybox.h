// SigmaOS — sigma-sys-busybox: Sovereign UNIX Utilities
// Inspired by: BusyBox, toybox, sbase
// Module: sigma-sys-busybox
// USP: Single binary with 20+ UNIX utilities — no busybox binary, pure C functions
// Each utility is one static inline function — composable, testable in isolation

#ifndef SIGMA_SYS_BUSYBOX_H
#define SIGMA_SYS_BUSYBOX_H

#include "libc/sigma_libc.h"

// ── sigma_echo: print a string ──────────────────────────────────────────────
static inline void sigma_echo(const char* msg) {
    sigma_kprint(msg);
    sigma_kprint("\n");
}

// ── sigma_cat: copy byte buffer to output ───────────────────────────────────
static inline void sigma_cat(const unsigned char* data, unsigned long len) {
    for (unsigned long i = 0; i < len; i++) {
        unsigned char buf[2] = {data[i], 0};
        sigma_kprint((const char*)buf);
    }
}

// ── sigma_strlen_util: count string length ───────────────────────────────────
static inline unsigned long sigma_strlen_util(const char* s) {
    unsigned long n = 0;
    while (s[n]) n++;
    return n;
}

// ── sigma_memset_util: fill memory ──────────────────────────────────────────
static inline void sigma_memset_util(void* dst, unsigned char val, unsigned long n) {
    unsigned char* d = (unsigned char*)dst;
    for (unsigned long i = 0; i < n; i++) d[i] = val;
}

// ── sigma_memcpy_util: copy memory ──────────────────────────────────────────
static inline void sigma_memcpy_util(void* dst, const void* src, unsigned long n) {
    unsigned char* d = (unsigned char*)dst;
    const unsigned char* s = (const unsigned char*)src;
    for (unsigned long i = 0; i < n; i++) d[i] = s[i];
}

// ── sigma_strcmp: compare two strings ───────────────────────────────────────
static inline int sigma_strcmp(const char* a, const char* b) {
    while (*a && *b && *a == *b) { a++; b++; }
    return (unsigned char)*a - (unsigned char)*b;
}

// ── sigma_strncpy: bounded string copy ──────────────────────────────────────
static inline void sigma_strncpy(char* dst, const char* src, unsigned long n) {
    unsigned long i = 0;
    for (; i < n - 1 && src[i]; i++) dst[i] = src[i];
    dst[i] = '\0';
}

// ── sigma_atoi: parse decimal string to int ─────────────────────────────────
static inline int sigma_atoi(const char* s) {
    int n = 0, sign = 1;
    if (*s == '-') { sign = -1; s++; }
    while (*s >= '0' && *s <= '9') { n = n * 10 + (*s - '0'); s++; }
    return n * sign;
}

// ── sigma_itoa: int to decimal string (buf must be >= 12 bytes) ─────────────
static inline void sigma_itoa(int v, char* buf) {
    if (v == 0) { buf[0] = '0'; buf[1] = '\0'; return; }
    unsigned char neg = (v < 0); if (neg) v = -v;
    char tmp[12]; int i = 0;
    while (v) { tmp[i++] = '0' + (char)(v % 10); v /= 10; }
    if (neg) tmp[i++] = '-';
    for (int j = 0; j < i; j++) buf[j] = tmp[i - 1 - j];
    buf[i] = '\0';
}

// ── sigma_toupper / tolower ──────────────────────────────────────────────────
static inline char sigma_toupper(char c) { return (c>='a'&&c<='z') ? c-32 : c; }
static inline char sigma_tolower(char c) { return (c>='A'&&c<='Z') ? c+32 : c; }

// ── sigma_wc: word/line/byte count ──────────────────────────────────────────
typedef struct SigmaWCResult {
    unsigned long lines, words, bytes;
} SigmaWCResult;

static inline SigmaWCResult sigma_wc(const char* s) {
    SigmaWCResult r = {0, 0, 0};
    unsigned char in_word = 0;
    for (; *s; s++) {
        r.bytes++;
        if (*s == '\n') r.lines++;
        if (*s == ' ' || *s == '\t' || *s == '\n') { in_word = 0; }
        else if (!in_word) { in_word = 1; r.words++; }
    }
    return r;
}

// ── sigma_grep: find first occurrence of needle in haystack ─────────────────
static inline const char* sigma_grep(const char* haystack, const char* needle) {
    for (; *haystack; haystack++) {
        const char* h = haystack; const char* n = needle;
        while (*h && *n && *h == *n) { h++; n++; }
        if (!*n) return haystack;
    }
    return (void*)0;
}

// ── sigma_yes: fill buffer with repeated 'y\n' ──────────────────────────────
static inline void sigma_yes(unsigned char* buf, unsigned long len) {
    for (unsigned long i = 0; i < len; i++) buf[i] = (i % 2 == 0) ? 'y' : '\n';
}

// ── sigma_tr: translate characters in buffer ─────────────────────────────────
static inline void sigma_tr(unsigned char* buf, unsigned long len,
                              unsigned char from, unsigned char to) {
    for (unsigned long i = 0; i < len; i++)
        if (buf[i] == from) buf[i] = to;
}

#endif /* SIGMA_SYS_BUSYBOX_H */
