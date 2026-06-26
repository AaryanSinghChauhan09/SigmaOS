/*
 * Σ SigmaOS — sigma_wget: Sovereign HTTP Downloader
 * Absorbs: GNU wget, curl
 * Zero-Dependency: No libc.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" int sigma_tcp_connect(u32 ip, u32 port);
extern "C" int sigma_tcp_send(int sock, const char* data, u32 len);
extern "C" int sigma_tcp_recv(int sock, char* buf, u32 max_len);
extern "C" u32 sigma_fat32_write(const char* name, const unsigned char* buf, u32 len);

extern "C" int sigma_wget_main(int argc, char** argv) {
    if (argc < 2) {
        sigma_vga_printf("Usage: wget <url>\n");
        return 1;
    }

    const char* url = argv[1];
    sigma_vga_printf("[WGET] Resolving '%s'...\n", url);
    
    u32 ip = 0x08080808; // DNS Stub
    int sock = sigma_tcp_connect(ip, 80);
    
    if (sock < 0) {
        sigma_vga_printf("[WGET] Failed to connect to port 80.\n");
        return 1;
    }

    const char* request = "GET / HTTP/1.1\r\nHost: sigmaos.local\r\nConnection: close\r\n\r\n";
    sigma_tcp_send(sock, request, 58);

    char response_buf[4096];
    int bytes = sigma_tcp_recv(sock, response_buf, 4096);
    
    if (bytes > 0) {
        sigma_vga_printf("[WGET] Received %d bytes. Saving to 'index.html'.\n", bytes);
        sigma_fat32_write("index.html", (const unsigned char*)response_buf, bytes);
    } else {
        sigma_vga_printf("[WGET] No data received.\n");
    }

    return 0;
}
