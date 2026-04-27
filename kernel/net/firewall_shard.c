/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: SOVEREIGN FIREWALL SHARD (v1.0)
 * =============================================================================
 * Principles: Shard-Level Packet Filtering & Zero-Trust Mesh.
 * =============================================================================
 */
#include "../../include/sigma_kernel_types.h"

typedef struct Rule {
    u32 src_ip;
    u16 port;
    bool_t allow;
} fw_rule_t;

#define MAX_RULES 64
static fw_rule_t rules[MAX_RULES];
static u32 rule_count = 0;

void fw_init() {
    /* Default Deny for unknown shards */
    rule_count = 0;
}

bool_t fw_audit_packet(u32 src_ip, u16 port) {
    for (u32 i = 0; i < rule_count; i++) {
        if (rules[i].src_ip == src_ip && rules[i].port == port) {
            return rules[i].allow;
        }
    }
    return FALSE; /* Default Deny */
}

void fw_add_rule(u32 ip, u16 port, bool_t allow) {
    if (rule_count < MAX_RULES) {
        rules[rule_count++] = (fw_rule_t){ip, port, allow};
    }
}
