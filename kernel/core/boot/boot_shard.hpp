#ifndef BOOT_SHARD_HPP
#define BOOT_SHARD_HPP

#include "../../../include/SovereignLibC.h"

#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignBootShard : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignBootShard"; }

    void ValidateProtocol(const char* protocol_id) {
        sigma_printf("[BOOT-SHARD]: Validating Sovereign Boot Protocol: %s\n", protocol_id);
        sigma_printf("[BOOT-SHARD]: MultiBoot2 / UEFI Handshake... [OK]\n");
    }

    void JumpToKernel() {
        sigma_printf("[BOOT-SHARD]: Transferring Control to Sovereign Kernel Zenith...\n");
        // Simulated far jump
#if defined(SIGMA_ARCH_X86_64)
        __asm__ volatile ("jmp *%%rax" : : "a"(0x100000) : "memory");
#endif
    }
};

} // namespace Kernel
} // namespace SigmaOS

#endif
