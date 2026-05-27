/**
 * @file sigma_secure_boot.h
 * @brief Roadmap Feature #8 — Secure Boot Verifier
 *        Roadmap Feature #19 — Dilithium-5 Signed Package Validation
 *
 * Cryptographic attestation chain from firmware → bootloader → kernel.
 * Uses post-quantum Dilithium-5 signatures for sovereign trust.
 */

#ifndef SIGMA_SECURE_BOOT_H
#define SIGMA_SECURE_BOOT_H

#include "sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

#define SIGMA_SIG_LEN           4627u   /* Dilithium-5 signature bytes */
#define SIGMA_PUBKEY_LEN        2592u   /* Dilithium-5 public key      */
#define SIGMA_HASH_LEN          64u     /* SHA-512 digest              */

typedef enum {
    SIGMA_BOOT_UNTRUSTED = 0,
    SIGMA_BOOT_VERIFIED  = 1,
    SIGMA_BOOT_SEALED    = 2   /* TPM-bound measurement */
} sigma_boot_trust_t;

typedef struct {
    sigma_u8          signature[SIGMA_SIG_LEN];
    sigma_u8          pubkey[SIGMA_PUBKEY_LEN];
    sigma_u8          image_hash[SIGMA_HASH_LEN];
    sigma_u64         image_size;
    sigma_boot_trust_t trust_level;
} sigma_boot_attestation_t;

/* Verify a kernel image against its attestation record */
sigma_status secure_boot_verify(const void* image, sigma_u64 size,
                                 const sigma_boot_attestation_t* att);

/* Verify a signed package (Feature #19) */
sigma_status secure_pkg_verify(const sigma_u8* pkg_data, sigma_u64 pkg_size,
                                const sigma_u8* sig, sigma_u32 sig_len,
                                const sigma_u8* pubkey, sigma_u32 key_len);

/* Seal a measurement into the TPM PCR chain */
sigma_status secure_boot_seal(sigma_u32 pcr_index, const sigma_u8* digest);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SECURE_BOOT_H */
