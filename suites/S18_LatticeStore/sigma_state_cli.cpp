#include "Web3Persistence.hpp"

namespace SigmaOS {
namespace CLI {

// Handler for `sigma-state sync --decentralized`
void handle_sigma_state(int argc, char** argv) {
    static Persistence::Web3StateLedger ledger;
    
    if (argc > 1 && sigma_strcmp(argv[1], "sync") == 0) {
        if (argc > 2 && sigma_strcmp(argv[2], "--decentralized") == 0) {
            ledger.toggle_persistence(true);
            ledger.sync_state();
        } else {
            sigma_log("[CLI] Running local state sync.");
        }
    } else {
        sigma_log("Usage: sigma-state sync [--decentralized]");
    }
}

} // namespace CLI
} // namespace SigmaOS
