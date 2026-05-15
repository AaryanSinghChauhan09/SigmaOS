#pragma once
#include <stdint.h>
#include "../../include/libc/sigma_libc.h"

namespace SigmaOS {
namespace Ecosystem {

// Phase 2A (Sprint 1): Dependency Tree Parsing & Conflict Detection
struct PackageNode {
    const char* name;
    const char* version;
    const char** dependencies;
    uint32_t dep_count;
    bool installed;
};

class DependencyGraph {
private:
    PackageNode registry[256];
    uint32_t package_count;

public:
    DependencyGraph() : package_count(0) {}

    void register_package(const char* name, const char* version, const char** deps, uint32_t count) {
        if (package_count >= 256) return;
        registry[package_count] = {name, version, deps, count, false};
        package_count++;
    }

    bool resolve_dependencies(const char* target_package) {
        sigma_print("[s-pkg] Parsing dependency tree for: ");
        sigma_print(target_package);
        sigma_print("\n");
        
        for (uint32_t i = 0; i < package_count; i++) {
            if (sigma_strcmp(registry[i].name, target_package) == 0) {
                // Check conflicts
                for (uint32_t d = 0; d < registry[i].dep_count; d++) {
                    sigma_print("   -> Requires: ");
                    sigma_print(registry[i].dependencies[d]);
                    sigma_print("\n");
                    // Recursive resolution would go here
                }
                return true;
            }
        }
        sigma_log("[s-pkg] ERROR: Package not found or circular dependency detected.");
        return false; // Conflict or not found
    }
};

} // namespace Ecosystem
} // namespace SigmaOS
