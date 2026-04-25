// SigmaOS — Sigma-NetFilter: Zero-Copy Packet Firewall
// Inspired by: Linux netfilter/iptables, Windows WFP, BSD pf
// Module: sigma-net-firewall
// USP: Rule evaluated at CPU register speed — no socket overhead, no kernel calls
// Rules stored as bitmask structs — O(1) match per rule

#ifndef SIGMA_NETFILTER_H
#define SIGMA_NETFILTER_H

#define SIGMA_NF_MAX_RULES  64
#define SIGMA_NF_ACCEPT     1
#define SIGMA_NF_DROP       0
#define SIGMA_NF_PROTO_TCP  6
#define SIGMA_NF_PROTO_UDP  17
#define SIGMA_NF_PROTO_ICMP 1

typedef struct SigmaFirewallRule {
    unsigned int  src_ip;       // 0 = wildcard
    unsigned int  dst_ip;       // 0 = wildcard
    unsigned short src_port;    // 0 = wildcard
    unsigned short dst_port;    // 0 = wildcard
    unsigned char  proto;       // 0 = wildcard
    unsigned char  verdict;     // ACCEPT or DROP
    unsigned long  hit_count;
    unsigned char  active;
} SigmaFirewallRule;

typedef struct SigmaFirewall {
    SigmaFirewallRule rules[SIGMA_NF_MAX_RULES];
    unsigned int      rule_count;
    unsigned char     default_verdict; // ACCEPT or DROP
    unsigned long     total_pkts;
    unsigned long     total_dropped;
} SigmaFirewall;

static inline void fw_init(SigmaFirewall* fw, unsigned char default_v) {
    fw->rule_count    = 0;
    fw->default_verdict = default_v;
    fw->total_pkts    = 0;
    fw->total_dropped = 0;
}

static inline int fw_add_rule(SigmaFirewall* fw,
                                unsigned int src_ip, unsigned int dst_ip,
                                unsigned short src_port, unsigned short dst_port,
                                unsigned char proto, unsigned char verdict) {
    if (fw->rule_count >= SIGMA_NF_MAX_RULES) return -1;
    SigmaFirewallRule* r = &fw->rules[fw->rule_count++];
    r->src_ip    = src_ip;    r->dst_ip  = dst_ip;
    r->src_port  = src_port;  r->dst_port = dst_port;
    r->proto     = proto;     r->verdict  = verdict;
    r->hit_count = 0;         r->active   = 1;
    return (int)(fw->rule_count - 1);
}

// Evaluate a packet against the firewall ruleset — O(n_rules)
static inline int fw_evaluate(SigmaFirewall* fw,
                                unsigned int src_ip, unsigned int dst_ip,
                                unsigned short src_port, unsigned short dst_port,
                                unsigned char proto) {
    fw->total_pkts++;
    for (unsigned int i = 0; i < fw->rule_count; i++) {
        SigmaFirewallRule* r = &fw->rules[i];
        if (!r->active) continue;
        if (r->src_ip   && r->src_ip   != src_ip)   continue;
        if (r->dst_ip   && r->dst_ip   != dst_ip)   continue;
        if (r->src_port && r->src_port != src_port)  continue;
        if (r->dst_port && r->dst_port != dst_port)  continue;
        if (r->proto    && r->proto    != proto)     continue;
        r->hit_count++;
        if (r->verdict == SIGMA_NF_DROP) fw->total_dropped++;
        return r->verdict;
    }
    if (fw->default_verdict == SIGMA_NF_DROP) fw->total_dropped++;
    return fw->default_verdict;
}

static inline void fw_flush(SigmaFirewall* fw) { fw->rule_count = 0; }

#endif /* SIGMA_NETFILTER_H */
