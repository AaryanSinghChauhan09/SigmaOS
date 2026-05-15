#include "../../../include/core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN-BNSS-SHARD (Bharatiya Nagarik Suraksha Sanhita 2023)
 * =============================================================================
 */
#include "../../../include/core/sigma_kernel_types.h"

extern void add_item(void* t, const char* desc, const char* ref, const char* prereq, const char* proc, sigma_u32 days, sigma_u32 penalty_rs);

void init_bnss_template(void* t) {
    add_item(t, "Register FIR (Mandatory)", "Sec 173 BNSS", 
        "PREREQ: Information of cognizable offence.", 
        "STEP 1: Report to SHO. STEP 2: SHO enters in FIR book. STEP 3: Mandatory e-FIR/Audio-Video recording.", 0, 0);
    add_item(t, "Production before Magistrate within 24 hours", "Sec 58 BNSS", 
        "PREREQ: Arrest without warrant.", 
        "STEP 1: Physical production. STEP 2: Remand application if needed.", 1, 0);
    add_item(t, "Police Custody Remand (Max 15 days)", "Sec 187 BNSS", 
        "PREREQ: Investigation requires custody. *V. Senthil Balaji v. State (2023 SC)* - Police custody can be granted even after first 15 days of judicial remand.", 
        "STEP 1: Written application. STEP 2: Magistrate order. STEP 3: Medical exam every 48 hrs.", 15, 0);
    add_item(t, "Search and Seizure (Video Mandatory)", "Sec 105 BNSS", 
        "PREREQ: Search warrant or emergency.", 
        "STEP 1: Presence of 2 witnesses. STEP 2: Audio-Video recording of process. STEP 3: Inventory list.", 0, 0);
    add_item(t, "Arrest of Female (Sunrise/Sunset rule)", "Sec 43(2) BNSS", 
        "PREREQ: Female accused.", 
        "STEP 1: Arrest only by female officer. STEP 2: Only between sunrise and sunset (unless magistrate permit).", 0, 0);
    add_item(t, "Default Bail (60/90 days)", "Sec 187(5) BNSS", 
        "PREREQ: Charge-sheet not filed in time.", 
        "STEP 1: Application for bail. STEP 2: Mandatory release if criteria met.", 60, 0);
    add_item(t, "Zero FIR (Jurisdiction no bar)", "Sec 173(1) BNSS", 
        "PREREQ: Offence occurred outside station limits.", 
        "STEP 1: Register Zero FIR. STEP 2: Transfer to relevant station.", 0, 0);
}

