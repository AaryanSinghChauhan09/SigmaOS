/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S11_PQC/shards/sigma_pqc.h
 * =========================================================================
 * Post-Quantum Cryptography Suite — gap-closes:
 *   OpenSSL : RSA/ECC/AES (now quantum-vulnerable)
 *   BoringSSL: CECPQ2 hybrid key exchange
 *   NSS     : NIST PQC Round 3 draft support
 *   wolfSSL : ML-KEM/ML-DSA early integration
 * =========================================================================
 * Implements NIST-selected PQC standards (FIPS 203/204/205):
 *   • ML-KEM   (Module Lattice Key Encapsulation — Kyber)
 *   • ML-DSA   (Module Lattice Digital Signature  — Dilithium)
 *   • SLH-DSA  (Stateless Hash-Based Signature    — SPHINCS+)
 *   • AES-256-GCM (symmetric layer, quantum-safe for 256-bit keys)
 *   • BLAKE3   (hash, used as PRF/KDF backbone)
 * All implementations are pure C11, constant-time where noted.
 * =========================================================================
 */

#ifndef SIGMA_PQC_H
#define SIGMA_PQC_H

typedef unsigned char      pq_u8;
typedef unsigned int       pq_u32;
typedef unsigned long long pq_u64;
typedef signed   int       pq_i32;
typedef unsigned char      pq_bool;
#define PQ_TRUE  ((pq_bool)1)
#define PQ_FALSE ((pq_bool)0)
#define PQ_OK    ((pq_i32) 0)
#define PQ_ERR   ((pq_i32)-1)

/* ── ML-KEM (Kyber-1024) parameters [FIPS 203] ────────────────────────────── */
#define MLKEM_K             4       /* security level: 1=512,2=768,4=1024 */
#define MLKEM_Q             3329    /* prime modulus                       */
#define MLKEM_N             256     /* polynomial degree                   */
#define MLKEM_ETA1          2
#define MLKEM_ETA2          2

#define MLKEM_PK_LEN        1568    /* public key bytes (k=4)              */
#define MLKEM_SK_LEN        3168    /* secret key bytes                    */
#define MLKEM_CT_LEN        1568    /* ciphertext bytes                    */
#define MLKEM_SS_LEN          32    /* shared secret bytes                 */

/* ── ML-DSA (Dilithium5) parameters [FIPS 204] ──────────────────────────── */
#define MLDSA_PK_LEN        2592
#define MLDSA_SK_LEN        4864
#define MLDSA_SIG_LEN       4627

/* ── SLH-DSA (SPHINCS+-SHA2-256f) parameters [FIPS 205] ─────────────────── */
#define SLHDSA_PK_LEN         64
#define SLHDSA_SK_LEN        128
#define SLHDSA_SIG_LEN     49856

/* ── AES-256-GCM parameters ─────────────────────────────────────────────── */
#define AES_KEY_LEN           32
#define AES_IV_LEN            12
#define AES_TAG_LEN           16

/* ── BLAKE3 output ───────────────────────────────────────────────────────── */
#define BLAKE3_OUT_LEN        32

/* ── Key handles ─────────────────────────────────────────────────────────── */
typedef struct { pq_u8 pk[MLKEM_PK_LEN]; pq_u8 sk[MLKEM_SK_LEN]; } sigma_kem_keypair_t;
typedef struct { pq_u8 pk[MLDSA_PK_LEN]; pq_u8 sk[MLDSA_SK_LEN]; } sigma_dsa_keypair_t;
typedef struct { pq_u8 pk[SLHDSA_PK_LEN]; pq_u8 sk[SLHDSA_SK_LEN]; } sigma_hash_keypair_t;

/* ── Public API ─────────────────────────────────────────────────────────── */
void sigma_pqc_init(void);

/* ML-KEM key encapsulation */
pq_i32 sigma_mlkem_keygen(sigma_kem_keypair_t *kp, const pq_u8 *seed32);
pq_i32 sigma_mlkem_encaps(const sigma_kem_keypair_t *kp,
                           pq_u8 ct[MLKEM_CT_LEN],
                           pq_u8 ss[MLKEM_SS_LEN]);
pq_i32 sigma_mlkem_decaps(const sigma_kem_keypair_t *kp,
                           const pq_u8 ct[MLKEM_CT_LEN],
                           pq_u8 ss[MLKEM_SS_LEN]);

/* ML-DSA digital signatures */
pq_i32 sigma_mldsa_keygen(sigma_dsa_keypair_t *kp, const pq_u8 *seed32);
pq_i32 sigma_mldsa_sign(const sigma_dsa_keypair_t *kp,
                         const pq_u8 *msg, pq_u32 msg_len,
                         pq_u8 sig[MLDSA_SIG_LEN]);
pq_i32 sigma_mldsa_verify(const sigma_dsa_keypair_t *kp,
                           const pq_u8 *msg, pq_u32 msg_len,
                           const pq_u8 sig[MLDSA_SIG_LEN]);

/* AES-256-GCM authenticated encryption */
pq_i32 sigma_aes256gcm_encrypt(const pq_u8 key[AES_KEY_LEN],
                                const pq_u8 iv[AES_IV_LEN],
                                const pq_u8 *pt, pq_u32 pt_len,
                                pq_u8 *ct,
                                pq_u8 tag[AES_TAG_LEN]);
pq_i32 sigma_aes256gcm_decrypt(const pq_u8 key[AES_KEY_LEN],
                                const pq_u8 iv[AES_IV_LEN],
                                const pq_u8 *ct, pq_u32 ct_len,
                                const pq_u8 tag[AES_TAG_LEN],
                                pq_u8 *pt);

/* BLAKE3 hash / KDF */
void sigma_blake3(const pq_u8 *input, pq_u64 len,
                  pq_u8 out[BLAKE3_OUT_LEN]);
void sigma_blake3_kdf(const pq_u8 *ikm, pq_u64 ikm_len,
                      const pq_u8 *ctx, pq_u64 ctx_len,
                      pq_u8 *okm, pq_u64 okm_len);

/* Hybrid TLS-style handshake (ECDH + ML-KEM) */
pq_i32 sigma_hybrid_handshake(sigma_kem_keypair_t *local_kp,
                               const pq_u8 *remote_pk,
                               pq_u8 session_key[AES_KEY_LEN]);

void sigma_pqc_selftest(void);

#endif /* SIGMA_PQC_H */
