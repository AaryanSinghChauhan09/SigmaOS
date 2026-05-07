#include "core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN FIREWALL SHARD (v1.0)
 * =============================================================================
 * Principles: Shard-Level Packet Filtering & Zero-Trust Mesh.
 * =============================================================================
 */
#include "core/sigma_kernel_types.h"

typedef struct Rule {
    sigma_u32 src_ip;
    sigma_u16 port;
    sigma_bool allow;
} fw_rule_t;

#define MAX_RULES 64
static fw_rule_t rules[MAX_RULES];
static sigma_u32 rule_count = 0;

void fw_init() {
    /* Default Deny for unknown shards */
    rule_count = 0;
}

sigma_bool fw_audit_packet(sigma_u32 src_ip, sigma_u16 port) {
    for (sigma_u32 i = 0; i < rule_count; i++) {
        if (rules[i].src_ip == src_ip && rules[i].port == port) {
            return rules[i].allow;
        }
    }
    return SIGMA_FALSE; /* Default Deny */
}

void fw_add_rule(sigma_u32 ip, sigma_u16 port, sigma_bool allow) {
    if (rule_count < MAX_RULES) {
        rules[rule_count++] = (fw_rule_t){ip, port, allow};
    }
}
