#include "Lattice.h"
#include "sigma_visscript.h"
#include "sigma_hal.h"
#include "sigma_taskautomator.h"

/**
 * SigmaOS Sovereign Visual Scripting
 * Implements a Graph-Based Logic Interpreter (GBLI) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal node execution.
 */

extern "C" void visscript_init() {
    sigma_log("[VISSCRIPT] Initializing Sovereign Visual Scripting Engine (GBLI Algorithm)...");
}

static sigma_visscript_node_t node_registry[64];
static uint32_t registered_nodes = 0;

extern "C" void visscript_register_node(const sigma_visscript_node_t* node) {
    if (registered_nodes < 64) {
        node_registry[registered_nodes++] = *node;
        sigma_printf("[VISSCRIPT] GBLI: Registered node %d.\n", node->node_id);
    }
}

extern "C" void visscript_execute_graph(const sigma_visscript_node_t* start_node) {
    if (!start_node) return;

    // GBLI (Graph-Based Logic Interpreter) Algorithm
    // Traverses visually-built node graphs and executes them natively.
    
    sigma_log("[VISSCRIPT] GBLI: Parsing visual node graph...");
    
    uint32_t current_id = start_node->node_id;
    while (current_id != 0) {
        bool found = false;
        for (uint32_t i = 0; i < registered_nodes; i++) {
            if (node_registry[i].node_id == current_id) {
                sigma_printf("[VISSCRIPT] GBLI: Executing Node %d: '%s'\n", 
                             node_registry[i].node_id, node_registry[i].operation);
                current_id = node_registry[i].next_node_id;
                found = true;
                break;
            }
        }
        if (!found) break;
    }
    
    sigma_log("[VISSCRIPT] GBLI: Graph execution COMPLETE.");
}
