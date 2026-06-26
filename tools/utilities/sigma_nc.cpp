/*
 * Σ SigmaOS — sigma_nc: Sovereign Netcat
 * Absorbs: netcat (nc)
 * Zero-Dependency: No libc.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" char sigma_keyboard_getch();
extern "C" int sigma_tcp_connect(u32 ip, u32 port);
extern "C" int sigma_tcp_listen(u32 port);
extern "C" int sigma_tcp_accept(int listen_sock);
extern "C" int sigma_tcp_send(int sock, const char* data, u32 len);
extern "C" int sigma_tcp_recv(int sock, char* buf, u32 max_len);

extern "C" int sigma_nc_main(int argc, char** argv) {
    if (argc < 3) {
        sigma_vga_printf("Usage: nc <host> <port>   (Connect)\n");
        sigma_vga_printf("       nc -l <port>       (Listen)\n");
        return 1;
    }

    bool listen_mode = false;
    u32 port = 0;

    if (argv[1][0] == '-' && argv[1][1] == 'l') {
        listen_mode = true;
        // Basic atoi
        const char* p = argv[2];
        while (*p >= '0' && *p <= '9') {
            port = port * 10 + (*p - '0');
            p++;
        }
    } else {
        // Stub atoi for port
        const char* p = argv[2];
        while (*p >= '0' && *p <= '9') {
            port = port * 10 + (*p - '0');
            p++;
        }
    }

    int sock = -1;

    if (listen_mode) {
        sigma_vga_printf("[NC] Listening on port %d...\n", port);
        int lsock = sigma_tcp_listen(port);
        sock = sigma_tcp_accept(lsock);
        sigma_vga_printf("[NC] Connection accepted.\n");
    } else {
        sigma_vga_printf("[NC] Connecting to port %d...\n", port);
        sock = sigma_tcp_connect(0x7F000001, port); // Stub localhost
    }

    if (sock < 0) {
        sigma_vga_printf("[NC] Socket error.\n");
        return 1;
    }

    // In a real implementation this would use select/poll on keyboard and socket.
    // Stub: Send a test message and receive
    if (!listen_mode) {
        sigma_tcp_send(sock, "Hello from SigmaOS NC!\n", 23);
    }

    char buf[128];
    int bytes = sigma_tcp_recv(sock, buf, 127);
    if (bytes > 0) {
        buf[bytes] = '\0';
        sigma_vga_printf("%s", buf);
    }

    return 0;
}
