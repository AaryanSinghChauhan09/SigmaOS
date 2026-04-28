/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN LIBC (v20.0 - PURE C11 ZERO-DEPENDENCY)
 * =========================================================================
 * Mission: Neutralize all high-level language runtimes (glibc, msvcrt).
 * Capability: Direct x86_64 Syscall Integration via C11.
 * Principle: Absolute Low-Level. Zero External Symbols. 98% C Sovereign.
 * Standard: C11 (ISO/IEC 9899:2011) — no C++ dependencies.
 * =========================================================================
 */

#ifndef SOVEREIGN_LIBC_H
#define SOVEREIGN_LIBC_H

/* C11-native type definitions — no <stdint.h>, no <stddef.h> */
typedef unsigned long long  sigma_size_t;
typedef long long           sigma_ssize_t;
typedef unsigned char       sigma_u8;
typedef unsigned short      sigma_u16;
typedef unsigned int        sigma_u32;
typedef unsigned long long  sigma_u64;
typedef long long           sigma_i64;
typedef int                 sigma_i32;
typedef double              sigma_f64;
typedef float               sigma_f32;
typedef int                 sigma_bool;

#define SIGMA_TRUE   1
#define SIGMA_FALSE  0
#define SIGMA_NULL   ((void*)0)
#define SIGMA_OK     0x00000000u
#define SIGMA_ERROR  0xFFFFFFFFu

typedef sigma_u32 sigma_status;

/* =========================================================================
 * DIRECT SYSCALL DECLARATIONS (x86_64 Linux — no libc wrapper)
 * ========================================================================= */
void          sigma_exit(int code);
sigma_ssize_t sigma_write(int fd, const void* buf, sigma_size_t count);
sigma_ssize_t sigma_read(int fd, void* buf, sigma_size_t count);
int           sigma_open(const char* filename, int flags, int mode);
int           sigma_close(int fd);
void*         sigma_mmap(void* addr, sigma_size_t length, int prot,
                          int flags, int fd, sigma_u64 offset);
int           sigma_getdents64(unsigned int fd, void* dirp, unsigned int count);
int           sigma_execve(const char* filename, char* const argv[],
                            char* const envp[]);
int           sigma_fork(void);
int           sigma_pipe(int pipefd[2]);
unsigned int  sigma_sleep(unsigned int seconds);
int           sigma_wait(int* wstatus);
int           sigma_dup(int oldfd);
int           sigma_nanosleep(const void* req, void* rem);

/* =========================================================================
 * STRING / MEMORY PRIMITIVES (implemented in SovereignLibC.asm)
 * ========================================================================= */
sigma_size_t  sigma_strlen(const char* s);
void*         sigma_memset(void* s, int c, sigma_size_t n);
void*         sigma_memcpy(void* dest, const void* src, sigma_size_t n);

/* =========================================================================
 * HIGH-LEVEL PRIMITIVES (implemented in SovereignLibC.c — pure C11)
 * ========================================================================= */
int   sigma_streq(const char* s1, const char* s2);
int   sigma_compare(const char* s1, const char* s2);
void  sigma_strcat(char* dest, const char* src);
void  sigma_strcpy(char* dest, const char* src);
int   sigma_strcmp(const char* s1, const char* s2);
int   sigma_atoi(const char* s);

void  sigma_print(const char* str);
void  sigma_print_num(sigma_u64 val);
void  sigma_print_hex(sigma_u64 val);
void  sigma_printf(const char* format, ...);
void  sigma_log(const char* msg);

/* =========================================================================
 * SOVEREIGN MEMORY MANAGEMENT (bump-pointer slab, 128 MB shard)
 * ========================================================================= */
void* sigma_slab_alloc_raw(sigma_size_t size);
void* sigma_malloc(sigma_size_t size);
void  sigma_free(void* ptr);

#endif /* SOVEREIGN_LIBC_H */
