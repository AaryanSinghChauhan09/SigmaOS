// SPDX-License-Identifier: GPL-2.0-only
// sigma_datasov.h — SigmaOS Data Sovereignty Platform
// Purpose: Users own, control, and optionally monetize their own data.
//          Local encrypted vault. Consent-based marketplace. Zero-knowledge
//          proofs for privacy-preserving claims. DPDP Act 2023 compliant.

#pragma once
#include <stdint.h>
#include <stdbool.h>
#include <time.h>

#define SIGMA_DATASOV_VAULT_DIR   "/var/sigma-datasov/vault"
#define SIGMA_DATASOV_MARKET_API  "https://datasov.sigmaos.dev/api/v1"
#define SIGMA_DATASOV_ZK_BACKEND  "groth16"  // zk-SNARK backend

// ---------------------------------------------------------------------------
// Data Vault Categories
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_DS_CAT_FINANCIAL    = 1,   // Purchase history, bank statements, GST
    SIGMA_DS_CAT_HEALTH       = 2,   // Medical records, prescriptions, labs
    SIGMA_DS_CAT_TRAVEL       = 3,   // Location history, transport usage
    SIGMA_DS_CAT_PROFESSIONAL = 4,   // Work activities, filings, licenses
    SIGMA_DS_CAT_EDUCATION    = 5,   // Certificates, marks, courses
    SIGMA_DS_CAT_SOCIAL       = 6,   // Communication patterns (no content)
    SIGMA_DS_CAT_CONSUMPTION  = 7,   // App usage, energy, utilities
    SIGMA_DS_CAT_AGRICULTURAL = 8,   // Crop data, soil, yield (sigma-agri)
} sigma_ds_category_t;

typedef struct {
    char     record_id[32];
    sigma_ds_category_t category;
    char     source_app[32];         // "sigma-accounts", "sigma-health", etc.
    time_t   created_at;
    time_t   updated_at;
    uint64_t size_bytes;
    bool     encrypted;              // Always true in vault
    char     encryption_key_id[32];  // TPM-sealed key reference
    bool     shared_count;           // How many times shared
    double   earnings_total_inr;     // Total earned from this record
} sigma_ds_vault_record_t;

// ---------------------------------------------------------------------------
// Data Marketplace Request
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_DS_MARKET_STATUS_OPEN     = 1,
    SIGMA_DS_MARKET_STATUS_CLOSED   = 2,
    SIGMA_DS_MARKET_STATUS_FUNDED   = 3,  // Payment escrowed
} sigma_ds_market_status_t;

typedef struct {
    char     request_id[32];
    char     requester_name[128];    // Research institution / company
    char     requester_did[128];
    char     purpose[256];           // DPDP Act: purpose must be stated
    sigma_ds_category_t data_category;
    char     data_description[256];  // Exact data requested
    bool     anonymized_only;        // Cannot request identifiable data
    uint32_t data_points_needed;     // Number of participants needed
    double   payment_per_participant; // ₹ per participant
    char     payment_currency[4];    // "INR", "ERUPI"
    sigma_ds_market_status_t status;
    time_t   request_expires;
    char     legal_basis[128];       // DPDP Act Section cited
    bool     irb_approved;           // Institutional Review Board approval
    char     irb_reference[64];
} sigma_ds_market_request_t;

// ---------------------------------------------------------------------------
// Consent Record (DPDP Act 2023 Compliance)
// ---------------------------------------------------------------------------

typedef struct {
    char     consent_id[32];
    char     user_did[128];
    char     request_id[32];
    bool     consented;
    time_t   consented_at;
    time_t   consent_expires;        // User can set expiry
    bool     revoked;
    time_t   revoked_at;
    char     data_shared_hash[64];   // SHA-256 of what was shared (audit trail)
    double   payment_received;       // ₹ received for this consent
    char     payment_upi_ref[64];
    // DPDP Act fields
    char     data_processor[128];    // Entity that processed data
    char     processing_purpose[256];// Exact purpose (must match request)
    bool     data_deleted_after;     // Did requester confirm deletion?
    time_t   deletion_confirmed_at;
} sigma_ds_consent_t;

// ---------------------------------------------------------------------------
// Zero-Knowledge Proofs
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_DS_ZK_INCOME_ABOVE    = 1, // "Income > ₹X" without revealing amount
    SIGMA_DS_ZK_AGE_ABOVE       = 2, // "Age > 18" without revealing DOB
    SIGMA_DS_ZK_CREDENTIAL      = 3, // "I am a CA" without revealing ICAI number
    SIGMA_DS_ZK_RESIDENT        = 4, // "I live in Delhi" without Aadhaar
    SIGMA_DS_ZK_CREDIT_SCORE    = 5, // "CIBIL > 750" without revealing score
    SIGMA_DS_ZK_TAX_COMPLIANT   = 6, // "ITR filed for last 3 years"
    SIGMA_DS_ZK_CUSTOM          = 99,
} sigma_ds_zk_claim_type_t;

typedef struct {
    char     proof_id[32];
    sigma_ds_zk_claim_type_t claim_type;
    char     claim_statement[128];   // Human-readable claim
    char     public_inputs[256];     // Non-secret inputs to verifier
    char     proof_bytes[1024];      // Groth16 proof (serialized)
    char     verifier_key_hash[64];  // Hash of verification key
    char     prover_did[128];        // Who made the proof
    char     verifier_did[128];      // Who can verify it
    time_t   generated_at;
    time_t   expires_at;
    bool     verified;               // Has the verifier confirmed?
} sigma_ds_zk_proof_t;

// ---------------------------------------------------------------------------
// API
// ---------------------------------------------------------------------------

// Vault
int sigma_ds_vault_list(sigma_ds_vault_record_t *records, int *count);
int sigma_ds_vault_stats(uint64_t *total_bytes, uint32_t *record_count,
                          double *total_earnings_inr);
int sigma_ds_vault_delete(const char *record_id);
int sigma_ds_vault_export(const char *record_id, const char *output_path);

// Marketplace
int sigma_ds_market_list_requests(sigma_ds_market_request_t *requests,
                                    int *count);
int sigma_ds_market_get_request(const char *request_id,
                                  sigma_ds_market_request_t *out);
int sigma_ds_market_consent(const char *request_id, bool allow,
                              sigma_ds_consent_t *consent_out);
int sigma_ds_market_revoke_consent(const char *consent_id);
int sigma_ds_market_earnings(double *total_inr, double *this_month_inr);

// Zero-Knowledge Proofs
int sigma_ds_zk_prove(sigma_ds_zk_claim_type_t claim,
                       double threshold,        // For INCOME_ABOVE, AGE_ABOVE
                       const char *credential_id, // For CREDENTIAL claims
                       const char *verifier_did,
                       sigma_ds_zk_proof_t *proof_out);
int sigma_ds_zk_verify(const sigma_ds_zk_proof_t *proof, bool *valid);
int sigma_ds_zk_export_proof(const sigma_ds_zk_proof_t *proof,
                               const char *output_path); // QR / JSON

// CLI:
// sigma-datasov vault status
// sigma-datasov marketplace list
// sigma-datasov consent --request R001 --allow yes
// sigma-datasov zk prove --claim "income > 500000" --verifier HDFC-Bank-DID
// sigma-datasov earnings --month 2026-06
