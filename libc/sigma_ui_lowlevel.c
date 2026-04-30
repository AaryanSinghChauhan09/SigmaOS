#include "../include/sigma_types.h"

/* 
 * =========================================================================
 * SIGMAOS: LOW-LEVEL UI PRIMITIVES
 * =========================================================================
 * Implementing user-defined functions for direct hardware/memory manipulation.
 */

void sigma_ui_atomic_inc_frames(sigma_u64* frames) {
    // In a real sovereign kernel, this would be:
    // asm volatile("lock incq %0" : "+m" (*frames));
    if (frames) {
        (*frames)++;
    }
}

void sigma_ui_clear_glass_buffer(void* buffer, sigma_size_t size) {
    // Low-level buffer clearing
    for (sigma_size_t i = 0; i < size; ++i) {
        ((sigma_u8*)buffer)[i] = 0;
    }
}

