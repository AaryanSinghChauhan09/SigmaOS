/* SPDX-License-Identifier: GPL-2.0-or-later */
/**
 * disk_cipher.h — SigmaOS disk encryption layer (dm-crypt style)
 *
 * Intercepts block I/O and performs sector-level encryption/decryption.
 * Sits between the VFS layer and the raw block device.
 *
 * Supported ciphers:
 *   AES-XTS-256   — default, hardware AES-NI accelerated
 *   AES-CBC-256   — legacy compat
 *   ChaCha20      — ARM/RISC-V without AES hardware
 *
 * Key derivation:
 *   PBKDF2-SHA512 with 600,000 iterations (NIST SP 800-132)
 *   Or TPM2-sealed key (no password, relies on PCR policy)
 *
 * IV generation: XTS uses LBA as tweak (sector-level); CBC uses ESSIV.
 *
 * Inspired by: Linux dm-crypt, LUKS2 (on-disk format), VeraCrypt
 */

#pragma once
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

/* ── Cipher algorithms ───────────────────────────────────────────────────── */

typedef enum sigma_cipher_algo {
    SIGMA_CIPHER_AES_XTS_256 = 0,   /* default — AES-256-XTS */
    SIGMA_CIPHER_AES_XTS_128 = 1,
    SIGMA_CIPHER_AES_CBC_256 = 2,
    SIGMA_CIPHER_CHACHA20    = 3,   /* for no-AES-NI platforms */
} sigma_cipher_algo_t;

/* ── LUKS2-compatible header (on-disk, first 4MB of device) ─────────────── */

#define SIGMA_LUKS_MAGIC   "SIGMACRYPT\x00\x00"
#define SIGMA_LUKS_VERSION 2
#define SIGMA_LUKS_MAX_KEYSLOTS 8

typedef struct __attribute__((packed)) sigma_luks_header {
    char     magic[12];
    uint32_t version;
    uint64_t hdr_size;       /* header + keyslot area size */
    uint8_t  uuid[16];
    char     label[48];
    char     cipher[32];     /* e.g. "aes-xts-plain64" */
    char     hash[32];       /* e.g. "sha256" */
    uint64_t data_offset;    /* start of encrypted data (in 512-byte sectors) */
    uint32_t key_bits;       /* master key length in bits */
    /* Keyslots */
    struct {
        uint32_t active;     /* 1 = in use */
        uint32_t iter_count; /* PBKDF2 iterations */
        uint8_t  salt[32];
        uint8_t  key_digest[32]; /* SHA-256 of decrypted master key */
        uint8_t  encrypted_key[64]; /* master key encrypted with passphrase key */
    } keyslots[SIGMA_LUKS_MAX_KEYSLOTS];
    uint8_t  checksum[32];   /* SHA-256 of entire header (excl. checksum field) */
} sigma_luks_header_t;

/* ── Runtime encryption context ─────────────────────────────────────────── */

typedef struct sigma_disk_cipher {
    int                  block_fd;       /* underlying block device fd */
    sigma_cipher_algo_t  algo;
    uint8_t              master_key[64]; /* 512-bit max key */
    uint32_t             key_bits;
    uint64_t             data_offset;    /* sectors to skip (header area) */
    bool                 readonly;
} sigma_disk_cipher_t;

/* ── Key derivation ─────────────────────────────────────────────────────── */

/* Derive encryption key from passphrase using PBKDF2-SHA512.
 * This MUST NOT return 32 zero bytes — it must actually implement PBKDF2.
 * Replaces the stub in sigma-cryptfs (tracked in make check-stubs). */
int sigma_disk_derive_key(const char *passphrase, size_t passphrase_len,
                           const uint8_t *salt, size_t salt_len,
                           uint32_t iterations,
                           uint8_t *out_key, size_t key_len);

/* Derive key using TPM2-sealed blob (no passphrase). */
int sigma_disk_tpm_unseal_key(const uint8_t *sealed_blob, size_t sealed_len,
                               uint8_t *out_key, size_t key_len);

/* ── Device API ─────────────────────────────────────────────────────────── */

/* Create a new LUKS2-formatted encrypted device. */
int  sigma_disk_cipher_format(const char *device, sigma_cipher_algo_t algo,
                               const char *passphrase, bool tpm_seal);

/* Open an encrypted device by passphrase. */
int  sigma_disk_cipher_open  (const char *device, const char *passphrase,
                               sigma_disk_cipher_t *out);

/* Open an encrypted device using TPM2 (no passphrase). */
int  sigma_disk_cipher_open_tpm(const char *device, sigma_disk_cipher_t *out);

/* Close and zero the key material. */
int  sigma_disk_cipher_close (sigma_disk_cipher_t *ctx);

/* Sector-level encrypt/decrypt (called by block device shim). */
int  sigma_disk_cipher_encrypt(sigma_disk_cipher_t *ctx,
                                uint64_t sector, uint8_t *buf, size_t len);
int  sigma_disk_cipher_decrypt(sigma_disk_cipher_t *ctx,
                                uint64_t sector, uint8_t *buf, size_t len);

/* Add/remove passphrase keyslots. */
int  sigma_disk_add_keyslot   (const char *device, const char *existing_pass,
                                const char *new_pass);
int  sigma_disk_remove_keyslot(const char *device, const char *passphrase,
                                uint32_t slot_index);

/* Benchmark: measure enc/dec throughput in MB/s. */
int  sigma_disk_cipher_bench  (sigma_cipher_algo_t algo, double *enc_mbs,
                                double *dec_mbs);
