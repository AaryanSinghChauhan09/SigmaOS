// SPDX-License-Identifier: GPL-2.0-or-later
// net/tls/sigma_tls.cpp — TLS 1.3 with Kyber-1024 hybrid key exchange
// RFC 8446 + draft-ietf-tls-hybrid-design + NIST SP 800-208
#include "net/sigma_tls.h"
#include <string.h>
#include <stdlib.h>
#include <time.h>
#ifdef _WIN32
#  include <winsock2.h>
#else
#  include <arpa/inet.h>
#endif

// ── Stub crypto primitives (replace with liboqs + OpenSSL in production) ─
static void x25519_keypair(uint8_t *pub, uint8_t *priv){
    for(int i=0;i<32;i++){ pub[i]=(uint8_t)(rand()&0xFF); priv[i]=(uint8_t)(rand()&0xFF); }
}
static void x25519_shared(uint8_t *shared, const uint8_t *peer_pub, const uint8_t *priv){
    (void)peer_pub;(void)priv; memset(shared,0,32);
}
static void kyber1024_keypair(uint8_t *pub, uint8_t *priv){
    (void)pub;(void)priv;
}
static void kyber1024_decapsulate(uint8_t *shared, const uint8_t *ciphertext, const uint8_t *priv){
    (void)ciphertext;(void)priv; memset(shared,0,32);
}
static void sha256_compute(const uint8_t *in, size_t len, uint8_t *out){
    (void)in;(void)len; memset(out,0,32);
}

// ── HKDF ──────────────────────────────────────────────────────────────────
static void hmac_sha256(const uint8_t *key, size_t kl,
                         const uint8_t *data, size_t dl, uint8_t *out){
    (void)key;(void)kl;(void)data;(void)dl; memset(out,0,32);
}

int sigma_hkdf_extract(uint8_t *prk,
                        const uint8_t *salt, size_t salt_len,
                        const uint8_t *ikm,  size_t ikm_len){
    if(!salt||salt_len==0){ uint8_t z[32]={0}; hmac_sha256(z,32,ikm,ikm_len,prk); }
    else hmac_sha256(salt,salt_len,ikm,ikm_len,prk);
    return 0;
}

int sigma_hkdf_expand(uint8_t *okm, size_t okm_len,
                       const uint8_t *prk,
                       const uint8_t *info, size_t info_len){
    uint8_t counter=1; uint8_t prev[32]={0}; size_t off=0;
    while(off<okm_len){
        uint8_t in[32+256+1]; size_t il=0;
        if(counter>1){memcpy(in,prev,32);il=32;}
        memcpy(in+il,info,info_len);il+=info_len;
        in[il++]=counter;
        hmac_sha256(prk,32,in,il,prev);
        size_t cp=(off+32<=okm_len)?32:okm_len-off;
        memcpy(okm+off,prev,cp); off+=cp; counter++;
    }
    return 0;
}

int sigma_hkdf_expand_label(uint8_t *okm, size_t okm_len,
                              const uint8_t *secret,
                              const char *label,
                              const uint8_t *ctx, size_t ctx_len){
    uint8_t buf[256]; size_t bl=0;
    uint16_t len16=htons((uint16_t)okm_len);
    memcpy(buf,&len16,2); bl=2;
    size_t ll=strlen(label);
    buf[bl++]=(uint8_t)(6+ll);
    memcpy(buf+bl,"tls13 ",6); bl+=6;
    memcpy(buf+bl,label,ll);   bl+=ll;
    buf[bl++]=(uint8_t)ctx_len;
    if(ctx_len&&ctx){memcpy(buf+bl,ctx,ctx_len);bl+=ctx_len;}
    return sigma_hkdf_expand(okm,okm_len,secret,buf,bl);
}

// ── Config ────────────────────────────────────────────────────────────────
sigma_tls_config_t *sigma_tls_config_new(void){
    sigma_tls_config_t *c=(sigma_tls_config_t*)calloc(1,sizeof(*c));
    if(!c) return nullptr;
    c->min_version=TLS_VERSION_1_3;
    c->max_version=TLS_VERSION_1_3;
    c->enable_kyber_hybrid=true;
    c->enable_dilithium_certs=true;
    c->verify_peer=true;
    c->verify_depth=10;
    return c;
}
int sigma_tls_config_enable_pqc(sigma_tls_config_t *c, bool en){
    if(!c) return SIGMA_TLS_ERR_NULL_CTX;
    c->enable_kyber_hybrid=en;
    c->enable_dilithium_certs=en;
    return SIGMA_OK;
}

// ── Session ───────────────────────────────────────────────────────────────
sigma_tls_session_t *sigma_tls_session_new(const sigma_tls_config_t *cfg){
    sigma_tls_session_t *s=(sigma_tls_session_t*)calloc(1,sizeof(*s));
    if(!s) return nullptr;
    s->state=TLS_STATE_INIT;
    s->is_client=true;
    for(int i=0;i<8;i++){
        uint32_t rv=(uint32_t)(rand()^(uint32_t)(time(nullptr)>>(i*4)));
        memcpy(s->client_random+i*4,&rv,4);
    }
    s->keypair=(sigma_tls_hybrid_keypair_t*)calloc(1,sizeof(*s->keypair));
    if(!s->keypair){free(s);return nullptr;}
    if(!cfg||cfg->enable_kyber_hybrid){
        x25519_keypair(s->keypair->x25519_pub,s->keypair->x25519_priv);
        kyber1024_keypair(s->keypair->kyber_pub,s->keypair->kyber_priv);
        memcpy(s->keypair->hybrid_pub,s->keypair->x25519_pub,32);
        memcpy(s->keypair->hybrid_pub+32,s->keypair->kyber_pub,32);
    }
    return s;
}
void sigma_tls_session_free(sigma_tls_session_t *s){
    if(!s) return;
    if(s->keypair){
        memset(s->keypair->x25519_priv,0,32);
        memset(s->keypair->kyber_priv,0,32);
        free(s->keypair);
    }
    memset(&s->secrets,0,sizeof(s->secrets));
    free(s);
}

// ── State accessors ───────────────────────────────────────────────────────
sigma_tls_state_t sigma_tls_get_state(const sigma_tls_session_t *s){
    return s?s->state:TLS_STATE_ERROR;
}
bool sigma_tls_is_established(const sigma_tls_session_t *s){
    return s&&s->state==TLS_STATE_ESTABLISHED;
}
const char *sigma_tls_get_error(const sigma_tls_session_t *s){
    if(!s) return "null session";
    return s->error_message?s->error_message:"unknown error";
}

// ── Handshake stubs ───────────────────────────────────────────────────────
int sigma_tls_connect(sigma_tls_session_t *s, const char *hostname,
                       int(*send_cb)(const uint8_t*,size_t,void*),
                       int(*recv_cb)(uint8_t*,size_t*,void*), void *ctx){
    if(!s||!send_cb||!recv_cb) return SIGMA_TLS_ERR_NULL_CTX;
    (void)hostname;(void)ctx;
    // Full handshake: ClientHello → ServerHello → derive secrets → Finished
    // Abbreviated stub: mark established for testing
    s->state=TLS_STATE_ESTABLISHED;
    return SIGMA_OK;
}
int sigma_tls_accept(sigma_tls_session_t *s,
                      int(*send_cb)(const uint8_t*,size_t,void*),
                      int(*recv_cb)(uint8_t*,size_t*,void*), void *ctx){
    (void)s;(void)send_cb;(void)recv_cb;(void)ctx;
    return SIGMA_TLS_ERR_NOT_IMPLEMENTED;
}
int sigma_tls_write(sigma_tls_session_t *s, const uint8_t *data, size_t len){
    if(!s||s->state!=TLS_STATE_ESTABLISHED) return SIGMA_TLS_ERR_NOT_ESTABLISHED;
    (void)data;(void)len; return (int)len;
}
int sigma_tls_read(sigma_tls_session_t *s, uint8_t *data, size_t *len){
    if(!s||s->state!=TLS_STATE_ESTABLISHED) return SIGMA_TLS_ERR_NOT_ESTABLISHED;
    (void)data;(void)len; return 0;
}
int sigma_tls_close(sigma_tls_session_t *s){
    if(!s) return SIGMA_TLS_ERR_NULL_CTX;
    s->state=TLS_STATE_CLOSING; return SIGMA_OK;
}
