/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN OOP FRAMEWORK (v19.0 - ZERO-DEPENDENCY)
 * =========================================================================
 * Mission: Extreme HLL Dependency Reduction (No Stdlib, No Iostream).
 * Capability: Ring-0 OOP via custom vtable sharding.
 * Principle: Bit-Perfect. Silicon-Direct. Zero-Dependency.
 * =========================================================================
 */

#ifndef SIGMA_OOP_HPP
#define SIGMA_OOP_HPP

#include "SovereignLibC.h"

namespace SigmaOS {

// --- Core Types (Low-Level Zenith) ---
typedef sigma_u32 sigma_status;
#define SIGMA_OK    0x00000000
#define SIGMA_ERROR 0xFFFFFFFF

// --- Sovereign Memory Management (Direct Syscalls) ---
class SigmaMemory {
public:
    static void* allocate(sigma_size_t length) {
        // Map direct shard memory via sigma_mmap (Syscall 9)
        return sigma_mmap(0, length, 3, 0x22, -1, 0); 
    }
};

// --- Sovereign Object Model (The Shard) ---
class SigmaObject {
public:
    virtual ~SigmaObject() = default;
    virtual const char* type_name() const noexcept = 0;
};

} // namespace SigmaOS

/* Global overrides for zero-dependency C++ support */
inline void* operator new(sigma_size_t size) {
    return sigma_malloc(size);
}

inline void* operator new[](sigma_size_t size) {
    return sigma_malloc(size);
}

inline void operator delete(void* ptr) noexcept {
    sigma_free(ptr);
}

inline void operator delete(void* ptr, sigma_size_t size) noexcept {
    (void)size;
    sigma_free(ptr);
}

inline void operator delete[](void* ptr) noexcept {
    sigma_free(ptr);
}

#endif
