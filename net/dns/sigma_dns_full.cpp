// SPDX-License-Identifier: GPL-2.0-or-later
// net/dns/sigma_dns_full.cpp — Full DNS RR decode + DNSSEC + cache prune
#include "net/sigma_dns.h"
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#ifdef _WIN32
#  include <winsock2.h>
#else
#  include <arpa/inet.h>
#endif

static uint16_t r16(const uint8_t *b){return (b[0]<<8)|b[1];}
static uint32_t r32(const uint8_t *b){
    return ((uint32_t)b[0]<<24)|((uint32_t)b[1]<<16)|((uint32_t)b[2]<<8)|b[3];}

// ── RR decoder ────────────────────────────────────────────────────────────
static int decode_rr(const uint8_t *pkt, size_t pkt_len,
                      size_t *off, sigma_dns_rr_t *rr){
    if(sigma_dns_parse_name(pkt,pkt_len,off,rr->name,sizeof(rr->name))<0)
        return -1;
    if(*off+10>pkt_len) return -1;
    rr->type    = r16(pkt+*off); *off+=2;
    rr->rclass  = r16(pkt+*off); *off+=2;
    rr->ttl     = r32(pkt+*off); *off+=4;
    rr->rdlength= r16(pkt+*off); *off+=2;
    if(*off+rr->rdlength>pkt_len) return -1;
    size_t rdo=*off;
    switch(rr->type){
    case DNS_TYPE_A:
        if(rr->rdlength==4) memcpy(rr->data.a.addr,pkt+rdo,4); break;
    case DNS_TYPE_AAAA:
        if(rr->rdlength==16) memcpy(rr->data.aaaa.addr,pkt+rdo,16); break;
    case DNS_TYPE_CNAME:{size_t t=rdo;sigma_dns_parse_name(pkt,pkt_len,&t,
        rr->data.cname.cname,sizeof(rr->data.cname.cname));}break;
    case DNS_TYPE_NS:{size_t t=rdo;sigma_dns_parse_name(pkt,pkt_len,&t,
        rr->data.ns.nsname,sizeof(rr->data.ns.nsname));}break;
    case DNS_TYPE_PTR:{size_t t=rdo;sigma_dns_parse_name(pkt,pkt_len,&t,
        rr->data.ptr.ptrname,sizeof(rr->data.ptr.ptrname));}break;
    case DNS_TYPE_MX:
        if(rr->rdlength>=3){
            rr->data.mx.preference=r16(pkt+rdo);
            size_t t=rdo+2;
            sigma_dns_parse_name(pkt,pkt_len,&t,
                rr->data.mx.exchange,sizeof(rr->data.mx.exchange));}
        break;
    case DNS_TYPE_SRV:
        if(rr->rdlength>=7){
            rr->data.srv.priority=r16(pkt+rdo);
            rr->data.srv.weight  =r16(pkt+rdo+2);
            rr->data.srv.port    =r16(pkt+rdo+4);
            size_t t=rdo+6;
            sigma_dns_parse_name(pkt,pkt_len,&t,
                rr->data.srv.target,sizeof(rr->data.srv.target));}
        break;
    case DNS_TYPE_TXT:
        if(rr->rdlength<sizeof(rr->data.txt.txt)){
            memcpy(rr->data.txt.txt,pkt+rdo,rr->rdlength);
            rr->data.txt.txt[rr->rdlength]='\0';}
        break;
    case DNS_TYPE_DNSKEY:
        if(rr->rdlength>=4){
            rr->data.dnskey.flags    =r16(pkt+rdo);
            rr->data.dnskey.protocol =pkt[rdo+2];
            rr->data.dnskey.algorithm=pkt[rdo+3];
            size_t kl=rr->rdlength-4;
            if(kl<sizeof(rr->data.dnskey.data))
                memcpy(rr->data.dnskey.data,pkt+rdo+4,kl);}
        break;
    case DNS_TYPE_DS:
        if(rr->rdlength>=4){
            rr->data.ds.key_tag     =r16(pkt+rdo);
            rr->data.ds.algorithm   =pkt[rdo+2];
            rr->data.ds.digest_type =pkt[rdo+3];
            rr->data.ds.digest_len  =(uint8_t)(rr->rdlength-4);
            if(rr->data.ds.digest_len<=sizeof(rr->data.ds.digest))
                memcpy(rr->data.ds.digest,pkt+rdo+4,rr->data.ds.digest_len);}
        break;
    case DNS_TYPE_RRSIG:
        if(rr->rdlength>=18){
            rr->data.rrsig.type_covered         =r16(pkt+rdo);
            rr->data.rrsig.algorithm            =pkt[rdo+2];
            rr->data.rrsig.labels               =pkt[rdo+3];
            rr->data.rrsig.original_ttl         =r32(pkt+rdo+4);
            rr->data.rrsig.signature_expiration =r32(pkt+rdo+8);
            rr->data.rrsig.signature_inception  =r32(pkt+rdo+12);
            rr->data.rrsig.key_tag              =r16(pkt+rdo+16);
            size_t t=rdo+18;
            sigma_dns_parse_name(pkt,pkt_len,&t,
                rr->data.rrsig.signer_name,sizeof(rr->data.rrsig.signer_name));
            size_t sl=rr->rdlength-(t-rdo);
            if(sl<=sizeof(rr->data.rrsig.signature)){
                memcpy(rr->data.rrsig.signature,pkt+t,sl);
                rr->data.rrsig.signature_len=(uint32_t)sl;}}
        break;
    default:
        if(rr->rdlength<=sizeof(rr->rdata))
            memcpy(rr->rdata,pkt+rdo,rr->rdlength);
        break;
    }
    *off+=rr->rdlength;
    return 0;
}

// ── Response decoder ──────────────────────────────────────────────────────
int sigma_dns_decode_response(const uint8_t *buf, size_t len,
                               sigma_dns_response_t *resp){
    if(len<DNS_HEADER_SIZE||!resp) return -1;
    memset(resp,0,sizeof(*resp));
    uint16_t flags=r16(buf+2);
    resp->id               =(uint16_t)r16(buf);
    resp->rcode            =(uint8_t)(flags&0xF);
    resp->authoritative    =(flags&0x0400)!=0;
    resp->truncated        =(flags&0x0200)!=0;
    resp->authenticated_data=(flags&0x0020)!=0;
    uint16_t qd=r16(buf+4),an=r16(buf+6),ns=r16(buf+8),ar=r16(buf+10);
    size_t off=DNS_HEADER_SIZE;

    for(uint16_t i=0;i<qd&&off<len;i++){
        if(sigma_dns_parse_name(buf,len,&off,
            resp->question.qname,sizeof(resp->question.qname))<0) return -1;
        if(off+4>len) return -1;
        resp->question.qtype =r16(buf+off); off+=2;
        resp->question.qclass=r16(buf+off); off+=2;
    }
    if(an>0){
        resp->answers=(sigma_dns_rr_t*)calloc(an,sizeof(sigma_dns_rr_t));
        if(!resp->answers) return -1;
        for(uint16_t i=0;i<an&&off<len;i++){
            if(decode_rr(buf,len,&off,&resp->answers[i])<0) break;
            resp->answer_count++;
        }
    }
    if(ns>0){
        resp->authority=(sigma_dns_rr_t*)calloc(ns,sizeof(sigma_dns_rr_t));
        if(resp->authority)
            for(uint16_t i=0;i<ns&&off<len;i++){
                if(decode_rr(buf,len,&off,&resp->authority[i])<0) break;
                resp->authority_count++;
            }
    }
    if(ar>0){
        resp->additional=(sigma_dns_rr_t*)calloc(ar,sizeof(sigma_dns_rr_t));
        if(resp->additional)
            for(uint16_t i=0;i<ar&&off<len;i++){
                if(decode_rr(buf,len,&off,&resp->additional[i])<0) break;
                resp->additional_count++;
            }
    }
    return 0;
}

// ── Address extraction ────────────────────────────────────────────────────
int sigma_dns_get_address_a(const sigma_dns_response_t *r,
                             uint8_t *addr, uint32_t *ttl){
    if(!r||!addr) return -1;
    for(uint16_t i=0;i<r->answer_count;i++)
        if(r->answers[i].type==DNS_TYPE_A){
            memcpy(addr,r->answers[i].data.a.addr,4);
            if(ttl)*ttl=r->answers[i].ttl;
            return 0;}
    return -1;
}
int sigma_dns_get_address_aaaa(const sigma_dns_response_t *r,
                                uint8_t *addr, uint32_t *ttl){
    if(!r||!addr) return -1;
    for(uint16_t i=0;i<r->answer_count;i++)
        if(r->answers[i].type==DNS_TYPE_AAAA){
            memcpy(addr,r->answers[i].data.aaaa.addr,16);
            if(ttl)*ttl=r->answers[i].ttl;
            return 0;}
    return -1;
}

// ── DNSSEC validation stub ────────────────────────────────────────────────
bool sigma_dnssec_validate(sigma_dns_response_t *resp,
                            const uint8_t *trust_anchor, size_t anchor_len){
    if(!resp) return false;
    (void)trust_anchor;(void)anchor_len;
    if(resp->authenticated_data){
        for(uint16_t i=0;i<resp->answer_count;i++)
            resp->answers[i].validated=true;
        return true;
    }
    // Full validation: build chain from trust anchor, verify RRSIG, check DS
    // Left as TODO pending liboqs/OpenSSL integration
    return false;
}

// ── Cache store + prune ───────────────────────────────────────────────────
void sigma_dns_cache_store(sigma_dns_resolver_t *r, const char *name,
                            uint16_t type, const sigma_dns_response_t *resp){
    if(!r||!name||!resp||!r->config.cache_enabled) return;
    uint32_t min_ttl=UINT32_MAX;
    for(uint16_t i=0;i<resp->answer_count;i++)
        if(resp->answers[i].ttl<min_ttl) min_ttl=resp->answers[i].ttl;
    if(min_ttl==UINT32_MAX) min_ttl=r->config.cache_min_ttl;
    if(min_ttl>r->config.cache_max_ttl) min_ttl=r->config.cache_max_ttl;
    if(min_ttl<r->config.cache_min_ttl) min_ttl=r->config.cache_min_ttl;

    sigma_dns_cache_entry_t *e=(sigma_dns_cache_entry_t*)calloc(1,sizeof(*e));
    if(!e) return;
    snprintf(e->key,sizeof(e->key),"%s:%u",name,type);
    e->record_count=resp->answer_count;
    if(resp->answer_count>0){
        e->records=(sigma_dns_rr_t*)calloc(resp->answer_count,sizeof(sigma_dns_rr_t));
        if(e->records) memcpy(e->records,resp->answers,resp->answer_count*sizeof(sigma_dns_rr_t));
    }
    e->expires_at=time(nullptr)+(time_t)min_ttl;
    e->valid=true;
    e->next=r->cache;
    if(r->cache) r->cache->prev=e;
    r->cache=e;
    r->cache_count++;
    if(r->cache_count>r->config.cache_max_entries)
        sigma_dns_cache_prune(r);
}

void sigma_dns_cache_prune(sigma_dns_resolver_t *r){
    if(!r) return;
    time_t now=time(nullptr);
    sigma_dns_cache_entry_t *e=r->cache;
    while(e){
        sigma_dns_cache_entry_t *n=e->next;
        if(!e->valid||e->expires_at<=now){
            if(e->prev) e->prev->next=e->next;
            else        r->cache=e->next;
            if(e->next) e->next->prev=e->prev;
            if(e->records) free(e->records);
            free(e);
            r->cache_count--;
        }
        e=n;
    }
    // Hard-trim oldest if still over limit
    while(r->cache_count>r->config.cache_max_entries&&r->cache){
        sigma_dns_cache_entry_t *old=r->cache;
        r->cache=old->next;
        if(r->cache) r->cache->prev=nullptr;
        if(old->records) free(old->records);
        free(old);
        r->cache_count--;
    }
}

// ── Async resolve (simple synchronous wrapper) ────────────────────────────
int sigma_dns_resolve_async(sigma_dns_resolver_t *r, const char *name,
                             uint16_t type,
                             void(*cb)(sigma_dns_response_t*,void*),void *ctx){
    sigma_dns_response_t *resp=nullptr;
    int rc=sigma_dns_resolve(r,name,type,&resp);
    if(cb) cb(resp,ctx);
    return rc;
}

void sigma_dns_response_free(sigma_dns_response_t *resp){
    if(!resp) return;
    if(resp->answers)    free(resp->answers);
    if(resp->authority)  free(resp->authority);
    if(resp->additional) free(resp->additional);
    free(resp);
}

// ── rcode string ─────────────────────────────────────────────────────────
const char *sigma_dns_rcode_to_string(uint16_t rcode){
    static const char *rc[]={
        "NOERROR","FORMERR","SERVFAIL","NXDOMAIN",
        "NOTIMP","REFUSED","YXDOMAIN","YXRRSET",
        "NXRRSET","NOTAUTH","NOTZONE"};
    if(rcode<11) return rc[rcode];
    static char b[16]; snprintf(b,sizeof(b),"RCODE%u",rcode); return b;
}
