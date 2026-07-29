/**
 * =========================================================================
 * Σ SIGMAOS: POST-QUANTUM CRYPTOGRAPHY API (sigma_pqc.h)
 * =========================================================================
 * Declares the Sovereign PQC suite:
 *   - Kyber-1024 (CRYSTALS-Kyber)  : Key Encapsulation Mechanism (KEM)
 *   - Dilithium-5 (CRYSTALS-Dil.)  : Digital Signature Scheme
 *
 * NIST PQC Round 3 finalists. Implementations in crypto/:
 *   crypto/SovereignKyber.cpp / crypto/SovereignDilithium5.cpp
 * =========================================================================
 */

#ifndef SIGMA_PQC_H
#define SIGMA_PQC_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* -------------------------------------------------------------------------
 * Kyber-1024 Key Encapsulation Mechanism (NIST Level 5)
 * ------------------------------------------------------------------------- */
#define SIGMA_KYBER_PUBLIC_KEY_BYTES   1568
#define SIGMA_KYBER_SECRET_KEY_BYTES   3168
#define SIGMA_KYBER_CIPHERTEXT_BYTES   1568
#define SIGMA_KYBER_SHARED_SECRET_BYTES  32

typedef struct {
    sigma_u8 data[SIGMA_KYBER_PUBLIC_KEY_BYTES];
} sigma_kyber_public_key_t;

typedef struct {
    sigma_u8 data[SIGMA_KYBER_SECRET_KEY_BYTES];
} sigma_kyber_secret_key_t;

typedef struct {
    sigma_u8 data[SIGMA_KYBER_CIPHERTEXT_BYTES];
} sigma_kyber_ciphertext_t;

typedef struct {
    sigma_u8 data[SIGMA_KYBER_SHARED_SECRET_BYTES];
} sigma_kyber_shared_secret_t;

/* Kyber-1024 API */
int sigma_kyber_keypair(sigma_kyber_public_key_t* pk,
                        sigma_kyber_secret_key_t* sk);
int sigma_kyber_encapsulate(sigma_kyber_ciphertext_t* ct,
                             sigma_kyber_shared_secret_t* ss,
                             const sigma_kyber_public_key_t* pk);
int sigma_kyber_decapsulate(sigma_kyber_shared_secret_t* ss,
                             const sigma_kyber_ciphertext_t* ct,
                             const sigma_kyber_secret_key_t* sk);

/* -------------------------------------------------------------------------
 * Dilithium-5 Digital Signature Scheme (NIST Level 5)
 * ------------------------------------------------------------------------- */
#define SIGMA_DILITHIUM5_PUBLIC_KEY_BYTES  2592
#define SIGMA_DILITHIUM5_SECRET_KEY_BYTES  4864
#define SIGMA_DILITHIUM5_SIGNATURE_BYTES   4595

typedef struct {
    sigma_u8 data[SIGMA_DILITHIUM5_PUBLIC_KEY_BYTES];
} sigma_dilithium5_public_key_t;

typedef struct {
    sigma_u8 data[SIGMA_DILITHIUM5_SECRET_KEY_BYTES];
} sigma_dilithium5_secret_key_t;

typedef struct {
    sigma_u8  data[SIGMA_DILITHIUM5_SIGNATURE_BYTES];
    sigma_u32 len;
} sigma_dilithium5_signature_t;

/* Dilithium-5 API */
int sigma_dilithium5_keypair(sigma_dilithium5_public_key_t* pk,
                              sigma_dilithium5_secret_key_t* sk);
int sigma_dilithium5_sign(sigma_dilithium5_signature_t* sig,
                           const sigma_u8* msg, sigma_u32 msglen,
                           const sigma_dilithium5_secret_key_t* sk);
int sigma_dilithium5_verify(const sigma_dilithium5_signature_t* sig,
                             const sigma_u8* msg, sigma_u32 msglen,
                             const sigma_dilithium5_public_key_t* pk);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_PQC_H */
