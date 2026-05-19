#include "../sigma_libc.h"

/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: DOMAIN NAME SYSTEM (DNS) RESOLVER (v1.0)
 * =============================================================================
 * Lightweight name resolution mapping domain names to IPv4 destinations.
 * =============================================================================
 */

typedef struct {
    const char* domain;
    sigma_u32 ip;
} dns_entry_t;

static dns_entry_t dns_table[] = {
    {"localhost",      0x7F000001}, // 127.0.0.1
    {"sigma.nexus",    0xC0A80101}, // 192.168.1.1
    {"google.com",     0x08080808}, // 8.8.8.8
    {"attestation.sh", 0xC0A8010F}  // 192.168.1.15
};

#define DNS_TABLE_SIZE (sizeof(dns_table) / sizeof(dns_entry_t))

sigma_u32 dns_resolve(const char* name) {
    if (!name) return 0;
    
    sigma_printf("[dns] Querying host: %s...\n", name);
    for (sigma_u32 i = 0; i < DNS_TABLE_SIZE; i++) {
        if (sigma_strcmp(dns_table[i].domain, name) == 0) {
            sigma_u32 ip = dns_table[i].ip;
            sigma_printf("[dns] Resolved %s -> %u.%u.%u.%u\n",
                         name,
                         (ip >> 24) & 0xFF,
                         (ip >> 16) & 0xFF,
                         (ip >> 8) & 0xFF,
                         ip & 0xFF);
            return ip;
        }
    }
    
    sigma_printf("[dns] ERR: Hostname %s resolution timed out.\n", name);
    return 0; // Not resolved
}
