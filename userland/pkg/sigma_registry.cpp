/**
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN PACKAGE REGISTRY (Phase E)
 * =========================================================================
 * Fleshing out the reproducible .spkg registry and community recipe pipeline.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_error_codes.h"
#include <sigma_libc.h>

namespace SigmaOS {
namespace Packaging {

#define MAX_REGISTRY_ENTRIES 1024

struct PackageRegistryEntry {
    char name[64];
    char version[32];
    char hash[65]; // SHA-256 hex string
    bool is_installed;
};

static PackageRegistryEntry g_registry[MAX_REGISTRY_ENTRIES];
static sigma_u32 g_registry_count = 0;

// -------------------------------------------------------------------------
// Registry Management
// -------------------------------------------------------------------------
void sigma_registry_init() {
    sys_print("[PackageRegistry] Initializing local .spkg registry.\n");
    sigma_memset(g_registry, 0, sizeof(g_registry));
    g_registry_count = 0;
}

sigma_status sigma_registry_add_recipe(const char* name, const char* version, const char* hash) {
    if (g_registry_count >= MAX_REGISTRY_ENTRIES) {
        sys_print("[PackageRegistry] Registry full! Cannot add recipe %s.\n", name);
        return K_ERR_NO_MEM; // Or generic error
    }

    PackageRegistryEntry* entry = &g_registry[g_registry_count++];
    sigma_strncpy(entry->name, name, sizeof(entry->name));
    sigma_strncpy(entry->version, version, sizeof(entry->version));
    sigma_strncpy(entry->hash, hash, sizeof(entry->hash));
    entry->is_installed = false;

    sys_print("[PackageRegistry] Added recipe: %s-%s [%s]\n", entry->name, entry->version, entry->hash);
    return SIGMA_SUCCESS;
}

sigma_status sigma_registry_mark_installed(const char* name) {
    for (sigma_u32 i = 0; i < g_registry_count; i++) {
        if (sigma_strcmp(g_registry[i].name, name) == 0) {
            g_registry[i].is_installed = true;
            sys_print("[PackageRegistry] Marked %s as installed.\n", name);
            return SIGMA_SUCCESS;
        }
    }
    sys_print("[PackageRegistry] Recipe %s not found in registry.\n", name);
    return K_ERR_INVAL; // Not found
}

// -------------------------------------------------------------------------
// Recipe Pipeline (Simulated)
// -------------------------------------------------------------------------
sigma_status sigma_registry_fetch_community_recipes() {
    sys_print("[PackageRegistry] Fetching community recipes from Sovereign mirror...\n");
    
    // Simulate fetching
    sigma_registry_add_recipe("zenith-browser", "1.0.0", "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4");
    sigma_registry_add_recipe("sigma-clang", "18.1.0", "f1e2d3c4b5a6f1e2d3c4b5a6f1e2d3c4");
    sigma_registry_add_recipe("core-utils", "2.1.4", "09876543210987654321098765432109");
    
    sys_print("[PackageRegistry] Community recipes synchronized.\n");
    return SIGMA_SUCCESS;
}

} // namespace Packaging
} // namespace SigmaOS

extern "C" {
    void sigma_registry_init_c() {
        SigmaOS::Packaging::sigma_registry_init();
    }
    
    void sigma_registry_sync_c() {
        SigmaOS::Packaging::sigma_registry_fetch_community_recipes();
    }
}
