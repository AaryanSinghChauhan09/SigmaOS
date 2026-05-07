#include "core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN-PMLA-SHARD (Prevention of Money Laundering Act 2002)
 * =============================================================================
 */
#include "core/sigma_kernel_types.h"

extern void add_item(void* t, const char* desc, const char* ref, const char* prereq, const char* proc, sigma_u32 days, sigma_u32 penalty_rs);

void init_pmla_template(void* t) {
    add_item(t, "Arrest under PMLA", "Sec 19 PMLA", 
        "PREREQ: Reason to believe (in writing) that person is guilty of offence.", 
        "STEP 1: Inform grounds of arrest. STEP 2: Produce before Special Court within 24 hrs. STEP 3: *Vijay Madanlal Choudhary v. UOI (2022 SC)* - ED must provide grounds in writing.", 0, 0);
    add_item(t, "Attachment of Property", "Sec 5 PMLA", 
        "PREREQ: Possession of proceeds of crime.", 
        "STEP 1: Provisional attachment order (180 days). STEP 2: File complaint before Adjudicating Authority. STEP 3: Confirm attachment.", 180, 0);
    add_item(t, "Strict Bail Conditions (Double Negative Test)", "Sec 45 PMLA", 
        "PREREQ: Accused seeks bail.", 
        "STEP 1: Court must be satisfied that accused is not guilty AND unlikely to commit offence. *Pankaj Bansal v. UOI (2023 SC)* - Written grounds of arrest are mandatory.", 0, 0);
    add_item(t, "Reverse Burden of Proof", "Sec 24 PMLA", 
        "PREREQ: Prosecution for money laundering.", 
        "STEP 1: Presumption of guilt. STEP 2: Accused must prove innocence.", 0, 0);
}
