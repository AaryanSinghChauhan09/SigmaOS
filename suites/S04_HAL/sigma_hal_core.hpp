// SigmaOS — sigma-hal-core: Hardware Abstraction Core
// Module: sigma-hal-core
// USP: Universal abstraction across CPU architectures, isolating the kernel
//      from raw silicon implementations (x86_64, ARM64, RISC-V).

#ifndef SIGMA_HAL_CORE_HPP
#define SIGMA_HAL_CORE_HPP

namespace sigma {
namespace hal {

enum class ArchitectureType {
    X86_64,
    AARCH64,
    RISCV64
};

class HALCore {
public:
    static ArchitectureType get_architecture() {
#if defined(__x86_64__)
        return ArchitectureType::X86_64;
#elif defined(__aarch64__)
        return ArchitectureType::AARCH64;
#elif defined(__riscv) && __riscv_xlen == 64
        return ArchitectureType::RISCV64;
#else
        return ArchitectureType::X86_64; // Default mockup
#endif
    }

    // Abstracted interrupt manipulation
    static inline void enable_interrupts() {
#if defined(__x86_64__)
        __asm__ __volatile__("sti\n\t" ::: "memory");
#endif
    }

    static inline void disable_interrupts() {
#if defined(__x86_64__)
        __asm__ __volatile__("cli\n\t" ::: "memory");
#endif
    }
};

} // namespace hal
} // namespace sigma

#endif /* SIGMA_HAL_CORE_HPP */
