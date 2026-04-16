#include "../../include/sigma_network.h"
#include <stdio.h>
#include <stdlib.h>

#ifdef _WIN32
    #include <winsock2.h>
    #pragma comment(lib, "ws2_32.lib")
#else
    #include <sys/socket.h>
    #include <netinet/in.h>
    #include <unistd.h>
#endif

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
