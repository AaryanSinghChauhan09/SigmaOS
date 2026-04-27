/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: SOVEREIGN-BSA-SHARD (Bharatiya Sakshya Adhiniyam 2023)
 * =============================================================================
 */
#include "sigma_kernel_types.h"

extern void add_item(void* t, const char* desc, const char* ref, const char* prereq, const char* proc, u32 days, u32 penalty_rs);

void init_bsa_template(void* t) {
    add_item(t, "Certificate for Electronic Records (Sec 63 BSA)", "Sec 63 BSA", 
        "PREREQ: Electronic evidence (emails, logs, CCTV).", 
        "STEP 1: Hash file with SHA-256. STEP 2: Fill Sec 63 certificate form. STEP 3: Sign by administrator.", 0, 0);
    add_item(t, "Primary Evidence of Digital Device (Sec 62 BSA)", "Sec 62 BSA", 
        "PREREQ: Original device available.", 
        "STEP 1: Physical seizure with video recording. STEP 2: Forensic imaging with write-blocker.", 0, 0);
    add_item(t, "Oral Evidence Admissibility", "Sec 22 BSA", 
        "PREREQ: Witness present.", 
        "STEP 1: Administer oath. STEP 2: Record statement. STEP 3: Cross-examination.", 0, 0);
    add_item(t, "Secondary Evidence (Copies/Extracts)", "Sec 64 BSA", 
        "PREREQ: Primary evidence lost or inaccessible.", 
        "STEP 1: Proof of existence of primary. STEP 2: Certified copy production.", 0, 0);
    add_item(t, "Latest SC Interpretation on Sec 63 BSA (Parity with 65B IEA)", "Supreme Court 2024", 
        "PREREQ: Arjun Panditrao Khotkar v. Kailash Kushanrao Gorantyal interpretation.", 
        "STEP 1: Certificate is a CONDITION PRECEDENT for admissibility. No certificate, no evidence.", 0, 0);
}
