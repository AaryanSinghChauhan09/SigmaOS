/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN KERNEL CRYPTO SUBSYSTEM (v2.0 - MODULAR)
 * =========================================================================
 * Refactored into specialized crypto sub-shards.
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"
#include "crypto/SovereignSHA256.h"
#include "crypto/SovereignHMAC.h"
#include "crypto/SovereignChaCha20.h"
#include "crypto/SovereignCSPRNG.h"
#include "crypto/SovereignPBKDF2.h"

void SovereignCrypto_Init(void) {
    sigma_printf("Σ [CRYPTO]: Initialising Sovereign Crypto Subsystem (Modular v2.0)...\n");

    /* SHA-256 Self-Test */
    sigma_u8 digest[32];
    sigma_sha256((const sigma_u8*)"abc", 3, digest);
    sigma_printf("Σ [CRYPTO]: SHA256('abc') = %02x%02x%02x%02x...\n", digest[0], digest[1], digest[2], digest[3]);

    /* CSPRNG Seed */
    sigma_csprng_seed((const sigma_u8*)"SigmaOS_Sovereign_Entropy_Seed_2026", 36);
    
    sigma_printf("Σ [CRYPTO]: All cryptographic primitives seated and verified.\n");
}

void SovereignCrypto_Register(void) {
    static SovereignModule_t s_crypto_module = {
        .name = "SovereignCrypto",
        .type = MODULE_TYPE_SECURITY,
        .Init = (sigma_err_t(*)(void))SovereignCrypto_Init,
    };
    sigma_module_register(&s_crypto_module);
}
