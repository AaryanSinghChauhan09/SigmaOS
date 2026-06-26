// SPDX-License-Identifier: GPL-2.0-or-later
#ifndef SIGMA_DNS_H
#define SIGMA_DNS_H

/**
 * SigmaOS DNS Resolver — RFC 1035 UDP/TCP, RFC 8484 DoH, RFC 4033-4035 DNSSEC
 */

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ── Constants ─────────────────────────────────────────────────────────────── */
#define DNS_PORT                  53
#define DOH_PORT                  443
#define DNS_MAX_DOMAIN_LEN        253
#define DNS_MAX_LABEL_LEN         63
#define DNS_HEADER_SIZE           12
#define DNS_MAX_UDP_SIZE          512
#define DNS_MAX_TCP_SIZE          65535

/* Record Types */
#define DNS_TYPE_A       1
#define DNS_TYPE_NS      2
#define DNS_TYPE_CNAME   5
#define DNS_TYPE_SOA     6
#define DNS_TYPE_PTR     12
#define DNS_TYPE_MX      15
#define DNS_TYPE_TXT     16
#define DNS_TYPE_AAAA    28
#define DNS_TYPE_SRV     33
#define DNS_TYPE_DNSKEY  48
#define DNS_TYPE_DS      43
#define DNS_TYPE_RRSIG   46
#define DNS_TYPE_NSEC    47
#define DNS_TYPE_NSEC3   50
#define DNS_TYPE_ANY     255

/* Classes */
#define DNS_CLASS_IN  1
#define DNS_CLASS_ANY 255

/* Response Codes */
#define DNS_RCODE_NOERROR   0
#define DNS_RCODE_FORMERR   1
#define DNS_RCODE_SERVFAIL  2
#define DNS_RCODE_NXDOMAIN  3
#define DNS_RCODE_NOTIMP    4
#define DNS_RCODE_REFUSED   5

/* EDNS0 flags */
#define DNS_OPT_DO  0x8000   /* DNSSEC OK */

/* Defaults */
#define DNS_DEFAULT_TIMEOUT_MS   5000
#define DNS_CACHE_DEFAULT_TTL    300
#define DNS_CACHE_MAX_ENTRIES    1024
#define DNS_MAX_RETRIES          3

/* ── Wire header ────────────────────────────────────────────────────────────── */
typedef struct __attribute__((packed)) {
    uint16_t id;
    uint16_t flags;
    uint16_t qdcount;
    uint16_t ancount;
    uint16_t nscount;
    uint16_t arcount;
} sigma_dns_header_t;

#define DNS_FLAG_QR    0x8000
#define DNS_FLAG_AA    0x0400
#define DNS_FLAG_TC    0x0200
#define DNS_FLAG_RD    0x0100
#define DNS_FLAG_RA    0x0080
#define DNS_FLAG_AD    0x0020
#define DNS_FLAG_CD    0x0010
#define DNS_FLAG_RCODE 0x000F

/* ── Question ───────────────────────────────────────────────────────────────── */
typedef struct {
    char     qname[DNS_MAX_DOMAIN_LEN + 1];
    uint16_t qtype;
    uint16_t qclass;
} sigma_dns_question_t;

/* ── Resource Record ────────────────────────────────────────────────────────── */
typedef struct {
    char     name[DNS_MAX_DOMAIN_LEN + 1];
    uint16_t type;
    uint16_t rclass;
    uint32_t ttl;
    uint16_t rdlength;
    uint8_t  rdata[4096];
    union {
        struct { uint8_t addr[4];  } a;
        struct { uint8_t addr[16]; } aaaa;
        struct { char nsname[DNS_MAX_DOMAIN_LEN + 1]; } ns;
        struct { uint16_t preference; char exchange[DNS_MAX_DOMAIN_LEN + 1]; } mx;
        struct { char cname[DNS_MAX_DOMAIN_LEN + 1]; } cname;
        struct { char ptrname[DNS_MAX_DOMAIN_LEN + 1]; } ptr;
        struct { char txt[1024]; } txt;
        struct { uint16_t priority; uint16_t weight; uint16_t port;
                 char target[DNS_MAX_DOMAIN_LEN + 1]; } srv;
        struct { uint16_t flags; uint8_t protocol; uint8_t algorithm;
                 uint8_t data[512]; uint16_t key_tag; } dnskey;
        struct { uint16_t key_tag; uint8_t algorithm; uint8_t digest_type;
                 uint8_t digest[64]; uint16_t digest_len; } ds;
    } data;
    bool validated;
    bool bogus;
} sigma_dns_rr_t;

/* ── Response ───────────────────────────────────────────────────────────────── */
typedef struct {
    uint16_t             id;
    uint16_t             rcode;
    bool                 authoritative;
    bool                 truncated;
    bool                 authenticated_data;
    sigma_dns_question_t question;
    sigma_dns_rr_t*      answers;
    uint16_t             answer_count;
    sigma_dns_rr_t*      authority;
    uint16_t             authority_count;
    sigma_dns_rr_t*      additional;
    uint16_t             additional_count;
    uint32_t             elapsed_ms;
    uint32_t             cache_ttl;
    bool                 from_cache;
    char                 resolver[64];
} sigma_dns_response_t;

/* ── Transport ──────────────────────────────────────────────────────────────── */
typedef enum {
    DNS_TRANSPORT_UDP   = 0,
    DNS_TRANSPORT_TCP   = 1,
    DNS_TRANSPORT_HTTPS = 2,   /* DoH */
    DNS_TRANSPORT_TLS   = 3,   /* DoT */
    DNS_TRANSPORT_AUTO  = 4,
} sigma_dns_transport_t;

/* ── Config ─────────────────────────────────────────────────────────────────── */
typedef struct {
    char**                servers;
    size_t                server_count;
    sigma_dns_transport_t transport;
    uint16_t              port;
    char*                 doh_path;
    bool                  doh_post;
    uint32_t              timeout_ms;
    uint8_t               max_retries;
    bool                  dnssec_enabled;
    bool                  dnssec_required;
    bool                  edns0;
    uint16_t              edns0_udp_size;
    char**                search_domains;
    size_t                search_domain_count;
    bool                  cache_enabled;
    uint32_t              cache_max_ttl;
    uint32_t              cache_min_ttl;
    size_t                cache_max_entries;
    void (*on_resolve)(const sigma_dns_response_t*, void*);
    void*                 cb_ctx;
} sigma_dns_config_t;

/* ── Resolver context ───────────────────────────────────────────────────────── */
typedef struct {
    sigma_dns_config_t config;
    void*              cache;    /* opaque cache implementation */
    size_t             cache_count;
    uint16_t           next_id;
    bool               initialized;
    uint64_t           queries_total;
    uint64_t           queries_cached;
    uint64_t           queries_failed;
} sigma_dns_resolver_t;

/* ── API ──────────────────────────────────────────────────────────────────── */
sigma_dns_resolver_t* sigma_dns_resolver_new(void);
sigma_dns_resolver_t* sigma_dns_resolver_new_with_config(const sigma_dns_config_t*);
void sigma_dns_resolver_free(sigma_dns_resolver_t*);

int  sigma_dns_config_add_server(sigma_dns_config_t*, const char*);
int  sigma_dns_config_add_search_domain(sigma_dns_config_t*, const char*);
int  sigma_dns_config_set_doh(sigma_dns_config_t*, const char* url, bool use_post);

int  sigma_dns_resolve(sigma_dns_resolver_t*, const char* name, uint16_t type,
                        sigma_dns_response_t**);
int  sigma_dns_resolve_async(sigma_dns_resolver_t*, const char*, uint16_t,
                              void (*)(sigma_dns_response_t*, void*), void*);
void sigma_dns_response_free(sigma_dns_response_t*);

void sigma_dns_cache_clear(sigma_dns_resolver_t*);
void sigma_dns_cache_prune(sigma_dns_resolver_t*);
bool sigma_dns_cache_lookup(sigma_dns_resolver_t*, const char*, uint16_t,
                             sigma_dns_response_t**);

bool sigma_dnssec_validate(sigma_dns_response_t*, const uint8_t*, size_t);

int sigma_dns_encode_query(uint8_t*, size_t*, const sigma_dns_question_t*, bool dnssec);
int sigma_dns_decode_response(const uint8_t*, size_t, sigma_dns_response_t*);

const char* sigma_dns_type_to_string(uint16_t);
const char* sigma_dns_rcode_to_string(uint16_t);

int sigma_dns_get_address_a   (const sigma_dns_response_t*, uint8_t[4],  uint32_t*);
int sigma_dns_get_address_aaaa(const sigma_dns_response_t*, uint8_t[16], uint32_t*);

/* Root KSK 2017 */
#define DNS_ROOT_KSK_2017_LEN  421
extern const uint8_t SIGMA_DNS_ROOT_KSK_2017[DNS_ROOT_KSK_2017_LEN];

#ifdef __cplusplus
}
#endif
#endif /* SIGMA_DNS_H */
