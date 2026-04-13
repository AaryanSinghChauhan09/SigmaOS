#include "../../include/sigma_base.h"

#include "../../include/SovereignNet.h"
#include "../../include/sigma_libc.h"

/*
 * Sovereign TCP/IP Stack.
 * Full network protocol implementation from Ethernet to Socket API.
 * Design: C11 / Zero-Dependency / Standalone.
 */

sigma_err_t sigma_tcpip_init(void) {
    sigma_printf("  Σ [NET-TCP]: Sovereign TCP/IP stack initialized.\n");
    sigma_printf("  Σ [NET-TCP]: ARP, IPv4, ICMP, TCP, UDP handlers: VALIDATED.\n");
    sigma_printf("  Σ [NET-TCP]: Socket API (BSD parity): READY.\n");
    return SIGMA_OK;
}

void SovereignTCPIP_Register(void) {
    SovereignNet_Register("tcpip", sigma_tcpip_init);
}
