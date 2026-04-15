/*
 * =========================================================================
 * S SIGMAOS include/sigma_libc.h (v3.0 — SELF-CONTAINED)
 * =========================================================================
 * Design: Zero external includes. All primitives declared inline.
 *         Previously included sigma_types.h which pulled in stdint.h via
 *         the std-alias block and caused recursive preamble errors in
 *         clangd. This version eliminates that entire include chain.
 * =========================================================================
 */

#ifndef SIGMA_LIBC_H
#define SIGMA_LIBC_H

/* ── Inline primitives (no sigma_types.h, no stdint.h) ───────────────────── */
typedef unsigned char      sigma_u8;
typedef unsigned short     sigma_u16;
typedef unsigned int       sigma_u32;
typedef unsigned long long sigma_u64;
typedef signed   int       sigma_i32;
typedef signed   long long sigma_i64;
typedef sigma_u64          sigma_uptr;
typedef sigma_u64          sigma_sz_t;
typedef sigma_i64          sigma_ssz_t;
typedef sigma_i32          sigma_err_t;

#define SIGMA_NULL ((void*)0)

/* ── Syscall wrappers ─────────────────────────────────────────────────────── */
void            sigma_exit(int code);
sigma_ssz_t   sigma_write(int fd, const void *buf, sigma_sz_t count);
sigma_ssz_t   sigma_read(int fd, void *buf, sigma_sz_t count);
int             sigma_open(const char *filename, int flags, int mode);
int             sigma_close(int fd);
void           *sigma_mmap(void *addr, sigma_sz_t length,
                            int prot, int flags, int fd, sigma_u64 offset);
void           *sigma_malloc(sigma_sz_t size);
void            sigma_free(void *ptr);

/* ── libc utilities ───────────────────────────────────────────────────────── */
sigma_sz_t    sigma_strlen(const char *s);
void           *sigma_memset(void *s, int c, sigma_sz_t n);
void           *sigma_memcpy(void *dest, const void *src, sigma_sz_t n);
void           *sigma_memmove(void *dest, const void *src, sigma_sz_t n);
int             sigma_streq(const char *s1, const char *s2);
int             sigma_strcmp(const char *s1, const char *s2);
char           *sigma_strcpy(char *dest, const char *src);
char           *sigma_strncpy(char *dest, const char *src, sigma_sz_t n);
int             sigma_snprintf(char *buf, sigma_sz_t size,
                               const char *fmt, ...);

/* ── Output ──────────────────────────────────────────────────────────────── */
void sigma_print(const char *str);
void sigma_printf(const char *format, ...);

#endif /* SIGMA_LIBC_H */
