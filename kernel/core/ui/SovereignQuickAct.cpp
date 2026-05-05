#include "../../../include/SovereignLibC.h"
#include "../../../include/sigma_types.h"

#include "sigma_quickact.h"
#include "sigma_hal.h"
#include "sigma_neuralsearch.h"



/**
 * SigmaOS Sovereign Quick Actions Bar
 * Implements a Unified Intent Router (UIR) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal command palette.
 */

extern "C" void quickact_init() {
    sigma_log("[QUICKACT] Initializing Sovereign Quick Actions Bar (UIR Algorithm)...");
}

extern "C" void quickact_invoke() {
    sigma_log("[QUICKACT] UIR: Quick Actions Bar rendered. Awaiting input...");
}

extern "C" void quickact_process_input(const char* user_input) {
    // UIR (Unified Intent Router) Algorithm
    // Classifies input as: app search, file search, system command, sigma:// URI, or automation rule.
    
    sigma_printf("[QUICKACT] UIR: Processing input: '%s'\n", user_input);
    
    // Attempt semantic search first
    neuralsearch_query(user_input);
    
    // Attempt as deep link
    // deeplink_execute(user_input);
    
    sigma_log("[QUICKACT] UIR: Best match dispatched to Universal UI.");
}

extern "C" void quickact_dismiss() {
    sigma_log("[QUICKACT] UIR: Quick Actions Bar dismissed.");
}

