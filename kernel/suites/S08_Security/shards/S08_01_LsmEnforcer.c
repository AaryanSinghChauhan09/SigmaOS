// =============================================================================
// SigmaOS — S08_Security — S08_01_LsmEnforcer.c
// Sovereign Security Module (SSM) Enforcer
// =============================================================================
// Competitor Parity:
//   • Linux LSM (AppArmor/SELinux) — Hook-based security mediation
//   • Windows Defender — Kernel-level process monitoring
//   • Sigma SSM — Formal Verification (S08) backed policy enforcement
// =============================================================================

#include <stdint.h>
#include <stdbool.h>

typedef struct {
    uint32_t pid;
    uint64_t active_capabilities;
    char     sandbox_profile[64];
} SecurityContext;

// ── Public API ────────────────────────────────────────────────────────────────

// Mediate a system access request (File/Net/Process)
bool ssm_mediate_access(uint32_t pid, uint32_t target_id, uint8_t op);

// Load a formal security policy from the Sovereign Vault (S10)
void ssm_load_policy(const char* policy_path);

// Log an access violation to the Neural Oracle (S13)
void ssm_audit_violation(uint32_t pid, uint8_t op);
