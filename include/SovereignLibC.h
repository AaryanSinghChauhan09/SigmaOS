/*
 * =========================================================================
 * SIGMAOS: SOVEREIGN LIBC (v20.0 - PURE C11 ZERO-DEPENDENCY)
 * =========================================================================
 */

#ifndef SOVEREIGN_LIBC_H
#define SOVEREIGN_LIBC_H

#include "SovereignLibC.h"

#include "./sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* =========================================================================
 * DIRECT SYSCALL DECLARATIONS (x86_64 Linux Ã¢â‚¬â€ no libc wrapper)
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
 * HIGH-LEVEL PRIMITIVES (implemented in SovereignLibC.c Ã¢â‚¬â€ pure C11)
 * ========================================================================= */
int   sigma_streq(const char* s1, const char* s2);
int   sigma_compare(const char* s1, const char* s2);
void  sigma_strcat(char* dest, const char* src);
void  sigma_strncat(char* dest, const char* src, sigma_size_t n);
void  sigma_strcpy(char* dest, const char* src);
void  sigma_strncpy(char* dest, const char* src, sigma_size_t n);
int   sigma_strcmp(const char* s1, const char* s2);
int   sigma_atoi(const char* s);

void  sigma_print(const char* str);
void  sigma_print_num(sigma_u64 val);
void  sigma_print_hex(sigma_u64 val);
void  sigma_printf(const char* format, ...);
void  sigma_log(const char* msg);

/* =========================================================================
 * SECURITY-HARDENED PRIMITIVES (Inspired by Alpine/musl)
 * ========================================================================= */
void* sigma_secure_memset(void* s, int c, sigma_size_t n); // Prevents compiler optimization removals
void  sigma_hardened_strcpy(char* dest, const char* src, sigma_size_t dest_size); // Bounds-checked strcpy
int   sigma_hardened_strcmp(const char* s1, const char* s2);
int   sigma_hardened_strncmp(const char* s1, const char* s2, sigma_size_t n);

/* =========================================================================
 * SOVEREIGN MEMORY MANAGEMENT (bump-pointer slab, 128 MB shard)
 * ========================================================================= */
void* sigma_slab_alloc_raw(sigma_size_t size);
void* sigma_malloc(sigma_size_t size);
void  sigma_free(void* ptr);

#ifdef __cplusplus
}
#endif

#endif /* SOVEREIGN_LIBC_H */
