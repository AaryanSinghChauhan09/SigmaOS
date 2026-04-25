// SigmaOS — sigma-sec-integrity: Continuous Integrity Verification
// Module: sigma-sec-integrity
// USP: Defeats Windows enterprise trust models by continuously verifying module hashes
//      against the sovereign registry. Triggers auto-rollback upon mismatch.

#ifndef SIGMA_SEC_INTEGRITY_HPP
#define SIGMA_SEC_INTEGRITY_HPP

#include "../S41_SiliconBoot/sigma_auto_rollback.hpp"

namespace sigma {
namespace security {

class IntegrityVerifier {
private:
    sigma::auto_layer::RollbackManager* rollback_manager;

    unsigned long calculate_fnv1a(const unsigned char* data, unsigned int size) {
        unsigned long h = 14695981039346656037UL;
        for (unsigned int i = 0; i < size; i++) {
            h ^= data[i];
            h *= 1099511628211UL;
        }
        return h;
    }

public:
    IntegrityVerifier(sigma::auto_layer::RollbackManager* rm) : rollback_manager(rm) {}

    // Continuously called in the background via idle tasks
    void verify_module_integrity(const char* module_name, const unsigned char* module_mem, unsigned int size, unsigned long expected_hash) {
        (void)module_name;
        unsigned long computed = calculate_fnv1a(module_mem, size);
        
        if (computed != expected_hash) {
            // INTEGRITY COMPROMISED: Subsystem modified at runtime (rootkit or bit flip)
            // Trigger auto-rollback instantly
            if (rollback_manager) {
                rollback_manager->register_boot_failure(sigma::auto_layer::RollbackTrigger::SECURE_BOOT_VIOLATION);
            }
        }
    }
};

} // namespace security
} // namespace sigma

#endif /* SIGMA_SEC_INTEGRITY_HPP */
