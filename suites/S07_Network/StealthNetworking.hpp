#pragma once
#include <stdint.h>
#include "../S01_Genesis/sigma_libc.h"

namespace SigmaOS {
namespace Network {

// Phase 3/7: Stealth Networking & VPN & Firewall
struct FirewallRule {
    uint16_t port;
    bool allow;
    bool active;
};

class StealthNetworking {
private:
    bool vpn_active;
    bool firewall_strict;
    FirewallRule rules[128];
    uint32_t rule_count;

public:
    StealthNetworking() : vpn_active(false), firewall_strict(true), rule_count(0) {
        sigma_log("[NET-SECURITY] Sovereign Firewall & Stealth Module Online.");
    }

    void toggle_stealth_mode(bool enable) {
        vpn_active = enable;
        sigma_print("[NET-SECURITY] Stealth Networking Mode: ");
        sigma_print(enable ? "ACTIVE (Routing through Sovereign VPN Mesh)\n" : "DISABLED\n");
    }

    void add_firewall_rule(uint16_t port, bool allow) {
        if (rule_count >= 128) return;
        rules[rule_count] = {port, allow, true};
        sigma_print("[NET-SECURITY] Firewall Rule Added: Port ");
        sigma_print_num(port);
        sigma_print(allow ? " -> ALLOW\n" : " -> DENY\n");
        rule_count++;
    }

    bool filter_packet(uint16_t dst_port) {
        if (firewall_strict) {
            for (uint32_t i = 0; i < rule_count; i++) {
                if (rules[i].active && rules[i].port == dst_port) {
                    return rules[i].allow;
                }
            }
            return false; // Default deny
        }
        return true;
    }
};

} // namespace Network
} // namespace SigmaOS
