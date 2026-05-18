// SigmaOS Sovereign DNS Resolver & DHCP Client/Server
// Absorbs systemd-resolved (Linux) + Windows DNS Client + macOS mDNSResponder
// Modular zero-dependency C11 network shard

#include "core/sigma_types.h"


#define SIGMA_DNS_CACHE_SIZE    512
#define SIGMA_DHCP_LEASE_MAX    256

typedef struct {
    char     hostname[253];
    uint32_t ip_v4;
    uint8_t  ip_v6[16];
    uint32_t ttl_seconds;
} SigmaDNSRecord;

typedef struct {
    uint32_t client_mac[6];
    uint32_t assigned_ip;
    uint32_t lease_duration_sec;
    bool     is_active;
} SigmaDHCPLease;

static SigmaDNSRecord  dns_cache[SIGMA_DNS_CACHE_SIZE];
static SigmaDHCPLease  dhcp_leases[SIGMA_DHCP_LEASE_MAX];

// Resolve a hostname using the kernel DNS cache, then recursive lookup
uint32_t dns_resolve_v4(const char* hostname);

// Support mDNS for local device discovery (Bonjour/Avahi equivalent)
void dns_init_mdns_responder(void);

// DHCP Client: broadcast DISCOVER and negotiate a lease
bool dhcp_client_request(const char* interface_id, uint32_t* out_ip);

// DHCP Server: respond to client DISCOVER on local subnets
void dhcp_server_start(uint32_t subnet, uint32_t netmask, uint32_t ip_pool_start);

// Clear stale DNS cache entries
void dns_flush_cache(void);



