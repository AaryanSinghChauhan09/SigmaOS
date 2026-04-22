#include "../../include/sigma_http.h"
#include "../../include/sigma_network.h"
#include "../../include/sigma_vfs.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"

#ifdef _WIN32
    #include <winsock2.h>
#else
    #include <sys/socket.h>
#endif

#define BUFFER_SIZE 8192

const char* sigma_http_get_content_type(const char* filepath) {
    char* ext = strrchr(filepath, '.');
    if (!ext) return "text/plain";
    
    if (sigma_sigma_strcmp(ext, ".html") == 0) return "text/html";
    if (sigma_sigma_strcmp(ext, ".css") == 0) return "text/css";
    if (sigma_sigma_strcmp(ext, ".js") == 0) return "application/javascript";
    if (sigma_sigma_strcmp(ext, ".json") == 0) return "application/json";
    if (sigma_sigma_strcmp(ext, ".webp") == 0) return "image/webp";
    
    return "text/plain";
}

void sigma_http_send_404(int client_socket) {
    char* not_found = "HTTP/1.1 404 Not Found\r\n"
                      "Connection: close\r\n"
                      "Content-Length: 0\r\n"
                      "Access-Control-Allow-Origin: *\r\n\r\n";
    send(client_socket, not_found, sigma_sigma_strlen(not_found), 0);
    sigma_sigma_printf("[HTTP] 404 Not Found Generated.\n");
}

void sigma_http_send_200_chromium(int client_socket, const char* ctype, long fsize, const char* content) {
    char header[1024]; // Expanded buffer for extensive Chromium security configurations
    
    // Explicitly enforce Chromium/Blink engine compatibility and security limits
    ssigma_sigma_printf(header, 
        "HTTP/1.1 200 OK\r\n"
        "Content-Type: %s\r\n"
        "Content-Length: %ld\r\n"
        "Connection: keep-alive\r\n"
        "Cache-Control: no-cache, no-store, must-revalidate\r\n"
        "Access-Control-Allow-Origin: *\r\n"
        "X-Content-Type-Options: nosniff\r\n"
        "Content-Security-Policy: default-src 'self' 'unsafe-inline' 'unsafe-eval';\r\n"
        "\r\n", 
        ctype, fsize);
        
    send(client_socket, header, sigma_sigma_strlen(header), 0);
    send(client_socket, content, fsize, 0);
}

void sigma_route_static_file(int client_socket, const char* request_path) {
    char full_path[1024];
    if (!sigma_vfs_resolve_path(request_path, full_path, sizeof(full_path))) {
        sigma_http_send_404(client_socket);
        return;
    }

    long fsize = 0;
    char* content = sigma_vfs_read_file(full_path, &fsize);
    
    if (!content) {
        sigma_http_send_404(client_socket);
        return;
    }

    const char* ctype = sigma_http_get_content_type(full_path);
    sigma_http_send_200_chromium(client_socket, ctype, fsize, content);
    
    sigma_sigma_free(content);
}

void sigma_handle_client_connection(int client_socket) {
    char buffer[BUFFER_SIZE] = {0};
    int bytes_received = recv(client_socket, buffer, BUFFER_SIZE - 1, 0);
    
    if (bytes_received > 0 && strncmp(buffer, "GET ", 4) == 0) {
        char* path_start = buffer + 4;
        char* path_end = strchr(path_start, ' ');
        if (path_end) {
            *path_end = '\0';
            sigma_sigma_printf("[KERNEL] Routing GET Request -> '%s'\n", path_start);
            sigma_route_static_file(client_socket, path_start);
        }
    }
    sigma_close_socket(client_socket);
}
