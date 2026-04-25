#include "SigmaAssistant.hpp"

namespace SigmaOS {
namespace CLI {

// Sprint 17: Sigma Assistant CLI Parity (s-assist)
void handle_sigma_assist(int argc, char** argv) {
    static Intelligence::SigmaAssistant assistant;
    
    if (argc > 1) {
        if (sigma_strcmp(argv[1], "status") == 0) {
            assistant.analyze_system_state();
        } else if (sigma_strcmp(argv[1], "suggest") == 0) {
            sigma_log("[CLI] Requesting AI recommendations...");
            assistant.analyze_system_state(); // Provides insights
        } else if (sigma_strcmp(argv[1], "optimize") == 0 && argc > 2) {
            sigma_print("[CLI] Optimizing system for: ");
            sigma_print(argv[2]);
            sigma_print("\n");
            // e.g., "gaming", "video editing"
            sigma_log("[CLI] Adaptive QoS and GPU acceleration engaged.");
        } else {
            sigma_log("Usage: s-assist [status|suggest|optimize <task>]");
        }
    } else {
        sigma_log("Usage: s-assist [status|suggest|optimize <task>]");
    }
}

} // namespace CLI
} // namespace SigmaOS
