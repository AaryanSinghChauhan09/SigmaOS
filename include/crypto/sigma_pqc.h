/**
 * =========================================================================
 * Σ SIGMAOS: POST-QUANTUM CRYPTOGRAPHY — TYPE DEFINITIONS
 * =========================================================================
 * NIST PQC standards (FIPS 203 / Kyber, FIPS 204 / Dilithium).
 * Sizes mirror the ML-KEM-1024 and ML-DSA-87 (Dilithium-5) parameter sets.
 * =========================================================================
 */
#ifndef SIGMA_PQC_H
#define SIGMA_PQC_H

#include "../sigma_kernel_types.h"

/* --- Key / signature sizes (ML-KEM-1024 / ML-DSA-87) --- */
#define PQC_PK_SIZE      1568   /* Kyber-1024 public key  */
#define PQC_SK_SIZE      3168   /* Kyber-1024 secret key  */
#define PQC_CT_SIZE      1568   /* Kyber-1024 ciphertext  */
#define PQC_SS_SIZE        32   /* Kyber shared secret    */

#define PQC_DI_PK_SIZE   2592   /* Dilithium-5 public key */
#define PQC_DI_SK_SIZE   4864   /* Dilithium-5 secret key */
#define PQC_SIG_SIZE     4595   /* Dilithium-5 signature  */

#ifdef __cplusplus
extern "C" {
#endif

/* --- Kyber (ML-KEM) types --- */
typedef struct { sigma_u8 data[PQC_PK_SIZE]; }  kyber_public_key_t;
typedef struct { sigma_u8 data[PQC_SK_SIZE]; }  kyber_secret_key_t;
typedef struct { sigma_u8 data[PQC_CT_SIZE]; }  kyber_ciphertext_t;
typedef struct { sigma_u8 data[PQC_SS_SIZE]; }  kyber_shared_secret_t;

/* --- Dilithium (ML-DSA) types --- */
typedef struct { sigma_u8 data[PQC_DI_PK_SIZE]; } pqc_public_key_t;
typedef struct { sigma_u8 data[PQC_DI_SK_SIZE]; } pqc_secret_key_t;
typedef struct { sigma_u8 data[PQC_SIG_SIZE]; sigma_u32 length; } pqc_signature_t;

/* --- Kyber C-API --- */
int kyber_keygen(kyber_public_key_t* pk, kyber_secret_key_t* sk);
int kyber_encapsulate(const kyber_public_key_t* pk, kyber_ciphertext_t* ct, kyber_shared_secret_t* ss);
int kyber_decapsulate(const kyber_secret_key_t* sk, const kyber_ciphertext_t* ct, kyber_shared_secret_t* ss);

/* --- Dilithium C-API --- */
int pqc_generate_keypair(pqc_public_key_t* pk, pqc_secret_key_t* sk);
int pqc_sign(const pqc_secret_key_t* sk, const sigma_u8* msg, sigma_usize len, pqc_signature_t* sig);
int pqc_verify(const pqc_public_key_t* pk, const sigma_u8* msg, sigma_usize len, const pqc_signature_t* sig);
void pqc_init(void);

#ifdef __cplusplus
}
#endif
#endif /* SIGMA_PQC_H */
