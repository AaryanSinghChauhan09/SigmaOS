#ifndef SIGMA_NETWORK_PROTOCOLS_EXT_H
#define SIGMA_NETWORK_PROTOCOLS_EXT_H


#include <sigma_types.h>

// SigmaOS Extended Network Protocol Shards
// Native, zero-dependency implementations of FTP and Email cores.

// FTP Client/Server initialization hooks
void net_ftp_start_server(uint16_t port);
bool net_ftp_client_transfer(const char* remote_ip, const char* file_path);

// Native SMTP/IMAP logic for Email system-level support
typedef struct {
    const char* server;
    uint16_t port;
    bool use_tls;
} MailConfig;

bool net_mail_send_secure(MailConfig* config, const char* recipient, const char* body);

#endif // SIGMA_NETWORK_PROTOCOLS_EXT_H

