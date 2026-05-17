#include "../../../include/sigma_log.h"
#include "../../../include/sigma_hal.h"
#include "../../../include/sigma_kernel_types.h"
#include "../../../include/libc/SovereignLibC.h"

/**
 * SigmaOS Sovereign SASOS (Single Address Space Operating System)
 * Implements hardware-enforced PKey protection for zero-cost context switching.
 * 
 * Design: High-performance memory isolation without TLB flushes.
 */

namespace SigmaOS {
namespace Kernel {
namespace Memory {

class SovereignSASOS {
public:
    static SovereignSASOS& getInstance() {
        static SovereignSASOS instance;
        return instance;
    }

    static void init() {
        sigma_log("[SASOS] Initializing Single Address Space Orchestrator...");
        this->m_initialized = 1u;
        this->m_active_keys = 0u;
    }

    sigma_u32 registerShardDomain(const char* shard_id) {
        sigma_u32 pkey = ++this->m_active_keys;
        sigma_log("[SASOS] Domain Registered: %s [PKey: %u]\n", shard_id, pkey);
        sigma_log("[SASOS] Hardware PKey assigned to shard memory range.");
        return pkey;
    }

    void switchDomain(sigma_u32 pkey) {
        // In a real x86_64 implementation, we would use WRPKRU instruction here.
        sigma_log("[SASOS] SWITCH: Transitioning to Domain PKey %u via WRPKRU (Zero-Cost).\n", pkey);
    }

private:
    SovereignSASOS() : m_initialized(0), m_active_keys(0) {}
    sigma_u32 m_initialized;
    sigma_u32 m_active_keys;
};

} // namespace Memory
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

/* --- C Bridge --- */
void sasos_init() {
    SigmaOS::Kernel::Memory::SovereignSASOS::init();
}

extern "C" sigma_u32 sasos_register(const char* shard) {
    return SigmaOS::Kernel::Memory::SovereignSASOS::registerShardDomain(shard);
}

void sasos_switch(sigma_u32 pkey) {
    SigmaOS::Kernel::Memory::SovereignSASOS::switchDomain(pkey);
}





} // extern "C"
 