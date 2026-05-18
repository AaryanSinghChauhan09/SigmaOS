// =============================================================================
// SigmaOS — S08_Security — SovereignIntrusionDetection.c
// Behavioural Intrusion Detection System (IDS) Shard
// =============================================================================
// Competitor USPs Absorbed:
//   • Linux eBPF/Falco  — kernel syscall tracing, runtime behavioural rules
//   • Windows Defender  — process hollowing, memory injection detection
//   • macOS XProtect    — signature + heuristic scanning on exec()
//   • Snort/Suricata    — network anomaly rules engine (inline IDS mode)
//   • OSSEC             — log-based correlation, rootkit detection
// Architecture:
//   • Hooks into SCI syscall dispatch — zero-overhead eBPF-style probes
//   • Rule engine: evaluates behaviour chains, not just signatures
//   • Network IDS: plugged into S07 packet filter post-routing hook
//   • Anomaly baseline: learns normal shard behaviour via S09 inference
//   • On alert: logs to S08 AuditLog, optionally kills offending PID
// =============================================================================

#include "core/sigma_types.h"


#define IDS_MAX_RULES     512
#define IDS_RULE_NAME_LEN  64
#define IDS_MAX_ALERTS    2048

// ── Alert Severity ────────────────────────────────────────────────────────────
typedef enum {
    IDS_SEV_LOW      = 0,
    IDS_SEV_MEDIUM   = 1,
    IDS_SEV_HIGH     = 2,
    IDS_SEV_CRITICAL = 3,  // Auto-kill + lockdown
} IDSSeverity;

// ── Rule Types ────────────────────────────────────────────────────────────────
typedef enum {
    IDS_RULE_SYSCALL_SEQUENCE  = 0,  // Detect dangerous syscall chains
    IDS_RULE_NETWORK_SIGNATURE = 1,  // Snort-style network pattern
    IDS_RULE_MEMORY_ANOMALY    = 2,  // Detect mmap(EXEC) injection (Defender)
    IDS_RULE_FILE_INTEGRITY    = 3,  // Hash-verified file tamper check (OSSEC)
    IDS_RULE_PRIVILEGE_ESCALATE= 4,  // cap_check violation chain
    IDS_RULE_AI_BEHAVIOURAL    = 5,  // S09 NPU anomaly baseline deviation
} IDSRuleType;

// ── Rule Descriptor ────────────────────────────────────────────────────────────
typedef struct {
    char        name[IDS_RULE_NAME_LEN];
    IDSRuleType type;
    IDSSeverity severity;
    bool        auto_kill;    // Kill offending PID on trigger
    bool        auto_lockdown;// Invoke sec_fw_lockdown_node() on CRITICAL
    uint8_t     pattern[64];  // Rule payload (depends on type)
    uint32_t    pattern_len;
} IDSRule;

// ── Alert Record ──────────────────────────────────────────────────────────────
typedef struct {
    uint64_t    timestamp_ns;
    uint32_t    offending_pid;
    IDSRule*    rule_triggered;
    char        detail[128];
    IDSSeverity severity;
} IDSAlert;

static IDSRule  rule_table[IDS_MAX_RULES];
static IDSAlert alert_ring[IDS_MAX_ALERTS];
static uint32_t rule_count  = 0;
static uint32_t alert_head  = 0;

// ── Public API ────────────────────────────────────────────────────────────────

// Register a new IDS rule
bool ids_add_rule(IDSRule* rule);

// Probe hook: called by SCI for every syscall (zero-overhead fast path)
void ids_syscall_probe(uint32_t pid, uint32_t syscall_id, uint64_t arg0);

// Network probe: called by S07 packet filter with every inbound packet
void ids_network_probe(const uint8_t* pkt, uint32_t len, uint32_t src_ip);

// Evaluate AI behavioural baseline deviation (calls S09 AiInferenceEngine)
void ids_ai_evaluate_process(uint32_t pid);

// Drain alert ring to S08 AuditLog and optionally to syslog socket
uint32_t ids_drain_alerts(IDSAlert* out, uint32_t max);

// Load default sovereign ruleset (syscall + network signatures)
void ids_load_default_rules(void);



