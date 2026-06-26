/*
 * Σ SigmaOS — sigma_firewall: Network Packet Filtering Engine
 * Zero-Dependency: Integrates directly with the sovereign TCP/IP stack.
 * Absorbs: iptables / nftables concept of chains and rules.
 */

typedef unsigned int u32;
typedef unsigned short u16;
typedef unsigned char u8;

extern "C" void sigma_vga_printf(const char* fmt, ...);

#define ACTION_ACCEPT 1
#define ACTION_DROP   2

struct FirewallRule {
    u32 src_ip;
    u32 src_mask;
    u32 dst_ip;
    u32 dst_mask;
    u16 dst_port;
    u32 protocol; /* e.g., TCP, UDP, ICMP */
    u8  action;
    bool active;
};

#define MAX_RULES 128
static FirewallRule rule_chain[MAX_RULES];

/* Add a rule to the chain */
extern "C" int sigma_fw_add_rule(u32 sip, u32 smask, u32 dip, u32 dmask, u16 port, u32 proto, u8 action) {
    for (int i = 0; i < MAX_RULES; i++) {
        if (!rule_chain[i].active) {
            rule_chain[i].src_ip = sip;
            rule_chain[i].src_mask = smask;
            rule_chain[i].dst_ip = dip;
            rule_chain[i].dst_mask = dmask;
            rule_chain[i].dst_port = port;
            rule_chain[i].protocol = proto;
            rule_chain[i].action = action;
            rule_chain[i].active = true;
            return i;
        }
    }
    return -1;
}

/* Inspect an incoming/outgoing packet against rules */
extern "C" u8 sigma_fw_inspect(u32 src_ip, u32 dst_ip, u16 dst_port, u32 protocol) {
    for (int i = 0; i < MAX_RULES; i++) {
        if (rule_chain[i].active) {
            /* Check protocol */
            if (rule_chain[i].protocol != 0 && rule_chain[i].protocol != protocol) continue;
            
            /* Check port */
            if (rule_chain[i].dst_port != 0 && rule_chain[i].dst_port != dst_port) continue;
            
            /* Check Source IP via mask */
            if ((src_ip & rule_chain[i].src_mask) != (rule_chain[i].src_ip & rule_chain[i].src_mask)) continue;
            
            /* Check Dest IP via mask */
            if ((dst_ip & rule_chain[i].dst_mask) != (rule_chain[i].dst_ip & rule_chain[i].dst_mask)) continue;
            
            /* Match found! Execute action */
            sigma_vga_printf("[FW] Rule %d triggered -> Action: %d\n", i, rule_chain[i].action);
            return rule_chain[i].action;
        }
    }
    /* Default policy */
    return ACTION_ACCEPT; 
}
