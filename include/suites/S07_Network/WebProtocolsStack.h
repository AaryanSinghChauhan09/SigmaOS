#ifndef SIGMA_WEB_PROTOCOLS_H
#define SIGMA_WEB_PROTOCOLS_H

#include "suites/S01_Genesis/shards/sigma_types.h"


// SigmaOS Web & Connectivity Protocol Stack
// Natively accelerates modern web interactions bypassing traditional bloated libraries.

// Enforce system-wide TLS 1.3 encryption leveraging the S08_Security enclave bounds
void net_web_enforce_ssl_tls(void);

// Initialize a zero-dependency HTTP/HTTPS proxy configuration manager
void net_web_init_proxy(const char* proxy_ip, uint16_t port);

// Hook for Web Browser integrators to interface directly with the compositor and network stack natively
void* net_web_create_browser_viewport(uint32_t width, uint32_t height);

// Native WebDAV client integration mapping directly into the S06_Storage VFS layer
bool net_web_mount_webdav(const char* target_url, const char* local_mount_point);

// Start the remote desktop listener prioritizing GPU hardware-encoding streams
void net_web_start_remote_desktop(uint16_t listen_port);

#endif // SIGMA_WEB_PROTOCOLS_H

