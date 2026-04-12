/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN CAPABILITY SHARD (v1.0)
 * =========================================================================
 * Mission: Absorb Fuchsia/Zircon USP — Native Object Capabilities.
 * Design: C11 / Zero-Dependency / Handle-Based Access Control.
 * Standard: Zenith Industrial Sovereignty.
 * =========================================================================
 */

#include "../../include/sigma_kernel.h"

// -------------------------------------------------------------------------
// Capability Structures
// -------------------------------------------------------------------------

typedef enum {
    CAP_READ    = 0x1,
    CAP_WRITE   = 0x2,
    CAP_EXEC    = 0x4,
    CAP_DESTROY = 0x8
} SigmaCapRights_t;

typedef struct {
    char      resource_name[32];
    sigma_u32 handle_id;
    sigma_u32 rights;
    sigma_bool active;
} SigmaCapability_t;

#define MAX_CAPABILITIES 32
static SigmaCapability_t s_cap_matrix[MAX_CAPABILITIES];
static sigma_u32 s_cap_count = 0;

// -------------------------------------------------------------------------
// Capability Logic (Fuchsia Zircon/seL4 Parity)
// -------------------------------------------------------------------------

/**
 * sigma_cap_grant: Grants an industrial capability handle to a target silicon shard.
 */
sigma_u32 sigma_cap_grant(const char* resource, sigma_u32 rights) {
    if (s_cap_count >= MAX_CAPABILITIES) return 0;
    
    SigmaCapability_t* c = &s_cap_matrix[s_cap_count++];
    sigma_strcpy(c->resource_name, resource);
    c->handle_id = 0x1000 + s_cap_count;
    c->rights = rights;
    c->active = SIGMA_TRUE;
    
    sigma_printf("[CAPABILITY]: Granted industrial handle [0x%X] to '%s' with rights 0x%X.\n", 
                 c->handle_id, resource, rights);
    return c->handle_id;
}

/**
 * sigma_cap_verify: Verifies if an industrial mission has the capability to access a resource.
 */
sigma_bool sigma_cap_verify(sigma_u32 handle, sigma_u32 required_rights) {
    for (sigma_u32 i = 0; i < s_cap_count; i++) {
        if (s_cap_matrix[i].handle_id == handle && s_cap_matrix[i].active) {
            if ((s_cap_matrix[i].rights & required_rights) == required_rights) {
                return SIGMA_TRUE;
            }
        }
    }
    sigma_printf("[DENIED]: Handle [0x%X] lacks industrial rights 0x%X.\n", handle, required_rights);
    return SIGMA_FALSE;
}

// -------------------------------------------------------------------------
// Industrial Capability Audit
// -------------------------------------------------------------------------

void SovereignCapability_Audit() {
    sigma_printf("\n--- SOVEREIGN CAPABILITY AUDIT ---\n");
    sigma_printf("HANDLE       RESOURCE             RIGHTS       STATE\n");
    sigma_printf("----------------------------------------------------------\n");
    for (sigma_u32 i = 0; i < s_cap_count; i++) {
        sigma_printf("0x%-10X %-20s 0x%-10X %s\n", 
                     s_cap_matrix[i].handle_id,
                     s_cap_matrix[i].resource_name,
                     s_cap_matrix[i].rights,
                     s_cap_matrix[i].active ? "ACTIVE" : "REVOKED");
    }
    sigma_printf("----------------------------------------------------------\n");
}

// -------------------------------------------------------------------------
// Factory / Constructor
// -------------------------------------------------------------------------

void SovereignCapabilityShard_Init() {
    sigma_printf("[SOC]: Seating Native Capability Shard (Fuchsia/Zircon Parity v1.0)...\n");
    sigma_cap_grant("Kernel_Memory", CAP_READ | CAP_WRITE);
}
