/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S07_Network/shards/sigma_netstack.h
 * =========================================================================
 * Sovereign Network Stack — gap-closes:
 *   Linux  : sk_buff, netfilter, routing table, tc/qdisc
 *   BSD    : mbuf, pf firewall, IPFW
 *   macOS  : Network.framework, XPC transport
 *   Windows: NDIS, Winsock, Windows Filtering Platform
 * =========================================================================
 */

#ifndef SIGMA_NETSTACK_H
#define SIGMA_NETSTACK_H

#include "suites/S01_Genesis/shards/SovereignCommon.h"

typedef sigma_u32   net_u32;
typedef sigma_u16   net_u16;
typedef sigma_u8    net_u8;
typedef sigma_sz_t  net_u64;
typedef sigma_err_t net_i32;
typedef sigma_bool  net_bool;

#define NET_TRUE  SIGMA_TRUE
#define NET_FALSE SIGMA_FALSE
#define NET_NULL  SIGMA_NULL
#define NET_OK    SIGMA_OK
#define NET_ERR   -1

/* ── Network frame (sovereign sk_buff equivalent) ───────────────────────── */
#define SIGMA_NET_MTU      1514
#define SIGMA_NET_HDR_MAX   256

typedef struct {
    sigma_obj_t base;           /* Inheritance from SovereignObject */
    net_u8   data[SIGMA_NET_MTU];
    net_u32  len;
    net_u32  head_room;             /* bytes reserved for L2/L3 headers */
    net_u32  protocol;              /* EtherType: 0x0800=IP, 0x0806=ARP */
    net_u32  ifindex;               /* interface index                   */
    net_u64  rx_timestamp;          /* receive TSC                       */
} sigma_skb_t;

/* ── Interface flags ─────────────────────────────────────────────────────── */
#define IFF_UP        (1 << 0)
#define IFF_RUNNING   (1 << 1)
#define IFF_PROMISC   (1 << 2)
#define IFF_MULTICAST (1 << 3)
#define IFF_LOOPBACK  (1 << 4)

/* ── Network interface descriptor (Object-Oriented) ─────────────────────── */
#define SIGMA_IF_NAME_LEN  16
#define SIGMA_NET_MAX_IFS  16

typedef struct {
    sigma_obj_t base;                     /* Inheritance from SovereignObject */
    net_u8     mac[6];                    /* hardware MAC address       */
    net_u32    ip4;                       /* IPv4 address (host order)  */
    net_u32    netmask;
    net_u32    flags;
    net_u64    rx_bytes;
    net_u64    tx_bytes;
    net_u64    rx_packets;
    net_u64    tx_packets;
    net_u64    rx_errors;
} sigma_netif_t;
 domestic

/* ── Firewall rule (nftables/pf/WFP parity) ─────────────────────────────── */
typedef enum {
    FW_ACCEPT = 0,
    FW_DROP,
    FW_REJECT,
    FW_LOG
} sigma_fw_action_t;

typedef struct {
    net_u32          src_ip;
    net_u32          dst_ip;
    net_u16          src_port;
    net_u16          dst_port;
    net_u8           proto;       /* 6=TCP, 17=UDP, 1=ICMP */
    sigma_fw_action_t action;
    net_bool          inbound;
} sigma_fw_rule_t;

#define SIGMA_FW_MAX_RULES 512

/* ── Routing table entry ────────────────────────────────────────────────── */
typedef struct {
    net_u32  dest;
    net_u32  mask;
    net_u32  gateway;
    net_u32  ifindex;
    net_u32  metric;
} sigma_route_t;

#define SIGMA_ROUTE_MAX 256

/* ── Public API ─────────────────────────────────────────────────────────── */
void     sigma_net_init(void);
net_i32  sigma_net_if_register(const char *name, const net_u8 *mac,
                                net_u32 ip4, net_u32 netmask);
void     sigma_net_if_up(net_u32 ifindex);
void     sigma_net_if_down(net_u32 ifindex);
void     sigma_net_if_status(void);

net_i32  sigma_net_route_add(net_u32 dest, net_u32 mask,
                              net_u32 gw, net_u32 ifindex, net_u32 metric);
net_i32  sigma_net_route_lookup(net_u32 dst_ip);

net_i32  sigma_fw_rule_add(sigma_fw_rule_t *rule);
sigma_fw_action_t sigma_fw_match(sigma_skb_t *skb);

net_i32  sigma_net_tx(net_u32 ifindex, sigma_skb_t *skb);
net_i32  sigma_net_rx(net_u32 ifindex, sigma_skb_t *skb);

void     sigma_net_stats(void);

#endif /* SIGMA_NETSTACK_H */
