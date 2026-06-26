// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_dns_sinkhole.h — DNS sinkholing for malware domain filtering
 *
 * Integrates with sigma-shield: DNS responses for blocked domains return
 * NXDOMAIN or a sinkhole IP (default: 0.0.0.0). Block lists are updated
 * via sigma-pkg (signed .sinkhole bundles) or manually.
 *
 * Usage:
 *   sigma_dns_sinkhole_load("/sigma/etc/dns/blocklist.txt");
 *   sigma_dns_sinkhole_add("malware.example.com");
 *
 *   // In DNS resolver path:
 *   if (sigma_dns_sinkhole_check(hostname)) {
 *       return SIGMA_DNS_NXDOMAIN;
 *   }
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

#define SIGMA_SINKHOLE_MAX_ENTRIES   100000  /* 100K domains (hash table)  */
#define SIGMA_SINKHOLE_HASH_SLOTS    131072  /* next power of 2            */

typedef enum {
    SIGMA_SINKHOLE_ACTION_NXDOMAIN  = 0,  /* return NXDOMAIN (default)     */
    SIGMA_SINKHOLE_ACTION_BLACKHOLE = 1,  /* return 0.0.0.0                */
    SIGMA_SINKHOLE_ACTION_REDIRECT  = 2,  /* return sinkhole_ip            */
    SIGMA_SINKHOLE_ACTION_LOG_ONLY  = 3,  /* allow but log the query       */
} sigma_sinkhole_action_t;

typedef struct {
    sigma_sinkhole_action_t default_action;
    sigma_u32               sinkhole_ip;     /* used when action=REDIRECT    */
    bool                    log_blocked;     /* write to audit ring          */
    sigma_u64               blocked_count;   /* total queries blocked        */
} sigma_sinkhole_config_t;

/* ── API ──────────────────────────────────────────────────────────────────── */

void sigma_dns_sinkhole_init(const sigma_sinkhole_config_t* cfg);

/* Load a blocklist file (one domain per line, # comments supported) */
int  sigma_dns_sinkhole_load(const char* blocklist_path);

/* Add/remove a single domain */
int  sigma_dns_sinkhole_add(const char* domain);
int  sigma_dns_sinkhole_remove(const char* domain);

/*
 * Check a DNS query — returns true if the domain should be blocked.
 * Also matches subdomains: "evil.malware.com" matches blocklist "malware.com".
 */
bool sigma_dns_sinkhole_check(const char* hostname);

/* Get statistics */
void sigma_dns_sinkhole_stats(sigma_u64* blocked, sigma_u64* total);

/* Reload all lists (called when blocklist packages update) */
int  sigma_dns_sinkhole_reload(void);
