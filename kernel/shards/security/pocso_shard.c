#include "../../../include/core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN-POCSO-SHARD (POCSO Act 2012)
 * =============================================================================
 */
#include "../../../include/sigma_kernel_types.h"

extern void add_item(void* t, const char* desc, const char* ref, const char* prereq, const char* proc, sigma_u32 days, sigma_u32 penalty_rs);

void init_pocso_template(void* t) {
    add_item(t, "Mandatory Reporting of POCSO offence", "Sec 19 POCSO", 
        "PREREQ: Knowledge of sexual offence against child.", 
        "STEP 1: Report to SJPU or local police. STEP 2: Entry in special POCSO register. STEP 3: *Mandatory* reporting (failure is an offence).", 0, 0);
    add_item(t, "Child-Friendly Statement Recording", "Sec 24 POCSO", 
        "PREREQ: Victim is a child.", 
        "STEP 1: Recorded by woman officer. STEP 2: At child's residence. STEP 3: No police uniform during recording.", 0, 0);
    add_item(t, "In-Camera Trial", "Sec 37 POCSO", 
        "PREREQ: Trial starts in Special POCSO Court.", 
        "STEP 1: Closed court proceedings. STEP 2: Child must not see the accused during testimony.", 0, 0);
    add_item(t, "Medical Examination of Child", "Sec 27 POCSO", 
        "PREREQ: Arrest or report of offence.", 
        "STEP 1: Within 24 hours. STEP 2: Presence of parent/trusted person.", 1, 0);
}
