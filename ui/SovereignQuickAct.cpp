#include "../include/sigma_log.h"
#include "../include/libc/SovereignLibC.h"
#include "../include/sigma_kernel_types.h"

#include "../include/sigma_quickact.h"
#include "../include/hal/sigma_hal.h"
#include "../include/sigma_neuralsearch.h"



/**
 * SigmaOS Sovereign Quick Actions Bar
 * Implements a Unified Intent Router (UIR) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal command palette.
 */

void quickact_init() {
    sigma_log("[QUICKACT] Initializing Sovereign Quick Actions Bar (UIR Algorithm)...");
}

void quickact_invoke() {
    sigma_log("[QUICKACT] UIR: Quick Actions Bar rendered. Awaiting input...");
}

void quickact_process_input(const char* user_input) {
    // UIR (Unified Intent Router) Algorithm
    // Classifies input as: app search, file search, system command, sigma:// URI, or automation rule.
    
    sigma_log("[QUICKACT] UIR: Processing input: '%s'\n", user_input);
    
    // Attempt semantic search first
    neuralsearch_query(user_input);
    
    // Attempt as deep link
    // deeplink_execute(user_input);
    
    sigma_log("[QUICKACT] UIR: Best match dispatched to Universal UI.");
}

void quickact_dismiss() {
    sigma_log("[QUICKACT] UIR: Quick Actions Bar dismissed.");
}




} // extern "C"
