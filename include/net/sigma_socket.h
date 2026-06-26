/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SOCKET API
 * =========================================================================
 * Native, POSIX-free kernel networking socket interface.
 * =========================================================================
 */

#ifndef SIGMA_SOCKET_H
#define SIGMA_SOCKET_H

#include "../sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    sigma_i32 fd;
    sigma_u32 src_ip;
    sigma_u32 dst_ip;
    sigma_u16 src_port;
    sigma_u16 dst_port;
    
    /* Internal tracking for TCP/UDP buffers */
    void*     rx_ring;
    void*     tx_ring;
} sigma_socket_t;

/* Open a new socket and allocate kernel buffers */
int sigma_socket_open(sigma_socket_t* sock);

/* Send payload over the socket (pushes to TCP state machine -> IP -> NIC) */
int sigma_socket_send(sigma_socket_t* sock, const void* buf, sigma_size_t len);

/* Receive payload from the socket (polls the internal rx_ring) */
int sigma_socket_recv(sigma_socket_t* sock, void* buf, sigma_size_t len);

/* Close the socket and free buffers */
void sigma_socket_close(sigma_socket_t* sock);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SOCKET_H */
