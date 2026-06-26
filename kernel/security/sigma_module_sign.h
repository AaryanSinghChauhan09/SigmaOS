// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_module_sign.h — Kernel module signing verification
 *
 * Every SigmaOS kernel module (.smod) must carry a Dilithium3 signature
 * computed over the module's SHA-3-256 hash. The module loader calls
 * sigma_module_verify() before any module code runs. Unsigned modules
 * are rejected unless SIGMA_MODULE_ALLOW_UNSIGNED=1 (dev builds only).
 *
 * Module format:
 *   [ ELF binary ]
 *   [ sigma_module_sig_t at end of file ]
 */
#include <sigma_kernel_types.h>

/* Size constants (Dilithium3 / ML-DSA-65) */
#define SIGMA_DILITHIUM3_SIG_BYTES    3293
#define SIGMA_DILITHIUM3_PUBKEY_BYTES 1952
#define SIGMA_MODULE_SIG_MAGIC        0x5349474D4F445349ULL  /* "SIGMODSI" */

typedef struct __attribute__((packed)) {
    sigma_u64 magic;                                /* SIGMA_MODULE_SIG_MAGIC   */
    sigma_u8  module_hash[32];                     /* SHA-3-256 of ELF content */
    sigma_u8  signature[SIGMA_DILITHIUM3_SIG_BYTES]; /* Dilithium3 signature    */
    sigma_u32 sig_len;                             /* actual signature length  */
    sigma_u8  signer_id[64];                       /* e.g. "sigma-official-v1" */
} sigma_module_sig_t;

/* ── Trusted public keys (stored in TPM NVRAM or kernel data section) ──────── */
typedef struct {
    sigma_u8  pubkey[SIGMA_DILITHIUM3_PUBKEY_BYTES];
    char      id[64];
    bool      active;
} sigma_module_key_t;

/* ── API ──────────────────────────────────────────────────────────────────── */

/* Initialise the module signing subsystem, load trusted keys from TPM/kdata  */
void sigma_module_sign_init(void);

/*
 * Verify a module before loading.
 * @data:     pointer to the start of the module (ELF binary)
 * @data_len: total file size including the sigma_module_sig_t appended at end
 * Returns 0 if valid, -EKEYREJECTED if signature is bad, -ENOKEY if unsigned.
 */
int sigma_module_verify(const sigma_u8* data, sigma_size_t data_len);

/* Add a trusted public key (called during key enrollment) */
int sigma_module_add_key(const sigma_module_key_t* key);

/* Remove a key by ID (key revocation) */
int sigma_module_revoke_key(const char* key_id);

/* List all trusted module signing keys */
void sigma_module_list_keys(void (*cb)(const sigma_module_key_t*, void*), void* ctx);

/*
 * Sign a module (called by the module build tool, not at runtime).
 * Writes a sigma_module_sig_t to the end of the output file.
 * Requires the private key to be available (typically sigma-trustd handles this).
 */
int sigma_module_sign_file(const char* input_path, const char* output_path,
                            const sigma_u8* privkey, sigma_size_t privkey_len);
