/* 
 Σ SIGMAOS ZENITH: SOVEREIGN INTERNAL UTILS (v2400.0)
 Mission: SSE-Accelerated Silicon Performance & Stack Retrieval.
*/

#ifndef SIGMA_INTERNAL_H
#define SIGMA_INTERNAL_H

#include <stdint.h>
#include <stdbool.h>

// Σ BARE-METAL PRINTS
void sigma_print(const char* s);

// Σ SSE-ACCELERATED MEMORY SHARD
inline void* sigma_memcpy_sse(void* dest, const void* src, uint32_t n);

// Σ STACK TRACE RECOVERY (v2400.0)
// Crawls the frame pointer (RBP) to identify mission violations.
typedef struct {
    uint64_t rbp;
    uint64_t rip;
} sigma_stack_frame;

inline void sigma_stack_trace(uint32_t depth) {
    sigma_print("\nΣ [STACK]: Mission Context Retrieval...\n");
    sigma_stack_frame* frame;
    __asm__ volatile ("mov %%rbp, %0" : "=r"(frame));
    
    for (uint32_t i = 0; i < depth && frame; i++) {
        sigma_print("  FRAME "); // sigma_print_int(i);
        sigma_print(": RIP 0x"); // sigma_print_hex(frame->rip);
        sigma_print("\n");
        frame = (sigma_stack_frame*)frame->rbp;
    }
}

// Σ UTILS: STRING SHARDS
char* sigma_strcpy(char* dest, const char* src);
const char* sigma_strstr(const char* str, const char* substr);

#endif
