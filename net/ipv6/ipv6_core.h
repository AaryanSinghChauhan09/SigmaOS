#ifndef SIGMA_IPV6_CORE_H
#define SIGMA_IPV6_CORE_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN IPV6 STACK
 * =========================================================================
 * Dependency-free, dual-stack IPv6 core.
 * =========================================================================
 */

#define IPV6_ADDR_LEN 16

typedef struct {
    uint8_t addr[IPV6_ADDR_LEN];
} sigma_ipv6_addr_t;

typedef struct __attribute__((packed)) {
    uint32_t vtc_flow;      // Version, Traffic Class, Flow Label
    uint16_t payload_len;
    uint8_t  next_header;
    uint8_t  hop_limit;
    sigma_ipv6_addr_t src;
    sigma_ipv6_addr_t dst;
} sigma_ipv6_hdr_t;

/**
 * Initialize IPv6 stack and SLAAC (Stateless Address Autoconfiguration).
 */
void sigma_ipv6_init(void);

/**
 * Process an incoming IPv6 packet from the DDK NIC driver.
 */
void sigma_ipv6_rx(const uint8_t* buffer, uint32_t length);

/**
 * Transmit an IPv6 packet via the DDK NIC driver.
 */
void sigma_ipv6_tx(const sigma_ipv6_addr_t* dst, uint8_t next_hdr, const uint8_t* payload, uint32_t payload_len);

#ifdef __cplusplus
}
#endif

#endif // SIGMA_IPV6_CORE_H
