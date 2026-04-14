// =============================================================================
// SigmaOS — S08_Security — SovereignSentinelIDS.c
// AI-Augmented Kernel Intrusion Detection System (IDS)
// =============================================================================
// Exceeding Competitors:
//   • Windows Defender / macOS XProtect — Signature-based (reactive).
//   • EDR Solutions (CrowdStrike) — Userland-heavy, prone to system crashes.
//   • Sigma Sentinel — KERNEL-NATIVE Behavioural Analysis: 
//     Uses S13 Sentience to identify abnormal syscall chains *before* exploit.
// =============================================================================

#include <sigma_types.h>


#define MAX_THREAT_SIGNATURES 1024

typedef struct {
    uint32_t syscall_chain[8]; // Pattern of suspicious calls
    uint8_t  severity;         // 1-10
    char     description[64];
} ThreatPattern;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Sentinel IDS (Hooks into all S10 Syscall bridges)
void sentinel_init(void);

// Audit a syscall event against known and learned threat patterns
bool sentinel_audit_event(uint32_t pid, uint32_t syscall_num);

// Block a process permanently if its "Threat Score" exceeds threshold
void sentinel_quarantine_pid(uint32_t pid);

// Lock down the Sovereign Vault (S06) and Registry (S10) under heavy attack
void sentinel_seal_system(void);

// Report threat analysis to ZenithUI Security HUD (S02 Dashboard)
void sentinel_report_threat(const char* details);

// Sync learned "Attack Patterns" across the Hive mesh (Immunity through Mesh)
void sentinel_mesh_sync(void);



