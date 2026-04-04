/* 
 Σ SIGMAOS ZENITH: SOVEREIGN INTERNAL UTILS (v2800.0)
 Mission: SSE-Accelerated Silicon Performance & Absolute Type Sovereignty.
*/

#ifndef SIGMA_INTERNAL_H
#define SIGMA_INTERNAL_H

#include "sigma_kernel_types.h"

/** Σ SOVEREIGN STRING UTILITIES (v2600.0)
 * Logic: Primitive Shards for Silicon Orchestration.
 */
// Redundant declarations removed - using sigma_kernel_types.h inlines.
void sigma_print(const char* s);
void sigma_clear_screen(void);
char* sigma_strcpy(char* dest, const char* src);

// Σ SSE-ACCELERATED MEMORY SHARD
inline void* sigma_memcpy_sse(void* dest, const void* src, u32 n);

// Σ STACK TRACE RECOVERY
typedef struct {
    u64 rbp;
    u64 rip;
} sigma_stack_frame;

inline void sigma_stack_trace(u32 depth);

#endif
