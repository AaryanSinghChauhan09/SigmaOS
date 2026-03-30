/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN LIBC (v21.0 - INDUSTRIAL SCALING MATRIX)
 * =========================================================================
 * Mission: Universal Multi-Arch Sovereignty & Competitor Crushing.
 * Capability: TCP/UDP Networking, Shared Memory Sharding, W^X Security.
 * Principal: Scaling Everything with Proper Functional Sovereignty.
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
 * ABSOLUTE RAW SYSCALLS (implemented in SovereignZeroLib.asm)
 * ========================================================================= */
extern sigma_ssize_t _sigma_sys_write(int fd, const void* buf, sigma_size_t count);
extern sigma_ssize_t _sigma_sys_read(int fd, void* buf, sigma_size_t count);
extern void*         _sigma_sys_mmap(void* addr, sigma_size_t length, int prot, int flags, int fd, sigma_u64 offset);
extern void          _sigma_sys_exit(int code);
extern int           _sigma_sys_socket(int domain, int type, int protocol);
extern int           _sigma_sys_bind(int sockfd, const void* addr, sigma_u32 addrlen);
extern int           _sigma_sys_connect(int sockfd, const void* addr, sigma_u32 addrlen);

/* =========================================================================
 * SOVEREIGN WRAPPERS (redirected to raw ASM)
 * ========================================================================= */
#define sigma_exit(code)               _sigma_sys_exit(code)
#define sigma_write(fd, buf, count)    _sigma_sys_write(fd, buf, count)
#define sigma_read(fd, buf, count)     _sigma_sys_read(fd, buf, count)
#define sigma_mmap(addr, len, p, f, d, o) _sigma_sys_mmap(addr, len, p, f, d, o)
#define sigma_socket(d, t, p)          _sigma_sys_socket(d, t, p)
#define sigma_bind(f, a, l)            _sigma_sys_bind(f, a, l)
#define sigma_connect(f, a, l)         _sigma_sys_connect(f, a, l)

int           sigma_open(const char* filename, int flags, int mode);
int           sigma_close(int fd);
int           sigma_ioctl(int fd, unsigned long request, ...);
int           sigma_mprotect(void* addr, sigma_size_t len, int prot);
int           sigma_getdents64(unsigned int fd, void* dirp, unsigned int count);

/* =========================================================================
 * INDUSTRIAL IPC & SHARDING
 * ========================================================================= */
int           sigma_shm_open(const char* name, int oflag, int mode);
int           sigma_shm_unlink(const char* name);
int           sigma_sem_init(const char* name, sigma_u32 value);
int           sigma_sem_wait(const char* name);
int           sigma_sem_post(const char* name);

/* =========================================================================
 * CORE OS PRIMITIVES
 * ========================================================================= */
int           sigma_execve(const char* filename, char* const argv[], char* const envp[]);
int           sigma_fork(void);
int           sigma_pipe(int pipefd[2]);
unsigned int  sigma_sleep(unsigned int seconds);
int           sigma_wait(int* wstatus);
int           sigma_dup(int oldfd);

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
int   sigma_atoi(const char* s);

void  sigma_print(const char* str);
void  sigma_print_num(sigma_u64 val);
void  sigma_print_hex(sigma_u64 val);
void  sigma_printf(const char* format, ...);
void  sigma_snprintf(char* buf, sigma_size_t n, const char* format, ...);
void  sigma_log(const char* msg);

/* =========================================================================
 * ADVANCED STRING UTILITIES (Zero-Dependency)
 * ========================================================================= */
const char* sigma_strstr(const char* haystack, const char* needle);
const char* sigma_strrchr(const char* s, int c);

/* =========================================================================
 * SOVEREIGN MEMORY MANAGEMENT (bump-pointer slab, 128 MB shard)
 * ========================================================================= */
void* sigma_slab_alloc_raw(sigma_size_t size);
void* sigma_malloc(sigma_size_t size);
void  sigma_free(void* ptr);

#endif /* SOVEREIGN_LIBC_H */
