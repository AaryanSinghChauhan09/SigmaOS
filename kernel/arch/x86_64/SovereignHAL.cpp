#include "sigma_log.h"
#include "core/sigma_types.h"
#include "core/sigma_kernel_types.h"
#include "libc/SovereignLibC.h"
#include "core/SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Hardware Abstraction Layer (HAL) Shard
 * Principles: Silicon-Direct, Ring-0 Hardware Sharding.
 */


namespace SigmaOS {
namespace Kernel {
namespace HAL {

class SovereignHAL : public SigmaObject {
public:
    static SovereignHAL& getInstance() {
        static SovereignHAL instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignHAL"; }

    static void init() {
        sigma_log("[HAL] Orchestrating Hardware Lattice Shards...");
        serial_init();
        sigma_log("[HAL] Serial IO Shard ONLINE.");
    }

    void outb(sigma_u16 port, sigma_u8 val) {
        __asm__ volatile ("outb %0, %1" : : "a"(val), "Nd"(port));
    }

    sigma_u8 inb(sigma_u16 port) {
        sigma_u8 ret;
        __asm__ volatile ("inb %1, %0" : "=a"(ret) : "Nd"(port));
        return ret;
    }

    void reboot() {
        sigma_log("[HAL] TR-G-00: Initiating Silicon Lattice Reset...");
        // Fast reboot via PS/2 controller
        outb(0x64, 0xFE);
    }

private:
    SovereignHAL() {}
};

} // namespace HAL
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void hal_init() {
    SigmaOS::Kernel::HAL::SovereignHAL::init();
}

extern "C" void hal_reboot() {
    SigmaOS::Kernel::HAL::SovereignHAL::reboot();
}

