// SPDX-License-Identifier: GPL-2.0-or-later
#ifndef SIGMA_WPA3_H
#define SIGMA_WPA3_H
/*
 * sigma_wpa3.h — WPA3/SAE Authentication
 *
 * References:
 *   IEEE 802.11-2020: WPA3-Personal (SAE)
 *   RFC 7664:         Simultaneous Authentication of Equals (SAE)
 *   IEEE 802.11az:    WPA3-Enterprise Suite B
 */
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <time.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ── Authentication Algorithms ───────────────────────────────────────────── */
#define WPA_AUTH_ALG_OPEN            0x0001
#define WPA_AUTH_ALG_SHARED          0x0002
#define WPA_AUTH_ALG_LEAP            0x0004
#define WPA_AUTH_ALG_SAE             0x0008  /* WPA3-Personal */

/* ── Key Management ──────────────────────────────────────────────────────── */
#define WPA_KEY_MGMT_NONE            0x00000001
#define WPA_KEY_MGMT_WPA_PSK         0x00000002
#define WPA_KEY_MGMT_WPA_NONE        0x00000004
#define WPA_KEY_MGMT_IEEE8021X       0x00000008
#define WPA_KEY_MGMT_WPA_PSK_SHA256  0x00000020
#define WPA_KEY_MGMT_WPA_EAP_SHA256  0x00000040
#define WPA_KEY_MGMT_SAE             0x00000200  /* WPA3-Personal */
#define WPA_KEY_MGMT_OWE             0x00000800  /* Opportunistic Wireless Encryption */
#define WPA_KEY_MGMT_IEEE8021X_SHA384 0x00004000 /* WPA3-Enterprise Suite B */

/* ── Cipher Suites ───────────────────────────────────────────────────────── */
#define WPA_CIPHER_NONE              0x00000001
#define WPA_CIPHER_WEP40             0x00000002
#define WPA_CIPHER_WEP104            0x00000004
#define WPA_CIPHER_TKIP              0x00000008
#define WPA_CIPHER_CCMP              0x00000010
#define WPA_CIPHER_GCMP              0x00000020
#define WPA_CIPHER_CCMP_256          0x00000040
#define WPA_CIPHER_GCMP_256          0x00000080
#define WPA_CIPHER_GTK_NOT_CONFIGURED 0x80000000

/* ── AKM Suites (IEEE 802.11 OUI 00:0F:AC) ──────────────────────────────── */
#define WPA_AKM_8021X                0x000FAC01
#define WPA_AKM_PSK                  0x000FAC02
#define WPA_AKM_FT_8021X             0x000FAC03
#define WPA_AKM_FT_PSK               0x000FAC04
#define WPA_AKM_WPA_PSK_SHA256       0x000FAC06
#define WPA_AKM_SAE                  0x000FAC08  /* WPA3-Personal */
#define WPA_AKM_FT_SAE               0x000FAC09
#define WPA_AKM_OWE                  0x000FAC12
#define WPA_AKM_SUITE_B_192          0x000FAC13  /* WPA3-Enterprise 192-bit */

/* ── SAE Finite Cyclic Groups (IANA group IDs) ───────────────────────────── */
#define SAE_GROUP_SECP256R1          19   /* NIST P-256 (default, mandatory) */
#define SAE_GROUP_SECP384R1          20   /* NIST P-384 */
#define SAE_GROUP_SECP521R1          21   /* NIST P-521 */
#define SAE_GROUP_FF_2048            1    /* 2048-bit MODP */
#define SAE_GROUP_FF_3072            2    /* 3072-bit MODP */
#define SAE_GROUP_FF_4096            5    /* 4096-bit MODP */
#define SAE_GROUP_FF_6144            16   /* 6144-bit MODP */
#define SAE_GROUP_FF_8192            17   /* 8192-bit MODP */

/* ── SAE Status Codes ────────────────────────────────────────────────────── */
#define SAE_STATUS_SUCCESS           0
#define SAE_STATUS_HASH              1   /* Hash-to-element failure */
#define SAE_STATUS_UNSET             2   /* Group not set */
#define SAE_STATUS_REJECTED          3   /* SAE rejected by peer */
#define SAE_STATUS_COMMIT_RECEIVED   4   /* Replay / unexpected commit */
#define SAE_STATUS_NOT_SUPPORTED     5   /* Unsupported group */
#define SAE_STATUS_INVALID_ELEMENT   6   /* Element fails validation */
#define SAE_STATUS_BAD_PASSWORD      7   /* Confirm mismatch (wrong pwd) */
#define SAE_STATUS_INVALID_CONFIG    8   /* Configuration error */

/* ── EAPOL Frame Types ───────────────────────────────────────────────────── */
#define IEEE802_1X_TYPE_EAP_PACKET   0
#define IEEE802_1X_TYPE_EAPOL_START  1
#define IEEE802_1X_TYPE_EAPOL_LOGOFF 2
#define IEEE802_1X_TYPE_EAPOL_KEY    3
#define IEEE802_1X_TYPE_EAPOL_ENCAPS_ASF 5

/* ── EAPOL Key Types ─────────────────────────────────────────────────────── */
#define EAPOL_KEY_TYPE_RC4           254  /* WEP (legacy) */
#define EAPOL_KEY_TYPE_RSN           2    /* WPA2/WPA3 RSN */

/* ── EAPOL Key Info Flags ────────────────────────────────────────────────── */
#define WPA_KEY_INFO_KEY_TYPE        0x0008  /* 1=Pairwise, 0=Group */
#define WPA_KEY_INFO_INSTALL         0x0040
#define WPA_KEY_INFO_ACK             0x0080
#define WPA_KEY_INFO_MIC             0x0100
#define WPA_KEY_INFO_SECURE          0x0200
#define WPA_KEY_INFO_ERROR           0x0400
#define WPA_KEY_INFO_REQUEST         0x0800
#define WPA_KEY_INFO_ENCR_KEY_DATA   0x1000
#define WPA_KEY_INFO_SMK_MESSAGE     0x2000

/* ── Key Descriptor Versions ─────────────────────────────────────────────── */
#define WPA_KEY_DESC_V1              1   /* WPA  — HMAC-MD5 + RC4 */
#define WPA_KEY_DESC_V2              2   /* WPA2 — HMAC-SHA1-128 + AES-128 */
#define WPA_KEY_DESC_V3              3   /* WPA3-SAE — AES-SIV */

/* ── Timeout defaults ────────────────────────────────────────────────────── */
#define WPA3_AUTH_TIMEOUT_MS         5000
#define SAE_COMMIT_TIMEOUT_MS        2000
#define SAE_CONFIRM_TIMEOUT_MS       2000

/* ══════════════════════════════════════════════════════════════════════════ */
/* Data Structures                                                            */
/* ══════════════════════════════════════════════════════════════════════════ */

/* ── SAE Finite Field Element ────────────────────────────────────────────── */
typedef struct {
    uint8_t *data;
    size_t   len;
} sigma_sae_element_t;

/* ── SAE Elliptic Curve Point ────────────────────────────────────────────── */
/* Supports up to P-521 (66 bytes per coordinate) */
typedef struct {
    uint8_t  x[64];    /* x coordinate, big-endian, zero-padded */
    uint8_t  y[64];    /* y coordinate, big-endian, zero-padded */
    uint8_t  x_len;    /* valid bytes in x (32=P-256, 48=P-384, 66=P-521) */
    uint8_t  y_len;
    bool     infinity; /* point at infinity (identity element) */
} sigma_sae_point_t;

/* ── SAE Commit Frame (wire format, packed) ──────────────────────────────── */
struct __attribute__((packed)) sigma_sae_commit_frame {
    uint16_t group;                     /* Finite cyclic group ID */
    uint16_t scalar_len;                /* Length of scalar field */
    uint8_t  scalar[66];                /* Peer scalar (max P-521) */
    uint16_t element_len;               /* Length of element field */
    uint8_t  element[133];              /* Uncompressed EC point (max P-521) */
    uint16_t anti_clogging_token_len;   /* 0 if no token present */
    uint8_t  anti_clogging_token[32];
};

/* ── SAE Confirm Frame (wire format, packed) ─────────────────────────────── */
struct __attribute__((packed)) sigma_sae_confirm_frame {
    uint8_t send_confirm[2];  /* Send-confirm counter, big-endian */
    uint8_t confirm[32];      /* HMAC-SHA256 confirm value */
};

/* ── SAE Authentication State Machine ───────────────────────────────────── */
typedef enum {
    SAE_STATE_NOTHING   = 0,  /* Initial state, no exchange started */
    SAE_STATE_COMMITTED = 1,  /* Commit sent, waiting for peer commit */
    SAE_STATE_CONFIRMED = 2,  /* Confirm sent, waiting for peer confirm */
    SAE_STATE_ACCEPTED  = 3,  /* Authentication successful, PMK derived */
    SAE_STATE_FAILED    = 4,  /* Authentication failed (see error_code) */
} sigma_sae_state_t;

/* ── SAE Session State ───────────────────────────────────────────────────── */
typedef struct {
    sigma_sae_state_t  state;
    uint16_t           group;
    bool               is_ap;            /* true = AP role, false = STA */

    /* Passphrase + network identity */
    uint8_t  password[64];
    size_t   password_len;
    uint8_t  ssid[32];
    uint8_t  ssid_len;

    /* Local commit values */
    uint8_t            local_scalar[66];
    size_t             local_scalar_len;
    sigma_sae_point_t  local_element;
    uint8_t            local_rand[32];   /* Secret random (never transmitted) */

    /* Peer commit values (received) */
    uint8_t            peer_scalar[66];
    size_t             peer_scalar_len;
    sigma_sae_point_t  peer_element;

    /* Anti-clogging token (optional) */
    uint8_t  token[32];
    size_t   token_len;

    /* Computed shared values */
    uint8_t  k[32];         /* Shared secret from EC multiplication */
    uint8_t  keyseed[32];   /* HKDF seed derived from k */
    uint8_t  kck[32];       /* Key Confirmation Key */
    uint8_t  pmk[32];       /* Pairwise Master Key (output) */
    uint8_t  pmkid[16];     /* PMK identifier */

    /* Confirm exchange */
    uint8_t  local_confirm[32];
    uint8_t  peer_confirm[32];
    uint16_t send_confirm;   /* Monotonic counter, prevents replay */

    /* MAC addresses */
    uint8_t  local_addr[6];
    uint8_t  peer_addr[6];

    /* Timing + retries */
    time_t   last_rx_time;
    time_t   last_tx_time;
    uint8_t  retries;

    /* Error state */
    int         error_code;
    const char *error_msg;
} sigma_sae_session_t;

/* ── EAPOL Key Frame (wire format, packed) ───────────────────────────────── */
struct __attribute__((packed)) sigma_eapol_key {
    uint8_t  protocol_version;   /* 0x01=WPA, 0x02=WPA2/WPA3 */
    uint8_t  packet_type;        /* 0x03 = EAPOL-Key */
    uint16_t packet_length;
    uint8_t  descriptor_type;    /* 0x02 = RSN (WPA2/WPA3) */
    uint16_t key_info;           /* see WPA_KEY_INFO_* flags */
    uint16_t key_length;         /* PTK/GTK key length in bytes */
    uint64_t replay_counter;     /* Monotonic, prevents replay attacks */
    uint8_t  key_nonce[32];      /* ANonce (AP) or SNonce (STA) */
    uint8_t  key_iv[16];         /* Used in WPA1; zero in WPA2/WPA3 */
    uint8_t  key_rsc[8];         /* Receive Sequence Counter */
    uint8_t  key_id[8];          /* Reserved/Key ID */
    uint8_t  key_mic[16];        /* MIC over entire EAPOL-Key frame */
    uint16_t key_data_length;
    uint8_t  key_data[0];        /* Variable: GTK, PMKID, RSN IE, etc. */
};

/* ── WPA3-Enterprise (Suite B) Config ───────────────────────────────────── */
typedef struct {
    uint8_t *radius_server_cert;
    size_t   radius_server_cert_len;
    uint8_t *radius_client_cert;
    size_t   radius_client_cert_len;
    uint8_t *radius_client_key;
    size_t   radius_client_key_len;
    uint8_t *ca_cert;
    size_t   ca_cert_len;
    char     eap_method[32];       /* "EAP-TLS", "EAP-TTLS", "EAP-PEAP" */
    bool     suite_b_192;          /* true = 192-bit Suite B */
    char     radius_server[64];
    uint16_t radius_port;
    uint8_t  radius_secret[64];
    size_t   radius_secret_len;
} sigma_wpa3_enterprise_config_t;

/* ── WPA3 Configuration ──────────────────────────────────────────────────── */
typedef struct {
    uint8_t ssid[32];
    uint8_t ssid_len;
    uint8_t bssid[6];              /* Target AP MAC; zero = any */

    /* WPA3-Personal (SAE) */
    uint8_t  sae_password[64];
    size_t   sae_password_len;
    uint16_t sae_group;            /* SAE_GROUP_SECP256R1 recommended */

    /* WPA3-Enterprise */
    sigma_wpa3_enterprise_config_t enterprise;

    /* Mode flags */
    bool     wpa3_only;            /* false = WPA3/WPA2 transition mode */
    bool     owe_only;             /* true = OWE (open, encrypted) */

    /* Key management + cipher selection */
    uint32_t key_mgmt;             /* WPA_KEY_MGMT_SAE | WPA_KEY_MGMT_OWE */
    uint32_t pairwise_cipher;      /* WPA_CIPHER_CCMP or WPA_CIPHER_GCMP_256 */
    uint32_t group_cipher;

    /* Protected Management Frames (mandatory for WPA3) */
    bool     pmf_required;         /* MFP required (WPA3 = true) */
    bool     pmf_capable;          /* MFP capable */

    /* Timeouts */
    uint32_t auth_timeout_ms;
    uint32_t sae_timeout_ms;

    /* Event callbacks */
    void (*on_authenticated)(const uint8_t *pmk, void *ctx);
    void (*on_failure)(int error, const char *msg, void *ctx);
    void *cb_ctx;
} sigma_wpa3_config_t;

/* ── WPA3 Station Context ────────────────────────────────────────────────── */
typedef struct {
    sigma_wpa3_config_t config;

    /* SAE session state */
    sigma_sae_session_t sae;

    /* EAPOL 4-way handshake state */
    uint8_t  anonce[32];           /* AP nonce (from Message 1) */
    uint8_t  snonce[32];           /* STA nonce (sent in Message 2) */
    uint64_t replay_counter;

    /* Derived keying material */
    uint8_t ptk[64];               /* PTK = KCK|KEK|TK (48 or 64 bytes) */
    uint8_t ptk_kek[16];           /* Key Encryption Key (wrap GTK) */
    uint8_t ptk_kck[16];           /* Key Confirmation Key (MIC) */
    uint8_t gtk[32];               /* Group Temporal Key */

    /* Authentication state */
    bool authenticated;
    bool keys_installed;

    /* Statistics */
    uint64_t tx_frames;
    uint64_t rx_frames;
    uint64_t auth_attempts;
    uint64_t auth_failures;
} sigma_wpa3_sta_t;

/* ══════════════════════════════════════════════════════════════════════════ */
/* Public API                                                                 */
/* ══════════════════════════════════════════════════════════════════════════ */

/* ── STA lifecycle ───────────────────────────────────────────────────────── */

/** Allocate and initialise a WPA3 STA context from config. */
sigma_wpa3_sta_t *sigma_wpa3_sta_new(const sigma_wpa3_config_t *config);

/** Securely erase keys and free a STA context. */
void sigma_wpa3_sta_free(sigma_wpa3_sta_t *sta);

/* ── SAE protocol ────────────────────────────────────────────────────────── */

/** Begin SAE authentication: generate commit scalar + element. */
int sigma_wpa3_sae_start(sigma_wpa3_sta_t *sta);

/** Build a SAE Commit frame into *frame (caller provides buffer). */
int sigma_wpa3_sae_build_commit(sigma_wpa3_sta_t *sta,
                                 uint8_t *frame, size_t *len);

/** Process a received SAE Commit frame; advances state to CONFIRMED. */
int sigma_wpa3_sae_process_commit(sigma_wpa3_sta_t *sta,
                                   const uint8_t *frame, size_t len);

/** Build a SAE Confirm frame (must call after process_commit). */
int sigma_wpa3_sae_build_confirm(sigma_wpa3_sta_t *sta,
                                  uint8_t *frame, size_t *len);

/** Process a received SAE Confirm frame; advances state to ACCEPTED. */
int sigma_wpa3_sae_process_confirm(sigma_wpa3_sta_t *sta,
                                    const uint8_t *frame, size_t len);

/* ── EAPOL 4-way handshake ───────────────────────────────────────────────── */

/** Process an EAPOL-Key frame (handles all 4 messages). */
int sigma_wpa3_eapol_process_key(sigma_wpa3_sta_t *sta,
                                  const uint8_t *frame, size_t len);

/**
 * Build an EAPOL-Key frame.
 * key_info:     WPA_KEY_INFO_* bitmask
 * nonce:        ANonce/SNonce (32 bytes)
 * key_data:     encrypted key data (may be NULL)
 * key_data_len: length of key_data
 */
int sigma_wpa3_eapol_build_key(sigma_wpa3_sta_t *sta,
                                uint8_t *frame, size_t *len,
                                uint16_t key_info,
                                const uint8_t *nonce,
                                const uint8_t *key_data,
                                size_t key_data_len);

/* ── State queries ───────────────────────────────────────────────────────── */

bool           sigma_wpa3_is_authenticated(const sigma_wpa3_sta_t *sta);
const uint8_t *sigma_wpa3_get_pmk(const sigma_wpa3_sta_t *sta);
const uint8_t *sigma_wpa3_get_ptk(const sigma_wpa3_sta_t *sta);

/* ── SAE cryptographic primitives ────────────────────────────────────────── */

/**
 * Generate commit scalar + element from passphrase.
 * rand_out: caller-provided 32-byte buffer for the secret random value.
 */
int sigma_sae_generate_commit_element(uint16_t group,
                                       const uint8_t *pwd, size_t pwd_len,
                                       const uint8_t *ssid, size_t ssid_len,
                                       uint8_t *scalar, size_t *scalar_len,
                                       sigma_sae_point_t *element,
                                       uint8_t *rand_out, size_t rand_len);

/**
 * Hunting-and-pecking algorithm (RFC 7664 §3.1.3).
 * Derives a deterministic password element PE from pwd + ssid.
 */
int sigma_sae_password_element(uint16_t group,
                                const uint8_t *pwd, size_t pwd_len,
                                const uint8_t *ssid, size_t ssid_len,
                                sigma_sae_point_t *pe);

/**
 * Compute shared secret k from both parties' commit values.
 * k = (peer_element + PE)^(local_rand * peer_scalar + local_scalar^-1 * local_rand)
 */
int sigma_sae_compute_shared_secret(uint16_t group,
                                     const uint8_t *local_scalar, size_t local_scalar_len,
                                     const sigma_sae_point_t *local_element,
                                     const uint8_t *peer_scalar, size_t peer_scalar_len,
                                     const sigma_sae_point_t *peer_element,
                                     const uint8_t *rand, size_t rand_len,
                                     uint8_t *k, size_t k_len);

/** Validate that a peer-supplied EC point is on the curve and not the identity. */
bool sigma_sae_verify_element(uint16_t group, const sigma_sae_point_t *element);

/**
 * HKDF-based key derivation from shared secret k.
 * Outputs: keyseed (32B), kck (Key Confirmation Key, 32B), pmk (32B).
 */
int sigma_sae_key_derivation(const uint8_t *k, size_t k_len,
                              const uint8_t *local_scalar, size_t local_scalar_len,
                              const uint8_t *peer_scalar, size_t peer_scalar_len,
                              const uint8_t *local_addr,
                              const uint8_t *peer_addr,
                              uint16_t group,
                              uint8_t *keyseed,
                              uint8_t *kck,
                              uint8_t *pmk);

/**
 * Compute the SAE Confirm value: HMAC-SHA256(kck, send_confirm || scalars || elements || addrs).
 */
int sigma_sae_compute_confirm(const uint8_t *kck,
                               uint16_t send_confirm,
                               const uint8_t *local_scalar, size_t local_scalar_len,
                               const uint8_t *peer_scalar, size_t peer_scalar_len,
                               const sigma_sae_point_t *local_element,
                               const sigma_sae_point_t *peer_element,
                               const uint8_t *local_addr,
                               const uint8_t *peer_addr,
                               uint16_t group,
                               uint8_t *confirm);

/* ── EC P-256 operations ─────────────────────────────────────────────────── */

int  ec_p256_add(const sigma_sae_point_t *a, const sigma_sae_point_t *b,
                  sigma_sae_point_t *result);
int  ec_p256_mul(const sigma_sae_point_t *p, const uint8_t *scalar,
                  size_t scalar_len, sigma_sae_point_t *result);
int  ec_p256_inverse(const sigma_sae_point_t *p, sigma_sae_point_t *result);
bool ec_p256_on_curve(const sigma_sae_point_t *p);
int  ec_p256_hash_to_element(const uint8_t *data, size_t data_len,
                               sigma_sae_point_t *p);

/* ── OWE (Opportunistic Wireless Encryption) ─────────────────────────────── */

/** Generate an OWE DH keypair for the given group. pub_key is the public share. */
int sigma_owe_generate_key(uint16_t group, uint8_t *pub_key, size_t *pub_len);

/** Compute the OWE PMK from peer's public key + local private key. */
int sigma_owe_compute_pmk(const uint8_t *peer_pub_key, size_t peer_pub_len,
                           const uint8_t *local_priv_key, size_t priv_len,
                           uint16_t group,
                           uint8_t *pmk, size_t *pmk_len);

/* ── Key derivation helpers ──────────────────────────────────────────────── */

/** Compute PMKID = HMAC-SHA1-128(PMK, "PMK Name" || AA || SPA). */
int sigma_derive_pmkid(const uint8_t *pmk, size_t pmk_len,
                        const uint8_t *ssid, size_t ssid_len,
                        uint8_t *pmkid);

/** Derive PMK from a WPA2-PSK passphrase via PBKDF2-SHA1 (4096 iterations). */
int sigma_derive_pmk_from_psk(const char *psk,
                               const uint8_t *ssid, size_t ssid_len,
                               uint8_t *pmk, size_t *pmk_len);

/** Derive PTK = PRF-X(PMK, "Pairwise key expansion" || min(AA,SPA) || ...). */
int sigma_derive_ptk(const uint8_t *pmk, size_t pmk_len,
                      const uint8_t *anonce, const uint8_t *snonce,
                      const uint8_t *aa, const uint8_t *spa,
                      uint8_t *ptk, size_t *ptk_len);

/** Extract KCK (MIC key) and KEK (encryption key) from PTK. */
int sigma_derive_mic_key(const uint8_t *ptk, size_t ptk_len,
                          uint8_t *kck, uint8_t *kek);

/* ── HMAC primitives ─────────────────────────────────────────────────────── */

int sigma_hmac_sha256(const uint8_t *key, size_t key_len,
                       const uint8_t *data, size_t data_len,
                       uint8_t *mac);
int sigma_hmac_sha384(const uint8_t *key, size_t key_len,
                       const uint8_t *data, size_t data_len,
                       uint8_t *mac);
int sigma_hmac_sha512(const uint8_t *key, size_t key_len,
                       const uint8_t *data, size_t data_len,
                       uint8_t *mac);

/* ── String helpers ──────────────────────────────────────────────────────── */

const char *sigma_sae_state_to_string(sigma_sae_state_t state);
const char *sigma_sae_status_to_string(int status);
const char *sigma_wpa_key_mgmt_to_string(uint32_t key_mgmt);
const char *sigma_wpa_cipher_to_string(uint32_t cipher);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_WPA3_H */
