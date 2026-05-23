/*
 * Σ SigmaOS Zenith — TCP Protocol Shard (Foundation)
 * Zero-Dependency: No libc.
 */

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;

struct __attribute__((packed)) sigma_tcp_header {
    u16 src_port;
    u16 dest_port;
    u32 seq_num;
    u32 ack_num;
    u16 data_offset_flags;
    u16 window_size;
    u16 checksum;
    u16 urgent_ptr;
};

// TCP Flags
#define TCP_FIN 0x01
#define TCP_SYN 0x02
#define TCP_RST 0x04
#define TCP_PSH 0x08
#define TCP_ACK 0x10
#define TCP_URG 0x20

extern "C" void sigma_vga_printf(const char* fmt, ...);

static inline u16 byteswap16(u16 val) {
    return (val >> 8) | (val << 8);
}

extern "C" void sigma_tcp_receive(u32 src_ip, u8* payload, u32 len) {
    if (len < sizeof(sigma_tcp_header)) return;
    
    struct sigma_tcp_header* tcp = (struct sigma_tcp_header*)payload;
    u16 sport = byteswap16(tcp->src_port);
    u16 dport = byteswap16(tcp->dest_port);
    
    u16 flags = byteswap16(tcp->data_offset_flags) & 0x01FF;
    
    sigma_vga_printf("TCP: Packet %x:%u -> port %u [", src_ip, sport, dport);
    
    if (flags & TCP_SYN) sigma_vga_printf("SYN ");
    if (flags & TCP_ACK) sigma_vga_printf("ACK ");
    if (flags & TCP_FIN) sigma_vga_printf("FIN ");
    if (flags & TCP_RST) sigma_vga_printf("RST ");
    if (flags & TCP_PSH) sigma_vga_printf("PSH ");
    
    sigma_vga_printf("]\n");
    
    // TCP State Machine progression would occur here.
    // e.g. Answering SYN with SYN-ACK.
}
