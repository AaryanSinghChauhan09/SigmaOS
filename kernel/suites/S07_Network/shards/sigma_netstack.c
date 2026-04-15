/*
 * =========================================================================
 * Σ SIGMAOS kernel/suites/S07_Network/shards/sigma_netstack.c
 * =========================================================================
 */

#include "sigma_netstack.h"
#include "../../../../include/sigma_libc.h"

static sigma_netif_t  s_ifaces[SIGMA_NET_MAX_IFS];
static net_u32        s_if_count   = 0;

static sigma_route_t  s_routes[SIGMA_ROUTE_MAX];
static net_u32        s_route_count = 0;

static sigma_fw_rule_t s_fw_rules[SIGMA_FW_MAX_RULES];
static net_u32         s_fw_count  = 0;

/* ── Init ───────────────────────────────────────────────────────────────── */
void sigma_net_init(void) {
    sigma_memset(s_ifaces, 0, sizeof(s_ifaces));
    sigma_memset(s_routes, 0, sizeof(s_routes));
    sigma_memset(s_fw_rules, 0, sizeof(s_fw_rules));

    /* Register loopback automatically */
    net_u8 lo_mac[6] = {0};
    sigma_net_if_register("lo", lo_mac, 0x7F000001, 0xFF000000);
    sigma_net_if_up(0);
    /* Add default loopback route */
    sigma_net_route_add(0x7F000000, 0xFF000000, 0, 0, 0);

    sigma_printf("Σ [NET] Stack initialized. Loopback: 127.0.0.1/8\n");
}

/* ── Interface management ───────────────────────────────────────────────── */
net_i32 sigma_net_if_register(const char *name, const net_u8 *mac,
                               net_u32 ip4, net_u32 netmask) {
    if (s_if_count >= SIGMA_NET_MAX_IFS) return NET_ERR;
    sigma_netif_t *nif = &s_ifaces[s_if_count];
    sigma_strncpy(nif->name, name, SIGMA_IF_NAME_LEN - 1);
    if (mac) sigma_memcpy(nif->mac, mac, 6);
    nif->ip4     = ip4;
    nif->netmask = netmask;
    nif->flags   = 0;
    return (net_i32)s_if_count++;
}

void sigma_net_if_up(net_u32 idx) {
    if (idx >= s_if_count) return;
    s_ifaces[idx].flags |= IFF_UP | IFF_RUNNING;
    sigma_printf("Σ [NET] IF UP: %s ip=%u\n",
                 s_ifaces[idx].name, s_ifaces[idx].ip4);
}

void sigma_net_if_down(net_u32 idx) {
    if (idx >= s_if_count) return;
    s_ifaces[idx].flags &= ~(IFF_UP | IFF_RUNNING);
    sigma_printf("Σ [NET] IF DOWN: %s\n", s_ifaces[idx].name);
}

void sigma_net_if_status(void) {
    sigma_printf("\nΣ NET INTERFACES\n");
    for (net_u32 i = 0; i < s_if_count; i++) {
        sigma_netif_t *n = &s_ifaces[i];
        sigma_printf("  %-8s ip=0x%08x flags=0x%x rx=%llu tx=%llu\n",
                     n->name, n->ip4, n->flags,
                     (unsigned long long)n->rx_bytes,
                     (unsigned long long)n->tx_bytes);
    }
}

/* ── Routing ────────────────────────────────────────────────────────────── */
net_i32 sigma_net_route_add(net_u32 dest, net_u32 mask,
                             net_u32 gw, net_u32 ifindex, net_u32 metric) {
    if (s_route_count >= SIGMA_ROUTE_MAX) return NET_ERR;
    sigma_route_t *r = &s_routes[s_route_count++];
    r->dest    = dest;
    r->mask    = mask;
    r->gateway = gw;
    r->ifindex = ifindex;
    r->metric  = metric;
    return NET_OK;
}

net_i32 sigma_net_route_lookup(net_u32 dst_ip) {
    net_u32 best_mask   = 0;
    net_i32 best_ifidx  = NET_ERR;
    for (net_u32 i = 0; i < s_route_count; i++) {
        if ((dst_ip & s_routes[i].mask) == s_routes[i].dest) {
            if (s_routes[i].mask >= best_mask) {
                best_mask  = s_routes[i].mask;
                best_ifidx = (net_i32)s_routes[i].ifindex;
            }
        }
    }
    return best_ifidx;
}

/* ── Firewall ───────────────────────────────────────────────────────────── */
net_i32 sigma_fw_rule_add(sigma_fw_rule_t *rule) {
    if (s_fw_count >= SIGMA_FW_MAX_RULES || !rule) return NET_ERR;
    s_fw_rules[s_fw_count++] = *rule;
    return NET_OK;
}

sigma_fw_action_t sigma_fw_match(sigma_skb_t *skb) {
    /* Default: ACCEPT (stateless — stateful conntrack is S08_Security) */
    (void)skb;
    return FW_ACCEPT;
}

/* ── TX/RX ──────────────────────────────────────────────────────────────── */
net_i32 sigma_net_tx(net_u32 ifindex, sigma_skb_t *skb) {
    if (ifindex >= s_if_count || !skb) return NET_ERR;
    sigma_fw_action_t action = sigma_fw_match(skb);
    if (action == FW_DROP) return NET_ERR;
    s_ifaces[ifindex].tx_bytes   += skb->len;
    s_ifaces[ifindex].tx_packets += 1;
    return NET_OK;
}

net_i32 sigma_net_rx(net_u32 ifindex, sigma_skb_t *skb) {
    if (ifindex >= s_if_count || !skb) return NET_ERR;
    sigma_fw_action_t action = sigma_fw_match(skb);
    if (action == FW_DROP || action == FW_REJECT) return NET_ERR;
    s_ifaces[ifindex].rx_bytes   += skb->len;
    s_ifaces[ifindex].rx_packets += 1;
    return NET_OK;
}

void sigma_net_stats(void) {
    sigma_printf("\nΣ NET STATS — routes=%u fw_rules=%u\n",
                 s_route_count, s_fw_count);
    sigma_net_if_status();
}
