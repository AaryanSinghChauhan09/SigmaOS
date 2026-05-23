/*
 * Σ SigmaOS Zenith — ICMP Protocol Shard
 * Zero-Dependency: No libc.
 */

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;

struct __attribute__((packed)) sigma_icmp_header {
    u8  type;
    u8  code;
    u16 checksum;
    u16 identifier;
    u16 sequence;
};

#define ICMP_TYPE_ECHO_REPLY   0
#define ICMP_TYPE_ECHO_REQUEST 8

extern "C" void sigma_vga_printf(const char* fmt, ...);

extern "C" void sigma_icmp_receive(u32 src_ip, u8* payload, u32 len) {
    if (len < sizeof(sigma_icmp_header)) return;
    
    struct sigma_icmp_header* icmp = (struct sigma_icmp_header*)payload;
    
    if (icmp->type == ICMP_TYPE_ECHO_REQUEST) {
        sigma_vga_printf("ICMP: Echo Request from %x. Responding...\n", src_ip);
        // Reply logic would reverse IPs and set type to ECHO_REPLY
    } else if (icmp->type == ICMP_TYPE_ECHO_REPLY) {
        sigma_vga_printf("ICMP: Echo Reply from %x\n", src_ip);
    } else {
        sigma_vga_printf("ICMP: Unknown type %u\n", icmp->type);
    }
}
