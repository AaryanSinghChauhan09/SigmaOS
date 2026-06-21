/*
 * Σ SigmaOS — sigma_posix: POSIX Standard Wrapper Library
 * Zero-Dependency: Maps POSIX standard calls directly to system calls.
 */

#include "../../include/sigma_kernel_types.h"

extern "C" {

// Inline syscall wrapper
static inline u64 posix_syscall(u64 num, u64 arg1 = 0, u64 arg2 = 0, u64 arg3 = 0) {
    u64 ret;
#if defined(__x86_64__)
    __asm__ volatile (
        "syscall"
        : "=a"(ret)
        : "a"(num), "D"(arg1), "S"(arg2), "d"(arg3)
        : "rcx", "r11", "memory"
    );
#elif defined(__aarch64__)
    register long x0 __asm__("x0") = arg1;
    register long x1 __asm__("x1") = arg2;
    register long x2 __asm__("x2") = arg3;
    register long x8 __asm__("x8") = num;
    __asm__ volatile("svc #0" : "+r"(x0) : "r"(x1), "r"(x2), "r"(x8) : "memory");
    ret = x0;
#else
    ret = 0;
#endif
    return ret;
}

int open(const char* path, int flags, int mode) {
    return (int)posix_syscall(2, (u64)path, (u64)flags, (u64)mode);
}

int read(int fd, void* buf, sigma_size_t count) {
    return (int)posix_syscall(0, (u64)fd, (u64)buf, (u64)count);
}

int write(int fd, const void* buf, sigma_size_t count) {
    return (int)posix_syscall(1, (u64)fd, (u64)buf, (u64)count);
}

int close(int fd) {
    return (int)posix_syscall(3, (u64)fd);
}

int fork(void) {
    return (int)posix_syscall(57);
}

int execve(const char* pathname, char* const argv[], char* const envp[]) {
    return (int)posix_syscall(59, (u64)pathname, (u64)argv, (u64)envp);
}

void exit(int status) {
    posix_syscall(60, (u64)status);
    while (1) { __asm__ volatile("hlt"); }
}

int pipe(int pipefd[2]) {
    // Pipe stub mapping
    pipefd[0] = 3;
    pipefd[1] = 4;
    return 0;
}

} // extern "C"
