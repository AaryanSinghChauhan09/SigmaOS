#include "sigma_kernel_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN-BNS-SHARD (Bharatiya Nyaya Sanhita 2023)
 * =============================================================================
 */
#include "sigma_kernel_types.h"

extern void add_item(void* t, const char* desc, const char* ref, const char* prereq, const char* proc, sigma_u32 days, sigma_u32 penalty_rs);

void init_bns_template(void* t) {
    add_item(t, "Murder (BNS 103 = old IPC 302)", "Sec 103 BNS", 
        "PREREQ: Death of a person. FIR registered.", 
        "STEP 1: FIR under Sec 103 BNS. STEP 2: Post-mortem report. STEP 3: Sessions trial.", 0, 0);
    add_item(t, "Terrorism (BNS 113)", "Sec 113 BNS", 
        "PREREQ: Threat to sovereignty.", 
        "STEP 1: FIR 113 BNS. STEP 2: NIA investigation. STEP 3: Special Court.", 0, 0);
    add_item(t, "Organised Crime (BNS 111)", "Sec 111 BNS", 
        "PREREQ: Syndicate activity.", 
        "STEP 1: Report syndicate. STEP 2: Property attachment.", 0, 0);
    add_item(t, "Cheating (BNS 318)", "Sec 318 BNS", 
        "PREREQ: Dishonest inducement.", 
        "STEP 1: Complaint with evidence. STEP 2: Recovery procedure.", 0, 0);
    add_item(t, "Mob Lynching (BNS 103(2))", "Sec 103(2) BNS", 
        "PREREQ: 5 or more persons acting in concert.", 
        "STEP 1: FIR 103(2) BNS. STEP 2: Identify all participants via video/witness.", 0, 0);
    add_item(t, "Petty Organised Crime (BNS 112)", "Sec 112 BNS", 
        "PREREQ: Theft, snatching, etc. by gang.", 
        "STEP 1: Report to police. STEP 2: Summary trial for small offences.", 0, 0);
}
