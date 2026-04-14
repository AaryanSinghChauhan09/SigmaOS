#ifndef SIGMA_SOCKET_API_H
#define SIGMA_SOCKET_API_H

#include <stdint.h>

// SigmaOS Universal Socket API
// Absorbing the reliability of BSD sockets with the asynchronous power of Windows IOCP (I/O Completion Ports)

typedef struct {
    uint32_t socket_id;
    uint8_t protocol; // 0 for TCP, 1 for UDP
    uint16_t port;
} SigmaSocket;

SigmaSocket* net_create_socket(uint8_t protocol);
int32_t net_bind(SigmaSocket* sock, uint16_t port);
int32_t net_listen(SigmaSocket* sock);
int32_t net_accept_async(SigmaSocket* sock, void (*callback)(SigmaSocket*));

#endif // SIGMA_SOCKET_API_H
