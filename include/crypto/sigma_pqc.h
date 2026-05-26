/*
 * =============================================================================
 * Σ SIGMAOS: POST-QUANTUM CRYPTOGRAPHY (Dilithium-5)
 * =============================================================================
 * Mission: NIST-standardized lattice-based signatures to secure OmniPkg and
 *          kernel modules against quantum threats.
 * Standard: C11/C++17 — Zero external dependencies.
 * =============================================================================
 */

#ifndef SIGMA_PQC_H
#define SIGMA_PQC_H

#include "../sigma_kernel_types.h"

#define PQC_PUB_KEY_SIZE  2592
#define PQC_SEC_KEY_SIZE  4864
#define PQC_SIG_SIZE      4627

typedef struct {
    sigma_u8 data[PQC_PUB_KEY_SIZE];
} pqc_public_key_t;

typedef struct {
    sigma_u8 data[PQC_SEC_KEY_SIZE];
} pqc_secret_key_t;

typedef struct {
    sigma_u8 data[PQC_SIG_SIZE];
    sigma_usize length;
} pqc_signature_t;

#ifdef __cplusplus
extern "C" {
#endif

void pqc_init(void);
int  pqc_generate_keypair(pqc_public_key_t* pk, pqc_secret_key_t* sk);
int  pqc_sign(const pqc_secret_key_t* sk, const sigma_u8* msg, sigma_usize msg_len, pqc_signature_t* out_sig);
int  pqc_verify(const pqc_public_key_t* pk, const sigma_u8* msg, sigma_usize msg_len, const pqc_signature_t* sig);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_PQC_H */
