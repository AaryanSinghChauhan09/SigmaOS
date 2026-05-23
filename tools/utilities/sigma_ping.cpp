/*
 * Σ SigmaOS Zenith — ping Utility
 * Absorbs: iputils ping, busybox ping
 * Zero-Dependency: No libc, no socket.h.
 */

typedef unsigned char  u8;
typedef unsigned short u16;
typedef unsigned int   u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);

extern "C" void sigma_icmp_send_echo(u32 dest_ip, u16 id, u16 seq);
extern "C" int  sigma_icmp_wait_reply(u32 dest_ip, u16 id, u16 seq, u32 timeout_ms);

static u32 parse_ipv4(const char* s) {
    u32 octets[4] = {0, 0, 0, 0};
    int idx = 0;
    while (*s && idx < 4) {
        if (*s == '.') { idx++; s++; continue; }
        octets[idx] = octets[idx] * 10 + (*s - '0');
        s++;
    }
    return (octets[0] << 24) | (octets[1] << 16) | (octets[2] << 8) | octets[3];
}

extern "C" int sigma_ping_main(int argc, char** argv) {
    if (argc < 2) {
        sigma_vga_printf("Usage: ping <ip_address>\n");
        return 1;
    }

    u32 dest = parse_ipv4(argv[1]);
    u16 id = 0x1234;

    sigma_vga_printf("PING %u.%u.%u.%u: 64 bytes of data\n",
        (dest >> 24) & 0xFF, (dest >> 16) & 0xFF,
        (dest >> 8) & 0xFF, dest & 0xFF);

    for (u16 seq = 1; seq <= 4; seq++) {
        sigma_icmp_send_echo(dest, id, seq);

        int rtt = sigma_icmp_wait_reply(dest, id, seq, 1000);
        if (rtt >= 0) {
            sigma_vga_printf("64 bytes from %u.%u.%u.%u: icmp_seq=%u time=%u ms\n",
                (dest >> 24) & 0xFF, (dest >> 16) & 0xFF,
                (dest >> 8) & 0xFF, dest & 0xFF,
                seq, (u32)rtt);
        } else {
            sigma_vga_printf("Request timeout for icmp_seq %u\n", seq);
        }
    }
    return 0;
}
