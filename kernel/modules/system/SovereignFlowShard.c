/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN FLOW SHARD (v1.0)
 * =========================================================================
 * Mission: Absorb Apple Shortcuts / Zapier / IFTTT / Bash Scripting USP.
 *          Native Silicon System-Wide Automation & Flow Trigger Engine.
 * Design: C11 / Zero-Dependency / Event-Condition-Action (ECA) Dispatcher.
 * Standard: Zenith Industrial Sovereignty.
 * =========================================================================
 */

#include "../../include/sigma_kernel.h"

// -------------------------------------------------------------------------
// Flow Structures
// -------------------------------------------------------------------------

typedef struct {
    char        trigger_event[32];
    char        action_cmd[64];
    sigma_bool  enabled;
} SigmaFlow_t;

#define MAX_FLOWS 16
static SigmaFlow_t s_flow_registry[MAX_FLOWS];
static sigma_u32   s_flow_count = 0;

// -------------------------------------------------------------------------
// Flow Logic (Shortcuts / Zapier parity)
// -------------------------------------------------------------------------

/**
 * sigma_flow_register: Links a system event to a Sovereign CLI command.
 */
sigma_err_t sigma_flow_register(const char* trigger, const char* action) {
    if (s_flow_count >= MAX_FLOWS) return SIGMA_ENOSPC;
    
    SigmaFlow_t* f = &s_flow_registry[s_flow_count++];
    sigma_strcpy(f->trigger_event, trigger);
    sigma_strcpy(f->action_cmd, action);
    f->enabled = SIGMA_TRUE;
    
    sigma_printf("[FLOW]: Linked event '%s' -> Action: \"%s\".\n", trigger, action);
    return SIGMA_OK;
}

/**
 * sigma_flow_trigger: Manually fires a flow sequence.
 */
void sigma_flow_trigger(const char* trigger) {
    sigma_printf("[FLOW]: Searching for silicon triggers matching '%s'...\n", trigger);
    for (sigma_u32 i = 0; i < s_flow_count; i++) {
        if (sigma_streq(s_flow_registry[i].trigger_event, trigger)) {
            sigma_printf("  - [FIRE]: Executing Action -> \"%s\"\n", s_flow_registry[i].action_cmd);
            sigma_cli_dispatch(&g_sigma_cli, s_flow_registry[i].action_cmd);
        }
    }
}

// -------------------------------------------------------------------------
// Industrial Flow Audit
// -------------------------------------------------------------------------

void SovereignFlow_Audit() {
    sigma_printf("\n--- SOVEREIGN FLOW AUDIT ---\n");
    sigma_printf("Active Flows: %u | Engine: Silicon-ECA-Dispatcher\n", s_flow_count);
    sigma_printf("TRIGGER          ACTION_COMMAND                  STATUS\n");
    sigma_printf("-------------------------------------------------------------\n");
    for (sigma_u32 i = 0; i < s_flow_count; i++) {
        sigma_printf("%-16s %-32s %s\n", 
                     s_flow_registry[i].trigger_event, 
                     s_flow_registry[i].action_cmd,
                     s_flow_registry[i].enabled ? "ARMED" : "off");
    }
    sigma_printf("-------------------------------------------------------------\n");
}

// -------------------------------------------------------------------------
// Factory / Constructor
// -------------------------------------------------------------------------

void SovereignFlowShard_Init() {
    sigma_printf("[SOC]: Seating Native Flow Shard (Shortcuts/Zapier Parity v1.0)...\n");
    sigma_flow_register("cpu_over_90", "sigma-opt activate 0");
    sigma_flow_register("low_memory",  "sigma-gc sweep");
    sigma_flow_register("user_idle",  "sigma-clean run");
}
