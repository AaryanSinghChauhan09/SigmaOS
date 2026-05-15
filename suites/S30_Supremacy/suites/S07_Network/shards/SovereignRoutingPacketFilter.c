// SigmaOS Sovereign Routing Table & Packet Filter Manager
// Absorbs Linux Netfilter/iptables + BSD pf + Windows WFP
// Modular C11 shard — integrated into custom TCP/IP stack

#include "../../../../../include/core/sigma_types.h"


#define SIGMA_MAX_ROUTES   1024
#define SIGMA_MAX_PF_RULES 2048

typedef struct {
    uint32_t destination;    // Network prefix
    uint32_t netmask;
    uint32_t gateway;
    uint32_t interface_idx;
    uint32_t metric;         // Route priority (lower = preferred)
} SigmaRoute;

typedef struct {
    uint32_t src_ip;
    uint32_t dst_ip;
    uint16_t src_port;
    uint16_t dst_port;
    uint8_t  protocol;       // 6=TCP, 17=UDP, 1=ICMP
    bool     allow;          // true = ACCEPT, false = DROP
    bool     log_match;      // Feed matches to audittrail shard
} SigmaPFRule;

static SigmaRoute   route_table[SIGMA_MAX_ROUTES];
static SigmaPFRule  pf_rules[SIGMA_MAX_PF_RULES];
static uint32_t     route_count = 0;
static uint32_t     pf_rule_count = 0;

// Add a new route to the routing table
void net_route_add(SigmaRoute* route);

// Lookup best-match next-hop for a destination IP (longest prefix match)
uint32_t net_route_lookup(uint32_t dst_ip);

// Add a new packet filter rule (prepended — highest priority first)
void net_pf_add_rule(SigmaPFRule* rule);

// Evaluate an incoming/outgoing packet against the rule chain
bool net_pf_evaluate(uint32_t src, uint32_t dst, uint16_t sport, uint16_t dport, uint8_t proto);



