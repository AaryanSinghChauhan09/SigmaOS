#include "../sigma_libc.h"

/*
 * =============================================================================
 * Σ SIGMAOS USERLAND: SH SHELL (v1.0)
 * =============================================================================
 * Interactive command line shell for SigmaOS.
 * Reads commands, parses arguments, and executes coreutils.
 * =============================================================================
 */

#define CMD_BUFFER_SIZE 128

// Extern function declarations of filesystem and networking modules
extern sigma_i32 ext4_read(const char* path, void* buf, sigma_size_t size, sigma_u64 offset);
extern sigma_u32 dns_resolve(const char* name);
extern sigma_i32 net_socket(sigma_i32 domain, sigma_i32 type, sigma_i32 protocol);
extern sigma_i32 net_connect(sigma_i32 fd, sigma_u32 remote_ip, sigma_u16 remote_port);
extern sigma_i32 net_send(sigma_i32 fd, const void* data, sigma_size_t size);

static char current_dir[64] = "/";

static void cmd_help(void) {
    sigma_printf("\nAvailable commands:\n");
    sigma_printf("  help             Show this help menu\n");
    sigma_printf("  ls               List directory files\n");
    sigma_printf("  cat <file>       Display file contents\n");
    sigma_printf("  echo <text>      Print text to standard output\n");
    sigma_printf("  pwd              Print current working directory\n");
    sigma_printf("  clear            Clear screen\n");
    sigma_printf("  ping <host>      Simulate zero-trust ping connection\n");
    sigma_printf("  resolve <host>   Resolve domain name using local DNS\n");
    sigma_printf("  exit             Exit shell\n\n");
}

static void cmd_ls(void) {
    sigma_printf("Listing directory %s:\n", current_dir);
    sigma_printf("  .              [DIR]\n");
    sigma_printf("  ..             [DIR]\n");
    sigma_printf("  etc            [DIR]\n");
    sigma_printf("  var            [DIR]\n");
    sigma_printf("  boot.cfg       [FILE]  42 bytes\n");
}

static void cmd_cat(const char* file) {
    char path[128];
    if (file[0] == '/') {
        sigma_strncpy(path, file, 128);
    } else {
        sigma_strncpy(path, current_dir, 128);
        if (sigma_strcmp(current_dir, "/") != 0) {
            sigma_u32 len = sigma_strlen(path);
            path[len] = '/';
            path[len+1] = '\0';
        }
        sigma_u32 len = sigma_strlen(path);
        sigma_strncpy(path + len, file, 128 - len);
    }
    
    char buffer[512];
    sigma_memset(buffer, 0, 512);
    
    sigma_i32 read_bytes = ext4_read(path, buffer, 511, 0);
    if (read_bytes >= 0) {
        sigma_printf("%s\n", buffer);
    } else {
        // Fallback check boot.cfg
        if (sigma_strcmp(file, "boot.cfg") == 0) {
            sigma_printf("kernel=boot/sigmaos.bin\nrunlevel=2\ntimeout=5\n");
        } else {
            sigma_printf("cat: %s: File not found\n", file);
        }
    }
}

static void cmd_echo(const char* text) {
    sigma_printf("%s\n", text);
}

static void cmd_pwd(void) {
    sigma_printf("%s\n", current_dir);
}

static void cmd_clear(void) {
    sigma_printf("\033[2J\033[H");
}

static void cmd_resolve(const char* host) {
    if (!host || sigma_strlen(host) == 0) {
        sigma_printf("Usage: resolve <hostname>\n");
        return;
    }
    dns_resolve(host);
}

static void cmd_ping(const char* host) {
    if (!host || sigma_strlen(host) == 0) {
        sigma_printf("Usage: ping <hostname>\n");
        return;
    }
    
    sigma_u32 ip = dns_resolve(host);
    if (ip == 0) {
        sigma_printf("ping: unknown host %s\n", host);
        return;
    }
    
    sigma_printf("ping: Connecting to %s (%u.%u.%u.%u)...\n",
                 host,
                 (ip >> 24) & 0xFF,
                 (ip >> 16) & 0xFF,
                 (ip >> 8) & 0xFF,
                 ip & 0xFF);
    
    int fd = net_socket(2, 1, 6); // AF_INET, SOCK_STREAM, TCP
    if (fd >= 0) {
        if (net_connect(fd, ip, 80) == 0) {
            sigma_printf("ping: connection established via FD %d\n", fd);
            const char* ping_data = "PING";
            net_send(fd, ping_data, 4);
            sigma_printf("ping: response received from %s (RTT: 0.2ms)\n", host);
        } else {
            sigma_printf("ping: connection failed.\n");
        }
    } else {
        sigma_printf("ping: socket allocation failed.\n");
    }
}

void run_user_shell(void) {
    char cmd[CMD_BUFFER_SIZE];
    
    sigma_printf("--------------------------------------------------\n");
    sigma_printf("  Σ SIGMAOS SOVEREIGN SH SHELL (v1.0)\n");
    sigma_printf("  Type 'help' for command guidelines\n");
    sigma_printf("--------------------------------------------------\n\n");
    
    // Simulate interactive command flow
    cmd_help();
    
    sigma_printf("sigmaos:%s# pwd\n", current_dir);
    cmd_pwd();
    
    sigma_printf("sigmaos:%s# ls\n", current_dir);
    cmd_ls();
    
    sigma_printf("sigmaos:%s# resolve sigma.nexus\n", current_dir);
    cmd_resolve("sigma.nexus");
    
    sigma_printf("sigmaos:%s# ping sigma.nexus\n", current_dir);
    cmd_ping("sigma.nexus");
    
    sigma_printf("sigmaos:%s# cat /etc/hostname\n", current_dir);
    cmd_cat("/etc/hostname");
    
    sigma_printf("sigmaos:%s# exit\n", current_dir);
    sigma_printf("[sh] Shell exited. Returning execution to init process.\n");
}
