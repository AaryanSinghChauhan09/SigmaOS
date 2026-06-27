// SPDX-License-Identifier: GPL-2.0-or-later
// net/dhcp/sigma_dhcp.cpp — DHCP client (RFC 2131/2132)
// State machine: INIT → SELECTING → REQUESTING → BOUND → RENEWING → REBINDING
#include "net/sigma_dhcp.h"
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#ifdef _WIN32
#  include <winsock2.h>
#else
#  include <arpa/inet.h>
#endif

// ── Client lifecycle ──────────────────────────────────────────────────────
sigma_dhcp_client_t *sigma_dhcp_client_new(const char *iface){
    sigma_dhcp_client_t *c=(sigma_dhcp_client_t*)calloc(1,sizeof(*c));
    if(!c) return nullptr;
    strncpy(c->config.interface,iface,sizeof(c->config.interface)-1);
    c->config.timeout_ms=30000;
    c->config.max_retries=3;
    c->config.request_hostname=true;
    c->lease.state=DHCP_LEASE_STATE_INIT;
    c->initialized=true;
    return c;
}
void sigma_dhcp_client_free(sigma_dhcp_client_t *c){
    if(!c) return;
    memset(&c->lease,0,sizeof(c->lease));
    free(c);
}

// ── IP helpers ────────────────────────────────────────────────────────────
void sigma_dhcp_ip_to_string(uint32_t ip, char *buf, size_t len){
    snprintf(buf,(int)len,"%u.%u.%u.%u",
             (ip>>24)&0xFF,(ip>>16)&0xFF,(ip>>8)&0xFF,ip&0xFF);
}
uint32_t sigma_dhcp_string_to_ip(const char *s){
    if(!s) return 0;
    unsigned a,b,c,d;
    if(sscanf(s,"%u.%u.%u.%u",&a,&b,&c,&d)!=4) return 0;
    return ((uint32_t)a<<24)|((uint32_t)b<<16)|((uint32_t)c<<8)|(uint32_t)d;
}

// ── Lease helpers ─────────────────────────────────────────────────────────
int64_t sigma_dhcp_lease_remaining(const sigma_dhcp_client_t *c){
    if(!c||c->lease.state!=DHCP_LEASE_STATE_BOUND) return -1;
    return (int64_t)(c->lease.lease_expires - time(nullptr));
}

// ── Message type helpers ──────────────────────────────────────────────────
const char *sigma_dhcp_msg_type_to_string(sigma_dhcp_msg_type_t t){
    switch(t){
    case DHCP_MSG_DISCOVER: return "DISCOVER";
    case DHCP_MSG_OFFER:    return "OFFER";
    case DHCP_MSG_REQUEST:  return "REQUEST";
    case DHCP_MSG_DECLINE:  return "DECLINE";
    case DHCP_MSG_ACK:      return "ACK";
    case DHCP_MSG_NAK:      return "NAK";
    case DHCP_MSG_RELEASE:  return "RELEASE";
    case DHCP_MSG_INFORM:   return "INFORM";
    default:                return "UNKNOWN";
    }
}

// ── Option building ───────────────────────────────────────────────────────
int sigma_dhcp_add_msg_type(uint8_t *opts, size_t *off, sigma_dhcp_msg_type_t t){
    opts[(*off)++]=DHCP_OPT_MSG_TYPE;
    opts[(*off)++]=1;
    opts[(*off)++]=(uint8_t)t;
    return 0;
}
int sigma_dhcp_add_requested_ip(uint8_t *opts, size_t *off, uint32_t ip){
    opts[(*off)++]=DHCP_OPT_REQUESTED_IP;
    opts[(*off)++]=4;
    opts[(*off)++]=(ip>>24)&0xFF;
    opts[(*off)++]=(ip>>16)&0xFF;
    opts[(*off)++]=(ip>>8)&0xFF;
    opts[(*off)++]=ip&0xFF;
    return 0;
}

// ── Discovery / request (stub transport) ─────────────────────────────────
int sigma_dhcp_discover(sigma_dhcp_client_t *c){
    if(!c) return -1;
    c->lease.state=DHCP_LEASE_STATE_SELECTING;
    return 0;
}
int sigma_dhcp_request(sigma_dhcp_client_t *c, uint32_t offered_ip, uint32_t server_ip){
    if(!c) return -1;
    c->lease.ip_address=offered_ip;
    c->lease.server_ip=server_ip;
    c->lease.state=DHCP_LEASE_STATE_REQUESTING;
    return 0;
}
int sigma_dhcp_release(sigma_dhcp_client_t *c){
    if(!c) return -1;
    memset(&c->lease,0,sizeof(c->lease));
    c->lease.state=DHCP_LEASE_STATE_INIT;
    return 0;
}
