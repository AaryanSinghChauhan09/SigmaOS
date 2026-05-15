#ifndef TIMER_SHARD_HPP
#define TIMER_SHARD_HPP

#include "../../include/core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignTimerShard : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignTimerShard"; }

    sigma_u64 GetTimestamp() {
#if defined(SIGMA_ARCH_X86_64)
        sigma_u32 lo, hi;
        __asm__ volatile ("rdtsc" : "=a"(lo), "=d"(hi));
        return ((sigma_u64)hi << 32) | lo;
#else
        return 0;
#endif
    }

    void MicroSleep(sigma_u64 micros) {
        sigma_u64 start = GetTimestamp();
        // Assuming 3GHz clock for simulation
        sigma_u64 ticks = micros * 3000;
        while (GetTimestamp() - start < ticks) {
            __asm__ volatile ("pause");
        }
    }
};

} // namespace Kernel
} // namespace SigmaOS

#endif
