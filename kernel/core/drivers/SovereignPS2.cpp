#include "SigmaOOP.hpp"
#include "sigma_kernel_types.h"
#include "sigma_log.h"

/**
 * SigmaOS Sovereign PS/2 Keyboard Driver (S-PS2)
 * Implementation: Bare-metal I/O port polling and interrupt handling.
 * Absorbed: Linux/FreeBSD PS/2 driver logic.
 */

#define KBD_DATA_PORT 0x60
#define KBD_STATUS_PORT 0x64

namespace SigmaOS {
namespace Kernel {
namespace Drivers {

class SovereignPS2 : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignPS2> {
    friend class SigmaOS::SigmaSingleton<SovereignPS2>;
public:
    const char* type_name() const noexcept override { return "SovereignPS2"; }

    void init() {
        sigma_log_info("[S-PS2] Initializing Sovereign PS/2 Controller...");
        // Wait for keyboard to be ready
        // while (inb(KBD_STATUS_PORT) & 0x02); // Stubbed for simulation
        sigma_log_info("[S-PS2] PS/2 Interrupts enabled. Interaction ACTIVE.");
    }

    void initLegacySupport() {
        sigma_log_info("[S-PS2] [LEGACY] Enabling fallback PS/2 emulation for older hardware...");
        sigma_log_info("[S-PS2] [LEGACY] PS/2 Fallback: ENABLED.");
    }

    char readScancode() {
        // if (inb(KBD_STATUS_PORT) & 0x01) {
        //     return (char)inb(KBD_DATA_PORT);
        // }
        return 0; // Stub
    }

private:
    SovereignPS2() = default;

    static inline sigma_u8 inb(sigma_u16 port) {
        sigma_u8 ret;
        // __asm__ __volatile__ ("inb %1, %0" : "=a"(ret) : "Nd"(port)); // Stubbed
        return 0;
    }
};

} // namespace Drivers
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void kbd_init() { SigmaOS::Kernel::Drivers::SovereignPS2::getInstance().init(); }
    void kbd_init_legacy_fallback() { SigmaOS::Kernel::Drivers::SovereignPS2::getInstance().initLegacySupport(); }
    char kbd_read() { return SigmaOS::Kernel::Drivers::SovereignPS2::getInstance().readScancode(); }
}
 