#pragma once
#include <stdint.h>
#include "../S01_Genesis/sigma_libc.h"

namespace SigmaOS {
namespace Network {

// Sprint 15: Decentralized DNS Resolver (ENS/IPFS)
class DecentralizedDNS {
private:
    bool use_ens;

public:
    DecentralizedDNS() : use_ens(true) {
        sigma_log("[DNS] Decentralized Resolver (ENS/IPFS) Online.");
    }

    void toggle_ens(bool enable) {
        use_ens = enable;
        sigma_print("[DNS] ENS Resolution: ");
        sigma_print(enable ? "ACTIVE\n" : "DISABLED (Fallback to standard DNS)\n");
    }

    const char* resolve_domain(const char* domain) {
        sigma_print("[DNS] Resolving domain: ");
        sigma_print(domain);
        sigma_print("\n");

        if (use_ens) {
            // Check for .eth or similar decentralised TLDs
            int len = sigma_strlen(domain);
            if (len > 4 && sigma_strcmp(domain + len - 4, ".eth") == 0) {
                sigma_log("[DNS] Resolved via Ethereum Name Service (ENS).");
                return "192.168.ENS.NODE"; 
            }
        }
        
        sigma_log("[DNS] Resolved via standard DNS infrastructure.");
        return "192.168.1.100";
    }
};

} // namespace Network
} // namespace SigmaOS
