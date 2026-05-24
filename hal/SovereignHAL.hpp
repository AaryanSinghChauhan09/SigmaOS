/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN HARDWARE ABSTRACTION LAYER (S-HAL)
 * =========================================================================
 * Mission: Portable cross-architecture hardware control boundary.
 * Platforms: x86_64, ARM64 (Cortex-A), RISC-V (RV64GC)
 * Principle: Bit-perfect, zero-dependency.
 * =========================================================================
 */

#ifndef SOVEREIGN_HAL_HPP
#define SOVEREIGN_HAL_HPP

#include "sigma_kernel_types.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace HAL {

enum class CPULatticeArch : sigma_u8 {
    X86_64 = 0,
    ARM64  = 1,
    RISCV64 = 2
};

enum class DriverType : sigma_u8 {
    WIFI = 0,
    PRINTER = 1,
    USB = 2,
    IOT = 3
};

struct BoardTelemetry {
    CPULatticeArch architecture;
    const char* cpu_brand;
    sigma_u32 core_count;
    sigma_u64 total_physical_ram_bytes;
};

struct UnifiedDriver {
    sigma_u32 id;
    const char* name;
    DriverType type;
    bool active;
    sigma_status (*initialize)();
    sigma_status (*transmit)(const sigma_u8* buffer, sigma_usize size);
};

class SovereignHAL : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignHAL"; }

    static SovereignHAL& getInstance() {
        static SovereignHAL instance;
        return instance;
    }

    void initializeHAL();
    void configureInterrupts();
    void configureHardwareTimers(sigma_u32 frequency_hz);
    void mapPageTableMemory(sigma_u64 physical_address, sigma_u64 virtual_address, sigma_u64 range_bytes);
    
    // Unified Driver API registration
    sigma_status registerDriver(const UnifiedDriver& driver);
    sigma_status dispatchDriverCommand(sigma_u32 driver_id, const sigma_u8* buffer, sigma_usize size);

    BoardTelemetry getSystemTelemetry() const;
    [[noreturn]] void systemReset();

private:
    SovereignHAL();
    CPULatticeArch detectArchitecture();
    void detectCoreCount();

    CPULatticeArch m_arch;
    sigma_u32      m_active_cores;
    sigma_u64      m_total_ram_bytes;
    bool           m_interrupts_configured;
    
    UnifiedDriver m_registered_drivers[32];
    sigma_u32     m_driver_count;
};

} // namespace HAL
} // namespace SigmaOS

#endif // SOVEREIGN_HAL_HPP

