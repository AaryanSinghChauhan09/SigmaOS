// =============================================================================
// SigmaOS — S08_Security — SovereignFirewallIPS.c
// Zero-Trust Intelligent Network Defense Shard
// =============================================================================
// Competitor USPs Absorbed:
//   • Little Snitch (macOS) — App-level outbound connection control
//   • Windows Firewall / Defender — Port and signature-based blocking
//   • pf (OpenBSD) — Statefully tracking and packet normalization
//   • Falco/eBPF (Linux) — Kernel-level network observability
// Exceeding Competitors:
//   • Real-time AI Anomaly Detection: Blocks traffic that "feels" wrong for an app
//   • Per-Bundle Sandbox: Each .sab bundle has an isolated micro-firewall
//   • Invisible Mode: Zero response to unauthorized probes (Dark Cloud parity)
// =============================================================================

#include <sigma_types.h>


#define FIREWALL_MAX_RULES  1024

typedef enum {
    FW_ACTION_ALLOW     = 0,
    FW_ACTION_DENY      = 1,
    FW_ACTION_PROMPT    = 2, // Little Snitch style notification
    FW_ACTION_STEALTH   = 3  // Deny + Log + Silent
} FirewallAction;

// ── Firewall Rule ─────────────────────────────────────────────────────────────
typedef struct {
    char           app_id[128];
    uint32_t       remote_ip;
    uint16_t       remote_port;
    uint8_t        protocol; // TCP/UDP/ICMP
    FirewallAction action;
} FirewallRule;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Zero-Trust Firewall (Hooks into S07 Network stack)
void firewall_init(void);

// Check if a packet is authorized (Kernel-level gatekeeper)
FirewallAction firewall_filter_packet(void* packet_hdr, const char* app_id);

// Add a new intelligent rule (Gated by S08 Capability)
void firewall_add_rule(FirewallRule* rule);

// Enable "Stealth Mode" (Universal drop of all unrequested traffic)
void firewall_set_stealth(bool enabled);

// Report blocked traffic to S13 Oracle for analysis
void firewall_audit_anomalies(void);

// Sync rules across the S12 Continuity Mesh (Dark Cloud parity)
void firewall_mesh_sync(void);


