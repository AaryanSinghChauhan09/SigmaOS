/*
 * =========================================================================
 * Σ SIGMAOS: CANONICAL SigmaOOP HEADER (v19.0 - CANONICAL SHIM)
 * =========================================================================
 * Global shim for the Sovereign OOP framework. All sources that use
 * #include "../../include/SigmaOOP.hpp" will resolve here via -Isuites/include.
 * =========================================================================
 */
#ifndef SIGMA_OOP_HPP
#define SIGMA_OOP_HPP

#include "../../include/core/sigma_types.h"
#include "../../include/libc/SovereignLibC.h"

namespace SigmaOS {

/* Core Types */
typedef sigma_u32 sigma_status;
#define SIGMA_OK    0x00000000U
#define SIGMA_ERROR 0xFFFFFFFFU

/* Sovereign Memory Management (Direct Syscalls) */
class SigmaMemory {
public:
    static void* allocate(sigma_u64 length) {
        return sigma_mmap(0, length, 3, 0x22, -1, 0);
    }
};

/* Sovereign Object Model (The Shard) */
class SigmaObject {
public:
    virtual ~SigmaObject() = default;
    virtual const char* type_name() const noexcept = 0;
};

/* Low-Level Print Sharding */
inline void sigma_log(const char* msg) {
    sigma_print("[SIGMA_LOG]: ");
    sigma_print(msg);
    sigma_print("\n");
}

} // namespace SigmaOS

#endif /* SIGMA_OOP_HPP */
