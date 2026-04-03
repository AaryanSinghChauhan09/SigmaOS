/* 
 Σ SIGMAOS ZENITH: SOVEREIGN INTERNAL UTILS (v2600.0)
 Mission: SSE-Accelerated Silicon Performance & Total Dependency Purge.
*/

#ifndef SIGMA_INTERNAL_H
#define SIGMA_INTERNAL_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

// Σ BARE-METAL PRINTS
void sigma_print(const char* s);

/* Σ SOVEREIGN STRING UTILITIES (v2600.0)
   ZERO DEPENDENCY MISSION: Replacing <string.h> and <stdio.h>.
*/
size_t sigma_strlen(const char* s);
int sigma_strcmp(const char* s1, const char* s2);
void* sigma_memset(void* s, int c, size_t n);
char* sigma_strcpy(char* dest, const char* src);
const char* sigma_strstr(const char* str, const char* substr);

// Σ SSE-ACCELERATED MEMORY SHARD
inline void* sigma_memcpy_sse(void* dest, const void* src, uint32_t n);

// Σ STACK TRACE RECOVERY
typedef struct {
    uint64_t rbp;
    uint64_t rip;
} sigma_stack_frame;

inline void sigma_stack_trace(uint32_t depth);

#endif
