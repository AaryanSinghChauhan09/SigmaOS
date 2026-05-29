/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SOCKET IMPLEMENTATION
 * =========================================================================
 * Wires the socket struct down to the TCP/IP and NIC layers.
 * =========================================================================
 */

#include "../../include/net/sigma_socket.h"
#include "../../include/net/sigma_net_internal.h"
#include "../../include/sigma_zenithd_log.h"
#include "../../include/sigma_libc.h"

static sigma_i32 next_fd = 100;

int sigma_socket_open(sigma_socket_t* sock) {
    if (!sock) return -1;
    
    /* Allocate a file descriptor and zero the structure */
    sigma_memset(sock, 0, sizeof(sigma_socket_t));
    sock->fd = next_fd++;
    
    /* In a real scenario, we allocate rx_ring and tx_ring via buddy allocator */
    sock->rx_ring = sigma_malloc(4096);
    sock->tx_ring = sigma_malloc(4096);
    
    ZENITH_INFO("net_sock", "Socket opened successfully");
    return 0;
}

int sigma_socket_send(sigma_socket_t* sock, const void* buf, sigma_size_t len) {
    if (!sock || !buf) return -1;
    
    /* Example: passing payload down to the IP layer. 
     * In full TCP mode, this would queue in tx_ring and trigger tcp_process_packet. */
    sigma_ipv4_send(sock->dst_ip, 6 /* TCP */, buf, len);
    
    ZENITH_TRACE("net_sock", "Pushed payload to IP layer");
    return (int)len;
}

int sigma_socket_recv(sigma_socket_t* sock, void* buf, sigma_size_t len) {
    if (!sock || !buf) return -1;
    
    /* TODO: Pull data from sock->rx_ring populated by TCP receive interrupts */
    ZENITH_TRACE("net_sock", "Simulated recv from rx_ring");
    return 0;
}

void sigma_socket_close(sigma_socket_t* sock) {
    if (!sock) return;
    
    if (sock->rx_ring) sigma_free(sock->rx_ring);
    if (sock->tx_ring) sigma_free(sock->tx_ring);
    
    sock->fd = -1;
    ZENITH_INFO("net_sock", "Socket closed");
}
