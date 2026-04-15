#ifndef SIGMA_STRING_H
#define SIGMA_STRING_H

#include "sigma_types.h"

/* Atomic Shard Prototypes */
void* sigma_memcpy(void* dest, const void* src, sigma_size n);
void* sigma_memset(void* s, sigma_u8 c, sigma_size n);
sigma_size sigma_strlen(const char* s);
sigma_s32 sigma_strcmp(const char* s1, const char* s2);

#endif // SIGMA_STRING_H
