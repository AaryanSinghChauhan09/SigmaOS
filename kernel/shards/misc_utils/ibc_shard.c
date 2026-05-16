#include "../../../include/core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN-IBC-SHARD (Insolvency and Bankruptcy Code 2016)
 * =============================================================================
 */
#include "../../../include/sigma_kernel_types.h"

extern void add_item(void* t, const char* desc, const char* ref, const char* prereq, const char* proc, sigma_u32 days, sigma_u32 penalty_rs);

void init_ibc_template(void* t) {
    add_item(t, "Initiate CIRP (Financial Creditor)", "Sec 7 IBC", 
        "PREREQ: Default of Rs 1 Crore or more.", 
        "STEP 1: File application at NCLT. STEP 2: NCLT admission within 14 days. STEP 3: Moratorium begins.", 14, 0);
    add_item(t, "Operational Creditor Demand Notice", "Sec 8 IBC", 
        "PREREQ: Unpaid operational debt.", 
        "STEP 1: Send 10-day demand notice. STEP 2: File Sec 9 application if no payment or dispute.", 10, 0);
    add_item(t, "Corporate Insolvency Resolution Process (CIRP) Timeline", "Sec 12 IBC", 
        "PREREQ: CIRP admitted.", 
        "STEP 1: Mandatory completion within 180 days (ext. up to 330 days total).", 180, 0);
    add_item(t, "Liquidation Process", "Sec 33 IBC", 
        "PREREQ: No resolution plan approved or CoC decides liquidation.", 
        "STEP 1: NCLT passes liquidation order. STEP 2: Liquidator sells assets as per waterfall mechanism.", 0, 0);
}
