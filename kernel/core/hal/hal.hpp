/*
 * =========================================================================
 * Σ SIGMAOS: HARDWARE ABSTRACTION LAYER (HAL)
 * =========================================================================
 * ZERO-DEPENDENCY CPU/ARCHITECTURE ABSTRACTION
 * =========================================================================
 */
#pragma once
#include "../../../include/sigma_kernel_types.h"

namespace SigmaOS {
namespace HAL {

class AbstractHAL {
public:
    virtual void initCPU() = 0;
    virtual void initMemory() = 0;
    virtual void initInterrupts() = 0;
    virtual void initTimer() = 0;
    virtual void writePort(sigma_u16 port, sigma_u8 value) = 0;
    virtual sigma_u8 readPort(sigma_u16 port) = 0;
    virtual ~AbstractHAL() {}
};

} // namespace HAL
} // namespace SigmaOS
 