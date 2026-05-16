#include "../../../include/sigma_kernel_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN-GST-SHARD (GST and Tax Compliance)
 * =============================================================================
 */
#include "../../../include/sigma_kernel_types.h"

extern void add_item(void* t, const char* desc, const char* ref, const char* prereq, const char* proc, sigma_u32 days, sigma_u32 penalty_rs);

void init_gst_template(void* t) {
    add_item(t, "GSTR-1 (Outward Supplies)", "Sec 37 CGST Act", 
        "PREREQ: Registered GST Person.", 
        "STEP 1: Upload invoice details. STEP 2: File by 11th of next month.", 11, 50);
    add_item(t, "GSTR-3B (Summary Return)", "Sec 39 CGST Act", 
        "PREREQ: GST registration.", 
        "STEP 1: Verify ITC in GSTR-2B. STEP 2: Pay tax and file by 20th of next month.", 20, 50);
    add_item(t, "ITR Filing (Individuals)", "Sec 139 IT Act", 
        "PREREQ: Total income exceeds exemption limit.", 
        "STEP 1: Gather Form-16/AIS. STEP 2: Select ITR-1/2/4. STEP 3: File by 31st July.", 212, 5000);
    add_item(t, "TDS Payment and Return", "Sec 192-194 IT Act", 
        "PREREQ: Taxable payments made.", 
        "STEP 1: Deduct TDS. STEP 2: Deposit by 7th of next month. STEP 3: File quarterly return.", 7, 200);
}
