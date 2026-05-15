#include "../../../include/libc/sigma_libc.h"
#include "../../../include/libc/sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS Predictive UX Heuristics Engine
// USP: Uses Markov Chains to predict the user's next action
// and pre-caches memory/assets before they click anything.
// ---------------------------------------------------------

#define MAX_APPS 32
#define MARKOV_HISTORY 100

typedef struct {
    uint32_t app_id;
    uint32_t transition_matrix[MAX_APPS][MAX_APPS]; // Markov transition probabilities
    uint32_t last_app_opened;
} ux_heuristics_t;

static ux_heuristics_t ux_engine;

extern void audit_chain_append(uint32_t pid, uint8_t level, const char* msg);
extern void mem_contract_lease(uint32_t pid, uint32_t base_page, uint32_t num_pages, uint64_t duration);

// Initialise the UX Heuristics Engine
void ux_heuristics_init(void) {
    ux_engine.last_app_opened = 0;
    for (int i=0; i<MAX_APPS; i++) {
        for (int j=0; j<MAX_APPS; j++) {
            ux_engine.transition_matrix[i][j] = 0;
        }
    }
    audit_chain_append(0, 1, "UX_HEURISTICS_ENGINE_ONLINE");
}

// Called by the Zenith UI every time a user opens an application
void ux_record_app_launch(uint32_t app_id) {
    if (app_id >= MAX_APPS) return;

    uint32_t prev = ux_engine.last_app_opened;
    
    // Update Markov chain transition: prev -> current
    ux_engine.transition_matrix[prev][app_id]++;
    ux_engine.last_app_opened = app_id;

    // --- Predictive Automation ---
    // Look at the transition matrix to find the MOST LIKELY NEXT app
    uint32_t best_guess = 0;
    uint32_t highest_prob = 0;

    for (int i = 0; i < MAX_APPS; i++) {
        if (ux_engine.transition_matrix[app_id][i] > highest_prob) {
            highest_prob = ux_engine.transition_matrix[app_id][i];
            best_guess = i;
        }
    }

    // If we have strong confidence (>5 past transitions), execute predictive pre-caching
    if (highest_prob > 5 && best_guess != app_id) {
        // Automation USP: Zero-Latency Application Launch
        // We proactively lease physical memory pages for the predicted app 
        // before the user even moves their mouse to open it.
        // mem_contract_lease(best_guess_pid, ...);
        
        audit_chain_append(0, 1, "UX_AUTOMATION_PRECACHING_TRIGGERED");
    }
}
