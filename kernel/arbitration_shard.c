/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: SOVEREIGN-ARBITRATION-SHARD (Arbitration & Conciliation Act)
 * =============================================================================
 */
#include "sigma_kernel_types.h"

extern void add_item(void* t, const char* desc, const char* ref, const char* prereq, const char* proc, u32 days, u32 penalty_rs);

void init_arbitration_template(void* t) {
    add_item(t, "Invoke Arbitration Clause", "Sec 21 Arb Act", 
        "PREREQ: Valid arbitration agreement in contract.", 
        "STEP 1: Send notice of arbitration to respondent. STEP 2: Dispute begins on date of notice receipt.", 0, 0);
    add_item(t, "Interim Measures by Court", "Sec 9 Arb Act", 
        "PREREQ: Before/during arbitral proceedings.", 
        "STEP 1: File petition for stay/protection of assets. STEP 2: Court passes interim order.", 0, 0);
    add_item(t, "Appointment of Arbitrator", "Sec 11 Arb Act", 
        "PREREQ: Parties fail to agree on arbitrator.", 
        "STEP 1: File application before High Court (Domestic) or Supreme Court (International).", 30, 0);
    add_item(t, "Challenge to Arbitral Award", "Sec 34 Arb Act", 
        "PREREQ: Award passed by tribunal.", 
        "STEP 1: File within 3 months. STEP 2: Grounds: patent illegality, public policy violation.", 90, 0);
}
