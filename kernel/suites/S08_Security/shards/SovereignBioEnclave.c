// =============================================================================
// SigmaOS — S08_Security — SovereignBioEnclave.c
// Hardware-Isolated Biometric & Identity Vault
// =============================================================================
// Competitor USPs Absorbed:
//   • Apple Secure Enclave — Isolated co-processor for FaceID/TouchID
//   • Windows Hello / TPM 2.0 — Hardware-backed identity verification
//   • Android TEE (TrustZone) — Trusted Execution Environment for secure apps
// Exceeding Competitors:
//   • Sovereign Biometrics: Multi-modal fusion (Face + Voice + Neural-Pattern)
//   • Zero-Knowledge Keys: Private keys NEVER leave the silicon enclave.
//   • Anti-Tamper: Physical memory erasure on unauthorized hardware access.
// =============================================================================

#include "sigma_types.h"


#define ENCLAVE_KEY_LEN     64

typedef enum {
    BIO_MODALITY_FACE   = 0,
    BIO_MODALITY_VOICE  = 1,
    BIO_MODALITY_IRIS   = 2,
    BIO_MODALITY_NEURAL = 3  // Sentiment-based work pattern ID
} BioModality;

// ── Enclave Identity Record ──────────────────────────────────────────────────
typedef struct {
    uint8_t  bio_template[1024]; // Enclave-encrypted only
    BioModality primary_mode;
    uint8_t  trust_score;        // 0-100
    bool     is_locked;
} BioIdentity;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Biometric Enclave (Secure processor handshake)
void bio_enclave_init(void);

// Enroll a new multi-modal sovereign identity
bool bio_enclave_enroll(BioIdentity* identity);

// Verify current user presence (Continuous invisible auth)
bool bio_enclave_verify(BioModality mode, void* raw_input);

// Request an E2EE signature for a .sab bundle (S10)
bool bio_enclave_sign_app(const char* app_id, uint8_t* out_sig);

// Lock the entire system and wipe sensitive caches (Panic mode)
void bio_enclave_seal_system(void);

// Synchronise identity seeds across Continuity peers (S12 E2EE)
void bio_enclave_mesh_sync(void);



