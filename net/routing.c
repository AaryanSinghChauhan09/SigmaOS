/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: IPV4 ROUTING TABLE (FIB)
 * =============================================================================
 * Inspired by: Linux kernel net/ipv4/fib_trie.c
 *              FreeBSD sys/netinet/in_rmx.c
 * =============================================================================
 * Forwarding Information Base (FIB) determining network traffic paths.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define MAX_ROUTES 128

#define RT_FLAG_UP      0x0001
#define RT_FLAG_GATEWAY 0x0002

typedef struct {
    sigma_u32 dest_ip;
    sigma_u32 netmask;
    sigma_u32 gateway_ip;
    sigma_u32 metric;
    sigma_u16 flags;
    char      iface[16];
    sigma_bool active;
} sigma_route_t;

static sigma_route_t route_table[MAX_ROUTES];

void routing_init(void) {
    sigma_memset(route_table, 0, sizeof(route_table));
    sigma_printf("[route] IPv4 Forwarding Information Base initialized\n");
    
    /* Add loopback route by default */
    routing_add_route(0x7F000000, 0xFF000000, 0x00000000, 0, "lo");
}

int routing_add_route(sigma_u32 dest, sigma_u32 netmask, sigma_u32 gw, sigma_u32 metric, const char* iface) {
    for (sigma_u32 i = 0; i < MAX_ROUTES; i++) {
        if (!route_table[i].active) {
            route_table[i].dest_ip    = dest & netmask;
            route_table[i].netmask    = netmask;
            route_table[i].gateway_ip = gw;
            route_table[i].metric     = metric;
            route_table[i].flags      = RT_FLAG_UP | (gw ? RT_FLAG_GATEWAY : 0);
            route_table[i].active     = SIGMA_TRUE;
            
            sigma_u32 j = 0;
            while (j < 15 && iface[j]) { route_table[i].iface[j] = iface[j]; j++; }
            route_table[i].iface[j] = '\0';
            
            sigma_printf("[route] Added: Dest %u.%u.%u.%u / Mask %u.%u.%u.%u -> GW %u.%u.%u.%u (%s)\n",
                (dest >> 24) & 0xFF, (dest >> 16) & 0xFF, (dest >> 8) & 0xFF, dest & 0xFF,
                (netmask >> 24) & 0xFF, (netmask >> 16) & 0xFF, (netmask >> 8) & 0xFF, netmask & 0xFF,
                (gw >> 24) & 0xFF, (gw >> 16) & 0xFF, (gw >> 8) & 0xFF, gw & 0xFF,
                iface);
                
            return 0;
        }
    }
    sigma_printf("[route] ERR: Routing table full\n");
    return -1;
}

sigma_route_t* routing_lookup(sigma_u32 target_ip) {
    sigma_route_t* best_match = SIGMA_NULL;
    sigma_u32 longest_prefix = 0;
    
    for (sigma_u32 i = 0; i < MAX_ROUTES; i++) {
        if (route_table[i].active && (route_table[i].flags & RT_FLAG_UP)) {
            if ((target_ip & route_table[i].netmask) == route_table[i].dest_ip) {
                /* Longest Prefix Match (LPM) logic */
                if (route_table[i].netmask >= longest_prefix) {
                    if (route_table[i].netmask == longest_prefix && best_match) {
                        /* Tie-breaker: Metric */
                        if (route_table[i].metric < best_match->metric) {
                            best_match = &route_table[i];
                        }
                    } else {
                        best_match = &route_table[i];
                        longest_prefix = route_table[i].netmask;
                    }
                }
            }
        }
    }
    
    if (best_match) {
        sigma_printf("[route] Forwarding %u.%u.%u.%u via %s\n",
            (target_ip >> 24) & 0xFF, (target_ip >> 16) & 0xFF, 
            (target_ip >> 8) & 0xFF, target_ip & 0xFF, best_match->iface);
    } else {
        sigma_printf("[route] Network Unreachable: %u.%u.%u.%u\n",
            (target_ip >> 24) & 0xFF, (target_ip >> 16) & 0xFF, 
            (target_ip >> 8) & 0xFF, target_ip & 0xFF);
    }
    
    return best_match;
}
