#include "core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN-DPDP-SHARD (Digital Personal Data Protection Act 2023)
 * =============================================================================
 */
#include "core/sigma_kernel_types.h"

extern void add_item(void* t, const char* desc, const char* ref, const char* prereq, const char* proc, sigma_u32 days, sigma_u32 penalty_rs);

void init_dpdp_template(void* t) {
    add_item(t, "Obtain Consent for Processing", "Sec 6 DPDP Act", 
        "PREREQ: Intent to process personal data.", 
        "STEP 1: Clear and plain language notice. STEP 2: Affirmative action for consent. STEP 3: Right to withdraw consent.", 0, 250000000);
    add_item(t, "Data Breach Notification", "Sec 8(6) DPDP Act", 
        "PREREQ: Personal data breach occurred.", 
        "STEP 1: Notify Data Protection Board of India. STEP 2: Notify affected Data Principals.", 0, 200000000);
    add_item(t, "Data Principal Rights (Access/Correction)", "Sec 11-13 DPDP Act", 
        "PREREQ: Request from Data Principal.", 
        "STEP 1: Verify identity. STEP 2: Provide summary of processing. STEP 3: Correct/Erase as requested.", 30, 0);
    add_item(t, "Significant Data Fiduciary (SDF) Obligations", "Sec 10 DPDP Act", 
        "PREREQ: Large volume of sensitive data or threat to sovereignty.", 
        "STEP 1: Appoint Data Protection Officer (DPO). STEP 2: Conduct Data Protection Impact Assessment (DPIA).", 0, 0);
}
