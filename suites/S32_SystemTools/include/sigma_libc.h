#ifndef SIGMA_LIBC_H
#define SIGMA_LIBC_H

// Absolute Zero-Dependency Definitions (No stddef.h, no stdint.h)
#include "../../../include/sigma_kernel_types.h"

extern "C" {

// String Operations
sigma_size_t sigma_strlen(const char* str);
void sigma_strcpy(char* dest, const char* src, sigma_size_t max_len);
int sigma_strcmp(const char* s1, const char* s2);
void sigma_strcat(char* dest, const char* src, sigma_size_t dest_size);

// Memory Operations
void* sigma_memset(void* s, int c, sigma_size_t n);
void* sigma_memcpy(void* dest, const void* src, sigma_size_t n);

// Minimalist IO (Direct to Serial/Console)
void sigma_kprint(const char* str);
void sigma_kprint_int(int val);

}

#endif // SIGMA_LIBC_H
