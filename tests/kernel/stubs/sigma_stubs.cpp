// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sigma_stubs.cpp — minimal host-mode stubs for the SigmaOS kernel regress suite
 *
 * These replace bare-metal kernel primitives so the regression tests can
 * compile and run on a standard Linux host without any hardware or QEMU.
 * Each stub matches the signature expected by the kernel headers but provides
 * a host-compatible implementation.
 */

#include <cstdio>
#include <cstdarg>
#include <cstring>
#include <cstdlib>
#include <cstdint>

/* ── Kernel type aliases used in headers ─────────────────────────────────── */
#ifndef SIGMA_KERNEL_TYPES_H
typedef uint8_t  sigma_u8;
typedef uint32_t sigma_u32;
typedef uint64_t sigma_u64;
typedef int32_t  sigma_i32;
typedef size_t   sigma_size_t;
#endif

/* ── Logging stubs ───────────────────────────────────────────────────────── */
extern "C" void sigma_log_info(const char* fmt, ...) {
    va_list ap; va_start(ap, fmt);
    vprintf(fmt, ap); va_end(ap);
}
extern "C" void sigma_log_warn(const char* fmt, ...) {
    fprintf(stderr, "[WARN] ");
    va_list ap; va_start(ap, fmt);
    vfprintf(stderr, fmt, ap); va_end(ap);
}
extern "C" void sigma_log_err(const char* fmt, ...) {
    fprintf(stderr, "[ERR] ");
    va_list ap; va_start(ap, fmt);
    vfprintf(stderr, fmt, ap); va_end(ap);
}
extern "C" void sigma_printf(const char* fmt, ...) {
    va_list ap; va_start(ap, fmt);
    vprintf(fmt, ap); va_end(ap);
}
extern "C" void sigma_log(const char* msg) { puts(msg); }

/* ── String primitives (klib replacements) ───────────────────────────────── */
extern "C" int sigma_strcmp(const char* a, const char* b) {
    return strcmp(a, b);
}
extern "C" int sigma_strncmp(const char* a, const char* b, sigma_size_t n) {
    return strncmp(a, b, n);
}
extern "C" void sigma_strncpy(char* dst, const char* src, sigma_size_t n) {
    strncpy(dst, src, n);
}
extern "C" sigma_size_t sigma_strlen(const char* s) {
    return strlen(s);
}
extern "C" const char* sigma_strstr(const char* h, const char* n) {
    return strstr(h, n);
}
extern "C" void* sigma_memcpy(void* d, const void* s, sigma_size_t n) {
    return memcpy(d, s, n);
}
extern "C" void* sigma_memset(void* d, int c, sigma_size_t n) {
    return memset(d, c, n);
}

/* ── Process / signal stubs ──────────────────────────────────────────────── */
extern "C" sigma_u32 sigma_current_pid(void) {
    return (sigma_u32)getpid();
}
extern "C" void sigma_send_signal(sigma_u32 /*pid*/, int /*signo*/) {
    /* In host tests we don't actually deliver signals — violations are
     * verified by checking the return code and violation counter. */
}

/* ── Clock stub (monotonically increasing, not real time) ───────────────── */
static sigma_u64 g_tick = 0;
extern "C" sigma_u64 sigma_clock_monotonic_ns(void) {
    return g_tick += 1000000ULL;  /* advance 1 ms per call */
}

/* ── Sleep stub ──────────────────────────────────────────────────────────── */
extern "C" void sigma_msleep(int /*ms*/) { /* no-op in tests */ }

/* ── Unlink stub (no filesystem in host tests) ───────────────────────────── */
extern "C" int sigma_unlink(const char* path) {
    (void)path; return 0;
}

/* ── Hash stubs (return deterministic "good" values for sigma_acquire tests) */
extern "C" int sigma_sha256_file(const char* /*path*/, char out[65]) {
    /* Return a fixed "correct" hash — real crypto tested in SovereignPQC suite */
    strncpy(out,
            "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
            64);
    out[64] = '\0';
    return 0;
}
extern "C" int sigma_blake2b_file(const char* /*path*/, char out[129]) {
    strncpy(out,
            "aabbccdd11223344aabbccdd11223344aabbccdd11223344aabbccdd11223344"
            "aabbccdd11223344aabbccdd11223344aabbccdd11223344aabbccdd11223344",
            128);
    out[128] = '\0';
    return 0;
}

/* ── Panic stub ──────────────────────────────────────────────────────────── */
extern "C" void sigma_panic(const char* msg, int /*a*/, int /*b*/) {
    fprintf(stderr, "[PANIC] %s\n", msg);
    abort();
}
