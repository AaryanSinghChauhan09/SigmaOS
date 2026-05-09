#include "core/sigma_types.h"
#include "libc/SovereignLibC.h"
#include "hal/sigma_hal.h"
#include "../AgentBase.h"

/**
 * AI Orchestration: Command Interpreter
 * Parses and executes short sovereign commands.
 * Commands: agent.start, agent.quota set=..., agent.task run=..., agent.sync
 */

extern "C" void trigger_emergency_sync();

class CommandInterpreter {
public:
    static CommandInterpreter& getInstance() {
        static CommandInterpreter instance;
        return instance;
    }

    void parseAndExecute(const char* command) {
        if (!command) return;

        if (sigma_hardened_strncmp(command, "agent.start", 11) == 0) {
            sigma_log("[ORCHESTRATOR] Booting autonomous agent...");
        } 
        else if (sigma_hardened_strncmp(command, "agent.quota set=", 16) == 0) {
            // e.g. agent.quota set=GPU:80%
            sigma_log("[ORCHESTRATOR] Assigning quota...");
        } 
        else if (sigma_hardened_strncmp(command, "agent.task run=", 15) == 0) {
            sigma_log("[ORCHESTRATOR] Executing task...");
        } 
        else if (sigma_hardened_strncmp(command, "agent.sync", 10) == 0) {
            sigma_log("[ORCHESTRATOR] Triggering Emergency Lattice Sync...");
            trigger_emergency_sync();
        } else {
            sigma_log("[ORCHESTRATOR] Unknown command. Fallback to recovery.");
        }
    }
};

extern "C" void execute_agent_command(const char* command) {
    CommandInterpreter::getInstance().parseAndExecute(command);
}
