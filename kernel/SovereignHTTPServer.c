/* --------------------------------------------------------------------------
 * SIGMA OS SOVEREIGN HTTP KERNEL (ZERO-DEPENDENCY RAW TCP)
 * --------------------------------------------------------------------------
 * This module bypasses Node.js entirely, providing absolute bare-metal serving 
 * of the Zenith Web Dashboard via raw POSIX/Winsock TCP sockets. 
 * Eliminates high-level JS runtime dependencies.
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

const char* get_content_type(const char* ext) {
    if (strcmp(ext, ".html") == 0) return "text/html";
    if (strcmp(ext, ".css") == 0) return "text/css";
    if (strcmp(ext, ".js") == 0) return "application/javascript";
    if (strcmp(ext, ".json") == 0) return "application/json";
    return "text/plain";
}

void serve_file(int client_socket, const char* filepath) {
    char full_path[512] = "../web_ui/";
    
    // Parse root request
    if (strcmp(filepath, "/") == 0) {
        strcat(full_path, "index.html");
    } else {
        strcat(full_path, filepath + 1); // Skip leading slash
    }

    FILE* file = fopen(full_path, "rb");
    if (!file) {
        char* not_found = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
        send(client_socket, not_found, strlen(not_found), 0);
        return;
    }

    fseek(file, 0, SEEK_END);
    long fsize = ftell(file);
    fseek(file, 0, SEEK_SET);

    char* content = (char*)malloc(fsize + 1);
    fread(content, 1, fsize, file);
    fclose(file);

    char* ext = strrchr(full_path, '.');
    const char* ctype = ext ? get_content_type(ext) : "text/plain";

    char header[512];
    sprintf(header, "HTTP/1.1 200 OK\r\nContent-Type: %s\r\nContent-Length: %ld\r\nConnection: close\r\n\r\n", ctype, fsize);
    
    send(client_socket, header, strlen(header), 0);
    send(client_socket, content, fsize, 0);
    
    free(content);
}

int main() {
    #ifdef _WIN32
        WSADATA wsa;
        WSAStartup(MAKEWORD(2, 2), &wsa);
    #endif

    int server_fd = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in address;
    address.sin_family = AF_INET;
    address.sin_addr.s_addr = INADDR_ANY;
    address.sin_port = htons(PORT);

    #ifdef _WIN32
        bind(server_fd, (struct sockaddr*)&address, sizeof(address));
    #else
        int opt = 1;
        setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR | SO_REUSEPORT, &opt, sizeof(opt));
        bind(server_fd, (struct sockaddr*)&address, sizeof(address));
    #endif

    listen(server_fd, 5);
    printf("[KERNEL] Sovereign C HTTP Engine Online on Port %d\n", PORT);
    printf("[KERNEL] Node.js successfully bypassed. Serving native silicon.\n");

    while (1) {
        struct sockaddr_in client_addr;
        int client_len = sizeof(client_addr);
        int client_socket = accept(server_fd, (struct sockaddr*)&client_addr, &client_len);

        char buffer[BUFFER_SIZE] = {0};
        recv(client_socket, buffer, BUFFER_SIZE - 1, 0);

        if (strncmp(buffer, "GET ", 4) == 0) {
            char* path_start = buffer + 4;
            char* path_end = strchr(path_start, ' ');
            if (path_end) {
                *path_end = '\0';
                serve_file(client_socket, path_start);
            }
        }

        #ifdef _WIN32
            closesocket(client_socket);
        #else
            close(client_socket);
        #endif
    }

    #ifdef _WIN32
        WSACleanup();
    #endif
    return 0;
}
