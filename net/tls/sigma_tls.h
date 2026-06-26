// SPDX-License-Identifier: GPL-2.0-or-later
#ifndef SIGMA_TLS_H
#define SIGMA_TLS_H

/**
 * SigmaOS TLS 1.3 — RFC 8446 + X25519/Kyber-1024 hybrid key exchange
 * Certificate signatures: Dilithium5 (ML-DSA-87, NIST FIPS 204)
 *
 * NOTE: Kyber-1024 is used here as a KEM (key encapsulation) — NOT for
 * signing. Signatures use Dilithium5. This is the architecturally correct
 * usage of these primitives. See sigma_hypervisor.cpp for the bug where
 * Kyber was incorrectly used for verification (fixed separately).
 */

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ── TLS 1.3 Constants ──────────────────────────────────────────────────────── */
#define TLS_VERSION_1_3              0x0304
#define TLS_AES_128_GCM_SHA256       0x1301
#define TLS_AES_256_GCM_SHA384       0x1302
#define TLS_CHACHA20_POLY1305_SHA256 0x1303
#define TLS_SIG_ECDSA_SECP256R1_SHA256 0x0403
#define TLS_SIG_DILITHIUM5           0xFE00  /* private use: PQ signature   */
#define TLS_GROUP_X25519             0x001D
#define TLS_GROUP_KYBER1024          0xFE00  /* private use: PQ KEM         */
#define TLS_GROUP_X25519_KYBER1024   0xFE01  /* hybrid                      */

/* ── PQ crypto sizes ────────────────────────────────────────────────────────── */
#define KYBER1024_PUBKEY_BYTES       1568
#define KYBER1024_SECKEY_BYTES       3168
#define KYBER1024_CT_BYTES           1568
#define KYBER1024_SS_BYTES           32
#define DILITHIUM5_PUBKEY_BYTES      2592
#define DILITHIUM5_SECKEY_BYTES      4864
#define DILITHIUM5_SIG_BYTES         4595
#define X25519_PUBKEY_BYTES          32
#define X25519_SS_BYTES              32

/* ── State machine ──────────────────────────────────────────────────────────── */
typedef enum {
    TLS_STATE_INIT = 0,
    TLS_STATE_HANDSHAKE_STARTED,
    TLS_STATE_HELLO_SENT,
    TLS_STATE_HELLO_RECEIVED,
    TLS_STATE_KEY_EXCHANGE_DONE,
    TLS_STATE_FINISHED_SENT,
    TLS_STATE_FINISHED_RECEIVED,
    TLS_STATE_ESTABLISHED,
    TLS_STATE_CLOSING,
    TLS_STATE_CLOSED,
    TLS_STATE_ERROR,
} sigma_tls_state_t;

/* ── Hybrid keypair (X25519 + Kyber-1024) ───────────────────────────────────── */
typedef struct {
    uint8_t x25519_pub[X25519_PUBKEY_BYTES];
    uint8_t x25519_priv[X25519_PUBKEY_BYTES];
    uint8_t kyber_pub[KYBER1024_PUBKEY_BYTES];
    uint8_t kyber_priv[KYBER1024_SECKEY_BYTES];
    /* Combined: sent in key_share extension */
    uint8_t hybrid_pub[X25519_PUBKEY_BYTES + KYBER1024_PUBKEY_BYTES];
} sigma_tls_hybrid_keypair_t;

/* ── Per-session keys (TLS 1.3 key schedule) ────────────────────────────────── */
typedef struct {
    uint8_t early_secret[32];
    uint8_t handshake_secret[32];
    uint8_t master_secret[32];
    uint8_t client_hs_traffic[32];
    uint8_t server_hs_traffic[32];
    uint8_t client_app_traffic[32];
    uint8_t server_app_traffic[32];
    uint8_t client_write_key[32];
    uint8_t server_write_key[32];
    uint8_t client_write_iv[12];
    uint8_t server_write_iv[12];
} sigma_tls_secrets_t;

/* ── Certificate ────────────────────────────────────────────────────────────── */
typedef struct {
    uint8_t* der_data;
    size_t   der_len;
    uint8_t  public_key[DILITHIUM5_PUBKEY_BYTES];
    uint8_t  signature[DILITHIUM5_SIG_BYTES];
    bool     is_ca;
    bool     verified;
} sigma_tls_certificate_t;

/* ── Session ─────────────────────────────────────────────────────────────────── */
typedef struct {
    sigma_tls_state_t          state;
    uint32_t                   conn_id;
    bool                       is_client;
    uint16_t                   cipher_suite;
    sigma_tls_hybrid_keypair_t* keypair;
    uint8_t peer_public_key[X25519_PUBKEY_BYTES + KYBER1024_PUBKEY_BYTES];
    uint8_t shared_secret[X25519_SS_BYTES + KYBER1024_SS_BYTES];
    sigma_tls_secrets_t        secrets;
    uint8_t                    transcript_hash[32];
    sigma_tls_certificate_t*   local_cert;
    sigma_tls_certificate_t*   peer_cert;
    uint64_t                   client_seq;
    uint64_t                   server_seq;
    uint8_t                    client_random[32];
    uint8_t                    server_random[32];
    uint16_t                   key_exchange_group;
    char                       alpn_protocol[16];
    const char*                error_message;
} sigma_tls_session_t;

/* ── Config ─────────────────────────────────────────────────────────────────── */
typedef struct {
    uint16_t     min_version;
    uint16_t     max_version;
    uint16_t*    cipher_suites;
    size_t       cipher_suites_count;
    uint16_t*    sig_algorithms;
    size_t       sig_algorithms_count;
    uint16_t*    key_groups;
    size_t       key_groups_count;
    bool         enable_kyber_hybrid;   /* X25519+Kyber-1024 key exchange   */
    bool         enable_dilithium_certs;/* Dilithium5 certificate signatures */
    const char*  cert_path;
    const char*  key_path;
    const char*  ca_path;
    bool         verify_peer;
    int          verify_depth;
    const char** alpn_protocols;
    size_t       alpn_protocols_count;
} sigma_tls_config_t;

/* ── API ─────────────────────────────────────────────────────────────────────── */
sigma_tls_config_t*   sigma_tls_config_new(void);
int                   sigma_tls_config_enable_pqc(sigma_tls_config_t*, bool);
sigma_tls_session_t*  sigma_tls_session_new(const sigma_tls_config_t*);
void                  sigma_tls_session_free(sigma_tls_session_t*);

int  sigma_tls_connect(sigma_tls_session_t*, const char* hostname,
                        int (*send_cb)(const uint8_t*, size_t, void*),
                        int (*recv_cb)(uint8_t*, size_t*, void*), void* ctx);
int  sigma_tls_accept(sigma_tls_session_t*,
                       int (*send_cb)(const uint8_t*, size_t, void*),
                       int (*recv_cb)(uint8_t*, size_t*, void*), void* ctx);
int  sigma_tls_write(sigma_tls_session_t*, const uint8_t*, size_t);
int  sigma_tls_read(sigma_tls_session_t*, uint8_t*, size_t*);
int  sigma_tls_close(sigma_tls_session_t*);

sigma_tls_state_t sigma_tls_get_state(const sigma_tls_session_t*);
const char*       sigma_tls_get_error(const sigma_tls_session_t*);
bool              sigma_tls_is_established(const sigma_tls_session_t*);

/* ── PQ crypto primitives ────────────────────────────────────────────────────── */
int kyber1024_keypair(uint8_t* pub, uint8_t* priv);
int kyber1024_encapsulate(uint8_t* ct, uint8_t* ss, const uint8_t* pub);
int kyber1024_decapsulate(uint8_t* ss, const uint8_t* ct, const uint8_t* priv);

int dilithium5_keypair(uint8_t* pub, uint8_t* priv);
int dilithium5_sign(uint8_t* sig, size_t* sig_len,
                     const uint8_t* msg, size_t msg_len, const uint8_t* priv);
int dilithium5_verify(const uint8_t* sig, size_t sig_len,
                       const uint8_t* msg, size_t msg_len, const uint8_t* pub);

int x25519_keypair(uint8_t* pub, uint8_t* priv);
int x25519_shared(uint8_t* shared, const uint8_t* pub, const uint8_t* priv);

/* ── HKDF (TLS 1.3 key derivation) ──────────────────────────────────────────── */
int sigma_hkdf_extract(uint8_t* prk,
                        const uint8_t* salt, size_t salt_len,
                        const uint8_t* ikm,  size_t ikm_len);
int sigma_hkdf_expand(uint8_t* okm, size_t okm_len,
                       const uint8_t* prk,
                       const uint8_t* info, size_t info_len);
int sigma_hkdf_expand_label(uint8_t* okm, size_t okm_len,
                              const uint8_t* secret,
                              const char* label,
                              const uint8_t* context, size_t context_len);

/* ── AEAD ─────────────────────────────────────────────────────────────────────── */
int sigma_aead_aes_gcm_encrypt(uint8_t* ct, size_t* ct_len,
                                const uint8_t* pt, size_t pt_len,
                                const uint8_t* key,
                                const uint8_t* nonce, size_t nonce_len,
                                const uint8_t* aad, size_t aad_len);
int sigma_aead_aes_gcm_decrypt(uint8_t* pt, size_t* pt_len,
                                const uint8_t* ct, size_t ct_len,
                                const uint8_t* key,
                                const uint8_t* nonce, size_t nonce_len,
                                const uint8_t* aad, size_t aad_len);
int sigma_aead_chacha20_poly1305_encrypt(uint8_t*, size_t*, const uint8_t*, size_t,
                                          const uint8_t*, const uint8_t*,
                                          const uint8_t*, size_t);
int sigma_aead_chacha20_poly1305_decrypt(uint8_t*, size_t*, const uint8_t*, size_t,
                                          const uint8_t*, const uint8_t*,
                                          const uint8_t*, size_t);

#ifdef __cplusplus
}
#endif
#endif /* SIGMA_TLS_H */
