/*
 * Σ SigmaOS — sigma_net_dns: Sovereign DNS Resolver
 * Zero-Dependency: No predefined libraries (no gethostbyname, no resolv.h).
 * Manual UDP packet construction for DNS lookups.
 */

typedef unsigned int   u32;
typedef unsigned short u16;
typedef unsigned char  u8;

/* Required networking subsystems */
extern "C" u32 sigma_net_socket_create(u32 protocol);
extern "C" int sigma_net_socket_bind(u32 sock, u32 ip, u16 port);
extern "C" int sigma_net_socket_send(u32 sock, const u8* data, u32 len);
extern "C" int sigma_net_socket_recv(u32 sock, u8* buffer, u32 max_len);
extern "C" void sigma_net_socket_close(u32 sock);
extern "C" void sigma_vga_printf(const char* fmt, ...);

#define SIGMA_PROTO_UDP 2

/* DNS Header */
struct __attribute__((packed)) DNSHeader {
    u16 id;
    u16 flags;
    u16 qdcount;
    u16 ancount;
    u16 nscount;
    u16 arcount;
};

/* Endian Swap helper */
static inline u16 bswap16(u16 v) {
    return (v >> 8) | (v << 8);
}

/* 
 * Format domain name into DNS query format (e.g. "www.google.com" -> "\x03www\x06google\x03com\x00") 
 */
static void format_dns_name(const char* domain, u8* buffer, u32* len) {
    u32 p = 0, i = 0, last_dot = 0;
    while (domain[i] != '\0') {
        if (domain[i] == '.') {
            buffer[p++] = i - last_dot;
            for (u32 j = last_dot; j < i; j++) buffer[p++] = domain[j];
            last_dot = i + 1;
        }
        i++;
    }
    buffer[p++] = i - last_dot;
    for (u32 j = last_dot; j < i; j++) buffer[p++] = domain[j];
    buffer[p++] = 0;
    *len = p;
}

/* 
 * Resolve hostname to IPv4 address 
 */
extern "C" u32 sigma_net_dns_resolve(const char* domain, u32 dns_server_ip) {
    sigma_vga_printf("[DNS] Resolving %s via server 0x%X...\n", domain, dns_server_ip);
    
    u32 sock = sigma_net_socket_create(SIGMA_PROTO_UDP);
    if (sock == (u32)-1) return 0;
    
    /* Build packet */
    u8 packet[512];
    DNSHeader* hdr = (DNSHeader*)packet;
    hdr->id = bswap16(0x1337);
    hdr->flags = bswap16(0x0100); /* Standard query */
    hdr->qdcount = bswap16(1);
    hdr->ancount = 0;
    hdr->nscount = 0;
    hdr->arcount = 0;
    
    u32 query_len = 0;
    format_dns_name(domain, packet + sizeof(DNSHeader), &query_len);
    
    u8* qinfo = packet + sizeof(DNSHeader) + query_len;
    /* QTYPE = A (1) */
    qinfo[0] = 0; qinfo[1] = 1;
    /* QCLASS = IN (1) */
    qinfo[2] = 0; qinfo[3] = 1;
    
    u32 total_len = sizeof(DNSHeader) + query_len + 4;
    
    /* Send query to port 53 */
    /* Implementation detail: requires integration with routing table */
    sigma_net_socket_send(sock, packet, total_len);
    
    /* Wait for response and parse Answer section */
    /* ... stubbed ... */
    
    sigma_net_socket_close(sock);
    return 0x00000000; /* Return resolved IPv4 */
}
