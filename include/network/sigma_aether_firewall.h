/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN AETHER FIREWALL (Neural Nexus)
 * =========================================================================
 * Mission: AI-driven packet filtering and protocol ghosting.
 * Layer  : L2 — System Services / Network
 * =========================================================================
 */
#ifndef SIGMA_AETHER_FIREWALL_H
#define SIGMA_AETHER_FIREWALL_H

#include "include/sigma_types.h"

#ifdef __cplusplus
namespace SigmaOS {
namespace Kernel {
namespace Network {

class SovereignAetherFirewall {
public:
    static SovereignAetherFirewall& getInstance();

    void init();
    bool inspectPacket(const void* data, sigma_size_t size, const char* source);
    void triggerSelfHealing();
    void auditFirewall() const;

    sigma_u32 getBlockedCount() const { return m_blocked_threats; }

private:
    SovereignAetherFirewall() = default;
    SovereignAetherFirewall(const SovereignAetherFirewall&) = delete;
    SovereignAetherFirewall& operator=(const SovereignAetherFirewall&) = delete;

    sigma_u32 m_initialized{0u};
    sigma_u32 m_blocked_threats{0u};
};

} // namespace Network
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
#endif

void firewall_init(void);
int  firewall_inspect(const void* data, sigma_size_t size, const char* src);
void firewall_audit(void);

#ifdef __cplusplus
} // extern "C"
#endif

#endif /* SIGMA_AETHER_FIREWALL_H */
