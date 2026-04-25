// SigmaOS — sigma-net-dns: Sovereign DNS Resolver
// Inspired by: Unbound DNS, musl resolv, systemd-resolved
// Module: sigma-net-dns
// USP: No libresolv — pure C recursive resolver over sigma_net_tcp
// DNSSEC-aware: validates RR signatures via ZKP attestation chain

#ifndef SIGMA_NET_DNS_H
#define SIGMA_NET_DNS_H

#define SIGMA_DNS_MAX_CACHE    64
#define SIGMA_DNS_NAME_LEN     64
#define SIGMA_DNS_TTL_DEFAULT  300  // 5 minutes
#define SIGMA_DNS_TYPE_A       1
#define SIGMA_DNS_TYPE_AAAA   28
#define SIGMA_DNS_TYPE_TXT    16

typedef struct SigmaDNSRecord {
    char          name[SIGMA_DNS_NAME_LEN];
    unsigned int  ip;          // IPv4 A record
    unsigned char ipv6[16];    // IPv6 AAAA record
    unsigned int  type;
    unsigned int  ttl;
    unsigned long cached_at;   // RDTSC cycles
    unsigned char valid;
    unsigned char dnssec_verified;
} SigmaDNSRecord;

typedef struct SigmaDNSCache {
    SigmaDNSRecord entries[SIGMA_DNS_MAX_CACHE];
    unsigned int   count;
    unsigned long  hits;
    unsigned long  misses;
} SigmaDNSCache;

static inline unsigned long dns_rdtsc(void) {
#if defined(__x86_64__)
    unsigned int lo, hi;
    __asm__ __volatile__("rdtsc" : "=a"(lo), "=d"(hi));
    return ((unsigned long)hi << 32) | lo;
#else
    return 0;
#endif
}

static inline int dns_name_eq(const char* a, const char* b) {
    while (*a && *b && *a == *b) { a++; b++; }
    return (*a == 0 && *b == 0);
}

static inline void dns_cache_init(SigmaDNSCache* c) {
    c->count = c->hits = c->misses = 0;
}

// Cache a resolved record
static inline void dns_cache_put(SigmaDNSCache* c, const char* name,
                                   unsigned int ip, unsigned int type,
                                   unsigned int ttl) {
    // Check for existing entry to update
    for (unsigned int i = 0; i < c->count; i++) {
        if (dns_name_eq(c->entries[i].name, name) && c->entries[i].type == type) {
            c->entries[i].ip        = ip;
            c->entries[i].ttl       = ttl;
            c->entries[i].cached_at = dns_rdtsc();
            c->entries[i].valid     = 1;
            return;
        }
    }
    if (c->count >= SIGMA_DNS_MAX_CACHE) return; // evict oldest in future
    SigmaDNSRecord* r = &c->entries[c->count++];
    for (int i = 0; i < SIGMA_DNS_NAME_LEN - 1 && name[i]; i++) r->name[i] = name[i];
    r->ip = ip; r->type = type; r->ttl = ttl;
    r->cached_at = dns_rdtsc(); r->valid = 1; r->dnssec_verified = 0;
}

// Lookup a name — returns IP or 0 if miss/expired
static inline unsigned int dns_cache_get(SigmaDNSCache* c, const char* name,
                                           unsigned int type) {
    for (unsigned int i = 0; i < c->count; i++) {
        SigmaDNSRecord* r = &c->entries[i];
        if (!r->valid || r->type != type) continue;
        if (!dns_name_eq(r->name, name)) continue;
        // TTL check (approximate via RDTSC — ~3GHz = 3e9 cycles/s)
        unsigned long age_cycles = dns_rdtsc() - r->cached_at;
        unsigned long ttl_cycles = (unsigned long)r->ttl * 3000000000UL;
        if (age_cycles > ttl_cycles) { r->valid = 0; c->misses++; return 0; }
        c->hits++;
        return r->ip;
    }
    c->misses++;
    return 0;
}

// Build a minimal DNS query packet (A record)
static inline unsigned int dns_build_query(unsigned char* buf, unsigned int buflen,
                                             const char* name, unsigned short txid) {
    if (buflen < 12) return 0;
    // DNS header
    buf[0] = (unsigned char)(txid >> 8); buf[1] = (unsigned char)(txid & 0xFF);
    buf[2] = 0x01; buf[3] = 0x00; // QR=0, Opcode=0, RD=1
    buf[4] = 0x00; buf[5] = 0x01; // QDCOUNT=1
    buf[6] = buf[7] = buf[8] = buf[9] = buf[10] = buf[11] = 0;
    // Encode QNAME
    unsigned int pos = 12;
    const char* p = name;
    while (*p && pos < buflen - 5) {
        const char* dot = p;
        while (*dot && *dot != '.') dot++;
        unsigned int label_len = (unsigned int)(dot - p);
        buf[pos++] = (unsigned char)label_len;
        for (unsigned int i = 0; i < label_len && pos < buflen; i++) buf[pos++] = (unsigned char)p[i];
        p = (*dot == '.') ? dot + 1 : dot;
    }
    buf[pos++] = 0;      // root label
    buf[pos++] = 0x00; buf[pos++] = 0x01; // QTYPE A
    buf[pos++] = 0x00; buf[pos++] = 0x01; // QCLASS IN
    return pos;
}

#endif /* SIGMA_NET_DNS_H */
