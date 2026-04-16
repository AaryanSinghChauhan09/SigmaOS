#include "../../include/sigma_http.h"
#include "../../include/sigma_network.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef _WIN32
    #include <winsock2.h>
#else
    #include <sys/socket.h>
#endif

#define BUFFER_SIZE 8192

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

void sigma_route_static_file(int client_socket, const char* request_path) {
    char full_path[1024] = "../web_ui/";
    
    if (strcmp(request_path, "/") == 0) {
        strcat(full_path, "index.html");
    } else {
        strcat(full_path, request_path + 1);
    }

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
