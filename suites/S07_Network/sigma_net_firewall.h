// SigmaOS — sigma-net-firewall: Firewall Rules Engine
// Module: sigma-net-firewall
// USP: High-speed IP matching with bitwise masking, O(N) linear scan

#ifndef SIGMA_NET_FIREWALL_H
#define SIGMA_NET_FIREWALL_H

#define SIGMA_FW_MAX_RULES 128
#define SIGMA_FW_ALLOW 1
#define SIGMA_FW_DENY  0

typedef struct SigmaFirewallRule {
    unsigned int  src_ip;
    unsigned int  src_mask;
    unsigned int  dst_ip;
    unsigned int  dst_mask;
    unsigned short src_port; // 0 = any
    unsigned short dst_port; // 0 = any
    unsigned char  protocol; // TCP/UDP/ICMP/Any(0)
    unsigned char  action;   // ALLOW/DENY
    unsigned long  hit_count;
} SigmaFirewallRule;

typedef struct SigmaFirewall {
    SigmaFirewallRule rules[SIGMA_FW_MAX_RULES];
    unsigned int      count;
    unsigned char     default_action; // DENY by default
} SigmaFirewall;

static inline void firewall_init(SigmaFirewall* fw, unsigned char default_action) {
    fw->count = 0;
    fw->default_action = default_action;
}

static inline int firewall_add_rule(SigmaFirewall* fw, 
                                    unsigned int src_ip, unsigned int src_mask,
                                    unsigned int dst_ip, unsigned int dst_mask,
                                    unsigned short src_port, unsigned short dst_port,
                                    unsigned char proto, unsigned char action) {
    if (fw->count >= SIGMA_FW_MAX_RULES) return -1;
    SigmaFirewallRule* r = &fw->rules[fw->count++];
    r->src_ip = src_ip; r->src_mask = src_mask;
    r->dst_ip = dst_ip; r->dst_mask = dst_mask;
    r->src_port = src_port;
    r->dst_port = dst_port;
    r->protocol = proto;
    r->action = action;
    r->hit_count = 0;
    return (int)(fw->count - 1);
}

// Evaluate a packet against the firewall ruleset
static inline int firewall_evaluate(SigmaFirewall* fw,
                                    unsigned int src_ip, unsigned int dst_ip,
                                    unsigned short src_port, unsigned short dst_port,
                                    unsigned char proto) {
    for (unsigned int i = 0; i < fw->count; i++) {
        SigmaFirewallRule* r = &fw->rules[i];
        
        // IP Match
        if ((src_ip & r->src_mask) != (r->src_ip & r->src_mask)) continue;
        if ((dst_ip & r->dst_mask) != (r->dst_ip & r->dst_mask)) continue;
        
        // Port Match
        if (r->src_port != 0 && src_port != r->src_port) continue;
        if (r->dst_port != 0 && dst_port != r->dst_port) continue;
        
        // Protocol Match
        if (r->protocol != 0 && proto != r->protocol) continue;
        
        // Matched!
        r->hit_count++;
        return r->action;
    }
    return fw->default_action;
}

#endif /* SIGMA_NET_FIREWALL_H */
