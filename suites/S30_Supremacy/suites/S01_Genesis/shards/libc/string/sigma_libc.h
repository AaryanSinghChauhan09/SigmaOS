/* sigma_libc.h — Sovereign canonical shim */
#ifndef SIGMA_LIBC_H
#define SIGMA_LIBC_H

#include "../../../../../../../include/sigma_kernel_types.h"
void          sigma_exit(int code);
sigma_ssize_t sigma_write(int fd, const void* buf, sigma_size_t count);
sigma_ssize_t sigma_read(int fd, void* buf, sigma_size_t count);
int           sigma_open(const char* filename, int flags, int mode);
int           sigma_close(int fd);
void*         sigma_mmap(void* addr, sigma_size_t length, int prot, int flags, int fd, sigma_u64 offset);
sigma_size_t  sigma_strlen(const char* s);
int           sigma_streq(const char* s1, const char* s2);
void          sigma_print(const char* str);
void          sigma_print_num(sigma_u64 val);
void          sigma_print_hex(sigma_u64 val);
void          sigma_printf(const char* format, ...);
void*         sigma_malloc(sigma_size_t size);
void          sigma_free(void* ptr);
void          sigma_log(const char* msg);
#endif /* SIGMA_LIBC_H */
