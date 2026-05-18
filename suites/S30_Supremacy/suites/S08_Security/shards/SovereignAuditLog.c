// SigmaOS Sovereign Audit Logging Shard
// Absorbs Linux auditd + Windows Event Log + macOS OSLog
// Cryptographically chained, tamper-evident, kernel-level log ring.

#include "core/sigma_types.h"


#define SIGMA_AUDIT_MAX_ENTRIES  8192
#define SIGMA_AUDIT_MSG_LEN      256

typedef enum {
    SIGMA_AUDIT_BOOT         = 0,
    SIGMA_AUDIT_AUTH         = 1,  // Login / biometric
    SIGMA_AUDIT_SYSCALL      = 2,
    SIGMA_AUDIT_NET_CONNECT  = 3,
    SIGMA_AUDIT_FILE_ACCESS  = 4,
    SIGMA_AUDIT_PRIVILEGE    = 5,  // Capability escalation attempts
    SIGMA_AUDIT_SHARD_LOAD   = 6,
    SIGMA_AUDIT_CRYPTO       = 7,
    SIGMA_AUDIT_ANOMALY      = 8,  // IDS-flagged events
} SigmaAuditEventType;

typedef struct {
    uint64_t           timestamp_ns;  // Nanosecond kernel monotonic clock
    SigmaAuditEventType type;
    uint32_t           pid;
    uint32_t           uid;
    char               message[SIGMA_AUDIT_MSG_LEN];
    uint8_t            prev_hash[32]; // SHA-256 chain link (tamper detection)
    uint8_t            this_hash[32];
} SigmaAuditEntry;

static SigmaAuditEntry audit_ring[SIGMA_AUDIT_MAX_ENTRIES];
static uint32_t        audit_head = 0;

// Write a new audit event (hashed and chained atomically)
void audit_log(SigmaAuditEventType type, uint32_t pid, const char* message);

// Verify integrity of the entire audit chain (detect tampering)
bool audit_verify_chain(void);

// Export the audit ring to the VFS for persistence
void audit_flush_to_disk(const char* mount_point);



