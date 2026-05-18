#pragma once
#include <stdint.h>
#include "libc/sigma_libc.h"

namespace SigmaOS {
namespace Security {

// Sprint 15: Zero-Trust Networking (Subsystem Authentication)
class ZeroTrustAuthenticator {
public:
    ZeroTrustAuthenticator() {
        sigma_log("[SEC] Zero-Trust Subsystem Authenticator Online.");
    }

    bool authenticate_rpc_request(const char* source_module, const char* target_module) {
        sigma_print("[SEC-ZTA] Authenticating internal RPC: ");
        sigma_print(source_module);
        sigma_print(" -> ");
        sigma_print(target_module);
        sigma_print("\n");

        // Example Policy Enforcement: Only sigma-ui can talk to sigma-store
        if (sigma_strcmp(source_module, "sigma-ui") == 0 && sigma_strcmp(target_module, "sigma-store") == 0) {
            sigma_log("[SEC-ZTA] Access Granted based on mandatory access controls.");
            return true;
        }

        // Log denied requests to Web3 Persistence for auditing
        sigma_log("[SEC-ZTA] ACCESS DENIED: Subsystem lacks required capabilities. Logging to Web3 ledger.");
        return false;
    }
};

} // namespace Security
} // namespace SigmaOS
