/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN NETWORKING INTERFACE (v1.0)
 * =========================================================================
 * Mission: Modular protocol stack (Ethernet, IPv4, IPv6, TCP, UDP).
 * Design: C11 / Zero-Dependency / Registry-Based.
 * =========================================================================
 */

#ifndef SOVEREIGN_NET_H
#define SOVEREIGN_NET_H

#include "sigma_types.h"

typedef void (*sigma_net_handler_fn)(void* payload, sigma_sz_t size);

typedef struct {
    char name[32];
    sigma_u16 ethertype;
    sigma_net_handler_fn handler;
} sovereign_net_protocol_t;

/* Registry API */
void SovereignNet_InitRegistry(void);
sigma_err_t SovereignNet_RegisterProtocol(const char* name, sigma_u16 ethertype, sigma_net_handler_fn handler);
void SovereignNet_ProcessPacket(sigma_u16 ethertype, void* payload, sigma_sz_t size);

#endif /* SOVEREIGN_NET_H */
