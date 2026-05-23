/*
 * Σ SigmaOS Zenith — ifconfig (Network Interface Config) Utility
 * Absorbs: net-tools ifconfig, busybox ifconfig, iproute2 ip
 * Zero-Dependency: No libc, no socket.h.
 */

typedef unsigned char  u8;
typedef unsigned int   u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" void sigma_vga_putchar(char c);

struct sigma_net_iface {
    char     name[16];
    u8       mac[6];
    u32      ipv4_addr;
    u32      netmask;
    u32      gateway;
    u32      rx_packets;
    u32      tx_packets;
    u32      rx_bytes;
    u32      tx_bytes;
    bool     is_up;
};

extern "C" u32 sigma_net_get_ifaces(struct sigma_net_iface* buf, u32 max);

static void print_ip(u32 ip) {
    sigma_vga_printf("%u.%u.%u.%u",
        (ip >> 24) & 0xFF,
        (ip >> 16) & 0xFF,
        (ip >> 8)  & 0xFF,
        ip & 0xFF);
}

extern "C" int sigma_ifconfig_main(int argc, char** argv) {
    struct sigma_net_iface ifaces[4];
    u32 count = sigma_net_get_ifaces(ifaces, 4);

    for (u32 i = 0; i < count; i++) {
        sigma_vga_printf("%s: flags=%s\n", ifaces[i].name,
            ifaces[i].is_up ? "<UP,BROADCAST,RUNNING>" : "<DOWN>");

        sigma_vga_printf("        inet ");
        print_ip(ifaces[i].ipv4_addr);
        sigma_vga_printf("  netmask ");
        print_ip(ifaces[i].netmask);
        sigma_vga_putchar('\n');

        sigma_vga_printf("        ether %02x:%02x:%02x:%02x:%02x:%02x\n",
            ifaces[i].mac[0], ifaces[i].mac[1], ifaces[i].mac[2],
            ifaces[i].mac[3], ifaces[i].mac[4], ifaces[i].mac[5]);

        sigma_vga_printf("        RX packets %u  bytes %u\n",
            ifaces[i].rx_packets, ifaces[i].rx_bytes);
        sigma_vga_printf("        TX packets %u  bytes %u\n\n",
            ifaces[i].tx_packets, ifaces[i].tx_bytes);
    }
    return 0;
}
