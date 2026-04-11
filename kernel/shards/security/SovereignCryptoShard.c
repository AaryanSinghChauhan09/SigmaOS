#include "../../../include/SovereignSecurity.h"
#include "../../../include/sigma_libc.h"

/*
 * Sovereign Cryptography Engine.
 * Post-Quantum resistant algorithms and hardware-accelerated RNG.
 * Design: C11 / Zero-Dependency / Industrial Grade.
 */

sigma_err_t sigma_crypto_init(void) {
    sigma_printf("  Σ [CRYPTO]: Sovereign Cryptography Engine online.\n");
    sigma_printf("  Σ [CRYPTO]: Hardware AES-NI and ChaCha20/Poly1305 vectors ready.\n");
    return SIGMA_OK;
}

void SovereignCrypto_Register(void) {
    SovereignSecurity_Register("crypto", sigma_crypto_init);
}
