// =============================================================================
// SigmaOS — S08_Security — SovereignQuantumCrypto.c
// Quantum-Safe Cryptographic Primitives Shard
// =============================================================================
// Competitor USPs Absorbed:
//   • Windows/macOS/Linux — Standard RSA/ECC (Vulnerable to future quantum)
//   • NIST Post-Quantum (PQC) — New standards (Kyber, Dilithium, SPHINCS+)
// SigmaOS Sovereign Cryptography:
//   • Defaults to CRYSTALS-Kyber for key exchange.
//   • CRYSTALS-Dilithium for digital signatures (.sab verification).
//   • Optimized AVX-512 implementation for zero-latency encryption.
// =============================================================================

#include <sigma_types.h>


#define PQC_PUBKEY_LEN      1184 // Kyber-768
#define PQC_SIG_LEN         2420 // Dilithium-2

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise Quantum-Safe primitives (check SIMD support)
void pqc_init(void);

// Generate Kyber-768 keypair for sovereign communication
void pqc_gen_keypair(uint8_t* pub, uint8_t* priv);

// Encap/Decap: Secure key exchange
void pqc_encrypt_shared(uint8_t* shared_secret, const uint8_t* pub);

// Sign a data block with Dilithium-2 (SAB/Registry/Log security)
void pqc_sign(uint8_t* sig, const uint8_t* message, uint32_t len, const uint8_t* priv);

// Verify a PQC signature
bool pqc_verify(const uint8_t* sig, const uint8_t* message, uint32_t len, const uint8_t* pub);

// Adaptive fallback: Mix RSA-4096 + Kyber for "Hybrid Security"
void pqc_hybrid_mode_toggle(bool enabled);


