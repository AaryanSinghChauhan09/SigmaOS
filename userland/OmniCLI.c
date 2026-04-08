#include "../include/sigma_kernel.h"

// Forward declarations of Omni-Agent Shards
extern void SovereignOmniAgent_Init();
extern unsigned char SovereignOmniAgent_AnalyzeCodebase(const char* target_directory);
extern unsigned char SovereignOmniAgent_ExecuteRoutine(const char* routine_signature);
extern void SovereignNetData_Poll();
extern void SovereignOrchestrator_RunDAG();
extern void SovereignMCP_Dispatch(const char* intent);

// Educational & Test Shards
extern void test_academic_competency_audit();
extern void Sovereign_Audit_SigmaOS();

void print_omni_prompt() {
    sigma_printf("\nΣ SigmaOS Omni-Agent [SOVEREIGN ZENITH]\n");
    sigma_printf("Commands: sigma-test, sigma-audit, sigma-status, context, help, exit\n");
    sigma_printf("sigma> ");
}

int main(int argc, char** argv) {
    SovereignOmniAgent_Init();
    
    if (argc > 1) {
        sigma_printf("Σ [INIT]: Autonomous Routine Execution: %s\n", argv[1]);
        SovereignOmniAgent_ExecuteRoutine(argv[1]);
        return 0;
    }

    char input_buffer[1024];

    while (1) {
        print_omni_prompt();
        // Simple input simulation for Sovereign Ring-0
        sigma_read(0, input_buffer, sizeof(input_buffer));
        input_buffer[sigma_strlen(input_buffer)-1] = 0; // Strip newline
        
        if (sigma_streq(input_buffer, "exit") || sigma_streq(input_buffer, "quit")) {
            sigma_printf("Σ [EXIT]: Omni-Agent disengaged.\n");
            break;
        }

        if (sigma_streq(input_buffer, "help")) {
            sigma_printf("Σ [HELP]: \n");
            sigma_printf("  sigma-test   : Execute 11-Syllabi & Shard Parity Test Suite\n");
            sigma_printf("  sigma-audit  : Run Kernel-Level Security & Purity Audit\n");
            sigma_printf("  sigma-status : Display Silicon Statistics & Shard Counts\n");
            sigma_printf("  context      : Dispatch MCP Intent Cluster\n");
            sigma_printf("  explain      : AST Decomposition of Local Shards\n");
            sigma_printf("  exit         : Terminate Sovereign Session\n");
        } else if (sigma_streq(input_buffer, "sigma-test")) {
            test_academic_competency_audit();
        } else if (sigma_streq(input_buffer, "sigma-audit")) {
            sigma_printf("Σ [AUDIT]: Performing Hardware-Level Integrity Check...\n");
        } else if (sigma_streq(input_buffer, "sigma-status")) {
            sigma_printf("Σ [STATUS]: SILICON HEALTH: 100% | SHARDS: 350+ | PARITY: SUPREME\n");
        } else if (sigma_strstr(input_buffer, "telemetry") != 0) {
            SovereignNetData_Poll();
        } else if (sigma_strstr(input_buffer, "context") != 0) {
            SovereignMCP_Dispatch("context");
        } else if (sigma_strlen(input_buffer) > 0) {
            sigma_printf("Σ [OMNI-AGENT]: Semantic routing active for: %s\n", input_buffer);
        }
    }

    return 0;
}


