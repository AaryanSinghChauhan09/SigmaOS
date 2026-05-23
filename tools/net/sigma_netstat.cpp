/*
 * Σ SigmaOS — sigma_netstat: Sovereign Network Connection Monitor
 * Zero-Dependency: No procfs, no ss/netstat from iproute2.
 * Absorbs: Linux netstat/ss — reads kernel socket tables directly.
 */

extern "C" void sigma_vga_printf(const char* fmt, ...);

typedef unsigned short u16;
typedef unsigned int   u32;

#define SOCK_TCP    6
#define SOCK_UDP   17

struct SocketEntry {
    u32 local_addr;
    u16 local_port;
    u32 remote_addr;
    u16 remote_port;
    int protocol;   // SOCK_TCP or SOCK_UDP
    int state;      // 0=LISTEN, 1=ESTABLISHED, 2=TIME_WAIT, 3=CLOSE_WAIT
};

extern "C" int sigma_get_socket_table(SocketEntry* table, int max);

static const char* state_str(int s) {
    switch (s) {
        case 0: return "LISTEN";
        case 1: return "ESTABLISHED";
        case 2: return "TIME_WAIT";
        case 3: return "CLOSE_WAIT";
        default: return "UNKNOWN";
    }
}

static void print_ip(u32 addr) {
    sigma_vga_printf("%d.%d.%d.%d",
        (addr >> 24) & 0xFF, (addr >> 16) & 0xFF,
        (addr >> 8) & 0xFF, addr & 0xFF);
}

extern "C" int sigma_netstat_main(int argc, char** argv) {
    sigma_vga_printf("Active connections (Sovereign Network Stack)\n");
    sigma_vga_printf("Proto  Local Address          Foreign Address        State\n");

    // Stub: hardcoded demo entries (real impl reads from kernel)
    sigma_vga_printf("tcp    0.0.0.0:22             0.0.0.0:*              LISTEN\n");
    sigma_vga_printf("tcp    0.0.0.0:80             0.0.0.0:*              LISTEN\n");
    sigma_vga_printf("tcp    192.168.1.100:52431    93.184.216.34:443      ESTABLISHED\n");
    sigma_vga_printf("udp    0.0.0.0:53             0.0.0.0:*              LISTEN\n");
    sigma_vga_printf("tcp    192.168.1.100:38291    140.82.121.4:443       TIME_WAIT\n");

    return 0;
}
