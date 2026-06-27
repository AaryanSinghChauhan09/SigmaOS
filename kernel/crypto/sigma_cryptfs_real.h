// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_cryptfs_real.h — Real AES-256-GCM filesystem encryption interface
 *
 * Replaces the 32-zero-byte derive_key() stub (Issue #44).
 * Key derivation uses TPM2 unsealing + HKDF-SHA256.
 */

#include <sigma_kernel_types.h>
#include <kernel/security/sigma_secboot.h>

namespace sigma::cryptfs {

class CryptFS {
public:
    /*
     * derive_key — unseal master key from TPM2 and derive per-volume AES-256 key.
     * volume_uuid: 16-byte UUID identifying the encrypted volume.
     * key_out:     32-byte buffer filled with the derived AES-256 key.
     * Returns 0 on success, SIGMA_EPERM if TPM PCR policy check fails.
     */
    static int derive_key(const sigma_tpm_seal_ctx_t* tpm_ctx,
                          const sigma_u8* volume_uuid,
                          sigma_u8        key_out[32]);

    /*
     * encrypt_sector / decrypt_sector — AES-256-GCM per-sector I/O.
     * sector_no:   logical sector number (used as tweak for IV derivation)
     * sector_size: must be a multiple of 512 bytes
     * tag:         16-byte GCM authentication tag (produced by encrypt,
     *              verified by decrypt)
     */
    static int encrypt_sector(const sigma_u8* key,
                               sigma_u64       sector_no,
                               const sigma_u8* plaintext,
                               sigma_u8*       ciphertext,
                               sigma_size_t    sector_size);

    static int decrypt_sector(const sigma_u8* key,
                               sigma_u64       sector_no,
                               const sigma_u8* ciphertext,
                               sigma_u8*       plaintext,
                               sigma_size_t    sector_size,
                               const sigma_u8* tag);
};

} // namespace sigma::cryptfs
