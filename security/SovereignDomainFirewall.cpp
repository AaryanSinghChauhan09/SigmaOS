/**
 * SovereignDomainFirewall.cpp
 * Feature: Domain Firewall Manager (Qubes-style)
 * =====================================================================
 * Absorbs: Qubes OS inter-VM firewall, iptables/nftables rule engine.
 * Mission: Enforce strict per-domain network firewall rules with
 *          full GUI/CLI control. Each isolated domain gets its own
 *          ruleset governing ingress/egress traffic.
 * Branch:  security, kernel-exp
 * =====================================================================
 */
#include "sigma_kernel_types.h"
#include "sigma_log.h"

namespace SigmaOS {
namespace Security {
namespace Firewall {

static constexpr sigma_u32 MAX_DOMAINS   = 64;
static constexpr sigma_u32 MAX_RULES     = 256;

enum class RuleAction : sigma_u8 {
    ALLOW  = 0,
    DENY   = 1,
    LOG    = 2,  // allow but log
    DROP   = 3   // silent drop
};

enum class Protocol : sigma_u8 {
    ANY  = 0,
    TCP  = 6,
    UDP  = 17,
    ICMP = 1
};

enum class Direction : sigma_u8 {
    INGRESS = 0,
    EGRESS  = 1,
    BOTH    = 2
};

struct FirewallRule {
    sigma_u32  rule_id;
    sigma_u32  domain_id;
    Protocol   proto;
    Direction  dir;
    RuleAction action;
    sigma_u32  src_ip;       // 0 = any
    sigma_u32  dst_ip;       // 0 = any
    sigma_u16  src_port;     // 0 = any
    sigma_u16  dst_port;     // 0 = any
    sigma_u64  match_count;
    bool       active;
};

struct Domain {
    sigma_u32 id;
    char      name[48];
    bool      isolated;       // full isolation mode
    sigma_u32 rule_count;
    sigma_u32 packets_blocked;
    sigma_u32 packets_allowed;
};

class SovereignDomainFirewall {
public:
    static SovereignDomainFirewall& getInstance() {
        static SovereignDomainFirewall inst;
        return inst;
    }

    void init() {
        m_domain_count = 0;
        m_rule_count   = 0;
        m_total_evals  = 0;
        sigma_log("[DOMFW] Sovereign Domain Firewall Manager initialised.");
        sigma_log("[DOMFW] Mode: Qubes-style per-domain isolation with strict rule enforcement.");
    }

    // Register a security domain
    sigma_u32 registerDomain(const char* name, bool isolated) {
        if (m_domain_count >= MAX_DOMAINS) return 0;
        Domain& d = m_domains[m_domain_count];
        d.id = m_domain_count + 1;
        sigma_u32 i = 0;
        while (i < 47 && name[i]) { d.name[i] = name[i]; i++; }
        d.name[i] = '\0';
        d.isolated = isolated;
        d.rule_count = 0;
        d.packets_blocked = 0;
        d.packets_allowed = 0;
        m_domain_count++;
        sigma_log_info("[DOMFW] Domain registered: '%s' (id=%u, isolated=%d).\n",
                       d.name, d.id, (int)isolated);
        return d.id;
    }

    // Add a firewall rule
    sigma_u32 addRule(sigma_u32 domain_id, Protocol proto, Direction dir,
                      RuleAction action, sigma_u32 src_ip, sigma_u32 dst_ip,
                      sigma_u16 src_port, sigma_u16 dst_port) {
        if (m_rule_count >= MAX_RULES || domain_id == 0 || domain_id > m_domain_count) return 0;
        FirewallRule& r = m_rules[m_rule_count];
        r.rule_id     = m_rule_count + 1;
        r.domain_id   = domain_id;
        r.proto       = proto;
        r.dir         = dir;
        r.action      = action;
        r.src_ip      = src_ip;
        r.dst_ip      = dst_ip;
        r.src_port    = src_port;
        r.dst_port    = dst_port;
        r.match_count = 0;
        r.active      = true;
        m_rule_count++;
        m_domains[domain_id - 1].rule_count++;
        sigma_log_info("[DOMFW] Rule #%u added to domain %u: proto=%u action=%u.\n",
                       r.rule_id, domain_id, (sigma_u32)proto, (sigma_u32)action);
        return r.rule_id;
    }

    // Evaluate a packet against domain rules
    RuleAction evaluate(sigma_u32 domain_id, Protocol proto, Direction dir,
                        sigma_u32 src_ip, sigma_u32 dst_ip,
                        sigma_u16 src_port, sigma_u16 dst_port) {
        m_total_evals++;
        if (domain_id == 0 || domain_id > m_domain_count) return RuleAction::DENY;

        Domain& d = m_domains[domain_id - 1];

        // If domain is fully isolated, deny everything
        if (d.isolated) {
            d.packets_blocked++;
            return RuleAction::DROP;
        }

        // Match rules in order (first match wins)
        for (sigma_u32 i = 0; i < m_rule_count; i++) {
            FirewallRule& r = m_rules[i];
            if (!r.active || r.domain_id != domain_id) continue;
            if (r.proto != Protocol::ANY && r.proto != proto) continue;
            if (r.dir != Direction::BOTH && r.dir != dir) continue;
            if (r.src_ip != 0 && r.src_ip != src_ip) continue;
            if (r.dst_ip != 0 && r.dst_ip != dst_ip) continue;
            if (r.src_port != 0 && r.src_port != src_port) continue;
            if (r.dst_port != 0 && r.dst_port != dst_port) continue;

            // Match found
            r.match_count++;
            if (r.action == RuleAction::ALLOW || r.action == RuleAction::LOG) {
                d.packets_allowed++;
            } else {
                d.packets_blocked++;
            }
            return r.action;
        }

        // Default deny
        d.packets_blocked++;
        return RuleAction::DENY;
    }

    void printStatus() {
        sigma_log("\n--- DOMAIN FIREWALL STATUS ---");
        sigma_log_info("| Domains     : %u\n", m_domain_count);
        sigma_log_info("| Rules       : %u\n", m_rule_count);
        sigma_log_info("| Evaluations : %llu\n", (unsigned long long)m_total_evals);
        for (sigma_u32 i = 0; i < m_domain_count; i++) {
            Domain& d = m_domains[i];
            sigma_log_info("|  [%s] rules=%u blocked=%u allowed=%u isolated=%d\n",
                           d.name, d.rule_count, d.packets_blocked, d.packets_allowed, (int)d.isolated);
        }
        sigma_log("------------------------------");
    }

private:
    Domain       m_domains[MAX_DOMAINS];
    FirewallRule  m_rules[MAX_RULES];
    sigma_u32    m_domain_count = 0;
    sigma_u32    m_rule_count   = 0;
    sigma_u64    m_total_evals  = 0;

    SovereignDomainFirewall() = default;
};

} // namespace Firewall
} // namespace Security
} // namespace SigmaOS

extern "C" {

void domfw_init() {
    SigmaOS::Security::Firewall::SovereignDomainFirewall::getInstance().init();
}

sigma_u32 domfw_register_domain(const char* name, bool isolated) {
    return SigmaOS::Security::Firewall::SovereignDomainFirewall::getInstance()
               .registerDomain(name, isolated);
}

sigma_u32 domfw_add_rule(sigma_u32 domain_id, sigma_u8 proto, sigma_u8 dir,
                         sigma_u8 action, sigma_u32 src_ip, sigma_u32 dst_ip,
                         sigma_u16 src_port, sigma_u16 dst_port) {
    using P = SigmaOS::Security::Firewall::Protocol;
    using D = SigmaOS::Security::Firewall::Direction;
    using A = SigmaOS::Security::Firewall::RuleAction;
    return SigmaOS::Security::Firewall::SovereignDomainFirewall::getInstance()
               .addRule(domain_id, (P)proto, (D)dir, (A)action,
                        src_ip, dst_ip, src_port, dst_port);
}

void domfw_status() {
    SigmaOS::Security::Firewall::SovereignDomainFirewall::getInstance().printStatus();
}

} // extern "C"
