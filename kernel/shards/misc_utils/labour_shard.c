#include "../../../include/core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN-LABOUR-SHARD (New Labour Codes 2020)
 * =============================================================================
 */
#include "../../../include/core/sigma_kernel_types.h"

extern void add_item(void* t, const char* desc, const char* ref, const char* prereq, const char* proc, sigma_u32 days, sigma_u32 penalty_rs);

void init_labour_template(void* t) {
    add_item(t, "Wage Payment Timeline", "Sec 17 Code on Wages", 
        "PREREQ: Employer-Employee relationship.", 
        "STEP 1: Monthly payment by 7th of next month. STEP 2: Full and final settlement within 2 days of exit.", 7, 50000);
    add_item(t, "Industrial Dispute Conciliation", "IR Code 2020", 
        "PREREQ: Dispute regarding terms of employment.", 
        "STEP 1: Report to Conciliation Officer. STEP 2: Attempt settlement within 14 days.", 14, 0);
    add_item(t, "Gratuity Eligibility (5 Years)", "Social Security Code", 
        "PREREQ: Continuous service for 5 years.", 
        "STEP 1: Calculate as 15 days wages per year of service. STEP 2: Pay within 30 days of termination.", 30, 0);
}
