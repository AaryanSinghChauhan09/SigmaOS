// SPDX-License-Identifier: GPL-2.0-or-later
// net/sigma_net.cpp — Unified SigmaOS network stack init
#include "net/sigma_net.h"
#include <stdlib.h>
#include <string.h>

sigma_net_stack_t *sigma_net_init(void){
    sigma_net_stack_t *s = (sigma_net_stack_t*)calloc(1, sizeof(*s));
    if (!s) return nullptr;
    s->initialized = true;
    return s;
}

void sigma_net_shutdown(sigma_net_stack_t *s){
    if (!s) return;
    free(s);
}

int sigma_net_config_default(sigma_net_config_t *c){
    if (!c) return -1;
    memset(c, 0, sizeof(*c));
    c->tls_pqc_enabled  = true;
    c->dns_doh_enabled  = true;
    c->dns_dnssec_enabled = false;
    c->wpa3_enabled     = false;
    return 0;
}

int sigma_net_config_secure(sigma_net_config_t *c){
    if (!c) return -1;
    memset(c, 0, sizeof(*c));
    c->tls_pqc_enabled    = true;
    c->dns_doh_enabled    = true;
    c->dns_dnssec_enabled = true;
    c->wpa3_enabled       = true;
    return 0;
}

const char *sigma_net_error_string(int err){
    switch(err){
    case SIGMA_NET_OK:               return "OK";
    case SIGMA_NET_ERR_DNS_TIMEOUT:  return "DNS timeout";
    case SIGMA_NET_ERR_TLS_HANDSHAKE:return "TLS handshake failed";
    case SIGMA_NET_ERR_DHCP_TIMEOUT: return "DHCP timeout";
    case SIGMA_NET_ERR_CONNECT:      return "Connection refused";
    default:                         return "Unknown error";
    }
}
