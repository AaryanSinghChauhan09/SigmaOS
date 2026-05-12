/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN-RTI-SHARD (Right to Information Act 2005)
 * =============================================================================
 */
#include "sigma_kernel_types.h"

extern void add_item(void* t, const char* desc, const char* ref, const char* prereq, const char* proc, sigma_u32 days, sigma_u32 penalty_rs);

void init_rti_template(void* t) {
    add_item(t, "File RTI Application", "Sec 6 RTI Act", 
        "PREREQ: Information sought from Public Authority.", 
        "STEP 1: Identify CPIO. STEP 2: Draft request with Rs 10 fee. STEP 3: Submit via post/online.", 30, 250);
    add_item(t, "Life and Liberty Request", "Sec 7(1) RTI Act", 
        "PREREQ: Information concerns life/liberty of a person.", 
        "STEP 1: Explicitly mention 'Life and Liberty'. STEP 2: Response MUST be provided within 48 hours.", 2, 0);
    add_item(t, "First Appeal", "Sec 19(1) RTI Act", 
        "PREREQ: 30 days elapsed or unsatisfactory response.", 
        "STEP 1: File before First Appellate Authority (Senior to CPIO). STEP 2: Dispose within 30-45 days.", 30, 0);
    add_item(t, "Second Appeal to CIC/SIC", "Sec 19(3) RTI Act", 
        "PREREQ: First appeal failed.", 
        "STEP 1: File before Central/State Information Commission within 90 days.", 90, 0);
}
