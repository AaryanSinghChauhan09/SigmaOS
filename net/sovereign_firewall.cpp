#include "../include/sigma_kernel_types.h"

extern "C" {
    void sigma_log_info(const char* fmt, ...);
    void sigma_log_error(const char* fmt, ...);
}

namespace SigmaOS {
namespace Net {

struct FirewallRule {
    sigma_u16 port;
    bool is_tcp;
    bool allow;
};

// Simulated MAC configuration (Default-Deny)
bool firewall_enabled = true;
FirewallRule active_rules[16];
sigma_u32 rule_count = 0;

void enable_firewall() {
    sigma_log_info("[FIREWALL] Sovereign Firewall ENABLED. Default policy: DENY ALL.");
    firewall_enabled = true;
}

void add_firewall_rule(sigma_u16 port, bool is_tcp, bool allow) {
    if (rule_count < 16) {
        active_rules[rule_count++] = {port, is_tcp, allow};
        sigma_log_info("[FIREWALL] Rule added: %s Port %u -> %s", 
            is_tcp ? "TCP" : "UDP", port, allow ? "ALLOW" : "DENY");
    }
}

void flush_firewall_rules() {
    sigma_log_info("[FIREWALL] Flushing all active rules. Restoring pure Default-Deny state.");
    rule_count = 0;
}

// Simulated packet inspection
bool inspect_packet(sigma_u16 dst_port, bool is_tcp) {
    if (!firewall_enabled) return true;
    
    // Check MAC rules
    for (sigma_u32 i = 0; i < rule_count; i++) {
        if (active_rules[i].port == dst_port && active_rules[i].is_tcp == is_tcp) {
            if (active_rules[i].allow) {
                // sigma_log_info("[FIREWALL] Packet on port %u allowed by Sovereign MAC rule.", dst_port);
                return true;
            } else {
                sigma_log_error("[FIREWALL] BLOCKED: Packet on port %u explicitly denied.", dst_port);
                return false;
            }
        }
    }
    
    sigma_log_error("[FIREWALL] BLOCKED: Unsolicited packet on port %u dropped (Default-Deny).", dst_port);
    return false; // Default Deny
}

} // namespace Net
} // namespace SigmaOS
