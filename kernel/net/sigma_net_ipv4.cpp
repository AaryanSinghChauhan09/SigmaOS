/*
 * Σ SigmaOS Zenith — IPv4 Protocol Shard
 * Zero-Dependency: No libc.
 */

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;

struct __attribute__((packed)) sigma_ipv4_header {
    u8  ihl_version;
    u8  tos;
    u16 total_length;
    u16 identification;
    u16 flags_fragment_offset;
    u8  ttl;
    u8  protocol;
    u16 header_checksum;
    u32 source_ip;
    u32 dest_ip;
};

#define IPV4_PROTO_ICMP 1
#define IPV4_PROTO_TCP  6
#define IPV4_PROTO_UDP  17

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" void sigma_icmp_receive(u32 src_ip, u8* payload, u32 len);

/* RFC 1071 Checksum */
static u16 sovereign_checksum(void* data, u32 len) {
    u32 sum = 0;
    u16* ptr = (u16*)data;
    while (len > 1) {
        sum += *ptr++;
        len -= 2;
    }
    if (len > 0) sum += *(u8*)ptr;
    while (sum >> 16) sum = (sum & 0xFFFF) + (sum >> 16);
    return (u16)(~sum);
}

extern "C" void sigma_ipv4_receive(u8* packet, u32 len) {
    if (len < sizeof(sigma_ipv4_header)) return;
    
    struct sigma_ipv4_header* ip = (struct sigma_ipv4_header*)packet;
    
    // Check version (4)
    if ((ip->ihl_version >> 4) != 4) return;
    
    u32 header_len = (ip->ihl_version & 0x0F) * 4;
    if (header_len < 20 || header_len > len) return;

    // Verify checksum
    u16 orig_csum = ip->header_checksum;
    ip->header_checksum = 0;
    u16 calc_csum = sovereign_checksum(packet, header_len);
    if (calc_csum != orig_csum) {
        sigma_vga_printf("IPv4: Bad checksum!\n");
        return;
    }

    u8* payload = packet + header_len;
    u32 payload_len = len - header_len;

    if (ip->protocol == IPV4_PROTO_ICMP) {
        sigma_icmp_receive(ip->source_ip, payload, payload_len);
    } else {
        sigma_vga_printf("IPv4: Unsupported protocol %u\n", ip->protocol);
    }
}
