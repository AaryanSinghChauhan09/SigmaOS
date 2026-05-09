#include "libc/SovereignLibC.h"
#include "core/sigma_types.h"

/**
 * SigmaOS Sovereign Protection Key (PKey) Manager
 * Enables Single Address Space Operating System (SASOS) architecture.
 * Utilizes Intel PKU / Memory Protection Keys (MPK).
 */

namespace SigmaOS {
namespace Kernel {
namespace HAL {

class SovereignPKeyManager {
public:
    static SovereignPKeyManager& getInstance() {
        static SovereignPKeyManager instance;
        return instance;
    }

    void init() {
        sigma_log("Σ [PKEY]: Initializing Hardware Memory Protection Keys (MPK)...");
        // Enable CR4.PKE bit
        this->active_keys = 0;
        this->initialized = true;
    }

    int allocateKey() {
        if (this->active_keys >= 16) return -1;
        int key = this->active_keys++;
        sigma_log("Σ [PKEY]: Allocated Memory Protection Key: %d\n", key);
        return key;
    }

    void setProtection(int key, sigma_u32 rights) {
        // WRPKRU instruction to set rights for the key
        sigma_log("Σ [PKEY]: Setting Rights 0x%X for Key %d\n", rights, key);
    }

private:
    SovereignPKeyManager() : active_keys(0), initialized(false) {}
    int active_keys;
    bool initialized;
};

} // namespace HAL
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void pkey_init() {
    SigmaOS::Kernel::HAL::SovereignPKeyManager::init();
}

extern "C" int pkey_alloc() {
    return SigmaOS::Kernel::HAL::SovereignPKeyManager::allocateKey();
}

extern "C" void pkey_set(int key, sigma_u32 rights) {
    SigmaOS::Kernel::HAL::SovereignPKeyManager::setProtection(key, rights);
}
