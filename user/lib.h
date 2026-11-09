/* SigmaOS userlib — shared syscall wrappers for ELF32 ring-3 programs.
 * int 0x80 ABI: eax=nr, ebx/ecx/edx=args, ret in eax.
 * Include this header; compile with -ffreestanding -nostdlib -Os. */
#ifndef SIGMAOS_USERLIB_H
#define SIGMAOS_USERLIB_H

typedef unsigned int u32;

/* raw 3-arg syscall */
static int sc3(u32 n, u32 a, u32 b, u32 c) {
    int ret;
    __asm__ volatile("int $0x80" : "=a"(ret)
                     : "a"(n), "b"(a), "c"(b), "d"(c) : "memory");
    return ret;
}
/* 1-arg syscall */
static int sc1(u32 n, u32 a) { return sc3(n, a, 0, 0); }

/* syscall numbers */
enum {
    SYS_EXIT = 0, SYS_WRITE = 1, SYS_GETCHAR = 2, SYS_TICKS = 3,
    SYS_OPEN = 4, SYS_READ = 5, SYS_CLOSE = 6, SYS_SEEK = 7,
    SYS_WRITE_LEN = 8, SYS_UNLINK = 9, SYS_MKDIR = 10, SYS_CREAT = 11,
    SYS_WRITE_FILE = 12, SYS_RENAME = 13, SYS_APPEND = 14,
    SYS_FSTAT = 15, SYS_STAT = 16, SYS_GETCWD = 17, SYS_CHDIR = 18,
    SYS_READDIR = 19, SYS_READDIRX = 20
};

static void uputs(const char* s) {
    int n = 0; while (s[n]) n++;
    sc3(SYS_WRITE_LEN, (u32)s, n, 0);
}
static void uputn(u32 v) {
    char b[12]; int i = 11; b[i] = 0;
    if (!v) { uputs("0"); return; }
    while (v) { b[--i] = '0' + v % 10; v /= 10; }
    uputs(b + i);
}
static int ulower(int c) { return (c >= 'A' && c <= 'Z') ? c + 32 : c; }

/* argc/argv from initial stack */
static u32* uargs(void) {
    u32* sp;
    __asm__ volatile("mov %%esp, %0" : "=r"(sp));
    return sp;
}

#endif
