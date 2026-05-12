#ifndef SIGMA_LIBC_H
#define SIGMA_LIBC_H

#include "core/sigma_types.h"

extern "C" {
    void* sigma_mmap(void* addr, sigma_u64 length, int prot, int flags, int fd, sigma_u64 offset);
    void  sigma_print(const char* msg);
    void* sigma_memcpy(void* dest, const void* src, sigma_size_t n);
    void* sigma_memset(void* s, int c, sigma_size_t n);
}

#endif
