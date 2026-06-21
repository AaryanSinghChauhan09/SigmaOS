/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: ARP — ADDRESS RESOLUTION PROTOCOL
 * =============================================================================
 * Inspired by: Linux kernel net/ipv4/arp.c
 *              RFC 826 (An Ethernet Address Resolution Protocol)
 *              FreeBSD if_arp.h / in_arp.c
 * =============================================================================
 * Maps IPv4 addresses to Ethernet MAC addresses within the local network.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define ARP_TABLE_SIZE    64
#define ARP_HW_ETHERNET  0x0001
#define ARP_PROTO_IPV4   0x0800
#define ARP_OP_REQUEST   1
#define ARP_OP_REPLY     2

/* ARP cache entry states (inspired by Linux NUD states) */
#define ARP_STATE_NONE       0
#define ARP_STATE_INCOMPLETE 1
#define ARP_STATE_REACHABLE  2
#define ARP_STATE_STALE      3
#define ARP_STATE_DELAY      4
#define ARP_STATE_FAILED     5

typedef struct {
    sigma_u8  mac[6];
} sigma_mac_t;

typedef struct {
    sigma_u32   ip_addr;
    sigma_mac_t mac_addr;
    sigma_u32   state;
    sigma_u32   ttl;         /* Time-to-live in seconds */
    sigma_u32   retries;
} arp_entry_t;

typedef struct {
    arp_entry_t entries[ARP_TABLE_SIZE];
    sigma_u32   count;
} arp_table_t;

/* ARP packet structure (RFC 826) */
typedef struct {
    sigma_u16 hw_type;
    sigma_u16 proto_type;
    sigma_u8  hw_len;
    sigma_u8  proto_len;
    sigma_u16 opcode;
    sigma_mac_t sender_mac;
    sigma_u32   sender_ip;
    sigma_mac_t target_mac;
    sigma_u32   target_ip;
} __attribute__((packed)) arp_packet_t;

static arp_table_t arp_cache;

static void format_ip(sigma_u32 ip, char* buf) {
    sigma_printf("%u.%u.%u.%u",
        (ip >> 24) & 0xFF, (ip >> 16) & 0xFF,
        (ip >> 8) & 0xFF, ip & 0xFF);
    (void)buf;
}

static void format_mac(const sigma_mac_t* mac) {
    sigma_printf("%02x:%02x:%02x:%02x:%02x:%02x",
        mac->mac[0], mac->mac[1], mac->mac[2],
        mac->mac[3], mac->mac[4], mac->mac[5]);
}

void arp_init(void) {
    sigma_memset(&arp_cache, 0, sizeof(arp_cache));
    sigma_printf("[arp] ARP cache initialized (capacity: %d entries)\n", ARP_TABLE_SIZE);
}

int arp_add_entry(sigma_u32 ip, const sigma_mac_t* mac, sigma_u32 state) {
    /* Check for existing entry — update if found */
    for (sigma_u32 i = 0; i < arp_cache.count; i++) {
        if (arp_cache.entries[i].ip_addr == ip) {
            sigma_memcpy(&arp_cache.entries[i].mac_addr, mac, sizeof(sigma_mac_t));
            arp_cache.entries[i].state = state;
            arp_cache.entries[i].ttl   = 300; /* 5 minutes default */
            sigma_printf("[arp] Updated entry: ");
            format_ip(ip, SIGMA_NULL);
            sigma_printf(" -> ");
            format_mac(mac);
            sigma_printf("\n");
            return 0;
        }
    }

    if (arp_cache.count >= ARP_TABLE_SIZE) {
        /* Evict oldest stale entry */
        for (sigma_u32 i = 0; i < arp_cache.count; i++) {
            if (arp_cache.entries[i].state == ARP_STATE_STALE ||
                arp_cache.entries[i].state == ARP_STATE_FAILED) {
                arp_cache.entries[i].ip_addr = ip;
                sigma_memcpy(&arp_cache.entries[i].mac_addr, mac, sizeof(sigma_mac_t));
                arp_cache.entries[i].state = state;
                arp_cache.entries[i].ttl   = 300;
                sigma_printf("[arp] Evicted stale entry, replaced with: ");
                format_ip(ip, SIGMA_NULL);
                sigma_printf("\n");
                return 0;
            }
        }
        sigma_printf("[arp] ERR: ARP table full, no stale entries to evict\n");
        return -1;
    }

    arp_entry_t* e = &arp_cache.entries[arp_cache.count++];
    e->ip_addr = ip;
    sigma_memcpy(&e->mac_addr, mac, sizeof(sigma_mac_t));
    e->state   = state;
    e->ttl     = 300;
    e->retries = 0;
    sigma_printf("[arp] New entry: ");
    format_ip(ip, SIGMA_NULL);
    sigma_printf(" -> ");
    format_mac(mac);
    sigma_printf(" [%s]\n",
        state == ARP_STATE_REACHABLE ? "REACHABLE" :
        state == ARP_STATE_STALE ? "STALE" : "INCOMPLETE");
    return 0;
}

const sigma_mac_t* arp_lookup(sigma_u32 ip) {
    for (sigma_u32 i = 0; i < arp_cache.count; i++) {
        if (arp_cache.entries[i].ip_addr == ip &&
            arp_cache.entries[i].state == ARP_STATE_REACHABLE) {
            return &arp_cache.entries[i].mac_addr;
        }
    }
    sigma_printf("[arp] Cache miss for ");
    format_ip(ip, SIGMA_NULL);
    sigma_printf(" — sending ARP request\n");
    extern void sigma_ethernet_send_arp_request(sigma_u32 target_ip);
    sigma_ethernet_send_arp_request(ip);
    return SIGMA_NULL;
}

void arp_process_packet(const arp_packet_t* pkt) {
    if (pkt->opcode == ARP_OP_REQUEST) {
        sigma_printf("[arp] ARP Request: Who has ");
        format_ip(pkt->target_ip, SIGMA_NULL);
        sigma_printf("? Tell ");
        format_ip(pkt->sender_ip, SIGMA_NULL);
        sigma_printf("\n");
        /* Cache the sender's mapping */
        arp_add_entry(pkt->sender_ip, &pkt->sender_mac, ARP_STATE_REACHABLE);
    } else if (pkt->opcode == ARP_OP_REPLY) {
        sigma_printf("[arp] ARP Reply: ");
        format_ip(pkt->sender_ip, SIGMA_NULL);
        sigma_printf(" is at ");
        format_mac(&pkt->sender_mac);
        sigma_printf("\n");
        arp_add_entry(pkt->sender_ip, &pkt->sender_mac, ARP_STATE_REACHABLE);
    }
}

void arp_age_entries(void) {
    for (sigma_u32 i = 0; i < arp_cache.count; i++) {
        if (arp_cache.entries[i].ttl > 0) {
            arp_cache.entries[i].ttl--;
        }
        if (arp_cache.entries[i].ttl == 0 &&
            arp_cache.entries[i].state == ARP_STATE_REACHABLE) {
            arp_cache.entries[i].state = ARP_STATE_STALE;
            sigma_printf("[arp] Entry for ");
            format_ip(arp_cache.entries[i].ip_addr, SIGMA_NULL);
            sigma_printf(" aged to STALE\n");
        }
    }
}

void arp_dump_cache(void) {
    sigma_printf("\n--- Σ ARP CACHE (%u entries) ---\n", arp_cache.count);
    for (sigma_u32 i = 0; i < arp_cache.count; i++) {
        sigma_printf("  ");
        format_ip(arp_cache.entries[i].ip_addr, SIGMA_NULL);
        sigma_printf("  ->  ");
        format_mac(&arp_cache.entries[i].mac_addr);
        sigma_printf("  [ttl=%u, state=%u]\n",
            arp_cache.entries[i].ttl, arp_cache.entries[i].state);
    }
    sigma_printf("--------------------------------\n");
}
