#include "../../../include/core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN-CYBER-SHARD (IT Act 2000 / Cyber Law)
 * =============================================================================
 */
#include "../../../include/sigma_kernel_types.h"

extern void add_item(void* t, const char* desc, const char* ref, const char* prereq, const char* proc, sigma_u32 days, sigma_u32 penalty_rs);

void init_cyber_template(void* t) {
    add_item(t, "Report Cyber Incident to CERT-In (6-hour rule)", "Sec 70B IT Act", 
        "PREREQ: Detection of ransomware, data breach, etc.", 
        "STEP 1: Report to incident@cert-in.org.in within 6 hours. STEP 2: Maintain logs for 180 days.", 0, 1000000);
    add_item(t, "Intermediary Due Diligence", "IT Rules 2021", 
        "PREREQ: Social Media platform or Intermediary.", 
        "STEP 1: Appoint Grievance Officer. STEP 2: Monthly compliance reports. STEP 3: Takedown within 36 hrs of order.", 0, 0);
    add_item(t, "Hacking/Unauthorised Access", "Sec 66 IT Act", 
        "PREREQ: Damage to computer system.", 
        "STEP 1: Gather forensic evidence. STEP 2: File FIR at Cyber Cell. STEP 3: *Sharat Babu Digumarti v. Govt of NCT Delhi (2017 SC)* - IT Act overrides IPC for cyber obscenity.", 0, 0);
    add_item(t, "Electronic Signature Verification", "Sec 3 IT Act", 
        "PREREQ: Digital contract.", 
        "STEP 1: Verify DSC (Digital Signature Certificate). STEP 2: Check CCA root chain.", 0, 0);
}
