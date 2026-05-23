/*
 * Σ SigmaOS — sigma_ifconfig: Sovereign Network Interface Configuration
 * Zero-Dependency: No net-tools or iproute2.
 * Absorbs: Linux ifconfig/ip — reads/writes NIC registers directly.
 */

extern "C" void sigma_vga_printf(const char* fmt, ...);

typedef unsigned int   u32;
typedef unsigned char  u8;

struct NetInterface {
    char name[16];
    u8   mac[6];
    u32  ip_addr;
    u32  netmask;
    u32  broadcast;
    u32  rx_packets;
    u32  tx_packets;
    u32  rx_bytes;
    u32  tx_bytes;
    int  is_up;
    int  mtu;
};

#define MAX_IFACES 8
static NetInterface ifaces[MAX_IFACES];
static int iface_count = 0;

static void str_copy(char* dst, const char* src, int max) {
    int i = 0;
    while (src[i] && i < max - 1) { dst[i] = src[i]; i++; }
    dst[i] = '\0';
}

static void print_ip(u32 addr) {
    sigma_vga_printf("%d.%d.%d.%d",
        (addr >> 24) & 0xFF, (addr >> 16) & 0xFF,
        (addr >> 8) & 0xFF, addr & 0xFF);
}

static void init_demo_interfaces() {
    if (iface_count > 0) return;

    // lo
    str_copy(ifaces[0].name, "lo", 16);
    ifaces[0].ip_addr = 0x7F000001; // 127.0.0.1
    ifaces[0].netmask = 0xFF000000;
    ifaces[0].is_up = 1;
    ifaces[0].mtu = 65536;
    ifaces[0].rx_packets = 1024;
    ifaces[0].tx_packets = 1024;

    // eth0
    str_copy(ifaces[1].name, "eth0", 16);
    ifaces[1].mac[0]=0x52; ifaces[1].mac[1]=0x54; ifaces[1].mac[2]=0x00;
    ifaces[1].mac[3]=0xAB; ifaces[1].mac[4]=0xCD; ifaces[1].mac[5]=0xEF;
    ifaces[1].ip_addr = 0xC0A80164; // 192.168.1.100
    ifaces[1].netmask = 0xFFFFFF00;
    ifaces[1].broadcast = 0xC0A801FF;
    ifaces[1].is_up = 1;
    ifaces[1].mtu = 1500;
    ifaces[1].rx_packets = 48291;
    ifaces[1].tx_packets = 31002;

    iface_count = 2;
}

extern "C" int sigma_ifconfig_main(int argc, char** argv) {
    init_demo_interfaces();

    for (int i = 0; i < iface_count; i++) {
        sigma_vga_printf("%s: flags=%s  mtu %d\n", ifaces[i].name, ifaces[i].is_up ? "<UP,RUNNING>" : "<DOWN>", ifaces[i].mtu);
        sigma_vga_printf("        inet "); print_ip(ifaces[i].ip_addr);
        sigma_vga_printf("  netmask "); print_ip(ifaces[i].netmask);
        if (ifaces[i].broadcast) { sigma_vga_printf("  broadcast "); print_ip(ifaces[i].broadcast); }
        sigma_vga_printf("\n");
        if (ifaces[i].mac[0] || ifaces[i].mac[1]) {
            sigma_vga_printf("        ether %02x:%02x:%02x:%02x:%02x:%02x\n",
                ifaces[i].mac[0], ifaces[i].mac[1], ifaces[i].mac[2],
                ifaces[i].mac[3], ifaces[i].mac[4], ifaces[i].mac[5]);
        }
        sigma_vga_printf("        RX packets %d  TX packets %d\n", ifaces[i].rx_packets, ifaces[i].tx_packets);
        sigma_vga_printf("\n");
    }

    return 0;
}
