/* --------------------------------------------------------------------------
 * SIGMA OS SOVEREIGN HTTP KERNEL (ZERO-DEPENDENCY RAW TCP)
 * --------------------------------------------------------------------------
 * Architecture: Modularized Custom Function Shards
 * This module bypasses Node.js entirely, providing absolute bare-metal serving 
 * of the Zenith Web Dashboard via raw POSIX/Winsock TCP sockets.
 * -------------------------------------------------------------------------- */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef _WIN32
    #include <winsock2.h>
    #pragma comment(lib, "ws2_32.lib")
#else
    #include <sys/socket.h>
    #include <netinet/in.h>
    #include <unistd.h>
#endif

#define PORT 3334
#define BUFFER_SIZE 8192

/* =========================================================================
 * SOVEREIGN NETWORK SUBSYSTEM
 * ========================================================================= */

void sigma_network_init() {
    #ifdef _WIN32
        WSADATA wsa;
        if (WSAStartup(MAKEWORD(2, 2), &wsa) != 0) {
            printf("[FATAL] Kernel failed to initialize Winsock.\n");
            exit(1);
        }
    #endif
}

void sigma_network_cleanup() {
    #ifdef _WIN32
        WSACleanup();
    #endif
}

void sigma_close_socket(int socket_fd) {
    #ifdef _WIN32
        closesocket(socket_fd);
    #else
        close(socket_fd);
    #endif
}

int sigma_create_server_socket(int port) {
    int server_fd = socket(AF_INET, SOCK_STREAM, 0);
    if (server_fd < 0) {
        printf("[FATAL] Sovereign Engine failed to acquire raw socket descriptor.\n");
        exit(1);
    }

    struct sockaddr_in address;
    address.sin_family = AF_INET;
    address.sin_addr.s_addr = INADDR_ANY;
    address.sin_port = htons(port);

    #ifdef _WIN32
        bind(server_fd, (struct sockaddr*)&address, sizeof(address));
    #else
        int opt = 1;
        setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR | SO_REUSEPORT, &opt, sizeof(opt));
        bind(server_fd, (struct sockaddr*)&address, sizeof(address));
    #endif

    if (listen(server_fd, 10) < 0) {
        printf("[FATAL] Sovereign Engine failed to listen on Port %d.\n", port);
        exit(1);
    }

    return server_fd;
}

/* =========================================================================
 * HTTP PROTOCOL SHARDS
 * ========================================================================= */

const char* sigma_http_get_content_type(const char* filepath) {
    char* ext = strrchr(filepath, '.');
    if (!ext) return "text/plain";
    
    if (strcmp(ext, ".html") == 0) return "text/html";
    if (strcmp(ext, ".css") == 0) return "text/css";
    if (strcmp(ext, ".js") == 0) return "application/javascript";
    if (strcmp(ext, ".json") == 0) return "application/json";
    if (strcmp(ext, ".webp") == 0) return "image/webp";
    
    return "text/plain";
}

void sigma_http_send_404(int client_socket) {
    char* not_found = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
    send(client_socket, not_found, strlen(not_found), 0);
    printf("[HTTP] 404 Not Found Generated.\n");
}

void sigma_http_send_200(int client_socket, const char* ctype, long fsize, const char* content) {
    char header[512];
    sprintf(header, "HTTP/1.1 200 OK\r\nContent-Type: %s\r\nContent-Length: %ld\r\nConnection: close\r\n\r\n", ctype, fsize);
    send(client_socket, header, strlen(header), 0);
    send(client_socket, content, fsize, 0);
}

/* =========================================================================
 * VIRTUAL FILE SYSTEM ROUTING
 * ========================================================================= */

void sigma_route_static_file(int client_socket, const char* request_path) {
    char full_path[1024] = "../web_ui/";
    
    // Resolve root endpoint to dashboard
    if (strcmp(request_path, "/") == 0) {
        strcat(full_path, "index.html");
    } else {
        strcat(full_path, request_path + 1); // Ignore leading slash
    }

    // Secure memory allocation for file streaming
    FILE* file = fopen(full_path, "rb");
    if (!file) {
        sigma_http_send_404(client_socket);
        return;
    }

    fseek(file, 0, SEEK_END);
    long fsize = ftell(file);
    fseek(file, 0, SEEK_SET);

    char* content = (char*)malloc(fsize + 1);
    if (!content) {
        sigma_http_send_404(client_socket);
        fclose(file);
        return;
    }

    fread(content, 1, fsize, file);
    fclose(file);

    const char* ctype = sigma_http_get_content_type(full_path);
    sigma_http_send_200(client_socket, ctype, fsize, content);
    
    free(content);
}

void sigma_handle_client_connection(int client_socket) {
    char buffer[BUFFER_SIZE] = {0};
    int bytes_received = recv(client_socket, buffer, BUFFER_SIZE - 1, 0);
    
    if (bytes_received > 0 && strncmp(buffer, "GET ", 4) == 0) {
        char* path_start = buffer + 4;
        char* path_end = strchr(path_start, ' ');
        if (path_end) {
            *path_end = '\0';
            printf("[KERNEL] Routing GET Request -> '%s'\n", path_start);
            sigma_route_static_file(client_socket, path_start);
        }
    }
    sigma_close_socket(client_socket);
}

/* =========================================================================
 * MAIN ORCHESTRATOR LOOP
 * ========================================================================= */

int main() {
    sigma_network_init();

    int server_socket = sigma_create_server_socket(PORT);

    printf("====================================================\n");
    printf("[KERNEL] Σ SIGMA OS SOVEREIGN C-WEB ENGINE\n");
    printf("====================================================\n");
    printf("[KERNEL] C11 Raw TCP Sockets Online (Port %d)\n", PORT);
    printf("[KERNEL] Node.js successfully eliminated.\n");
    printf("[KERNEL] Awaiting incoming visual-dashboard telemetry...\n\n");

    while (1) {
        struct sockaddr_in client_addr;
        int client_len = sizeof(client_addr);
        int client_socket = accept(server_socket, (struct sockaddr*)&client_addr, &client_len);

        if (client_socket >= 0) {
            sigma_handle_client_connection(client_socket);
        }
    }

    sigma_network_cleanup();
    return 0;
}
