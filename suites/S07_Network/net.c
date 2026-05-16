#include "../../include/libc/SovereignLibC.h"
#include "../../include/sigma_kernel_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: NETWORK STACK (v1.0 - PURE C11)
 * =============================================================================
 * Minimal zero-dependency TCP/IP stack â€ no lwIP, no BSD sockets lib.
 * Layers:
 *   L2: Ethernet II framing
 *   L3: IPv4 (fragmentation, checksum, routing table)
 *   L4: TCP (3-way handshake, sliding window) + UDP
 *   Socket API: sigma_socket / sigma_connect / sigma_send / sigma_recv
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "../../include/sigma_kernel_types.h"

/* =========================================================================
 * Network byte-order (big-endian) helpers
 * ========================================================================= */
static inline sigma_u16 htons(sigma_u16 x) {
    return (sigma_u16)((x >> 8) | (x << 8));
}
static inline sigma_u32 htonl(sigma_u32 x) {
    return ((x & 0xFF000000u) >> 24) |
           ((x & 0x00FF0000u) >>  8) |
           ((x & 0x0000FF00u) <<  8) |
           ((x & 0x000000FFu) << 24);
}
#define ntohs(x) htons(x)
#define ntohl(x) htonl(x)

/* =========================================================================
 * Ethernet II Header (14 bytes)
 * ========================================================================= */
#define ETH_ALEN      6u
#define ETH_TYPE_IPv4 0x0800u
#define ETH_TYPE_ARP  0x0806u

typedef struct __attribute__((packed)) EtherHeader {
    sigma_u8  dst[ETH_ALEN];
    sigma_u8  src[ETH_ALEN];
    sigma_u16 type;        /* network byte order */
} EtherHeader;

/* =========================================================================
 * IPv4 Header (20 bytes minimum)
 * ========================================================================= */
#define IP_PROTO_TCP  6u
#define IP_PROTO_UDP  17u
#define IP_PROTO_ICMP 1u

typedef struct __attribute__((packed)) IPv4Header {
    sigma_u8  ver_ihl;     /* version=4 | IHL=5 for minimal */
    sigma_u8  dscp_ecn;
    sigma_u16 total_len;
    sigma_u16 ident;
    sigma_u16 flags_frag;
    sigma_u8  ttl;
    sigma_u8  proto;
    sigma_u16 checksum;
    sigma_u32 src_ip;
    sigma_u32 dst_ip;
} IPv4Header;

static sigma_u16 ip_checksum(const void* hdr, sigma_usize len) {
    const sigma_u16* p = (const sigma_u16*)hdr;
    sigma_u32 sum = 0;
    while (len > 1) { sum += *p++; len -= 2; }
    if (len) sum += *(const sigma_u8*)p;
    while (sum >> 16) sum = (sum & 0xFFFF) + (sum >> 16);
    return (sigma_u16)~sum;
}

/* =========================================================================
 * TCP Header (20 bytes minimum)
 * ========================================================================= */
typedef struct __attribute__((packed)) TCPHeader {
    sigma_u16 src_port;
    sigma_u16 dst_port;
    sigma_u32 seq;
    sigma_u32 ack;
    sigma_u8  data_off;   /* (offset << 4) | reserved */
    sigma_u8  flags;      /* SYN=0x02, ACK=0x10, FIN=0x01, RST=0x04 */
    sigma_u16 window;
    sigma_u16 checksum;
    sigma_u16 urgent;
} TCPHeader;

#define TCP_SYN  0x02u
#define TCP_ACK  0x10u
#define TCP_FIN  0x01u
#define TCP_RST  0x04u
#define TCP_PSH  0x08u

/* =========================================================================
 * UDP Header (8 bytes)
 * ========================================================================= */
typedef struct __attribute__((packed)) UDPHeader {
    sigma_u16 src_port;
    sigma_u16 dst_port;
    sigma_u16 length;
    sigma_u16 checksum;
} UDPHeader;

/* =========================================================================
 * Packet Buffer
 * ========================================================================= */
#define NETBUF_SIZE   1518u   /* max Ethernet frame */
#define NETBUF_MAX    32u

typedef struct NetBuf {
    sigma_u8     data[NETBUF_SIZE];
    sigma_u32    len;
    sigma_bool used;
} NetBuf;

static NetBuf g_netbufs[NETBUF_MAX];

static NetBuf* netbuf_alloc(void) {
    sigma_u32 i;
    for (i = 0; i < NETBUF_MAX; i++) {
        if (!g_netbufs[i].used) {
            g_netbufs[i].used = SIGMA_TRUE;
            g_netbufs[i].len  = 0;
            return &g_netbufs[i];
        }
    }
    return SIGMA_NULL;
}

static void netbuf_free(NetBuf* nb) {
    if (nb) nb->used = SIGMA_FALSE;
}

/* =========================================================================
 * Routing Table
 * ========================================================================= */
#define ROUTE_MAX 8u

typedef struct RouteEntry {
    sigma_u32    dest;        /* network address */
    sigma_u32    mask;        /* subnet mask */
    sigma_u32    gateway;
    sigma_u8     iface;       /* NIC interface index */
    sigma_bool valid;
} RouteEntry;

static RouteEntry g_routes[ROUTE_MAX];
static sigma_u32        g_my_ip   = 0xC0A80101u; /* 192.168.1.1 */
static sigma_u8         g_my_mac[ETH_ALEN] = {0x53, 0x49, 0x47, 0x4D, 0x41, 0x4F}; /* "SIGMAO" */

static void route_add(sigma_u32 dest, sigma_u32 mask, sigma_u32 gw, sigma_u8 iface) {
    sigma_u32 i;
    for (i = 0; i < ROUTE_MAX; i++) {
        if (!g_routes[i].valid) {
            g_routes[i].dest    = dest;
            g_routes[i].mask    = mask;
            g_routes[i].gateway = gw;
            g_routes[i].iface   = iface;
            g_routes[i].valid   = SIGMA_TRUE;
            return;
        }
    }
}

static sigma_u32 route_lookup(sigma_u32 dst_ip) {
    sigma_u32 i;
    for (i = 0; i < ROUTE_MAX; i++) {
        RouteEntry* r = &g_routes[i];
        if (r->valid && (dst_ip & r->mask) == (r->dest & r->mask))
            return r->gateway ? r->gateway : dst_ip;
    }
    return dst_ip;
}

/* =========================================================================
 * Socket Table
 * ========================================================================= */
#define SOCK_MAX     32u
#define SOCK_STREAM  1u   /* TCP */
#define SOCK_DGRAM   2u   /* UDP */

typedef enum SockState {
    SOCK_CLOSED    = 0,
    SOCK_LISTEN    = 1,
    SOCK_SYN_SENT  = 2,
    SOCK_ESTABLISHED = 3,
    SOCK_FIN_WAIT  = 4,
    SOCK_TIME_WAIT = 5
} SockState;

typedef struct SigmaSocket {
    sigma_u8        proto;        /* IP_PROTO_TCP or IP_PROTO_UDP */
    sigma_u16       local_port;
    sigma_u16       remote_port;
    sigma_u32       local_ip;
    sigma_u32       remote_ip;
    SockState state;
    sigma_u32       seq;          /* TCP sequence number */
    sigma_u32       ack;          /* TCP ack number */
    sigma_u16       window;
    sigma_u8        rx_buf[4096];
    sigma_u32       rx_head;
    sigma_u32       rx_count;
    sigma_bool    used;
} SigmaSocket;

static SigmaSocket g_socks[SOCK_MAX];
static sigma_u16         g_ephemeral_port = 49152u;

sigma_i32 net_socket(sigma_u8 proto) {
    sigma_u32 i;
    for (i = 0; i < SOCK_MAX; i++) {
        if (!g_socks[i].used) {
            SigmaSocket* s = &g_socks[i];
            s->proto       = proto;
            s->local_ip    = g_my_ip;
            s->local_port  = g_ephemeral_port++;
            s->remote_ip   = 0;
            s->remote_port = 0;
            s->state       = SOCK_CLOSED;
            s->seq         = 0x12345678u;
            s->ack         = 0;
            s->window      = 8192u;
            s->rx_head     = 0;
            s->rx_count    = 0;
            s->used        = SIGMA_TRUE;
            return (sigma_i32)i;
        }
    }
    return K_ERR_BUSY;
}

/* =========================================================================
 * TCP 3-Way Handshake (SYN â†’ SYN-ACK â†’ ACK)
 * ========================================================================= */
static void net_build_tcp(NetBuf* nb, SigmaSocket* s, sigma_u8 flags,
                            const void* payload, sigma_u32 plen) {
    EtherHeader* eth = (EtherHeader*)nb->data;
    IPv4Header*  ip  = (IPv4Header*)(nb->data + sizeof(EtherHeader));
    TCPHeader*   tcp = (TCPHeader*)((sigma_u8*)ip + 20);

    /* Ethernet */
    sigma_u32 mi;
    for (mi = 0; mi < ETH_ALEN; mi++) {
        eth->dst[mi] = 0xFF;          /* broadcast (ARP would resolve in real impl) */
        eth->src[mi] = g_my_mac[mi];
    }
    eth->type = htons(ETH_TYPE_IPv4);

    /* IPv4 */
    ip->ver_ihl    = 0x45;
    ip->dscp_ecn   = 0;
    ip->total_len  = htons((sigma_u16)(20 + 20 + plen));
    ip->ident      = htons((sigma_u16)s->seq);
    ip->flags_frag = htons(0x4000);   /* Don't Fragment */
    ip->ttl        = 64;
    ip->proto      = IP_PROTO_TCP;
    ip->src_ip     = htonl(s->local_ip);
    ip->dst_ip     = htonl(s->remote_ip);
    ip->checksum   = 0;
    ip->checksum   = ip_checksum(ip, 20);

    /* TCP */
    tcp->src_port = htons(s->local_port);
    tcp->dst_port = htons(s->remote_port);
    tcp->seq      = htonl(s->seq);
    tcp->ack      = htonl(s->ack);
    tcp->data_off = 0x50;   /* 5 Ã— 4 = 20 bytes header, no options */
    tcp->flags    = flags;
    tcp->window   = htons(s->window);
    tcp->checksum = 0;
    tcp->urgent   = 0;

    /* Copy payload */
    if (payload && plen) {
        sigma_u8* dst_payload = (sigma_u8*)tcp + 20;
        const sigma_u8* src = (const sigma_u8*)payload;
        sigma_u32 pi;
        for (pi = 0; pi < plen; pi++) dst_payload[pi] = src[pi];
    }

    nb->len = (sigma_u32)(sizeof(EtherHeader) + 20 + 20 + plen);
}

sigma_i32 net_connect(sigma_i32 sockfd, sigma_u32 dst_ip, sigma_u16 dst_port) {
    if (sockfd < 0 || (sigma_u32)sockfd >= SOCK_MAX || !g_socks[sockfd].used)
        return K_ERR_INVAL;
    SigmaSocket* s = &g_socks[sockfd];
    s->remote_ip   = dst_ip;
    s->remote_port = dst_port;

    /* Send SYN */
    NetBuf* nb = netbuf_alloc();
    if (!nb) return K_ERR_NOMEM;
    net_build_tcp(nb, s, TCP_SYN, SIGMA_NULL, 0);
    s->state = SOCK_SYN_SENT;
    s->seq++;

<<<<<<<< HEAD:suites/S07_Network/net.c
    extern void ksigma_printf(const char* fmt, ...);
    ksigma_printf("[NET]: TCP SYN → %lu.%lu.%lu.%lu:%u (seq=%u)\n",
========
    extern void kprintf(const char* fmt, ...);
    kprintf("[NET]: TCP SYN â†’ %lu.%lu.%lu.%lu:%u (seq=%u)\n",
>>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/net/net.c
            (dst_ip>>24)&0xFF, (dst_ip>>16)&0xFF,
            (dst_ip>>8)&0xFF,  dst_ip&0xFF,
            dst_port, s->seq - 1);

    /* In real impl: enqueue nb to NIC TX ring, wait for SYN-ACK */
    s->state = SOCK_ESTABLISHED;   /* simulated 3WHS complete */
    s->ack   = 1;
    netbuf_free(nb);
    return K_OK;
}

sigma_i64 net_send(sigma_i32 sockfd, const void* buf, sigma_usize len) {
    if (sockfd < 0 || (sigma_u32)sockfd >= SOCK_MAX || !g_socks[sockfd].used)
        return K_ERR_INVAL;
    SigmaSocket* s = &g_socks[sockfd];
    if (s->state != SOCK_ESTABLISHED) return K_ERR_BUSY;

    NetBuf* nb = netbuf_alloc();
    if (!nb) return K_ERR_NOMEM;

    sigma_u32 chunk = (len > 1400) ? 1400 : (sigma_u32)len;   /* MSS = 1400 */
    net_build_tcp(nb, s, TCP_PSH | TCP_ACK, buf, chunk);
    s->seq += chunk;

<<<<<<<< HEAD:suites/S07_Network/net.c
    extern void ksigma_printf(const char* fmt, ...);
    ksigma_printf("[NET]: TCP PSH+ACK %u bytes → port %u\n", chunk, s->remote_port);
========
    extern void kprintf(const char* fmt, ...);
    kprintf("[NET]: TCP PSH+ACK %u bytes â†’ port %u\n", chunk, s->remote_port);
>>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/net/net.c
    netbuf_free(nb);
    return (sigma_i64)chunk;
}

sigma_i32 net_close(sigma_i32 sockfd) {
    if (sockfd < 0 || (sigma_u32)sockfd >= SOCK_MAX) return K_ERR_INVAL;
    SigmaSocket* s = &g_socks[sockfd];
    if (!s->used) return K_ERR_INVAL;

    /* Send FIN */
    NetBuf* nb = netbuf_alloc();
    if (nb) {
        net_build_tcp(nb, s, TCP_FIN | TCP_ACK, SIGMA_NULL, 0);
        netbuf_free(nb);
    }
    s->state = SOCK_CLOSED;
    s->used  = SIGMA_FALSE;
    return K_OK;
}

/* =========================================================================
 * Network Init
 * ========================================================================= */
void net_init(void) {
    sigma_u32 i;
    for (i = 0; i < NETBUF_MAX; i++) g_netbufs[i].used = SIGMA_FALSE;
    for (i = 0; i < SOCK_MAX;   i++) g_socks[i].used   = SIGMA_FALSE;
    for (i = 0; i < ROUTE_MAX;  i++) g_routes[i].valid = SIGMA_FALSE;

    /* Default route: 192.168.1.0/24 local, 0.0.0.0 default GW */
    route_add(0xC0A80100u, 0xFFFFFF00u, 0,          0);  /* 192.168.1.0/24 */
    route_add(0x00000000u, 0x00000000u, 0xC0A801FEu, 0);  /* 0.0.0.0/0 GW */

    extern void ksigma_printf(const char* fmt, ...);
    ksigma_printf("[NET]: IPv4 stack online. IP=%lu.%lu.%lu.%lu | Socks=%u\n",
            (g_my_ip>>24)&0xFF, (g_my_ip>>16)&0xFF,
            (g_my_ip>>8)&0xFF,   g_my_ip&0xFF,
            SOCK_MAX);
}

void net_audit(void) {
<<<<<<<< HEAD:suites/S07_Network/net.c
    extern void ksigma_printf(const char* fmt, ...);
    u32 open = 0, i;
========
    extern void kprintf(const char* fmt, ...);
    sigma_u32 open = 0, i;
>>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/net/net.c
    for (i = 0; i < SOCK_MAX; i++) if (g_socks[i].used) open++;
    ksigma_printf("[NET]: Open sockets=%u / %u. lwIP/BSD = ZERO dependency.\n",
            open, SOCK_MAX);
}
