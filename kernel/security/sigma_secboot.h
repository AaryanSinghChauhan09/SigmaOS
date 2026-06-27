// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_secboot.h — Secure Boot + TPM 2.0 integration
 *
 * Provides:
 *   1. UEFI Secure Boot chain verification (PK → KEK → db → image sig)
 *   2. TPM2 PCR extend / seal / unseal with policy assertions
 *   3. Measured Boot — every boot stage hashed into PCRs
 *   4. Remote Attestation — quote + verify against known-good PCR values
 *   5. sigma-cryptfs key unsealing (replaces 32-zero-byte stub)
 *
 * Boot chain:
 *   UEFI firmware (PCR 0-3) → sigma-bootloader (PCR 4) →
 *   kernel image (PCR 5) → initrd (PCR 6) → sigma_secboot_init() (PCR 7)
 *
 * All signing uses Dilithium3 (ML-DSA-65).  Kyber-1024 is used only for
 * the TPM2 session key exchange — never for signatures.
 */

#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── PCR index definitions (SigmaOS-reserved) ───────────────────────────── */
#define SIGMA_PCR_FIRMWARE        0   /* UEFI firmware measurements          */
#define SIGMA_PCR_OPTION_ROMS     2   /* Option ROM measurements             */
#define SIGMA_PCR_BOOTLOADER      4   /* sigma-bootloader binary hash        */
#define SIGMA_PCR_KERNEL          5   /* kernel image hash                   */
#define SIGMA_PCR_INITRD          6   /* initrd / early userspace            */
#define SIGMA_PCR_POLICY          7   /* Secure Boot policy + sigma-trustd   */
#define SIGMA_PCR_CMDLINE         8   /* kernel command line                 */
#define SIGMA_PCR_IMA             10  /* IMA runtime measurement list        */
#define SIGMA_PCR_USERLAND        11  /* sigma userland package set hash     */
#define SIGMA_PCR_APP             23  /* per-app extend at launch            */

#define SIGMA_PCR_SHA256_LEN      32

/* ── TPM2 PCR bank ───────────────────────────────────────────────────────── */
typedef struct {
    sigma_u8 sha256[SIGMA_PCR_SHA256_LEN];
} sigma_pcr_value_t;

typedef struct {
    sigma_pcr_value_t pcr[24];   /* PCR 0-23 SHA-256 bank snapshot         */
} sigma_pcr_snapshot_t;

/* ── Secure Boot status ──────────────────────────────────────────────────── */
typedef enum {
    SIGMA_SECBOOT_DISABLED   = 0,
    SIGMA_SECBOOT_SETUP_MODE = 1,   /* no PK enrolled, permissive            */
    SIGMA_SECBOOT_USER_MODE  = 2,   /* PK enrolled, verification active      */
    SIGMA_SECBOOT_AUDIT_MODE = 3,   /* log violations but do not block       */
    SIGMA_SECBOOT_DEPLOYED   = 4,   /* manufacturing lock — no key changes   */
} sigma_secboot_state_t;

/* ── Image verification result ───────────────────────────────────────────── */
typedef enum {
    SIGMA_SIG_OK            = 0,
    SIGMA_SIG_BAD_SIGNATURE = -1,
    SIGMA_SIG_KEY_NOT_FOUND = -2,
    SIGMA_SIG_REVOKED       = -3,
    SIGMA_SIG_HASH_MISMATCH = -4,
    SIGMA_SIG_CERT_EXPIRED  = -5,
} sigma_sig_result_t;

/* ── TPM2 seal/unseal context ────────────────────────────────────────────── */
typedef struct {
    sigma_u32  handle;         /* persistent TPM2 object handle              */
    sigma_u8   pcr_policy[32]; /* SHA-256 of PCR selection + expected values */
    sigma_u8   nonce[16];      /* anti-replay nonce                          */
} sigma_tpm_seal_ctx_t;

/* ── Attestation quote ───────────────────────────────────────────────────── */
typedef struct {
    sigma_pcr_snapshot_t pcrs;
    sigma_u8             quote_sig[4628];  /* Dilithium3 signature (max)     */
    sigma_u32            quote_sig_len;
    sigma_u8             nonce[32];
    sigma_u64            timestamp_ns;
    char                 firmware_version[64];
} sigma_attest_quote_t;

/* ══════════════════════════════════════════════════════════════════════════ */
/* Secure Boot API                                                            */
/* ══════════════════════════════════════════════════════════════════════════ */

/*
 * sigma_secboot_init — read UEFI Secure Boot state and TPM2 PCR bank.
 * Called once early in kernel init.  Panics (halts) if Secure Boot is
 * SETUP_MODE and SIGMA_ENFORCE_SECBOOT=y compile flag is set.
 */
int sigma_secboot_init(sigma_secboot_state_t* out_state);

/*
 * sigma_secboot_verify_image — verify Dilithium3 signature on a binary.
 * sig_db points to the enrolled certificate chain (PEM or DER).
 * Returns SIGMA_SIG_OK or negative error code.
 */
sigma_sig_result_t sigma_secboot_verify_image(const void*   image,
                                               sigma_size_t  image_len,
                                               const sigma_u8* sig,
                                               sigma_size_t  sig_len,
                                               const void*   sig_db,
                                               sigma_size_t  sig_db_len);

/* ══════════════════════════════════════════════════════════════════════════ */
/* TPM 2.0 API                                                                */
/* ══════════════════════════════════════════════════════════════════════════ */

/*
 * sigma_tpm_read_pcr — read current PCR value for a given index.
 */
int sigma_tpm_read_pcr(int pcr_index, sigma_pcr_value_t* out);

/*
 * sigma_tpm_extend_pcr — extend PCR with SHA-256 of data.
 * Records boot measurement event in the TPM event log.
 */
int sigma_tpm_extend_pcr(int pcr_index, const void* data, sigma_size_t len);

/*
 * sigma_tpm_seal — seal a secret under current PCR policy.
 * The secret can only be unsealed when PCR values match the snapshot
 * captured at seal time.
 *
 * secret / secret_len  — plaintext to seal
 * blob / blob_len_out  — output: TPM2B_SENSITIVE blob (allocates memory)
 */
int sigma_tpm_seal(const sigma_tpm_seal_ctx_t* ctx,
                   const void*    secret,      sigma_size_t  secret_len,
                   void**         blob,        sigma_size_t* blob_len_out);

/*
 * sigma_tpm_unseal — unseal a blob if PCRs match the sealed policy.
 * This is the replacement for the 32-zero-byte derive_key() stub in
 * sigma-cryptfs.  sigma-cryptfs now calls this to obtain the real AES key.
 *
 * Returns 0 on success; SIGMA_EPERM if PCR values have changed.
 */
int sigma_tpm_unseal(const sigma_tpm_seal_ctx_t* ctx,
                     const void*    blob,         sigma_size_t blob_len,
                     void*          secret_out,   sigma_size_t* secret_len_out);

/*
 * sigma_tpm_pcr_snapshot — capture all 24 PCR values atomically.
 */
int sigma_tpm_pcr_snapshot(sigma_pcr_snapshot_t* out);

/* ══════════════════════════════════════════════════════════════════════════ */
/* Remote Attestation                                                         */
/* ══════════════════════════════════════════════════════════════════════════ */

/*
 * sigma_attest_generate_quote — generate a Dilithium3-signed PCR quote.
 * nonce must be 32 bytes of random data provided by the verifier.
 * Used by sigma-trustd for mutual attestation between nodes.
 */
int sigma_attest_generate_quote(const sigma_u8*      nonce,
                                 sigma_attest_quote_t* out);

/*
 * sigma_attest_verify_quote — verify a quote received from a remote node.
 * reference_pcrs may be NULL (skip PCR match check; signature only).
 */
int sigma_attest_verify_quote(const sigma_attest_quote_t* quote,
                               const sigma_u8*             nonce,
                               const sigma_pcr_snapshot_t* reference_pcrs,
                               const void*                 pubkey,
                               sigma_size_t                pubkey_len);

/* ── dm-verity integration ───────────────────────────────────────────────── */

/*
 * sigma_secboot_verify_verity_root — confirm that the dm-verity root hash
 * of the active / partition matches the value sealed in TPM PCR 5.
 * Called after unseal; aborts mount if hash mismatches.
 */
int sigma_secboot_verify_verity_root(const char* verity_dev,
                                      const sigma_u8* expected_root_hash);
