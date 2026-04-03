// SOVEREIGN OMNI-CLI USERLAND
// Interactive Terminal Agent Interface

#include <stdio.h>
#include <string.h>

// Simulated external link to SovereignOmniAgent
extern void SovereignOmniAgent_Init();
extern unsigned char SovereignOmniAgent_AnalyzeCodebase(const char* target_directory);
extern unsigned char SovereignOmniAgent_ExecuteRoutine(const char* routine_signature);

extern void SovereignNetData_Poll();
extern void SovereignOrchestrator_RunDAG();
extern void SovereignMCP_Dispatch(const char* intent);

void print_omni_prompt() {
    printf("\nSigmaOS Omni-Agent \033[36m(Sovereign Mode)\033[0m\n");
    printf("Type commands (e.g., 'workflow', 'telemetry', 'context', 'commit changes')\n");
    printf("sigma> ");
}

int main(int argc, char** argv) {
    SovereignOmniAgent_Init();
    
    // In a real execution, arguments bypass interactive mode
    if (argc > 1) {
        printf("[DEBUG] Autonomous Routine Execution Triggered: %s\n", argv[1]);
        SovereignOmniAgent_ExecuteRoutine(argv[1]);
        return 0;
    }

    char input_buffer[1024];

    while (1) {
        print_omni_prompt();
        if (!fgets(input_buffer, sizeof(input_buffer), stdin)) {
            break;
        }

        // Strip newline
        input_buffer[strcspn(input_buffer, "\n")] = 0;
        
        if (strcmp(input_buffer, "exit") == 0 || strcmp(input_buffer, "quit") == 0) {
            printf("Omni-Agent disengaged.\n");
            break;
        }

        if (strstr(input_buffer, "telemetry") != NULL || strstr(input_buffer, "netdata") != NULL) {
            SovereignNetData_Poll();
        } else if (strstr(input_buffer, "workflow") != NULL || strstr(input_buffer, "dag") != NULL) {
            SovereignOrchestrator_RunDAG();
        } else if (strstr(input_buffer, "context") != NULL || strstr(input_buffer, "mcp") != NULL) {
            SovereignMCP_Dispatch("context");
        } else if (strstr(input_buffer, "explain") != NULL) {
            printf("[OMNI-AGENT] Parsing AST in local context...\n");
            SovereignOmniAgent_AnalyzeCodebase("./");
        } else if (strstr(input_buffer, "commit") != NULL || strstr(input_buffer, "git") != NULL) {
            printf("[OMNI-AGENT] Generating secure delta map natively.\n");
            SovereignOmniAgent_ExecuteRoutine("VCS_SYNC");
        } else if (strlen(input_buffer) > 0) {
            printf("[OMNI-AGENT] Awaiting further semantic context for: %s\n", input_buffer);
        }
    }

    return 0;
}
