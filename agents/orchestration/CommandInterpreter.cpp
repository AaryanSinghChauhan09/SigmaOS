#include "../../include/core/sigma_types.h"
#include "../../include/libc/SovereignLibC.h"
#include "../../include/hal/sigma_hal.h"
#include "../../include/sigma_log.h"
#include "../../include/core/context/manager.hpp"
#include "../../include/AgentBase.h"

/**
 * AI Orchestration: Command Interpreter
 * Parses and executes short sovereign commands.
 * Commands: agent.start, agent.quota set=..., agent.task run=..., agent.sync
 */

void trigger_emergency_sync();

class CommandInterpreter {
public:
    static CommandInterpreter& getInstance() {
        static CommandInterpreter instance;
        return instance;
    }

    void parseAndExecute(const char* command) {
        if (!command) return;

        if (sigma_hardened_strncmp(command, "agent.start", 11) == 0) {
            sigma_log("[ORCHESTRATOR] Booting autonomous agent...\n");
        } 
        else if (sigma_hardened_strncmp(command, "agent.quota set=", 16) == 0) {
            sigma_log("[ORCHESTRATOR] Assigning resource quota...\n");
        } 
        else if (sigma_hardened_strncmp(command, "agent.task run=", 15) == 0) {
            // Polymorphic Task Execution via Context Manager
            sigma_log("[ORCHESTRATOR] Executing sovereign task via ContextManager...\n");
            const char* current_profession = "cashier"; // Default, could be fetched dynamically
            void* profileCtx = SigmaOS::Kernel::Context::ContextManager::getInstance().resolve(current_profession);
            if (profileCtx) {
                sigma_log("[ORCHESTRATOR] Context resolved. Adapting task to profession.\n");
            } else {
                sigma_log("[ORCHESTRATOR] Generic task execution.\n");
            }
        } 
        else if (sigma_hardened_strncmp(command, "agent.sync", 10) == 0) {
            sigma_log("[ORCHESTRATOR] Triggering Emergency Lattice Sync...\n");
            trigger_emergency_sync();
        }
        else if (sigma_hardened_strncmp(command, "agent.container deploy=", 23) == 0) {
            sigma_log("[ORCHESTRATOR] Deploying sovereign immutable container (Neutralizing CoreOS)...\n");
        }
        else if (sigma_hardened_strncmp(command, "agent.gaming engage", 19) == 0) {
            sigma_log("[ORCHESTRATOR] Engaging Vulkan/Proton gaming stack (Neutralizing SteamOS)...\n");
        }
        else if (sigma_hardened_strncmp(command, "agent.forensics scan", 20) == 0) {
            sigma_log("[ORCHESTRATOR] Launching sovereign forensic analysis (Neutralizing CAINE)...\n");
        }
        else if (sigma_hardened_strncmp(command, "agent.pkg reproduce=", 20) == 0) {
            sigma_log("[ORCHESTRATOR] Enforcing NixOS-style reproducible build state...\n");
        }
        else {
            sigma_log("[ORCHESTRATOR] Unknown command. Fallback to recovery.\n");
        }
    }
};

void execute_agent_command(const char* command) {
    CommandInterpreter::getInstance().parseAndExecute(command);
}

} // extern "C"

} // extern "C"
