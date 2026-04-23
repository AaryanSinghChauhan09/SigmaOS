/* sigma_libc.h — libc canonical shim for ../libc/ relative paths */
#ifndef SIGMA_LIBC_H
#define SIGMA_LIBC_H
#include "../sigma_kernel_types.h"
void          sigma_exit(int code);
long          sigma_write(int fd, const void* buf, unsigned long count);
long          sigma_read(int fd, void* buf, unsigned long count);
void*         sigma_mmap(void* addr, unsigned long length, int prot, int flags, int fd, unsigned long long offset);
unsigned long sigma_strlen(const char* s);
void          sigma_print(const char* str);
void          sigma_print_num(unsigned long long val);
void          sigma_print_hex(unsigned long long val);
void          sigma_printf(const char* format, ...);
void*         sigma_malloc(unsigned long size);
void          sigma_free(void* ptr);
void          sigma_log(const char* msg);
#endif /* SIGMA_LIBC_H */
