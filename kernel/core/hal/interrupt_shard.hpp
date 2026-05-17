#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_hal.h"
#include "../../../include/libc/SovereignLibC.h"
#ifndef INTERRUPT_SHARD_HPP
#define INTERRUPT_SHARD_HPP

#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignInterruptShard : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignInterruptShard"; }

    void DisableInterrupts() {
#if defined(SIGMA_ARCH_X86_64)
        __asm__ volatile ("cli" : : : "memory");
#endif
    }

    void EnableInterrupts() {
#if defined(SIGMA_ARCH_X86_64)
        __asm__ volatile ("sti" : : : "memory");
#endif
    }

    void RaiseInterrupt(sigma_u8 vector) {
#if defined(SIGMA_ARCH_X86_64)
        // Simulated software interrupt
        (void)vector;
        __asm__ volatile ("int $0x80" : : : "memory");
#endif
    }
};

} // namespace Kernel
} // namespace SigmaOS

#endif

 