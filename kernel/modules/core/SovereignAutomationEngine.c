/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN AUTOMATION ENGINE (v1.0)
 * =========================================================================
 * Mission: Native execution of .sigma automation scripts.
 * Design: C11 / Zero-Dependency / CLI-Tier Logic Dispatch.
 * Standard: Zenith Industrial Sovereignty.
 * =========================================================================
 */

#include "../../include/sigma_kernel.h"
#include "../../include/SovereignCLI.h"

// -------------------------------------------------------------------------
// Automation Engine Structure
// -------------------------------------------------------------------------

CLASS_DECLARE(SovereignAutomationEngine) {
    SigmaObject_t core;
    sigma_u32     scripts_run;
    sigma_u32     total_commands;
    
    // Virtual Methods
    VIRTUAL(sigma_err_t, execute_script, struct SovereignAutomationEngine* self, const char* script_content);
    VIRTUAL(void, audit_automation, struct SovereignAutomationEngine* self);
};

// -------------------------------------------------------------------------
// Implementation: Script Execution (Line-by-Line Tokenization)
// -------------------------------------------------------------------------

static sigma_err_t sigma_auto_execute(SovereignAutomationEngine_t* self, const char* script) {
    sigma_printf("[AUTOMATION]: Executing Sovereign Script (%u bytes)...\n", (unsigned int)sigma_strlen(script));
    
    char line[256];
    const char* p = script;
    
    while (*p) {
        // Skip whitespace/newlines
        while (*p == '\n' || *p == '\r' || *p == ' ') p++;
        if (!*p) break;
        
        // Extract line
        sigma_u32 i = 0;
        while (*p && *p != '\n' && *p != '\r' && i < 255) {
            line[i++] = *p++;
        }
        line[i] = '\0';
        
        // Ignore comments
        if (line[0] == '#') continue;
        
        // Dispatch to Sovereign CLI
        sigma_printf("[AUTO_RUN]: %s\n", line);
        sigma_cli_dispatch(&g_sigma_cli, line);
        self->total_commands++;
    }
    
    self->scripts_run++;
    sigma_printf("[SUCCESS]: Automation mission complete.\n");
    return SIGMA_OK;
}

static void sigma_auto_audit(SovereignAutomationEngine_t* self) {
    sigma_printf("\n--- SOVEREIGN AUTOMATION AUDIT ---\n");
    sigma_printf("SCRIPTS_RUN:    %u\n", (unsigned int)self->scripts_run);
    sigma_printf("TOTAL_CMDS:     %u\n", (unsigned int)self->total_commands);
    sigma_printf("ENGINE_STATE:   ZENITH_OPTIMAL\n");
    sigma_printf("----------------------------------\n");
}

// -------------------------------------------------------------------------
// Factory / Constructor
// -------------------------------------------------------------------------

SovereignAutomationEngine_t SovereignAutomationEngine_Create() {
    SovereignAutomationEngine_t e;
    sigma_object_init(&e.core, "SovereignAutomationEngine", 909);
    
    e.scripts_run = 0;
    e.total_commands = 0;
    
    e.execute_script = sigma_auto_execute;
    e.audit_automation = sigma_auto_audit;
    
    return e;
}

void SovereignAutomationEngine_Init() {
    sigma_printf("[SOC]: Initializing Native Automation Engine (SigmaScript v1.0)...\n");
}
