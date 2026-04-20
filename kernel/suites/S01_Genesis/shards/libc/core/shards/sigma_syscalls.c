#include "sigma_kernel.h"
int sigma_fork() {
    long res; __asm__ __volatile__ ("syscall" : "=a"(res) : "a"(57) : "rcx", "r11", "memory");
    return (int)res;
}
int sigma_pipe(int pipefd[2]) {
    long res; __asm__ __volatile__ ("syscall" : "=a"(res) : "a"(22), "D"(pipefd) : "rcx", "r11", "memory");
    return (int)res;
}
unsigned int sigma_sleep(unsigned int seconds) {
    struct { long tv_sec; long tv_nsec; } req = { (long)seconds, 0 };
    long res; __asm__ __volatile__ ("syscall" : "=a"(res) : "a"(35), "D"(&req), "S"(0) : "rcx", "r11", "memory");
    return (unsigned int)res;
}
int sigma_wait(int* wstatus) {
    long res; register long r10 __asm__("r10") = 0; register long r8 __asm__("r8") = 0;
    __asm__ __volatile__ ("syscall" : "=a"(res) : "a"(61), "D"(-1), "S"(wstatus), "r"(r10), "r"(r8) : "rcx", "r11", "memory");
    return (int)res;
}
int sigma_dup(int oldfd) {
    long res; __asm__ __volatile__ ("syscall" : "=a"(res) : "a"(32), "D"(oldfd) : "rcx", "r11", "memory");
    return (int)res;
}
