// SPDX-License-Identifier: GPL-2.0-only
// sigma_fedlearn.h — SigmaOS Federated Learning Platform
// Purpose: Privacy-preserving AI improvement across SigmaOS machines.
//          Only model weights (never raw data) leave the device.
//          DPDP Act 2023 compliant. Sovereign AI that improves from
//          millions of Indian users without exposing any user data.

#pragma once
#include <stdint.h>
#include <stdbool.h>
#include <time.h>

// ---------------------------------------------------------------------------
// Federated Learning Protocol
// ---------------------------------------------------------------------------
// Round-based federated learning:
// 1. Server broadcasts global model weights
// 2. Each participant trains locally on their data
// 3. Each participant sends WEIGHT UPDATES (gradients) — NOT data
// 4. Server aggregates (FedAvg / FedProx) → new global model
// 5. Repeat
//
// Privacy guarantees:
// - Differential privacy: noise added to weights before upload
// - Secure aggregation: server never sees individual updates
// - Opt-in only: user controls participation
// - Opt-out: leave any time, data never leaves

#define SIGMA_FL_VERSION            "1.0.0"
#define SIGMA_FL_COORDINATOR_URL    "https://fl.sigmaos.dev/api/v1"
#define SIGMA_FL_DP_EPSILON         0.5    // Differential privacy budget
#define SIGMA_FL_DP_DELTA           1e-5   // DP delta
#define SIGMA_FL_MIN_SAMPLES        10     // Min local samples to contribute
#define SIGMA_FL_MAX_ROUNDS         1000
#define SIGMA_FL_WEIGHT_ENCRYPT     1      // Encrypt weights with Kyber-1024

// ---------------------------------------------------------------------------
// Federated Learning Networks (available in SigmaOS)
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_FL_NETWORK_CROP_DISEASE     = 1, // Crop disease detection (sigma-agri)
    SIGMA_FL_NETWORK_TAX_ANOMALY      = 2, // GST/tax error detection (sigma-ca)
    SIGMA_FL_NETWORK_HANDWRITING_OCR  = 3, // Devanagari/Indic OCR improvement
    SIGMA_FL_NETWORK_MEDICAL_ASSIST   = 4, // Clinical decision support (sigma-health)
    SIGMA_FL_NETWORK_VOICE_ASR        = 5, // Indian language ASR (sigma-bhashini)
    SIGMA_FL_NETWORK_FRAUD_DETECT     = 6, // UPI fraud detection (sigma-indiastack)
    SIGMA_FL_NETWORK_ENERGY_PREDICT   = 7, // Energy consumption prediction
    SIGMA_FL_NETWORK_CUSTOM           = 99,// User-defined network
} sigma_fl_network_type_t;

typedef struct {
    char     network_id[32];
    sigma_fl_network_type_t type;
    char     network_name[128];
    char     description[256];
    char     model_architecture[64];  // "mobilenet_v3", "distilbert", custom
    uint32_t global_participants;
    uint32_t current_round;
    double   global_accuracy;         // Current global model accuracy
    double   baseline_accuracy;       // Accuracy before federated training
    double   improvement_pct;         // Improvement over baseline
    bool     open_to_join;
    char     data_type[64];           // "images", "tabular", "text", "time_series"
    char     privacy_guarantee[128];  // "DP(ε=0.5, δ=1e-5) + SecAgg"
    // DPDP Act 2023 compliance fields
    bool     dpdp_compliant;
    char     data_processor[64];      // Entity processing data (SigmaOS FL server)
    char     purpose_limitation[256]; // Exact purpose stated
    bool     consent_required;
    time_t   created_at;
} sigma_fl_network_t;

// ---------------------------------------------------------------------------
// Local Training Job
// ---------------------------------------------------------------------------

typedef struct {
    char     job_id[32];
    char     network_id[32];
    uint32_t round_number;
    char     model_path[256];         // Local model file
    char     weights_path[256];       // Global weights received
    uint32_t local_samples;           // Number of local training samples
    uint32_t local_epochs;            // Epochs to train locally
    double   local_learning_rate;
    double   local_accuracy;          // Accuracy on local validation set
    double   local_loss;
    // Differential privacy
    bool     dp_enabled;
    double   dp_noise_multiplier;     // Gaussian noise σ
    double   dp_l2_sensitivity;       // Gradient clipping bound
    // Resource limits
    uint32_t max_cpu_pct;             // Don't use more than X% CPU
    uint32_t max_mem_mb;              // Memory limit
    bool     train_only_on_charge;    // Mobile: only train when charging
    bool     train_only_on_wifi;      // Only when on Wi-Fi (no mobile data)
    time_t   started_at;
    time_t   completed_at;
    uint32_t duration_s;
    bool     contributed;             // Did this round's update get uploaded?
} sigma_fl_training_job_t;

// ---------------------------------------------------------------------------
// Weight Update (what gets sent to server — never raw data)
// ---------------------------------------------------------------------------

typedef struct {
    char     update_id[32];
    char     network_id[32];
    uint32_t round_number;
    char     participant_did[128];    // Participant's DID (for SecAgg)
    uint32_t num_samples;             // How many samples trained on
    uint32_t weight_size_bytes;
    uint8_t *weight_delta;            // Encrypted gradient update
    char     kyber_ciphertext[512];   // Kyber-1024 encapsulated symmetric key
    double   dp_noise_applied;        // DP noise level used
    char     dilithium3_sig[512];     // Signature over update (integrity)
    time_t   created_at;
} sigma_fl_weight_update_t;

// ---------------------------------------------------------------------------
// Aggregation (Coordinator side)
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_FL_AGG_FEDAVG    = 1,  // Federated Averaging (standard)
    SIGMA_FL_AGG_FEDPROX   = 2,  // FedProx (heterogeneous data)
    SIGMA_FL_AGG_SCAFFOLD  = 3,  // SCAFFOLD (gradient variance reduction)
    SIGMA_FL_AGG_FEDOPT    = 4,  // FedOpt (server-side momentum)
} sigma_fl_aggregation_t;

// ---------------------------------------------------------------------------
// Participation Status
// ---------------------------------------------------------------------------

typedef struct {
    char     network_id[32];
    char     participant_did[128];
    bool     enrolled;
    bool     opted_out;
    uint32_t rounds_participated;
    uint32_t rounds_skipped;         // Low battery, no Wi-Fi, etc.
    uint64_t total_samples_trained;
    double   contribution_score;     // Federated contribution metric
    time_t   enrolled_at;
    time_t   last_contribution;
    // Personal benefit from federated learning
    double   model_accuracy_before;
    double   model_accuracy_current;
    double   improvement_from_federation;
} sigma_fl_participation_t;

// ---------------------------------------------------------------------------
// API
// ---------------------------------------------------------------------------

// List available networks
int sigma_fl_list_networks(sigma_fl_network_t *networks, int *count);
int sigma_fl_get_network(const char *network_id, sigma_fl_network_t *out);

// Participation
int sigma_fl_join(const char *network_id);
int sigma_fl_opt_out(const char *network_id);
int sigma_fl_participation_status(const char *network_id,
                                    sigma_fl_participation_t *out);

// Training
int sigma_fl_train_local(const char *network_id,
                          sigma_fl_training_job_t *job);
int sigma_fl_contribute(const char *job_id,
                          sigma_fl_weight_update_t *update);
int sigma_fl_fetch_global_model(const char *network_id,
                                  const char *output_path);

// Status
int sigma_fl_global_stats(const char *network_id,
                            uint32_t *participants,
                            double *accuracy,
                            uint32_t *current_round);

// CLI:
// sigma-fedlearn list
// sigma-fedlearn join --network sigma-agri-disease
// sigma-fedlearn status --network sigma-agri-disease
// sigma-fedlearn contribute --network crop-disease --local-samples 500
// sigma-fedlearn opt-out --network sigma-medical-assist
