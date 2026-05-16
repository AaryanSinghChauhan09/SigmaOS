#include "../../../include/core/sigma_types.h"
#include "../../../include/sigma_hal.h"
#include "../../../include/libc/SovereignLibC.h"
#ifndef PORT_SHARD_HPP
#define PORT_SHARD_HPP

#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignPortShard {
public:
    static inline void outb(sigma_u16 port, sigma_u8 val) {
#if defined(SIGMA_ARCH_X86_64)
        __asm__ volatile ("outb %0, %1" : : "a"(val), "Nd"(port));
#endif
    }

    static inline void outw(sigma_u16 port, sigma_u16 val) {
#if defined(SIGMA_ARCH_X86_64)
        __asm__ volatile ("outw %0, %1" : : "a"(val), "Nd"(port));
#endif
    }

    static inline sigma_u8 inb(sigma_u16 port) {
        sigma_u8 ret = 0;
#if defined(SIGMA_ARCH_X86_64)
        __asm__ volatile ("inb %1, %0" : "=a"(ret) : "Nd"(port));
#endif
        return ret;
    }

    static void WaitIO() {
        outb(0x80, 0); // Traditional POST port wait
    }
};

} // namespace Kernel
} // namespace SigmaOS

#endif

