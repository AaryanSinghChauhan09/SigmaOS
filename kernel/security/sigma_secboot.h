/* SPDX-License-Identifier: GPL-2.0-or-later */
/**
 * sigma_secboot.h — SigmaOS Secure Boot + TPM 2.0 subsystem
 *
 * Inspired by:
 *   • UEFI Secure Boot (EFI_IMAGE_SECURITY_DATABASE policy)
 *   • systemd-boot / shim (chain-of-trust loading)
 *   • TPM 2.0 spec (TCG Part 3: Commands) — PCR extend, seal/unseal
 *   • Google ChromeOS Verified Boot (vboot) — RO firmware validates RW
 *   • OpenBSD signify / Minisign — small signature surface
 *   • Heads project (Trammell Hudson) — measured boot philosophy
 *
 * Chain of trust:
 *
 *   UEFI firmware (OEM key)
 *       └── shim (signed by Microsoft CA)
 *           └── sigma-bootloader (signed by SigmaOS CA — Dilithium3)
 *               └── sigma kernel image (measured into TPM PCR[8])
 *                   └── initramfs + cmdline (measured into TPM PCR[9])
 *                       └── TPM unseals disk encryption key (PCR policy)
 *                           └── dm-verity root hash verified
 *                               └── userland runs
 *
 * All signature operations use Dilithium3 (ML-DSA-65).
 * Kyber-1024 is NOT used here — it is a KEM, not a signature scheme.
 */

#ifndef SIGMA_SECBOOT_H
#define SIGMA_SECBOOT_H

#include <stddef.h>   /* size_t, NULL */
#include <stdint.h>   /* uint8_t, uint32_t, uint64_t */
#include <stdbool.h>  /* bool */

#ifdef __cplusplus
extern "C" {
#endif

/* ── Error codes ─────────────────────────────────────────────────────────── */

typedef enum sigma_secboot_err {
    SIGMA_SECBOOT_OK              = 0,
    SIGMA_SECBOOT_ERR_NO_TPM      = -1,   /* TPM 2.0 device not present         */
    SIGMA_SECBOOT_ERR_TPM_IO      = -2,   /* TPM command failed                 */
    SIGMA_SECBOOT_ERR_NO_KEY      = -3,   /* no signing key in keystore         */
    SIGMA_SECBOOT_ERR_BAD_SIG     = -4,   /* signature verification failed      */
    SIGMA_SECBOOT_ERR_PCR_POLICY  = -5,   /* PCR values don't match seal policy */
    SIGMA_SECBOOT_ERR_REVOKED     = -6,   /* key is in the revocation list      */
    SIGMA_SECBOOT_ERR_HASH_MISMATCH = -7, /* measured hash does not match image */
    SIGMA_SECBOOT_ERR_LOCKED      = -8,   /* NV index is locked                 */
    SIGMA_SECBOOT_ERR_OVERFLOW    = -9,   /* buffer too small                   */
} sigma_secboot_err_t;

/* ── Signature constants (Dilithium3 / ML-DSA-65) ────────────────────────── */

#define SIGMA_DILITHIUM3_PK_BYTES   1952u
#define SIGMA_DILITHIUM3_SK_BYTES   4000u
#define SIGMA_DILITHIUM3_SIG_BYTES  3293u

/* SHA-256 digest size */
#define SIGMA_HASH_BYTES            32u

/* ── TPM PCR definitions ─────────────────────────────────────────────────── */

/* SigmaOS-allocated PCR indices (bank: SHA-256) */
#define SIGMA_PCR_FIRMWARE     0u   /* UEFI firmware measurements (standard)  */
#define SIGMA_PCR_BOOTLOADER   4u   /* sigma-bootloader measurement           */
#define SIGMA_PCR_KERNEL       8u   /* kernel image SHA-256                   */
#define SIGMA_PCR_INITRAMFS    9u   /* initramfs + kernel cmdline             */
#define SIGMA_PCR_CONFIG      10u   /* /sigma/boot/config.json                */
#define SIGMA_PCR_DMVERITY    11u   /* dm-verity root hash                    */
#define SIGMA_PCR_SYSROOT     12u   /* / mount point hash (post-pivot)        */

#define SIGMA_PCR_COUNT        24u

/* ── Image header ────────────────────────────────────────────────────────── */

/**
 * sigma_secboot_image_header — prepended to every signed binary.
 * Parsed by sigma-bootloader before handing off to the kernel.
 */
typedef struct __attribute__((packed)) sigma_secboot_image_header {
    uint8_t  magic[8];        /* "SIGMAIMG"                                   */
    uint32_t version;         /* header format version — must be 1            */
    uint32_t flags;           /* see SIGMA_IMGFLAG_* below                    */
    uint64_t image_size;      /* size of payload (after this header)          */
    uint8_t  sha256[SIGMA_HASH_BYTES];      /* SHA-256 of payload             */
    uint8_t  signature[SIGMA_DILITHIUM3_SIG_BYTES]; /* Dilithium3 signature   */
    uint8_t  signer_pubkey[SIGMA_DILITHIUM3_PK_BYTES]; /* signer public key   */
    uint64_t build_timestamp; /* SOURCE_DATE_EPOCH — for revocation check     */
    uint8_t  reserved[64];    /* pad to future fields                         */
} sigma_secboot_image_header_t;

#define SIGMA_IMG_MAGIC  "SIGMAIMG"

/* Image flags */
#define SIGMA_IMGFLAG_KERNEL      (1u << 0)   /* this is a kernel image       */
#define SIGMA_IMGFLAG_INITRAMFS   (1u << 1)   /* initramfs blob               */
#define SIGMA_IMGFLAG_MODULE      (1u << 2)   /* loadable kernel module       */
#define SIGMA_IMGFLAG_DRIVER      (1u << 3)   /* driver binary                */
#define SIGMA_IMGFLAG_DEBUG       (1u << 31)  /* debug build — warn loudly    */

/* ── Trust anchor ────────────────────────────────────────────────────────── */

/**
 * sigma_trust_anchor — a Dilithium3 public key that is trusted to sign images.
 * Multiple anchors can be registered (primary + recovery).
 */
typedef struct sigma_trust_anchor {
    uint8_t  pubkey[SIGMA_DILITHIUM3_PK_BYTES];
    uint8_t  key_id[8];     /* first 8 bytes of SHA-256(pubkey) — for logging */
    bool     revoked;
} sigma_trust_anchor_t;

/* Maximum trust anchors that can be registered at once */
#define SIGMA_MAX_TRUST_ANCHORS   4u

/* ── TPM context (opaque to callers) ─────────────────────────────────────── */

typedef struct sigma_tpm_ctx sigma_tpm_ctx_t;

/* ── API ─────────────────────────────────────────────────────────────────── */

/**
 * sigma_secboot_init — initialise the secure boot subsystem.
 *
 * Detects the TPM 2.0 device, performs a TPM_CC_SelfTest, and reads
 * PCR[0..3] to verify expected firmware measurements.
 *
 * @return SIGMA_SECBOOT_OK on success, or an error code.
 */
sigma_secboot_err_t sigma_secboot_init(void);

/**
 * sigma_secboot_tpm_measure — extend a PCR with a SHA-256 hash.
 *
 * Implements TPM2_PCR_Extend.  The hash is computed over @data[@len].
 *
 * @param pcr    PCR index (0–23).
 * @param data   pointer to data to measure.
 * @param len    length in bytes.
 */
sigma_secboot_err_t sigma_secboot_tpm_measure(uint32_t pcr,
                                               const void *data,
                                               size_t len);

/**
 * sigma_secboot_tpm_read_pcr — read the current value of a PCR.
 *
 * @param pcr    PCR index.
 * @param out    output buffer — must be SIGMA_HASH_BYTES wide.
 */
sigma_secboot_err_t sigma_secboot_tpm_read_pcr(uint32_t pcr,
                                                uint8_t out[SIGMA_HASH_BYTES]);

/**
 * sigma_secboot_tpm_seal — seal a secret blob to a PCR policy.
 *
 * Uses TPM2_Create with a PolicyPCR authorisation that matches
 * the current values of @pcrs[@pcr_count].
 *
 * @param secret     plaintext secret to seal (e.g. dm-crypt key).
 * @param secret_len length of secret (max 256 bytes for NV storage).
 * @param pcrs       array of PCR indices to include in the policy.
 * @param pcr_count  number of PCRs.
 * @param sealed_out pre-allocated buffer for the sealed blob.
 * @param sealed_len in/out: capacity on entry, bytes written on exit.
 */
sigma_secboot_err_t sigma_secboot_tpm_seal(const uint8_t *secret,
                                            size_t         secret_len,
                                            const uint32_t *pcrs,
                                            size_t          pcr_count,
                                            uint8_t        *sealed_out,
                                            size_t         *sealed_len);

/**
 * sigma_secboot_tpm_unseal — unseal a blob, verifying the PCR policy.
 *
 * Fails with SIGMA_SECBOOT_ERR_PCR_POLICY if any PCR value has changed
 * since sealing (e.g. if the kernel image was swapped).
 *
 * @param sealed     the sealed blob.
 * @param sealed_len length of sealed blob.
 * @param secret_out buffer for recovered secret.
 * @param secret_len in/out: capacity on entry, bytes written on exit.
 */
sigma_secboot_err_t sigma_secboot_tpm_unseal(const uint8_t *sealed,
                                              size_t         sealed_len,
                                              uint8_t       *secret_out,
                                              size_t        *secret_len);

/**
 * sigma_secboot_verify_image — verify a Dilithium3 signature on an image.
 *
 * Checks:
 *   1. Magic bytes match "SIGMAIMG".
 *   2. SHA-256 of payload matches header hash.
 *   3. Dilithium3 signature over (hash || flags || timestamp) is valid.
 *   4. Signer public key matches a registered trust anchor.
 *   5. build_timestamp is not before the revocation epoch.
 *
 * @param header   pointer to image header.
 * @param payload  pointer to image payload (immediately after header).
 */
sigma_secboot_err_t sigma_secboot_verify_image(
    const sigma_secboot_image_header_t *header,
    const void *payload);

/**
 * sigma_secboot_register_anchor — register a trusted public key.
 *
 * Can be called up to SIGMA_MAX_TRUST_ANCHORS times.
 * The first registered anchor is the primary; subsequent are recovery keys.
 *
 * @param pubkey   Dilithium3 public key bytes.
 */
sigma_secboot_err_t sigma_secboot_register_anchor(
    const uint8_t pubkey[SIGMA_DILITHIUM3_PK_BYTES]);

/**
 * sigma_secboot_revoke_anchor — mark a key as revoked by key_id.
 *
 * Future verify_image calls will reject images signed with this key.
 */
sigma_secboot_err_t sigma_secboot_revoke_anchor(const uint8_t key_id[8]);

/**
 * sigma_secboot_lock — lock the secure boot configuration.
 *
 * After this call, no new anchors can be registered and no anchors
 * can be revoked until next boot.  Called by the kernel after loading
 * all modules.
 */
void sigma_secboot_lock(void);

/**
 * sigma_secboot_is_locked — return true if configuration is locked.
 */
bool sigma_secboot_is_locked(void);

/**
 * sigma_secboot_report — write a human-readable boot attestation report
 * to @buf[@buflen].  Suitable for logging to sigma_audit_backend.
 *
 * Reports PCR values, trust anchor key_ids, and lockdown status.
 */
int sigma_secboot_report(char *buf, size_t buflen);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* SIGMA_SECBOOT_H */

/*
 * ── Secure Boot integration points ───────────────────────────────────────────
 *
 *  sigma-bootloader (arch/boot/sovereign_boot.asm + C wrapper):
 *      1. sigma_secboot_init()
 *      2. sigma_secboot_verify_image(&kernel_header, kernel_payload)
 *      3. sigma_secboot_tpm_measure(SIGMA_PCR_KERNEL, kernel_sha256, 32)
 *      4. sigma_secboot_tpm_measure(SIGMA_PCR_INITRAMFS, initrd_sha256, 32)
 *      5. sigma_secboot_tpm_unseal(sealed_dmcrypt_key, ...) → pass to dm-verity
 *      6. sigma_secboot_lock()
 *      7. Hand off to kernel entry point
 *
 *  Kernel module loader (kernel/drivers/core/sigma_driver_framework.h):
 *      • Call sigma_secboot_verify_image() before any insmod-equivalent
 *      • Reject modules with SIGMA_SECBOOT_ERR_* != OK
 */
