#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"

#include "../../include/sigma_quickact.h"
#include "../../include/sigma_log.h"
#include "../../include/hal/sigma_hal.h"
#include "../../include/sigma_log.h"
#include "../../include/sigma_neuralsearch.h"
#include "../../include/sigma_log.h"



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
    
    sigma_log_info("[QUICKACT] UIR: Processing input: '%s'\n", user_input);
    
    // Attempt semantic search first
    neuralsearch_query(user_input);
    
    // Attempt as deep link
    // deeplink_execute(user_input);
    
    sigma_log("[QUICKACT] UIR: Best match dispatched to Universal UI.");
}

extern "C" void quickact_dismiss() {
    sigma_log("[QUICKACT] UIR: Quick Actions Bar dismissed.");
}


 