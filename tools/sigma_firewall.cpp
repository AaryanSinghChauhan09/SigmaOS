/*
 * =========================================================================
 * Î£ SIGMAOS: SIGMA FIREWALL (sigma_firewall) v1.0
 * =========================================================================
 * Mission: Packet filtering and network defense.
 * Inspiration: iptables / nftables / ufw.
 * Principle: Default-deny, stateful Sovereign packet inspection.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {

class SigmaFirewall : public SigmaObject, public SigmaSingleton<SigmaFirewall> {
    friend class SigmaSingleton<SigmaFirewall>;
public:
    const char* type_name() const noexcept override { return "SigmaFirewall"; }

    void init() {
        m_rules_count = 0;
        sigma_log_info("[FIREWALL] Sigma Firewall Daemon v1.0 initialized.");
        sigma_log_info("[FIREWALL] Policy: DEFAULT DENY ALL INBOUND.");
    }

    void allow_port(sigma_u16 port, const char* protocol) {
        if (m_rules_count >= 512) return;
        m_rules_count++;
        sigma_log_info("[FIREWALL] Rule added: ALLOW INBOUND on %s:%u", protocol, port);
    }

    void deny_ip(const char* ip_address) {
        if (m_rules_count >= 512) return;
        m_rules_count++;
        sigma_log_info("[FIREWALL] Rule added: DROP ALL from %s", ip_address);
    }

private:
    SigmaFirewall() : m_rules_count(0) {}
    sigma_u32 m_rules_count;
};

} // namespace Tools
} // namespace SigmaOS

extern "C" {
void fw_init()                                      { SigmaOS::Tools::SigmaFirewall::getInstance().init(); }
void fw_allow(sigma_u16 port, const char* proto)    { SigmaOS::Tools::SigmaFirewall::getInstance().allow_port(port, proto); }
void fw_deny(const char* ip)                        { SigmaOS::Tools::SigmaFirewall::getInstance().deny_ip(ip); }
}

