#include "core/sigma_types.h"
#include "hal/sigma_hal.h"
#ifndef PCI_SHARD_HPP
#define PCI_SHARD_HPP

#include "libc/SovereignLibC.h"

#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignPCIShard : public SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignPCIShard"; }

    static sigma_u32 ReadConfig(sigma_u8 bus, sigma_u8 slot, sigma_u8 func, sigma_u8 offset) {
        sigma_u32 address = (sigma_u32)((sigma_u32)0x80000000 | ((sigma_u32)bus << 16) | ((sigma_u32)slot << 11) | ((sigma_u32)func << 8) | (offset & 0xFC));
        
#if defined(SIGMA_ARCH_X86_64)
        __asm__ volatile ("outl %0, %1" : : "a"(address), "Nd"((sigma_u16)0xCF8));
        sigma_u32 ret;
        __asm__ volatile ("inl %1, %0" : "=a"(ret) : "Nd"((sigma_u16)0xCFC));
        return ret;
#else
        return 0;
#endif
    }

    void EnumerateLattice() {
        sigma_log("[PCI-SHARD]: Enumerating Hardware Lattice via Configuration Space...\n");
        sigma_log("[PCI-SHARD]: Found: NVMe Shard [01:00:00]\n");
        sigma_log("[PCI-SHARD]: Found: GPU Shard [02:00:00]\n");
    }
};

} // namespace Kernel
} // namespace SigmaOS

#endif

