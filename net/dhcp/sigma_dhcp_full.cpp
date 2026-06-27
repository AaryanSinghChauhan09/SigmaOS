// SPDX-License-Identifier: GPL-2.0-or-later
// net/dhcp/sigma_dhcp_full.cpp — Full RFC 2131/2132 DHCP client
// State machine: INIT→SELECTING→REQUESTING→BOUND→RENEWING→REBINDING→EXPIRED
#include "net/sigma_dhcp.h"
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#ifdef _WIN32
#  include <winsock2.h>
#else
#  include <arpa/inet.h>
#  include <unistd.h>
#endif

// ── Utility ───────────────────────────────────────────────────────────────
void sigma_dhcp_hwaddr_to_string(const uint8_t *h, char *buf, size_t len){
    snprintf(buf,len,"%02X:%02X:%02X:%02X:%02X:%02X",
             h[0],h[1],h[2],h[3],h[4],h[5]);
}
int sigma_dhcp_string_to_hwaddr(const char *s, uint8_t *h){
    return sscanf(s,"%hhx:%hhx:%hhx:%hhx:%hhx:%hhx",
                  &h[0],&h[1],&h[2],&h[3],&h[4],&h[5])==6?0:-1;
}
const char *sigma_dhcp_state_to_string(sigma_dhcp_lease_state_t st){
    switch(st){
    case DHCP_LEASE_STATE_INIT:       return "INIT";
    case DHCP_LEASE_STATE_SELECTING:  return "SELECTING";
    case DHCP_LEASE_STATE_REQUESTING: return "REQUESTING";
    case DHCP_LEASE_STATE_BOUND:      return "BOUND";
    case DHCP_LEASE_STATE_RENEWING:   return "RENEWING";
    case DHCP_LEASE_STATE_REBINDING:  return "REBINDING";
    case DHCP_LEASE_STATE_EXPIRED:    return "EXPIRED";
    case DHCP_LEASE_STATE_RELEASED:   return "RELEASED";
    default:                          return "UNKNOWN";
    }
}

// ── Option building ───────────────────────────────────────────────────────
static int add_opt(uint8_t *o, size_t *off, size_t max,
                   uint8_t code, const void *data, uint8_t dlen){
    if(*off+2+dlen>max) return -1;
    o[(*off)++]=code; o[(*off)++]=dlen;
    if(dlen) memcpy(o+*off,data,dlen);
    *off+=dlen;
    return 0;
}

int sigma_dhcp_add_option(uint8_t *o,size_t *off,size_t max,
                           uint8_t code,const void *data,uint8_t dlen){
    return add_opt(o,off,max,code,data,dlen);
}
int sigma_dhcp_add_client_id(uint8_t *o,size_t *off,
                              const uint8_t *hw,uint8_t hlen){
    if(*off+3+hlen>312) return -1;
    o[(*off)++]=DHCP_OPT_CLIENT_ID;
    o[(*off)++]=1+hlen;
    o[(*off)++]=DHCP_HTYPE_ETHERNET;
    memcpy(o+*off,hw,hlen); *off+=hlen;
    return 0;
}
int sigma_dhcp_add_hostname(uint8_t *o,size_t *off,const char *h){
    uint8_t l=(uint8_t)strlen(h);
    return l?add_opt(o,off,312,DHCP_OPT_HOST_NAME,h,l):-1;
}
int sigma_dhcp_add_server_id(uint8_t *o,size_t *off,uint32_t ip){
    uint32_t n=htonl(ip);
    return add_opt(o,off,312,DHCP_OPT_SERVER_ID,&n,4);
}
int sigma_dhcp_add_param_list(uint8_t *o,size_t *off,
                               const uint8_t *list,uint8_t len){
    return add_opt(o,off,312,DHCP_OPT_PARAM_LIST,list,len);
}
int sigma_dhcp_add_end(uint8_t *o,size_t *off){
    if(*off>=312) return -1;
    o[(*off)++]=DHCP_OPT_END; return 0;
}

// ── Message building ──────────────────────────────────────────────────────
int sigma_dhcp_build_discover(const sigma_dhcp_client_t *c,
                               sigma_dhcp_message_t *msg,
                               uint8_t *opts_out, size_t *opts_len){
    if(!c||!msg||!opts_out) return -1;
    memset(msg,0,sizeof(*msg));
    msg->op=DHCP_OP_BOOTREQUEST; msg->htype=DHCP_HTYPE_ETHERNET; msg->hlen=6;
    msg->xid=htonl(c->xid_counter);
    msg->flags=c->config.broadcast_flag?htons(DHCP_FLAG_BROADCAST):0;
    memcpy(msg->chaddr,c->config.hwaddr,6);
    msg->magic=htonl(DHCP_MAGIC_COOKIE);

    size_t off=0;
    sigma_dhcp_add_msg_type(opts_out,&off,DHCP_MSG_DISCOVER);
    sigma_dhcp_add_client_id(opts_out,&off,c->config.hwaddr,6);
    if(c->config.hostname[0]) sigma_dhcp_add_hostname(opts_out,&off,c->config.hostname);
    static const uint8_t params[]={
        DHCP_OPT_SUBNET_MASK,DHCP_OPT_ROUTER,DHCP_OPT_DNS_SERVER,
        DHCP_OPT_DOMAIN_NAME,DHCP_OPT_BROADCAST_ADDR,
        DHCP_OPT_LEASE_TIME,DHCP_OPT_RENEWAL_TIME,DHCP_OPT_REBIND_TIME};
    sigma_dhcp_add_param_list(opts_out,&off,params,sizeof(params));
    sigma_dhcp_add_end(opts_out,&off);
    memcpy(msg->options,opts_out,off);
    *opts_len=off;
    return 0;
}

int sigma_dhcp_build_request(const sigma_dhcp_client_t *c,
                              sigma_dhcp_message_t *msg,
                              uint8_t *opts_out, size_t *opts_len,
                              uint32_t req_ip, uint32_t server_id){
    if(!c||!msg||!opts_out) return -1;
    memset(msg,0,sizeof(*msg));
    msg->op=DHCP_OP_BOOTREQUEST; msg->htype=DHCP_HTYPE_ETHERNET; msg->hlen=6;
    msg->xid=htonl(c->xid_counter);
    if(c->lease.state==DHCP_LEASE_STATE_RENEWING)
        msg->ciaddr=htonl(c->lease.ip_address);
    else
        msg->flags=c->config.broadcast_flag?htons(DHCP_FLAG_BROADCAST):0;
    memcpy(msg->chaddr,c->config.hwaddr,6);
    msg->magic=htonl(DHCP_MAGIC_COOKIE);

    size_t off=0;
    sigma_dhcp_add_msg_type(opts_out,&off,DHCP_MSG_REQUEST);
    sigma_dhcp_add_client_id(opts_out,&off,c->config.hwaddr,6);
    if(req_ip&&c->lease.state!=DHCP_LEASE_STATE_RENEWING)
        sigma_dhcp_add_requested_ip(opts_out,&off,req_ip);
    if(server_id&&c->lease.state==DHCP_LEASE_STATE_SELECTING)
        sigma_dhcp_add_server_id(opts_out,&off,server_id);
    if(c->config.hostname[0]) sigma_dhcp_add_hostname(opts_out,&off,c->config.hostname);
    sigma_dhcp_add_end(opts_out,&off);
    memcpy(msg->options,opts_out,off);
    *opts_len=off;
    return 0;
}

// ── Option parsing ────────────────────────────────────────────────────────
int sigma_dhcp_parse_options(const uint8_t *opts, size_t len,
                              sigma_dhcp_option_t **out, size_t *count){
    if(!opts||!out||!count) return -1;
    // Count first
    size_t n=0; const uint8_t *p=opts; size_t r=len;
    while(r>=1&&*p!=DHCP_OPT_END){
        if(*p==DHCP_OPT_PAD){p++;r--;continue;}
        if(r<2) break;
        uint8_t dl=p[1]; p+=2+dl; r-=2+dl; n++;
    }
    *out=(sigma_dhcp_option_t*)calloc(n,sizeof(**out));
    if(!*out) return -1;
    p=opts; r=len; size_t i=0;
    while(r>=1&&*p!=DHCP_OPT_END&&i<n){
        if(*p==DHCP_OPT_PAD){p++;r--;continue;}
        if(r<2) break;
        (*out)[i].code=*p++; (*out)[i].length=*p++;
        (*out)[i].data=(uint8_t*)p;
        p+=(*out)[i].length; r-=2+(*out)[i].length; i++;
    }
    *count=i;
    return 0;
}

// ── Apply lease options ───────────────────────────────────────────────────
static void apply_lease_options(sigma_dhcp_lease_t *l,
                                 const sigma_dhcp_option_t *opts, size_t n){
    for(size_t i=0;i<n;i++){
        const sigma_dhcp_option_t *o=&opts[i];
        switch(o->code){
        case DHCP_OPT_SUBNET_MASK:
            if(o->length==4) l->subnet_mask=ntohl(*(uint32_t*)o->data); break;
        case DHCP_OPT_ROUTER:
            if(o->length>=4) l->router=ntohl(*(uint32_t*)o->data); break;
        case DHCP_OPT_DNS_SERVER:
            l->dns_count=o->length/4; if(l->dns_count>4)l->dns_count=4;
            for(uint8_t j=0;j<l->dns_count;j++)
                l->dns_servers[j]=ntohl(*(uint32_t*)(o->data+j*4)); break;
        case DHCP_OPT_DOMAIN_NAME:
            if(o->length<sizeof(l->domain_name)){
                memcpy(l->domain_name,o->data,o->length);
                l->domain_name[o->length]='\0';} break;
        case DHCP_OPT_LEASE_TIME:
            if(o->length==4) l->lease_time=ntohl(*(uint32_t*)o->data); break;
        case DHCP_OPT_RENEWAL_TIME:
            if(o->length==4) l->renewal_time=ntohl(*(uint32_t*)o->data); break;
        case DHCP_OPT_REBIND_TIME:
            if(o->length==4) l->rebind_time=ntohl(*(uint32_t*)o->data); break;
        case DHCP_OPT_SERVER_ID:
            if(o->length==4) l->server_id=ntohl(*(uint32_t*)o->data); break;
        }
    }
    if(!l->lease_time)   l->lease_time=DHCP_DEFAULT_LEASE;
    if(!l->renewal_time) l->renewal_time=l->lease_time/2;
    if(!l->rebind_time)  l->rebind_time=l->lease_time*7/8;
}

// ── Process incoming message ──────────────────────────────────────────────
int sigma_dhcp_process_message(sigma_dhcp_client_t *c,
                                const sigma_dhcp_message_t *msg,
                                size_t msg_len){
    if(!c||!msg) return -1;
    if(msg->magic!=DHCP_MAGIC_COOKIE) return -1;

    sigma_dhcp_option_t *opts=nullptr; size_t opt_n=0;
    sigma_dhcp_parse_options(msg->options,sizeof(msg->options),&opts,&opt_n);

    uint8_t mtype=0;
    for(size_t i=0;i<opt_n;i++)
        if(opts[i].code==DHCP_OPT_MSG_TYPE){mtype=opts[i].data[0];break;}

    switch(mtype){
    case DHCP_MSG_OFFER:
        if(c->lease.state==DHCP_LEASE_STATE_SELECTING){
            c->lease.yiaddr=msg->yiaddr;
            c->lease.server_id=msg->siaddr;
            for(size_t i=0;i<opt_n;i++)
                if(opts[i].code==DHCP_OPT_SERVER_ID&&opts[i].length==4)
                    { c->lease.server_id=ntohl(*(uint32_t*)opts[i].data); break; }
            sigma_dhcp_request(c,c->lease.yiaddr,c->lease.server_id);
        } break;
    case DHCP_MSG_ACK:
        if(c->lease.state==DHCP_LEASE_STATE_REQUESTING||
           c->lease.state==DHCP_LEASE_STATE_RENEWING||
           c->lease.state==DHCP_LEASE_STATE_REBINDING){
            c->lease.ip_address=msg->yiaddr;
            apply_lease_options(&c->lease,opts,opt_n);
            c->lease.lease_obtained=time(nullptr);
            c->lease.lease_expires=c->lease.lease_obtained+c->lease.lease_time;
            c->lease.state=DHCP_LEASE_STATE_BOUND;
            if(c->config.on_bound) c->config.on_bound(&c->lease,c->config.cb_ctx);
        } break;
    case DHCP_MSG_NAK:
        memset(&c->lease,0,sizeof(c->lease));
        memcpy(c->lease.hwaddr,c->config.hwaddr,6);
        strncpy(c->lease.interface,c->config.interface,sizeof(c->lease.interface)-1);
        c->lease.state=DHCP_LEASE_STATE_INIT;
        if(c->config.on_error)
            c->config.on_error(c->config.interface,-1,"DHCP NAK",c->config.cb_ctx);
        break;
    }
    if(opts) free(opts);
    return 0;
}

// ── Lease query helpers ───────────────────────────────────────────────────
const sigma_dhcp_lease_t *sigma_dhcp_get_lease(const sigma_dhcp_client_t *c){
    return c?&c->lease:nullptr;
}
bool sigma_dhcp_lease_is_valid(const sigma_dhcp_client_t *c){
    if(!c) return false;
    return (c->lease.state==DHCP_LEASE_STATE_BOUND||
            c->lease.state==DHCP_LEASE_STATE_RENEWING||
            c->lease.state==DHCP_LEASE_STATE_REBINDING)
           && time(nullptr)<c->lease.lease_expires;
}

// ── Event tick ───────────────────────────────────────────────────────────
int sigma_dhcp_tick(sigma_dhcp_client_t *c){
    if(!c) return -1;
    time_t now=time(nullptr);
    switch(c->lease.state){
    case DHCP_LEASE_STATE_INIT: sigma_dhcp_discover(c); break;
    case DHCP_LEASE_STATE_BOUND:
        if(now>=c->lease.lease_obtained+(time_t)c->lease.renewal_time)
            sigma_dhcp_renew(c); break;
    case DHCP_LEASE_STATE_RENEWING:
        if(now>=c->lease.lease_obtained+(time_t)c->lease.rebind_time){
            c->lease.state=DHCP_LEASE_STATE_REBINDING;
            sigma_dhcp_request(c,c->lease.ip_address,0);} break;
    case DHCP_LEASE_STATE_REBINDING:
        if(now>=c->lease.lease_expires){
            c->lease.state=DHCP_LEASE_STATE_EXPIRED;
            if(c->config.on_expired)
                c->config.on_expired(c->config.interface,c->config.cb_ctx);} break;
    default: break;
    }
    return 0;
}

int sigma_dhcp_renew(sigma_dhcp_client_t *c){
    if(!c||c->lease.state!=DHCP_LEASE_STATE_BOUND) return -1;
    c->lease.state=DHCP_LEASE_STATE_RENEWING;
    return sigma_dhcp_request(c,0,c->lease.server_id);
}
int sigma_dhcp_decline(sigma_dhcp_client_t *c,uint32_t ip,const char *reason){
    (void)c;(void)ip;(void)reason; return 0;
}
// Socket stubs
int      sigma_dhcp_socket_open(const char *iface){(void)iface;return 0;}
int      sigma_dhcp_socket_bind(int fd,const char *iface){(void)fd;(void)iface;return 0;}
ssize_t  sigma_dhcp_socket_send(int fd,const uint8_t *d,size_t l,uint32_t dst)
         {(void)fd;(void)d;(void)dst;return(ssize_t)l;}
ssize_t  sigma_dhcp_socket_recv(int fd,uint8_t *d,size_t ml,uint32_t *src)
         {(void)fd;(void)d;(void)ml;(void)src;return 0;}
void     sigma_dhcp_socket_close(int fd){(void)fd;}
