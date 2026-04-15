/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN SYSCALL SHARD (v1.0 - PURE C11 / ASM)
 * =========================================================================
 * Mission: Zero-Library hardware interaction via inline assembly.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_libc.h"

void sigma_exit(int code) {
    __asm__ volatile (
        "mov $60, %%rax\n\t"    // syscall number for sys_exit
        "mov %0, %%rdi\n\t"     // exit code
        "syscall"
        : : "r"((long)code) : "rax", "rdi"
    );
}

sigma_ssz_t sigma_write(int fd, const void* buf, sigma_sz_t count) {
    sigma_ssz_t ret;
    __asm__ volatile (
        "mov $1, %%rax\n\t"     // syscall number for sys_write
        "mov %1, %%rdi\n\t"     // fd
        "mov %2, %%rsi\n\t"     // buf
        "mov %3, %%rdx\n\t"     // count
        "syscall\n\t"
        "mov %%rax, %0"
        : "=r"(ret)
        : "r"((long)fd), "r"(buf), "r"((long)count)
        : "rax", "rdi", "rsi", "rdx"
    );
    return ret;
}

sigma_ssz_t sigma_read(int fd, void* buf, sigma_sz_t count) {
    sigma_ssz_t ret;
    __asm__ volatile (
        "mov $0, %%rax\n\t"     // syscall number for sys_read
        "mov %1, %%rdi\n\t"     // fd
        "mov %2, %%rsi\n\t"     // buf
        "mov %3, %%rdx\n\t"     // count
        "syscall\n\t"
        "mov %%rax, %0"
        : "=r"(ret)
        : "r"((long)fd), "r"(buf), "r"((long)count)
        : "rax", "rdi", "rsi", "rdx"
    );
    return ret;
}
