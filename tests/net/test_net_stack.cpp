// SPDX-License-Identifier: GPL-2.0-or-later
// tests/net/test_net_stack.cpp — SigmaOS Network Stack unit tests
// Covers: TLS 1.3 + Kyber hybrid, DNS/DoH/DNSSEC, DHCP, WPA3/SAE
#include "net/sigma_net.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <time.h>

#define TEST(n)    static void test_##n(void)
#define RUN(n)     do { printf("  [TEST] %-45s", #n); test_##n(); printf("PASS\n"); } while(0)
#define EQ(a,b)    assert((a)==(b))
#define NE(a,b)    assert((a)!=(b))
#define NOTNULL(p) assert((p)!=nullptr)
#define ISTRUE(x)  assert(x)
#define ISFALSE(x) assert(!(x))

// ── TLS tests ─────────────────────────────────────────────────────────────
TEST(tls_config_new){
    sigma_tls_config_t *c=sigma_tls_config_new();
    NOTNULL(c);
    EQ(c->min_version,TLS_VERSION_1_3);
    EQ(c->max_version,TLS_VERSION_1_3);
    ISTRUE(c->enable_kyber_hybrid);
    ISTRUE(c->enable_dilithium_certs);
    free(c);
}
TEST(tls_session_new){
    sigma_tls_config_t *c=sigma_tls_config_new();
    sigma_tls_session_t *s=sigma_tls_session_new(c);
    NOTNULL(s); EQ(s->state,TLS_STATE_INIT); ISTRUE(s->is_client);
    sigma_tls_session_free(s); free(c);
}
TEST(tls_pqc_toggle){
    sigma_tls_config_t *c=sigma_tls_config_new();
    EQ(sigma_tls_config_enable_pqc(c,true),SIGMA_OK);
    ISTRUE(c->enable_kyber_hybrid);
    EQ(sigma_tls_config_enable_pqc(c,false),SIGMA_OK);
    ISFALSE(c->enable_kyber_hybrid);
    free(c);
}
TEST(tls_state_transitions){
    sigma_tls_config_t *c=sigma_tls_config_new();
    sigma_tls_session_t *s=sigma_tls_session_new(c);
    EQ(sigma_tls_get_state(s),TLS_STATE_INIT);
    ISFALSE(sigma_tls_is_established(s));
    s->state=TLS_STATE_ESTABLISHED;
    ISTRUE(sigma_tls_is_established(s));
    sigma_tls_session_free(s); free(c);
}

// ── DNS tests ─────────────────────────────────────────────────────────────
TEST(dns_resolver_new){
    sigma_dns_resolver_t *r=sigma_dns_resolver_new();
    NOTNULL(r); ISTRUE(r->initialized);
    EQ(r->config.server_count,4);
    ISTRUE(r->config.dnssec_enabled);
    ISTRUE(r->config.cache_enabled);
    sigma_dns_resolver_free(r);
}
TEST(dns_name_encode){
    uint8_t buf[256]; size_t off=0;
    EQ(sigma_dns_write_name(buf,&off,"www.example.com",sizeof(buf)),0);
    EQ(buf[0],3); EQ(buf[4],7); EQ(buf[12],3); EQ(buf[16],0);
}
TEST(dns_name_decode){
    uint8_t pkt[]={3,'w','w','w',7,'e','x','a','m','p','l','e',3,'c','o','m',0};
    char name[256]; size_t off=0;
    EQ(sigma_dns_parse_name(pkt,sizeof(pkt),&off,name,sizeof(name)),0);
    EQ(strcmp(name,"www.example.com"),0);
}
TEST(dns_type_strings){
    EQ(strcmp(sigma_dns_type_to_string(DNS_TYPE_A),"A"),0);
    EQ(strcmp(sigma_dns_type_to_string(DNS_TYPE_AAAA),"AAAA"),0);
    EQ(strcmp(sigma_dns_type_to_string(DNS_TYPE_MX),"MX"),0);
}
TEST(dns_cache_clear){
    sigma_dns_resolver_t *r=sigma_dns_resolver_new();
    sigma_dns_cache_clear(r);
    EQ(r->cache_count,0);
    sigma_dns_resolver_free(r);
}
TEST(dns_doh_config){
    sigma_dns_config_t cfg={};
    EQ(sigma_dns_config_set_doh(&cfg,"https://cloudflare-dns.com/dns-query",false),0);
    EQ(cfg.transport,DNS_TRANSPORT_HTTPS);
}

// ── DHCP tests ────────────────────────────────────────────────────────────
TEST(dhcp_client_new){
    sigma_dhcp_client_t *c=sigma_dhcp_client_new("eth0");
    NOTNULL(c); ISTRUE(c->initialized);
    EQ(strcmp(c->config.interface,"eth0"),0);
    EQ(c->lease.state,DHCP_LEASE_STATE_INIT);
    sigma_dhcp_client_free(c);
}
TEST(dhcp_msg_type_strings){
    EQ(strcmp(sigma_dhcp_msg_type_to_string(DHCP_MSG_DISCOVER),"DISCOVER"),0);
    EQ(strcmp(sigma_dhcp_msg_type_to_string(DHCP_MSG_OFFER),"OFFER"),0);
    EQ(strcmp(sigma_dhcp_msg_type_to_string(DHCP_MSG_ACK),"ACK"),0);
}
TEST(dhcp_ip_conversion){
    char buf[32];
    sigma_dhcp_ip_to_string(0x0A000001,buf,sizeof(buf));
    EQ(strcmp(buf,"10.0.0.1"),0);
    uint32_t ip=sigma_dhcp_string_to_ip("192.168.1.100");
    EQ(ip,(uint32_t)0xC0A80164);
}
TEST(dhcp_lease_remaining){
    sigma_dhcp_client_t *c=sigma_dhcp_client_new("eth0");
    c->lease.state=DHCP_LEASE_STATE_BOUND;
    c->lease.lease_obtained=time(nullptr);
    c->lease.lease_expires=time(nullptr)+3600;
    int64_t rem=sigma_dhcp_lease_remaining(c);
    ISTRUE(rem>3590 && rem<=3600);
    sigma_dhcp_client_free(c);
}

// ── WPA3 tests ────────────────────────────────────────────────────────────
TEST(wpa3_sta_new){
    sigma_wpa3_config_t cfg={};
    strcpy((char*)cfg.ssid,"TestNetwork");
    cfg.ssid_len=strlen("TestNetwork");
    cfg.sae_group=SAE_GROUP_SECP256R1;
    cfg.key_mgmt=WPA_KEY_MGMT_SAE;
    sigma_wpa3_sta_t *sta=sigma_wpa3_sta_new(&cfg);
    NOTNULL(sta);
    EQ(sta->sae.state,SAE_STATE_NOTHING);
    EQ(sta->sae.group,SAE_GROUP_SECP256R1);
    sigma_wpa3_sta_free(sta);
}
TEST(sae_state_strings){
    EQ(strcmp(sigma_sae_state_to_string(SAE_STATE_NOTHING),"NOTHING"),0);
    EQ(strcmp(sigma_sae_state_to_string(SAE_STATE_COMMITTED),"COMMITTED"),0);
    EQ(strcmp(sigma_sae_state_to_string(SAE_STATE_ACCEPTED),"ACCEPTED"),0);
}
TEST(wpa3_key_mgmt_strings){
    EQ(strcmp(sigma_wpa_key_mgmt_to_string(WPA_KEY_MGMT_SAE),"SAE"),0);
    EQ(strcmp(sigma_wpa_key_mgmt_to_string(WPA_KEY_MGMT_OWE),"OWE"),0);
    EQ(strcmp(sigma_wpa_key_mgmt_to_string(WPA_KEY_MGMT_WPA_PSK),"WPA-PSK"),0);
}
TEST(wpa3_cipher_strings){
    EQ(strcmp(sigma_wpa_cipher_to_string(WPA_CIPHER_CCMP),"CCMP"),0);
    EQ(strcmp(sigma_wpa_cipher_to_string(WPA_CIPHER_GCMP_256),"GCMP-256"),0);
}
TEST(sae_element_verify){
    sigma_sae_point_t pt={};
    pt.x_len=32; pt.y_len=32; pt.infinity=false;
    ISTRUE(sigma_sae_verify_element(SAE_GROUP_SECP256R1,&pt));
    pt.infinity=true;
    ISFALSE(sigma_sae_verify_element(SAE_GROUP_SECP256R1,&pt));
}

// ── Net stack integration tests ───────────────────────────────────────────
TEST(net_stack_init){
    sigma_net_stack_t *s=sigma_net_init();
    NOTNULL(s); ISTRUE(s->initialized);
    sigma_net_shutdown(s);
}
TEST(net_config_default){
    sigma_net_config_t c;
    EQ(sigma_net_config_default(&c),0);
    ISTRUE(c.tls_pqc_enabled);
    ISTRUE(c.dns_doh_enabled);
}
TEST(net_config_secure){
    sigma_net_config_t c;
    EQ(sigma_net_config_secure(&c),0);
    ISTRUE(c.tls_pqc_enabled);
    ISTRUE(c.dns_dnssec_enabled);
    ISTRUE(c.wpa3_enabled);
}
TEST(net_error_strings){
    EQ(strcmp(sigma_net_error_string(SIGMA_NET_OK),"OK"),0);
    NOTNULL(sigma_net_error_string(SIGMA_NET_ERR_DNS_TIMEOUT));
    NOTNULL(sigma_net_error_string(SIGMA_NET_ERR_TLS_HANDSHAKE));
}

// ── Runner ────────────────────────────────────────────────────────────────
int main(void){
    srand(42);
    printf("\n========================================\n");
    printf("  SigmaOS Network Stack Tests\n");
    printf("========================================\n\n");
    printf("[TLS Tests]\n");
    RUN(tls_config_new); RUN(tls_session_new);
    RUN(tls_pqc_toggle); RUN(tls_state_transitions);
    printf("\n[DNS Tests]\n");
    RUN(dns_resolver_new); RUN(dns_name_encode);
    RUN(dns_name_decode); RUN(dns_type_strings);
    RUN(dns_cache_clear); RUN(dns_doh_config);
    printf("\n[DHCP Tests]\n");
    RUN(dhcp_client_new); RUN(dhcp_msg_type_strings);
    RUN(dhcp_ip_conversion); RUN(dhcp_lease_remaining);
    printf("\n[WPA3 Tests]\n");
    RUN(wpa3_sta_new); RUN(sae_state_strings);
    RUN(wpa3_key_mgmt_strings); RUN(wpa3_cipher_strings);
    RUN(sae_element_verify);
    printf("\n[Net Stack Tests]\n");
    RUN(net_stack_init); RUN(net_config_default);
    RUN(net_config_secure); RUN(net_error_strings);
    printf("\n========================================\n");
    printf("  All %d tests passed!\n", 22);
    printf("========================================\n\n");
    return 0;
}
