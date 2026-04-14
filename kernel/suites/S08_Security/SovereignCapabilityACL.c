// =============================================================================
// SigmaOS — S08_Security — SovereignCapabilityACL.c
// Privilege Escalation Prevention via Capability-Based ACLs
// =============================================================================
// Competitor USPs Absorbed:
//   • Linux Capabilities — fine-grained privilege split from monolithic root
//   • macOS Entitlements — code-signed capability manifests per binary
//   • OpenBSD Pledge     — syscall whitelist commitment (irrevocable)
//   • Windows ACL/Token  — access token with privilege bitfield per process
//   • SELinux MTE        — mandatory type enforcement, no DAC bypass
// Architecture:
//   • Every process holds a bitmask of 64 capabilities (not uid=0)
//   • On exec(), capability bitmask is intersected with binary's signed manifest
//   • Pledge-style: process can irreversibly DROP capabilities (never regain)
//   • Syscall dispatcher checks capability before dispatch (zero-cost fast path)
// =============================================================================

#include <stdint.h>
#include <stdbool.h>

// ── Capability Bit Definitions ───────────────────────────────────────────────
#define CAP_NET_BIND_PORT    (1ULL <<  0)  // Bind ports < 1024
#define CAP_FS_WRITE_ROOT    (1ULL <<  1)  // Write to /sys, /boot
#define CAP_LOAD_SHARD       (1ULL <<  2)  // Load kernel shards at runtime
#define CAP_RAW_SOCKET       (1ULL <<  3)  // Create raw/packet sockets
#define CAP_SET_CLOCK        (1ULL <<  4)  // Modify system time
#define CAP_MMAP_EXEC        (1ULL <<  5)  // Map executable anonymous pages
#define CAP_SYS_PTRACE       (1ULL <<  6)  // Trace other processes
#define CAP_IPC_ADMIN        (1ULL <<  7)  // Create unrestricted IPC ports
#define CAP_BIOMETRIC_ENROLL (1ULL <<  8)  // Enroll biometric templates
#define CAP_GPU_ADMIN        (1ULL <<  9)  // Direct GPU VRAM access
#define CAP_NET_ADMIN        (1ULL << 10)  // Modify routing/firewall rules
#define CAP_AUDIT_WRITE      (1ULL << 11)  // Write audit log entries

// ── Process Capability Token ──────────────────────────────────────────────────
typedef struct {
    uint32_t  pid;
    uint64_t  permitted;    // Max capability ceiling
    uint64_t  effective;    // Currently active capabilities
    uint64_t  inheritable;  // Passed to children via exec()
    bool      is_pledged;   // If true, capabilities are locked (OpenBSD pledge)
} SigmaCapToken;

// ── Public API ───────────────────────────────────────────────────────────────

// Assign a signed capability manifest to a process at exec() time
void cap_assign_from_manifest(uint32_t pid, uint64_t manifest_bitfield);

// Check if a process holds a specific capability (called by SCI on each syscall)
bool cap_check(uint32_t pid, uint64_t required_cap);

// Irrevocably drop one or more capabilities (OpenBSD pledge equivalent)
void cap_pledge_drop(uint32_t pid, uint64_t caps_to_drop);

// Restrict a newly forked child to a subset of parent's capabilities
void cap_fork_restrict(uint32_t parent_pid, uint32_t child_pid, uint64_t mask);

// Log a capability violation to the S08 audit chain
void cap_log_violation(uint32_t pid, uint64_t attempted_cap);
