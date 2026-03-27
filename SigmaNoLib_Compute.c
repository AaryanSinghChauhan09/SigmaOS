/*
 * Σ SIGMA OS: ABSOLUTE ZERO-LIBRARY C COMPUTATION (v5.0 - SLACKWARE PURITY)
 * =========================================================================
 * USP Absorbed: Slackware (No-Frills), LFS (Linux From Scratch)
 * Capability: Bare-metal logic without a single #include or libc dependency.
 * Principle: Pure compilation, absolute hardware isolation.
 */

// NO INCLUDES. NO LIBC. NO STDARG.

/* Custom typedefs replacing <stdint.h> */
typedef unsigned long long uint64_t;
typedef long long int64_t;

/* Direct Syscall Implementation (x86_64) replacing <unistd.h> */
static inline int64_t sigma_syscall_write(int fd, const char* str, uint64_t len) {
    int64_t ret;
#if defined(__x86_64__)
    __asm__ volatile (
        "mov $1, %%rax\n"   // Syscall 1 (sys_write)
        "mov %1, %%rdi\n"   // FD
        "mov %2, %%rsi\n"   // Buffer pointer
        "mov %3, %%rdx\n"   // Length
        "syscall\n"
        "mov %%rax, %0\n"
        : "=r" (ret)
        : "r" ((int64_t)fd), "r" (str), "r" (len)
        : "%rax", "%rdi", "%rsi", "%rdx", "%rcx", "%r11", "memory"
    );
#else
    ret = len; // Non-x64 fallback
#endif
    return ret;
}

static inline void sigma_syscall_exit(int status) {
#if defined(__x86_64__)
    __asm__ volatile (
        "mov $60, %%rax\n"  // Syscall 60 (sys_exit)
        "mov %0, %%rdi\n"   // Exit status
        "syscall\n"
        : 
        : "r" ((int64_t)status)
        : "%rax", "%rdi"
    );
#endif
}

/* Custom string length replacing <string.h> */
static uint64_t sigma_strlen(const char* s) {
    uint64_t l = 0;
    while(s[l]) l++;
    return l;
}

/* 
 * Entry point overriding the standard C runtime 'main'.
 * Compiles with: gcc -nostdlib -fno-builtin 
 */
void _start(void) {
    const char* msg = "[PURE_C_ZEROLIB]: Bootstrapping Absolute Library-Free Logic Shard.\n";
    sigma_syscall_write(1, msg, sigma_strlen(msg));

    const char* success = "[SUCCESS]: Slackware Purity Online. Zero C-Library Dependency allowed.\n";
    sigma_syscall_write(1, success, sigma_strlen(success));

    sigma_syscall_exit(0);
}
