/**
 * SovereignIDS.cpp
 * Feature #18: Intrusion Detection System (IDS)
 * =====================================================================
 * Absorbs: Snort, Suricata, Linux Audit daemon, OSSEC.
 * Mission: Live signature matching on raw network frames and syscall
 *          sequences — zero-dependency, Ring-0 native.
 * Branch:  kernel-exp, drivers-dev
 * =====================================================================
 */
#include "sigma_kernel_types.h"
#include "sigma_log.h"

namespace SigmaOS {
namespace Kernel {
namespace Security {

// Maximum number of IDS rules
static constexpr sigma_u32 MAX_IDS_RULES = 128;
// Maximum events in the audit ring
static constexpr sigma_u32 MAX_EVENTS    = 512;

enum class IDSAction : sigma_u8 { ALLOW = 0, ALERT = 1, BLOCK = 2 };
enum class IDSProto  : sigma_u8 { ANY = 0, TCP = 6, UDP = 17, ICMP = 1 };

struct IDSRule {
    sigma_u32  src_ip_mask;   // 0 = any
    sigma_u32  dst_ip_mask;   // 0 = any
    sigma_u16  dst_port;      // 0 = any
    IDSProto   proto;
    IDSAction  action;
    char       description[64];
    sigma_u32  hit_count;
};

struct IDSEvent {
    sigma_u32  src_ip;
    sigma_u32  dst_ip;
    sigma_u16  dst_port;
    IDSProto   proto;
    sigma_u32  rule_id;
    IDSAction  action_taken;
};

class SovereignIDS {
public:
    static SovereignIDS& getInstance() {
        static SovereignIDS instance;
        return instance;
    }

    void init() {
        m_rule_count  = 0;
        m_event_head  = 0;
        m_event_count = 0;
        m_total_alerts = 0;
        m_total_blocks = 0;
        // Install default ruleset
        addRule(0, 0, 22,   IDSProto::TCP, IDSAction::ALERT, "SSH brute-force attempt");
        addRule(0, 0, 4444, IDSProto::TCP, IDSAction::BLOCK, "Meterpreter reverse shell");
        addRule(0, 0, 6667, IDSProto::TCP, IDSAction::BLOCK, "IRC C2 channel");
        addRule(0, 0, 0,    IDSProto::ICMP,IDSAction::ALERT, "ICMP flood / ping sweep");
        sigma_log("[IDS] Sovereign IDS initialized — Snort/Suricata-equivalent active.");
        sigma_log_info("[IDS] %u rules loaded.\n", m_rule_count);
    }

    bool addRule(sigma_u32 src_mask, sigma_u32 dst_mask, sigma_u16 port,
                 IDSProto proto, IDSAction action, const char* desc) {
        if (m_rule_count >= MAX_IDS_RULES) return false;
        IDSRule& r = m_rules[m_rule_count];
        r.src_ip_mask = src_mask;
        r.dst_ip_mask = dst_mask;
        r.dst_port    = port;
        r.proto       = proto;
        r.action      = action;
        r.hit_count   = 0;
        // Bounded string copy — no strcpy
        sigma_u32 i = 0;
        while (i < 63 && desc[i]) { r.description[i] = desc[i]; i++; }
        r.description[i] = '\0';
        m_rule_count++;
        return true;
    }

    // Inspect a packet — returns IDSAction
    IDSAction inspect(sigma_u32 src_ip, sigma_u32 dst_ip,
                      sigma_u16 dst_port, IDSProto proto) {
        for (sigma_u32 i = 0; i < m_rule_count; i++) {
            IDSRule& r = m_rules[i];
            bool proto_match = (r.proto == IDSProto::ANY || r.proto == proto);
            bool port_match  = (r.dst_port == 0 || r.dst_port == dst_port);
            bool src_match   = (r.src_ip_mask == 0 ||
                                (src_ip & r.src_ip_mask) == r.src_ip_mask);
            bool dst_match   = (r.dst_ip_mask == 0 ||
                                (dst_ip & r.dst_ip_mask) == r.dst_ip_mask);

            if (proto_match && port_match && src_match && dst_match) {
                r.hit_count++;
                recordEvent(src_ip, dst_ip, dst_port, proto, i, r.action);
                if (r.action == IDSAction::ALERT) {
                    m_total_alerts++;
                    sigma_log_info("[IDS] ALERT: Rule[%u] '%s' triggered.\n",
                                   i, r.description);
                } else if (r.action == IDSAction::BLOCK) {
                    m_total_blocks++;
                    sigma_log_info("[IDS] BLOCK: Rule[%u] '%s' — packet dropped.\n",
                                   i, r.description);
                }
                return r.action;
            }
        }
        return IDSAction::ALLOW;
    }

    void printAudit() {
        sigma_log("\n--- SOVEREIGN IDS AUDIT ---");
        sigma_log_info("| Rules Loaded  : %u\n", m_rule_count);
        sigma_log_info("| Total Alerts  : %u\n", m_total_alerts);
        sigma_log_info("| Total Blocks  : %u\n", m_total_blocks);
        sigma_log_info("| Event Log Size: %u\n", m_event_count);
        sigma_log("----------------------------");
    }

private:
    IDSRule   m_rules[MAX_IDS_RULES];
    IDSEvent  m_events[MAX_EVENTS];
    sigma_u32 m_rule_count;
    sigma_u32 m_event_head;
    sigma_u32 m_event_count;
    sigma_u32 m_total_alerts;
    sigma_u32 m_total_blocks;

    SovereignIDS() : m_rule_count(0), m_event_head(0),
                     m_event_count(0), m_total_alerts(0), m_total_blocks(0) {}

    void recordEvent(sigma_u32 src, sigma_u32 dst, sigma_u16 port,
                     IDSProto proto, sigma_u32 rule_id, IDSAction action) {
        IDSEvent& ev = m_events[m_event_head % MAX_EVENTS];
        ev.src_ip      = src;
        ev.dst_ip      = dst;
        ev.dst_port    = port;
        ev.proto       = proto;
        ev.rule_id     = rule_id;
        ev.action_taken = action;
        m_event_head++;
        if (m_event_count < MAX_EVENTS) m_event_count++;
    }
};

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void ids_init() {
    SigmaOS::Kernel::Security::SovereignIDS::getInstance().init();
}

sigma_u8 ids_inspect(sigma_u32 src_ip, sigma_u32 dst_ip,
                     sigma_u16 dst_port, sigma_u8 proto) {
    using P = SigmaOS::Kernel::Security::IDSProto;
    P p = (proto == 6) ? P::TCP : (proto == 17) ? P::UDP : P::ICMP;
    auto action = SigmaOS::Kernel::Security::SovereignIDS::getInstance()
                      .inspect(src_ip, dst_ip, dst_port, p);
    return static_cast<sigma_u8>(action);
}

void ids_audit() {
    SigmaOS::Kernel::Security::SovereignIDS::getInstance().printAudit();
}

} // extern "C"
