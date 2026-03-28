/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN LIBC (v19.0 - ZERO-DEPENDENCY)
 * =========================================================================
 * Mission: Neutralize all high-level language runtimes (glibc, msvcrt).
 * Capability: Direct x86_64 Syscall Integration.
 * Principle: Absolute Low-Level. Zero External Symbols.
 * =========================================================================
 */

#ifndef SOVEREIGN_LIBC_H
#define SOVEREIGN_LIBC_H

typedef unsigned long long sigma_size_t;
typedef long long          sigma_ssize_t;
typedef unsigned char      sigma_u8;
typedef unsigned int       sigma_u32;
typedef unsigned long long sigma_u64;

#define SIGMA_NULL ((void*)0)

// --- Direct Syscalls (Linux-Targeted for this Shard) ---
// Note: Can be sharded to Windows x64 syscalls if needed.

extern "C" {
    void sigma_exit(int code);
    sigma_ssize_t sigma_write(int fd, const void* buf, sigma_size_t count);
    void* sigma_mmap(void* addr, sigma_size_t length, int prot, int flags, int fd, sigma_u64 offset);
}

// --- High-Level primitives implemented at Low-Level ---
void sigma_print(const char* str);
void sigma_print_num(sigma_u64 val);

#endif
