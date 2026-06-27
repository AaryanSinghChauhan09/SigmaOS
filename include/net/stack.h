/* SPDX-License-Identifier: GPL-2.0-or-later */
/**
 * stack.h — SigmaOS microkernel network stack interface
 *
 * The network stack runs as a set of cooperating user-space shards:
 *   sigma-netd   — IP routing, interface management, socket API
 *   sigma-tcpd   — TCP connection state machine (per-socket coroutines)
 *   sigma-dnsd   — DNS resolver with DoH fallback
 *   sigma-firewalld — nftables-inspired packet filter
 *
 * Drivers call on_packet_rx() via IPC; user sockets call socket syscall stubs.
 *
 * Inspired by: MINIX 3 inet server, lwIP, Linux net/ipv4/
 */

#pragma once
#include <stdint.h>
#include <stddef.h>

/* ── Address families ────────────────────────────────────────────────────── */

#define AF_UNSPEC  0
#define AF_INET    2
#define AF_INET6   10

/* ── Socket types ────────────────────────────────────────────────────────── */

#define SOCK_STREAM 1   /* TCP */
#define SOCK_DGRAM  2   /* UDP */
#define SOCK_RAW    3

/* ── IPv4 address (network byte order) ──────────────────────────────────── */

typedef uint32_t in_addr_t;

typedef struct sigma_sockaddr_in {
    uint16_t    family;
    uint16_t    port;    /* big-endian */
    in_addr_t   addr;
    uint8_t     zero[8];
} sigma_sockaddr_in_t;

typedef struct sigma_sockaddr {
    uint16_t    family;
    char        data[14];
} sigma_sockaddr_t;

/* ── Socket descriptor ───────────────────────────────────────────────────── */

typedef struct sigma_socket {
    int         fd;
    int         domain;
    int         type;
    int         protocol;
    in_addr_t   local_addr;
    uint16_t    local_port;
    in_addr_t   remote_addr;
    uint16_t    remote_port;
    uint32_t    state;       /* TCP state machine */
    uint32_t    snd_buf_size;
    uint32_t    rcv_buf_size;
    uint32_t    shard_id;    /* owning shard */
} sigma_socket_t;

/* ── TCP state machine ────────────────────────────────────────────────────── */

typedef enum sigma_tcp_state {
    TCP_CLOSED      = 0,
    TCP_LISTEN      = 1,
    TCP_SYN_SENT    = 2,
    TCP_SYN_RECV    = 3,
    TCP_ESTABLISHED = 4,
    TCP_FIN_WAIT1   = 5,
    TCP_FIN_WAIT2   = 6,
    TCP_CLOSE_WAIT  = 7,
    TCP_CLOSING     = 8,
    TCP_LAST_ACK    = 9,
    TCP_TIME_WAIT   = 10,
} sigma_tcp_state_t;

/* ── Packet buffer (matches driver IPC format) ───────────────────────────── */

typedef struct sigma_pkt_buf {
    uint8_t    *data;
    size_t      len;
    size_t      capacity;
    uint32_t    ifindex;
    uint32_t    flags;
#define PKT_FLAG_CSUM_HW   (1u << 0)   /* hardware checksum offloaded */
#define PKT_FLAG_GSO       (1u << 1)   /* generic segmentation offload */
#define PKT_FLAG_TSO       (1u << 2)   /* TCP segmentation offload */
    uint32_t    rx_tstamp_ns;
} sigma_pkt_buf_t;

/* ── Network interface ───────────────────────────────────────────────────── */

typedef struct sigma_netif {
    char        name[16];       /* "eth0", "wlan0", "lo" */
    uint32_t    shard_id;       /* driver shard */
    uint8_t     mac[6];
    in_addr_t   ipv4_addr;
    in_addr_t   ipv4_mask;
    in_addr_t   ipv4_bcast;
    uint32_t    mtu;
    uint32_t    flags;
#define NETIF_UP     (1u << 0)
#define NETIF_BCAST  (1u << 1)
#define NETIF_LOOPBK (1u << 2)
#define NETIF_PROMISC(1u << 3)
    uint64_t    rx_packets;
    uint64_t    tx_packets;
    uint64_t    rx_bytes;
    uint64_t    tx_bytes;
    uint64_t    rx_errors;
    uint64_t    tx_errors;
} sigma_netif_t;

/* ── Route table entry ───────────────────────────────────────────────────── */

typedef struct sigma_route {
    in_addr_t   dst;
    in_addr_t   mask;
    in_addr_t   gateway;
    uint32_t    ifindex;
    uint32_t    metric;
    uint32_t    flags;
#define ROUTE_FLAG_UP      (1u << 0)
#define ROUTE_FLAG_GATEWAY (1u << 1)
#define ROUTE_FLAG_HOST    (1u << 2)
} sigma_route_t;

/* ── Socket API (maps to sigma-netd IPC) ────────────────────────────────── */

int  sigma_socket    (int domain, int type, int proto);
int  sigma_bind      (int fd, const sigma_sockaddr_t *addr, uint32_t addrlen);
int  sigma_listen    (int fd, int backlog);
int  sigma_accept    (int fd, sigma_sockaddr_t *addr, uint32_t *addrlen);
int  sigma_connect   (int fd, const sigma_sockaddr_t *addr, uint32_t addrlen);
ssize_t sigma_send   (int fd, const void *buf, size_t len, int flags);
ssize_t sigma_recv   (int fd, void *buf, size_t len, int flags);
ssize_t sigma_sendto (int fd, const void *buf, size_t len, int flags,
                      const sigma_sockaddr_t *dst, uint32_t dstlen);
ssize_t sigma_recvfrom(int fd, void *buf, size_t len, int flags,
                       sigma_sockaddr_t *src, uint32_t *srclen);
int  sigma_close_sock(int fd);
int  sigma_setsockopt(int fd, int level, int optname,
                      const void *optval, uint32_t optlen);
int  sigma_getsockopt(int fd, int level, int optname,
                      void *optval, uint32_t *optlen);
int  sigma_getifaddrs(sigma_netif_t *out, size_t max, size_t *count);
int  sigma_route_add (const sigma_route_t *route);
int  sigma_route_del (const sigma_route_t *route);
int  sigma_route_list(sigma_route_t *out, size_t max, size_t *count);
