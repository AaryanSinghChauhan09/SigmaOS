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

#include "sigma_types.h"

typedef sigma_u32 sigma_status;
/* SIGMA_OK/SIGMA_ERROR are defined in sigma_types.h */

// --- Direct Syscalls (Linux x64) ---
#ifdef __cplusplus
extern "C" {
#endif
    void          sigma_exit(int code);
    sigma_ssize_t sigma_write(int fd, const void* buf, sigma_size_t count);
    sigma_ssize_t sigma_read(int fd, void* buf, sigma_size_t count);
    int           sigma_open(const char* filename, int flags, int mode);
    int           sigma_close(int fd);
    void*         sigma_mmap(void* addr, sigma_size_t length, int prot, int flags, int fd, sigma_u64 offset);
    int           sigma_getdents64(unsigned int fd, void* dirp, unsigned int count);
    int           sigma_execve(const char* filename, char* const argv[], char* const envp[]);
    int           sigma_fork();
    int           sigma_pipe(int* pipefd);
    unsigned int  sigma_sleep(unsigned int seconds);
    int           sigma_wait(int* wstatus);
    int           sigma_dup(int oldfd);
    
    sigma_size_t  sigma_strlen(const char* s);
    void*         sigma_memset(void* s, int c, sigma_size_t n);
    void*         sigma_memcpy(void* dest, const void* src, sigma_size_t n);
    int           sigma_memcmp(const void* s1, const void* s2, sigma_size_t n);
    void*         sigma_memmove(void* dest, const void* src, sigma_size_t n);
    int           sigma_streq(const char* s1, const char* s2);
    int           sigma_compare(const char* s1, const char* s2);
    void          sigma_strlcat(char* dest, const char* src, sigma_size_t dstsize);
    char*         sigma_strcpy(char* dest, const char* src, sigma_size_t maxlen);
    int           sigma_atoi(const char* s);
    int           sigma_shm_open(const char* name, int oflag, int mode);
    int           sigma_shm_unlink(const char* name);
    int           sigma_ioctl(int fd, unsigned long request, ...);
    void          sigma_print_hex(sigma_u64 val);

    // --- Sovereign Math Primitives (ML Backbone) ---
    sigma_f64     sigma_math_exp(sigma_f64 x);
    sigma_f64     sigma_math_log(sigma_f64 x);
    sigma_f64     sigma_math_pow(sigma_f64 x, sigma_f64 y);

    int           sigma_socket(int domain, int type, int protocol);
    int           sigma_bind(int sockfd, const void* addr, sigma_u32 addrlen);
    int           sigma_connect(int sockfd, const void* addr, sigma_u32 addrlen);

    const char*   sigma_strstr(const char* haystack, const char* needle);
    const char*   sigma_strrchr(const char* s, int c);

    void          sigma_log(const char* msg);
#ifdef __cplusplus
}
#endif

// --- High-Level primitives implemented at Low-Level ---
void sigma_print(const char* str);
void sigma_print_num(sigma_u64 val);
void sigma_printf(const char* format, ...);
int  sigma_snprintf(char* str, sigma_size_t size, const char* format, ...);

// --- Memory Management (Sovereign Zenith) ---
void* sigma_slab_alloc_raw(sigma_size_t size);
void* sigma_malloc(sigma_size_t size);
void  sigma_free(void* ptr);

#endif


