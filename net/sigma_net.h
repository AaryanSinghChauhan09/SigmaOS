// SPDX-License-Identifier: GPL-2.0-or-later
#ifndef SIGMA_NET_H
#define SIGMA_NET_H

/**
 * sigma_net.h — Unified Network Stack Integration Header
 * Aggregates: TLS 1.3 + Kyber, DNS-over-HTTPS + DNSSEC, DHCP, WPA3/SAE
 */

#include "tls/sigma_tls.h"
#include "dns/sigma_dns.h"
#include "../net/dhcp/sigma_dhcp.h"

#ifdef __cplusplus
extern "C" {
#endif

#define SIGMA_NET_VERSION  "1.0.0"

/* ── Stack config ────────────────────────────────────────────────────────────── */
typedef struct {
    sigma_tls_config_t  tls;
    sigma_dns_config_t  dns;
    sigma_dhcp_config_t dhcp;
    bool                tls_pqc_enabled;
    bool                dns_doh_enabled;
    bool                dns_dnssec_enabled;
    bool                dhcp_enabled;
    char                hostname[64];
    char                primary_interface[32];
    uint32_t            connection_timeout_ms;
    uint32_t            resolution_timeout_ms;
} sigma_net_config_t;

/* ── Stack context ───────────────────────────────────────────────────────────── */
typedef struct {
    sigma_net_config_t   config;
    sigma_dns_resolver_t* dns_resolver;
    sigma_dhcp_client_t*  dhcp_client;
    bool                  initialized;
    bool                  network_up;
    struct {
        uint64_t bytes_sent, bytes_received;
        uint64_t tcp_connections;
        uint64_t dns_queries;
        uint64_t tls_handshakes;
        uint64_t dhcp_renewals;
    } stats;
} sigma_net_stack_t;

/* ── Error codes ─────────────────────────────────────────────────────────────── */
#define SIGMA_NET_OK                0
#define SIGMA_NET_ERR_INIT         -1
#define SIGMA_NET_ERR_DNS_TIMEOUT  -2
#define SIGMA_NET_ERR_DNS_NXDOMAIN -3
#define SIGMA_NET_ERR_TLS_HANDSHAKE -4
#define SIGMA_NET_ERR_TLS_CERT     -5
#define SIGMA_NET_ERR_DHCP_TIMEOUT -6
#define SIGMA_NET_ERR_NO_NETWORK   -8

/* ── API ─────────────────────────────────────────────────────────────────────── */
sigma_net_stack_t* sigma_net_init(void);
sigma_net_stack_t* sigma_net_init_with_config(const sigma_net_config_t*);
void               sigma_net_shutdown(sigma_net_stack_t*);
int                sigma_net_tick(sigma_net_stack_t*);
bool               sigma_net_is_available(const sigma_net_stack_t*);

int  sigma_net_get_ip_address  (const sigma_net_stack_t*, uint32_t* ip);
int  sigma_net_get_gateway     (const sigma_net_stack_t*, uint32_t* gw);
int  sigma_net_get_dns_servers (const sigma_net_stack_t*, uint32_t*, size_t*);

int  sigma_net_resolve         (sigma_net_stack_t*, const char* hostname,
                                  uint8_t* addr, size_t* addr_len);
int  sigma_net_connect_secure  (sigma_net_stack_t*, const char*, uint16_t,
                                  sigma_tls_session_t**);
int  sigma_net_set_pqc_enabled (sigma_net_stack_t*, bool);

/* Config factories */
int  sigma_net_config_default  (sigma_net_config_t*);   /* sane defaults         */
int  sigma_net_config_secure   (sigma_net_config_t*);   /* DNSSEC + PQC + full   */
int  sigma_net_config_fast     (sigma_net_config_t*);   /* caching, no DNSSEC    */

/* Helpers */
int  sigma_net_setup           (sigma_net_stack_t*);
int  sigma_net_wait_available  (sigma_net_stack_t*, uint32_t timeout_ms);
bool sigma_net_check_connectivity(sigma_net_stack_t*, const char* hostname);
int  sigma_net_get_info        (const sigma_net_stack_t*, char* buf, size_t len);
const char* sigma_net_error_string(int error);

#ifdef __cplusplus
}
#endif
#endif /* SIGMA_NET_H */
