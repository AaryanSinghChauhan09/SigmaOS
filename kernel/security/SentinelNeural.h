/**
 * SentinelNeural.h — AI-Driven Threat Detection Subsystem
 * SigmaOS Zenith v15.1
 *
 * Maps to: Syllabus-FCIT Unit III (Virus Detection & Prevention)
 *          Syllabus-AIML Unit II (Machine Learning — Anomaly Detection)
 *
 * SentinelNeural is the kernel-resident security intelligence layer.
 * It performs real-time behavioral analysis, signature matching, and
 * ML-based anomaly detection with zero external dependencies.
 */
#pragma once
#include "../../include/core/sigma_kernel_types.h"

namespace Sigma::Security {

// ─── Threat Classification ────────────────────────────────────────────────────
enum class ThreatType : sigma_u8 {
    NONE         = 0,
    VIRUS        = 1,   // Self-replicating, infects files
    WORM         = 2,   // Network-spreading, no host file
    TROJAN       = 3,   // Disguised malicious code
    RANSOMWARE   = 4,   // Encrypts user data for ransom
    ROOTKIT      = 5,   // Hides OS-level compromise
    SPYWARE      = 6,   // Exfiltrates user data
    ADWARE       = 7,   // Unwanted advertisements
    KEYLOGGER    = 8,   // Records keystrokes
    BOTNET       = 9,   // Remote command execution
    CRYPTO_MINER = 10,  // Unauthorized CPU mining
    ZERO_DAY     = 11,  // Previously unknown exploit
    APT          = 12,  // Advanced Persistent Threat
    ANOMALY      = 13,  // ML-detected behavioral anomaly
};

enum class ThreatSeverity : sigma_u8 {
    INFO     = 0,  // Informational — no action needed
    LOW      = 1,  // Monitor only
    MEDIUM   = 2,  // Alert + log
    HIGH     = 3,  // Quarantine + alert
    CRITICAL = 4,  // Terminate + quarantine + report
};

struct ThreatEvent {
    ThreatType     type;
    ThreatSeverity severity;
    sigma_u32      pid;               // Offending process ID
    sigma_u64      address;           // Memory address of detection
    char           process_name[64];  // Process name
    char           description[256];  // Human-readable description
    sigma_u64      timestamp_ns;      // Nanosecond timestamp
    sigma_u8       sha256_hash[32];   // SHA-256 of offending payload
    float          confidence;        // ML confidence [0.0, 1.0]
};

// ─── Signature Database ───────────────────────────────────────────────────────
struct VirusSignature {
    sigma_u8  pattern[64];       // Byte signature pattern
    sigma_u8  pattern_len;       // Length of pattern
    sigma_u8  wildcard_mask[64]; // 1 = wildcard byte
    ThreatType type;
    ThreatSeverity severity;
    char name[64];               // Signature name (e.g., "EICAR-Test-File")
};

// ─── Behavioral Rule ──────────────────────────────────────────────────────────
struct BehaviorRule {
    char      rule_name[64];
    // Trigger conditions (bit flags)
    sigma_u32 syscall_pattern;   // Suspicious syscall sequence
    sigma_u32 network_flags;     // Unexpected network access
    bool      file_mass_change;  // >100 files modified in <1s (ransomware)
    bool      privilege_escalation; // Ring 3 → Ring 0 attempt
    bool      self_replication;  // Process spawning copies of itself
    ThreatType maps_to;
    ThreatSeverity severity;
};

// ─── ML Anomaly Detector ──────────────────────────────────────────────────────
struct ProcessFeatureVector {
    float cpu_usage_pct;       // 0.0–100.0
    float mem_growth_rate;     // MB/s
    float syscall_rate;        // syscalls/second
    float network_send_rate;   // KB/s
    float network_recv_rate;   // KB/s
    float file_write_rate;     // files/second
    float child_spawn_rate;    // child processes/second
    float entropy;             // Shannon entropy of memory pages
    float runtime_seconds;
    sigma_u32 open_file_count;
};

class SentinelNeural {
public:
    // ─── Lifecycle ────────────────────────────────────────────────────────
    static SentinelNeural& instance();
    void init();
    void shutdown();

    // ─── Signature Scanning ───────────────────────────────────────────────
    /**
     * Scan memory region for known virus signatures.
     * Returns true + fills event if threat found.
     */
    bool scan_memory(const sigma_u8* data, sigma_usize len,
                     sigma_u32 pid, ThreatEvent* out_event);

    /**
     * Scan file on SovereignFS for signatures.
     */
    bool scan_file(const char* path, ThreatEvent* out_event);

    /**
     * Load signature database from /sigma/security/signatures.sigdb
     */
    sigma_u32 load_signatures(const char* sigdb_path);

    /**
     * Update signatures from cloud (SovereignCloudFS)
     */
    int update_signatures_from_cloud();

    // ─── Behavioral Analysis ──────────────────────────────────────────────
    /**
     * Monitor process behavior in real-time.
     * Called by the kernel scheduler on every context switch.
     */
    void observe_process(sigma_u32 pid, const ProcessFeatureVector& features);

    /**
     * Check syscall sequence for suspicious patterns.
     * Called from the syscall dispatcher.
     */
    bool check_syscall_sequence(sigma_u32 pid, sigma_u32 syscall_id);

    // ─── ML Anomaly Detection ─────────────────────────────────────────────
    /**
     * Feed process features to the neural anomaly detector.
     * Model: 3-layer MLP trained on normal process behavior baseline.
     * Returns anomaly score [0.0 = normal, 1.0 = certain anomaly]
     */
    float compute_anomaly_score(const ProcessFeatureVector& features);

    /**
     * Classify anomaly score into ThreatType using threshold rules.
     */
    ThreatType classify_anomaly(float score, const ProcessFeatureVector& features);

    /**
     * Update the ML model weights with new baseline observations (online learning).
     */
    void update_model_baseline(const ProcessFeatureVector* samples, sigma_usize count);

    // ─── Response Actions ─────────────────────────────────────────────────
    /**
     * Quarantine: move file to /sigma/security/quarantine/, deny execution.
     */
    int quarantine_file(const char* path);

    /**
     * Kill and quarantine a suspicious process.
     */
    int terminate_threat(sigma_u32 pid, const char* reason);

    /**
     * Report threat to SigmaOS Security Dashboard + cloud telemetry.
     */
    void report_threat(const ThreatEvent& event);

    /**
     * Heuristic: compute SHA-256 of payload (kernel implementation, no OpenSSL).
     */
    void compute_sha256(const sigma_u8* data, sigma_usize len, sigma_u8 out[32]);

    // ─── EICAR Test ───────────────────────────────────────────────────────
    /**
     * EICAR test file signature:
     * X5O!P%@AP[4\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*
     * Must be detected as ThreatType::VIRUS with severity CRITICAL.
     */
    bool is_eicar_test_file(const sigma_u8* data, sigma_usize len);

    // ─── Statistics ───────────────────────────────────────────────────────
    sigma_u64 threats_detected_total;
    sigma_u64 files_scanned_total;
    sigma_u64 processes_monitored;
    sigma_u32 signatures_loaded;
    float     false_positive_rate;   // Tracked over 30-day window

private:
    SentinelNeural() = default;

    // Signature DB (loaded at boot)
    VirusSignature* m_signatures;
    sigma_u32       m_sig_count;
    sigma_u32       m_sig_capacity;

    // Behavioral rules
    BehaviorRule    m_rules[64];
    sigma_u32       m_rule_count;

    // ML model weights (3-layer MLP: 10→32→16→1)
    float m_weights_1[10 * 32];
    float m_bias_1[32];
    float m_weights_2[32 * 16];
    float m_bias_2[16];
    float m_weights_3[16];
    float m_bias_3;
    float m_anomaly_threshold;   // Default: 0.75

    // Per-process behavior history ring buffer
    struct ProcHistory {
        sigma_u32 pid;
        ProcessFeatureVector history[32]; // last 32 observations
        sigma_u8  head;
    };
    ProcHistory m_proc_history[256];
    sigma_u32   m_proc_count;

    // Internal helpers
    float relu(float x) { return x > 0.0f ? x : 0.0f; }
    float sigmoid(float x);
    float mlp_forward(const float* input, int input_dim);
    bool  pattern_match(const sigma_u8* data, sigma_usize len,
                        const VirusSignature& sig);
};

} // namespace Sigma::Security
