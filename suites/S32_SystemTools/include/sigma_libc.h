#ifndef SIGMA_LIBC_H
#define SIGMA_LIBC_H

#include <stddef.h>
#include <stdint.h>

extern "C" {

// String Operations
size_t sigma_strlen(const char* str);
void sigma_strcpy(char* dest, const char* src);
int sigma_strcmp(const char* s1, const char* s2);
void sigma_strcat(char* dest, const char* src);

// Memory Operations
void* sigma_memset(void* s, int c, size_t n);
void* sigma_memcpy(void* dest, const void* src, size_t n);

// Minimalist IO (Direct to Serial/Console)
void sigma_kprint(const char* str);
void sigma_kprint_int(int val);

}

#endif // SIGMA_LIBC_H
