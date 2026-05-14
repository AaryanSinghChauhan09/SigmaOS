#ifndef SOVEREIGN_ARCH_HPP
#define SOVEREIGN_ARCH_HPP

#include "sigma_types.h"
#include "SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Architecture Abstraction (S-ARCH)
 * Purpose: Provide a unified interface for the 20+ supported industrial architectures.
 * Principle: ISA-Agnostic Lattice Execution.
 */

namespace SigmaOS {
namespace Kernel {
namespace Arch {

class SovereignArch : public SigmaOS::SigmaObject {
public:
    virtual const char* arch_name() const noexcept = 0;
    
    // Core Machine Control
    virtual void halt() = 0;
    virtual void reboot() = 0;
    
    // Memory Management (Specialized per ISA)
    virtual void setupPaging(sigma_u64 phys_base) = 0;
    
    // Interrupt Control
    virtual void enableInterrupts() = 0;
    virtual void disableInterrupts() = 0;
    
    // Multi-Core Orchestration (SMP)
    virtual sigma_u32 getCpuCount() = 0;
    virtual sigma_u32 getCurrentCpuId() = 0;

    virtual ~SovereignArch() = default;
};

} // namespace Arch
} // namespace Kernel
} // namespace SigmaOS

#endif
