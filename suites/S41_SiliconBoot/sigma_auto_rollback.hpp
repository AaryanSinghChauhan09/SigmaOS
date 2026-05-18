// SigmaOS — sigma-auto-rollback: OOP Rollback Automation
// Module: sigma-auto-rollback
// USP: Encapsulates hardware-level storage snapshot reversion logic upon boot failure.

#ifndef SIGMA_AUTO_ROLLBACK_HPP
#define SIGMA_AUTO_ROLLBACK_HPP

#include "S43_SovereignCaps/sigma_caps.h"

namespace sigma {
namespace auto_layer {

enum class RollbackTrigger {
    KERNEL_PANIC,
    WATCHDOG_TIMEOUT,
    SECURE_BOOT_VIOLATION,
    MANUAL_USER_REQUEST
};

class RollbackManager {
private:
    unsigned int active_slot; // 0 for A, 1 for B
    unsigned int failed_boot_count;

public:
    RollbackManager(unsigned int boot_slot) 
        : active_slot(boot_slot), failed_boot_count(0) {}

    void register_boot_failure(RollbackTrigger trigger) {
        failed_boot_count++;
        if (failed_boot_count >= 3 || trigger == RollbackTrigger::SECURE_BOOT_VIOLATION) {
            initiate_rollback(nullptr); // System level rollback without token
        }
    }

    bool initiate_rollback(SigmaCapToken* admin_token) {
        // Enforce capability check if manually requested
        if (admin_token && !(admin_token->capabilities & SIGMA_CAP_ADMIN)) {
            return false;
        }

        // Switch active slot and trigger reboot (Simulated)
        active_slot = (active_slot == 0) ? 1 : 0;
        
        // Inline ASM to force CPU reset (Triple fault via IDT corruption)
#if defined(__x86_64__) || defined(__i386__)
        __asm__ __volatile__ (
            "lidt (%0)\n\t"
            "int $3\n\t"
            : : "r"(0) : "memory"
        );
#endif
        return true;
    }
};

} // namespace auto_layer
} // namespace sigma

#endif /* SIGMA_AUTO_ROLLBACK_HPP */
