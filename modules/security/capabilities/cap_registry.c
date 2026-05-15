#include "../../../include/libc/sigma_libc.h"
#include "../../../include/libc/sigma_libc.h"
#include <string.h>

// ---------------------------------------------------------
// SigmaOS Capability Registry
// Central kernel service tracking module → capability ownership
// ---------------------------------------------------------

#define MAX_CAP_REGISTRATIONS 512
#define CAP_NAME_LEN 32

typedef struct {
    uint32_t cap_id;
    uint32_t owner_pid;
    uint32_t owner_module_id;
    char     resource_name[CAP_NAME_LEN];
    uint8_t  rights;
    uint8_t  auto_revoke_on_exit; // If 1, revoked when owner_pid exits
} cap_registration_t;

static cap_registration_t cap_registry[MAX_CAP_REGISTRATIONS];
static uint32_t cap_reg_count = 0;

// Register a capability in the central registry
int cap_registry_register(uint32_t cap_id, uint32_t pid, uint32_t module_id,
                           const char* resource, uint8_t rights, uint8_t auto_revoke) {
    if (cap_reg_count >= MAX_CAP_REGISTRATIONS) return -1;
    cap_registration_t* r = &cap_registry[cap_reg_count++];
    r->cap_id = cap_id;
    r->owner_pid = pid;
    r->owner_module_id = module_id;
    r->rights = rights;
    r->auto_revoke_on_exit = auto_revoke;
    strncpy(r->resource_name, resource, CAP_NAME_LEN - 1);
    return 0;
}

// Auto-revoke all capabilities for a process when it exits
int cap_registry_revoke_pid(uint32_t pid) {
    int revoked = 0;
    for (uint32_t i = 0; i < cap_reg_count; i++) {
        if (cap_registry[i].owner_pid == pid && cap_registry[i].auto_revoke_on_exit) {
            cap_registry[i].rights = 0;  // Strip all rights
            cap_registry[i].owner_pid = 0;
            revoked++;
        }
    }
    return revoked;
}

// Query: list all capabilities held by a module
int cap_registry_query_module(uint32_t module_id, cap_registration_t* out, uint32_t max_out) {
    uint32_t found = 0;
    for (uint32_t i = 0; i < cap_reg_count && found < max_out; i++) {
        if (cap_registry[i].owner_module_id == module_id && cap_registry[i].rights != 0) {
            out[found++] = cap_registry[i];
        }
    }
    return (int)found;
}

// Verify capability ownership before IPC / syscall
int cap_registry_verify(uint32_t cap_id, uint32_t pid, uint8_t required_rights) {
    for (uint32_t i = 0; i < cap_reg_count; i++) {
        if (cap_registry[i].cap_id == cap_id && cap_registry[i].owner_pid == pid) {
            return (cap_registry[i].rights & required_rights) == required_rights;
        }
    }
    return 0; // Not found or insufficient rights
}
