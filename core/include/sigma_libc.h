/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN LIBC (v20.0 - UNIFIED PURE SILICON)
 * =========================================================================
 */

#ifndef SOVEREIGN_LIBC_H
#define SOVEREIGN_LIBC_H

#ifdef __cplusplus
extern "C" {
#endif

/* --- Primitive Types --- */
typedef unsigned long long sigma_size_t;
typedef long long          sigma_ssize_t;
typedef unsigned char      sigma_u8;
typedef unsigned short     sigma_u16;
typedef unsigned int       sigma_u32;
typedef unsigned long long sigma_u64;
typedef int                sigma_bool;

#define SIGMA_TRUE  1
#define SIGMA_FALSE 0
#define SIGMA_NULL  ((void*)0)

/* --- Direct Syscalls (Sovereign Primitives) --- */
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

/* --- String & Memory Shards --- */
sigma_size_t  sigma_strlen(const char* s);
void*         sigma_memset(void* s, int c, sigma_size_t n);
void*         sigma_memcpy(void* dest, const void* src, sigma_size_t n);
int           sigma_streq(const char* s1, const char* s2);
int           sigma_compare(const char* s1, const char* s2);
void          sigma_strcat(char* dest, const char* src);
int           sigma_atoi(const char* s);

/* --- Output Shards --- */
void sigma_print(const char* str);
void sigma_print_num(sigma_u64 val);
void sigma_printf(const char* format, ...);

/* --- Memory Management (Sovereign Zenith) --- */
void* sigma_slab_alloc_raw(sigma_size_t size);
void* sigma_malloc(sigma_size_t size);
void  sigma_free(void* ptr);

#ifdef __cplusplus
}
#endif

#endif
