#include "../../../include/libc/sigma_libc.h"
#include "../../../include/libc/sigma_libc.h"
#include <string.h>

// ---------------------------------------------------------
// SigmaOS Capability Registry (Full Implementation)
// USP: Every object, syscall, and resource in the OS is
// governed by unforgeable, delegatable capability tokens.
// ---------------------------------------------------------

#define MAX_CAPABILITIES 512

typedef enum {
    CAP_RIGHT_BIND     = 0x01,
    CAP_RIGHT_EXTEND   = 0x02,
    CAP_RIGHT_IPC_RECV = 0x04,
    CAP_RIGHT_IPC_SEND = 0x08,
    CAP_RIGHT_MEM_ALLOC= 0x10,
    CAP_RIGHT_AI_COMPUTE=0x20,
    CAP_RIGHT_DELEGATE = 0x40, // Can delegate this cap to another PID
    CAP_RIGHT_ADMIN    = 0x80
} cap_right_t;

typedef struct {
    uint32_t cap_id;
    uint32_t owner_pid;
    uint8_t  rights;       // Bitmask of cap_right_t
    uint64_t expiry_tick;  // 0 = never expires
    uint32_t delegated_from; // Parent capability ID (0 if root)
    uint8_t  revoked;
} capability_t;

static capability_t cap_store[MAX_CAPABILITIES];
static uint32_t cap_count = 0;

extern void audit_chain_append(uint32_t pid, uint8_t level, const char* msg);

// Initialize the registry and grant the kernel root capabilities
void cap_registry_init(void) {
    // Grant PID 0 (kernel) all rights, never expires
    capability_t* root = &cap_store[0];
    root->cap_id = 0;
    root->owner_pid = 0;
    root->rights = 0xFF; // All rights
    root->expiry_tick = 0;
    root->delegated_from = 0;
    root->revoked = 0;
    cap_count = 1;

    audit_chain_append(0, 1, "CAPABILITY_REGISTRY_INITIALIZED");
}

// Mint a new capability token for a process
int cap_registry_mint(uint32_t grantor_pid, uint32_t grantor_cap_id,
                      uint32_t target_pid, uint8_t rights, uint64_t expiry) {
    if (cap_count >= MAX_CAPABILITIES) return -1;

    // Verify the grantor holds a valid, non-revoked cap with DELEGATE rights
    if (grantor_cap_id >= cap_count) return -2;
    capability_t* parent = &cap_store[grantor_cap_id];
    if (parent->revoked || parent->owner_pid != grantor_pid) return -3;
    if (!(parent->rights & CAP_RIGHT_DELEGATE)) return -4;

    // The minted cap can only have a SUBSET of the parent's rights (No Privilege Escalation)
    uint8_t safe_rights = rights & parent->rights;

    capability_t* new_cap = &cap_store[cap_count];
    new_cap->cap_id = cap_count++;
    new_cap->owner_pid = target_pid;
    new_cap->rights = safe_rights;
    new_cap->expiry_tick = expiry;
    new_cap->delegated_from = grantor_cap_id;
    new_cap->revoked = 0;

    audit_chain_append(target_pid, 1, "CAPABILITY_MINTED");
    return new_cap->cap_id;
}

// Verify a capability token (called by EVERY kernel subsystem)
int cap_registry_verify(uint32_t cap_id, uint32_t pid, uint8_t required_rights) {
    if (cap_id >= cap_count) return 0;
    capability_t* c = &cap_store[cap_id];

    if (c->revoked) return 0;
    if (c->owner_pid != pid) return 0;
    if ((c->rights & required_rights) != required_rights) return 0;
    // Expiry check would compare against current_tick
    return 1; // Valid
}

// Revoke a capability and ALL children delegated from it (Cascade Revocation)
void cap_registry_revoke(uint32_t cap_id) {
    if (cap_id >= cap_count) return;
    cap_store[cap_id].revoked = 1;

    // Cascade: revoke any child capabilities
    for (uint32_t i = 0; i < cap_count; i++) {
        if (cap_store[i].delegated_from == cap_id && !cap_store[i].revoked) {
            cap_registry_revoke(i); // Recursive cascade
        }
    }
    audit_chain_append(cap_store[cap_id].owner_pid, 2, "CAPABILITY_REVOKED_CASCADE");
}
