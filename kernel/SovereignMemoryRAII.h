#ifndef SOVEREIGN_MEMORY_RAII_H
#define SOVEREIGN_MEMORY_RAII_H

#include "../libc/SovereignLibC.h"
#include "../SigmaC11.h"

/* =========================================================================
 * Σ SIGMAOS: SOVEREIGN QUANTUM MEMORY MANAGER (RAII-Compliant in pure C)
 * =========================================================================
 * Restores the mistakenly deleted Matrix De-allocator functionality. 
 * Replaces high-level garbage collection or C++ constructors/destructors
 * by mapping GCC cleanup attributes to Sovereign's native memory shims!
 * ========================================================================= */

typedef struct {
    void* ptr;
    const char* shard_owner;
} SovereignRAIIMatrix;

/* The Sovereign Cleanup Routine - automatically fires at end of scope */
static inline void _sovereign_raii_cleanup(SovereignRAIIMatrix* mat) {
    if (mat && mat->ptr) {
        sigma_log("[SOVEREIGN-RAII]: Auto-purging dynamic memory shard scope.");
        sigma_dealloc(mat->ptr);
        mat->ptr = 0; /* Nullify to prevent double-free in low-level bounds */
    }
}

/* Macro to enforce Sovereign RAII encapsulation across the kernel */
#define SOVEREIGN_AUTOSHARD(type, name, size, owner) \
    SovereignRAIIMatrix name __attribute__((cleanup(_sovereign_raii_cleanup))) = \
        { sigma_alloc(size), owner }; \
    type* name##_ptr = (type*)name.ptr;

#endif /* SOVEREIGN_MEMORY_RAII_H */
