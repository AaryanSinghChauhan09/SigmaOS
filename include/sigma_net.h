/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN LATTICE NETWORKING (S-NET)
 * =========================================================================
 * Mission: Zero-latency, industrial-grade network stack.
 * Principle: PQC-signed packets, DoS-resilient routing.
 * =========================================================================
 */

#ifndef SIGMA_NET_H
#define SIGMA_NET_H

#include "core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    sigma_u8  ip[4];
    sigma_u16 port;
} sigma_endpoint_t;

/* --- Networking Primitives --- */
void      net_init(void);
int       net_socket(void);
int       net_bind(int sock, sigma_endpoint_t* ep);
int       net_send(int sock, const void* data, sigma_usize size);
int       net_recv(int sock, void* buf, sigma_usize size);
void      net_close(int sock);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_NET_H */
