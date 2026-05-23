/*
 * Σ SigmaOS Zenith — UDP Protocol Shard
 * Zero-Dependency: No libc.
 */

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;

struct __attribute__((packed)) sigma_udp_header {
    u16 src_port;
    u16 dest_port;
    u16 length;
    u16 checksum;
};

extern "C" void sigma_vga_printf(const char* fmt, ...);

// Endian swap utility
static inline u16 byteswap16(u16 val) {
    return (val >> 8) | (val << 8);
}

extern "C" void sigma_udp_receive(u32 src_ip, u8* payload, u32 len) {
    if (len < sizeof(sigma_udp_header)) return;
    
    struct sigma_udp_header* udp = (struct sigma_udp_header*)payload;
    u16 sport = byteswap16(udp->src_port);
    u16 dport = byteswap16(udp->dest_port);
    
    sigma_vga_printf("UDP: Received %u bytes from %x:%u to port %u\n", len, src_ip, sport, dport);
    
    // Hand off payload to userspace socket layer or internal services
    // u8* data = payload + sizeof(sigma_udp_header);
    // u32 data_len = byteswap16(udp->length) - sizeof(sigma_udp_header);
}
