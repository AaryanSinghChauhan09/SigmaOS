#ifndef SIGMA_HTTP_H
#define SIGMA_HTTP_H

/* =========================================================================
 * SIGMA OS: HTTP PROTOCOL LAYER (SYSTEM-LEVEL HEADER)
 * ========================================================================= */

const char* sigma_http_get_content_type(const char* filepath);
void sigma_http_send_404(int client_socket);
void sigma_http_send_200_chromium(int client_socket, const char* ctype, long fsize, const char* content);
void sigma_route_static_file(int client_socket, const char* request_path);
void sigma_handle_client_connection(int client_socket);

#endif
