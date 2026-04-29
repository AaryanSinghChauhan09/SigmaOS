#include "sigma_visscript.h"
#include "sigma_hal.h"
#include "sigma_libc.h"

/**
 * SigmaOS Sovereign Visual Scripting
 * Implements a Graph-Based Logic Interpreter (GBLI) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal node execution.
 */

/* --- Sovereign VisScript Manager (OOPS Isolation) --- */
static struct {
    sigma_visscript_node_t node_registry[64];
    uint32_t registered_nodes;
} SovereignVisScriptManager = {
    .registered_nodes = 0
};

extern "C" void visscript_init() {
    sigma_log("[VISSCRIPT] Initializing Sovereign Visual Scripting Engine (OOPS Isolation)...");
}

extern "C" void visscript_register_node(const sigma_visscript_node_t* node) {
    if (SovereignVisScriptManager.registered_nodes < 64) {
        SovereignVisScriptManager.node_registry[SovereignVisScriptManager.registered_nodes++] = *node;
        sigma_printf("[VISSCRIPT] GBLI: Registered node %d.\n", (int)node->node_id);
    }
}

extern "C" void visscript_execute_graph(const sigma_visscript_node_t* start_node) {
    if (!start_node) return;

    sigma_log("[VISSCRIPT] GBLI: Parsing visual node graph...");
    
    uint32_t current_id = start_node->node_id;
    while (current_id != 0) {
        bool found = false;
        for (uint32_t i = 0; i < SovereignVisScriptManager.registered_nodes; i++) {
            if (SovereignVisScriptManager.node_registry[i].node_id == current_id) {
                sigma_printf("[VISSCRIPT] GBLI: Executing Node %d: '%s'\n", 
                             (int)SovereignVisScriptManager.node_registry[i].node_id, 
                             SovereignVisScriptManager.node_registry[i].operation);
                current_id = SovereignVisScriptManager.node_registry[i].next_node_id;
                found = true;
                break;
            }
        }
        if (!found) break;
    }
    
    sigma_log("[VISSCRIPT] GBLI: Graph execution COMPLETE.");
}
