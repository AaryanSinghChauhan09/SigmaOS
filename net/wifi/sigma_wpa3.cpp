// SPDX-License-Identifier: GPL-2.0-or-later
// net/wifi/sigma_wpa3.cpp — WPA3/SAE authentication implementation
// IEEE 802.11-2020 WPA3-Personal (SAE / Dragonfly key exchange)
#include "net/sigma_wpa3.h"
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#ifdef _WIN32
#  include <winsock2.h>
#else
#  include <arpa/inet.h>
#endif
#ifdef __cplusplus
extern "C" {
#endif

// ── String helpers ────────────────────────────────────────────────────────
const char *sigma_sae_state_to_string(sigma_sae_state_t s){
    static const char *t[]={"NOTHING","COMMITTED","CONFIRMED","ACCEPTED","FAILED"};
    return s<5?t[s]:"UNKNOWN";
}
const char *sigma_wpa_key_mgmt_to_string(uint32_t k){
    if(k&WPA_KEY_MGMT_SAE)           return "SAE";
    if(k&WPA_KEY_MGMT_OWE)           return "OWE";
    if(k&WPA_KEY_MGMT_IEEE8021X_SHA384) return "WPA3-Enterprise";
    if(k&WPA_KEY_MGMT_WPA_PSK)       return "WPA-PSK";
    if(k&WPA_KEY_MGMT_IEEE8021X)     return "WPA-Enterprise";
    return "UNKNOWN";
}
const char *sigma_wpa_cipher_to_string(uint32_t c){
    if(c&WPA_CIPHER_GCMP_256) return "GCMP-256";
    if(c&WPA_CIPHER_CCMP_256) return "CCMP-256";
    if(c&WPA_CIPHER_GCMP)     return "GCMP";
    if(c&WPA_CIPHER_CCMP)     return "CCMP";
    if(c&WPA_CIPHER_TKIP)     return "TKIP";
    return "UNKNOWN";
}
const char *sigma_sae_state_to_string2(int s){ return sigma_sae_state_to_string((sigma_sae_state_t)s); }

// ── HMAC-SHA256 stub ──────────────────────────────────────────────────────
int sigma_hmac_sha256(const uint8_t *key, size_t kl,
                      const uint8_t *data, size_t dl, uint8_t *mac){
    (void)key;(void)kl;(void)data;(void)dl;
    memset(mac,0,32); return 0;
}
int sigma_hmac_sha384(const uint8_t *key, size_t kl,
                      const uint8_t *data, size_t dl, uint8_t *mac){
    (void)key;(void)kl;(void)data;(void)dl;
    memset(mac,0,48); return 0;
}

// ── EC P-256 stubs ────────────────────────────────────────────────────────
bool ec_p256_on_curve(const sigma_sae_point_t *p){ return p&&!p->infinity; }

bool sigma_sae_verify_element(uint16_t group, const sigma_sae_point_t *el){
    if(!el||el->infinity) return false;
    if(group==SAE_GROUP_SECP256R1) return ec_p256_on_curve(el);
    return false;
}

// ── Password element (hunting-and-pecking stub) ───────────────────────────
int sigma_sae_password_element(uint16_t group,
                                const uint8_t *pwd, size_t pwd_len,
                                const uint8_t *ssid, size_t ssid_len,
                                sigma_sae_point_t *pe){
    (void)group;(void)ssid;(void)ssid_len;
    uint8_t seed[32]; sigma_hmac_sha256(ssid,ssid_len,pwd,pwd_len,seed);
    memcpy(pe->x,seed,32); memset(pe->y,0,32);
    pe->x_len=32; pe->y_len=32; pe->infinity=false;
    return 0;
}

// ── WPA3 STA lifecycle ────────────────────────────────────────────────────
sigma_wpa3_sta_t *sigma_wpa3_sta_new(const sigma_wpa3_config_t *cfg){
    sigma_wpa3_sta_t *sta=(sigma_wpa3_sta_t*)calloc(1,sizeof(*sta));
    if(!sta) return nullptr;
    if(cfg) memcpy(&sta->config,cfg,sizeof(*cfg));
    else {
        sta->config.sae_group=SAE_GROUP_SECP256R1;
        sta->config.key_mgmt=WPA_KEY_MGMT_SAE;
        sta->config.pairwise_cipher=WPA_CIPHER_CCMP;
        sta->config.pmf_required=true;
        sta->config.auth_timeout_ms=WPA3_AUTH_TIMEOUT_MS;
    }
    sta->sae.state=SAE_STATE_NOTHING;
    sta->sae.group=sta->config.sae_group;
    // Random locally-administered MAC
    for(int i=0;i<6;i++) sta->sae.local_addr[i]=(uint8_t)(rand()&0xFF);
    sta->sae.local_addr[0]=(sta->sae.local_addr[0]&0xFE)|0x02;
    return sta;
}
void sigma_wpa3_sta_free(sigma_wpa3_sta_t *sta){
    if(!sta) return;
    memset(&sta->sae,0,sizeof(sta->sae));
    memset(sta->ptk,0,sizeof(sta->ptk));
    free(sta);
}

// ── SAE start ─────────────────────────────────────────────────────────────
int sigma_wpa3_sae_start(sigma_wpa3_sta_t *sta){
    if(!sta) return -1;
    sigma_sae_point_t pe;
    int r=sigma_sae_password_element(sta->sae.group,
        sta->config.sae_password, sta->config.sae_password_len,
        sta->config.ssid,          sta->config.ssid_len, &pe);
    if(r<0){ sta->sae.error_code=SAE_STATUS_HASH; return r; }
    memcpy(&sta->sae.local_element,&pe,sizeof(pe));
    memcpy(sta->sae.local_scalar,pe.x,32);
    sta->sae.local_scalar_len=32;
    sta->sae.state=SAE_STATE_COMMITTED;
    return 0;
}

// ── Key derivation stubs ──────────────────────────────────────────────────
int sigma_derive_ptk(const uint8_t *pmk, size_t pmk_len,
                     const uint8_t *anonce, const uint8_t *snonce,
                     const uint8_t *aa, const uint8_t *spa,
                     uint8_t *ptk, size_t *ptk_len){
    (void)pmk;(void)pmk_len;(void)anonce;(void)snonce;(void)aa;(void)spa;
    if(ptk_len) *ptk_len=48;
    memset(ptk,0,48); return 0;
}
int sigma_derive_pmk_from_psk(const char *psk, const uint8_t *ssid,
                               size_t ssid_len, uint8_t *pmk, size_t *pmk_len){
    (void)psk;(void)ssid;(void)ssid_len;
    if(pmk_len) *pmk_len=32; memset(pmk,0,32); return 0;
}
int sigma_derive_pmkid(const uint8_t *pmk, size_t pmk_len,
                       const uint8_t *ssid, size_t ssid_len, uint8_t *pmkid){
    (void)pmk;(void)pmk_len;(void)ssid;(void)ssid_len;
    memset(pmkid,0,16); return 0;
}

// ── OWE stubs ─────────────────────────────────────────────────────────────
int sigma_owe_generate_key(uint16_t group, uint8_t *pub_key, size_t *pub_len){
    (void)group; *pub_len=65; memset(pub_key,0,65); return 0;
}
int sigma_owe_compute_pmk(const uint8_t *peer_pub,size_t peer_pub_len,
                          const uint8_t *local_priv,size_t priv_len,
                          uint16_t group, uint8_t *pmk, size_t *pmk_len){
    (void)peer_pub;(void)peer_pub_len;(void)local_priv;(void)priv_len;(void)group;
    *pmk_len=32; memset(pmk,0,32); return 0;
}

// ── Status helpers ────────────────────────────────────────────────────────
bool sigma_wpa3_is_authenticated(const sigma_wpa3_sta_t *sta){return sta&&sta->authenticated;}
const uint8_t *sigma_wpa3_get_pmk(const sigma_wpa3_sta_t *sta){return sta?sta->sae.pmk:nullptr;}
const uint8_t *sigma_wpa3_get_ptk(const sigma_wpa3_sta_t *sta){return sta?sta->ptk:nullptr;}

#ifdef __cplusplus
}
#endif
