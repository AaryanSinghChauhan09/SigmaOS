// SPDX-License-Identifier: GPL-2.0-or-later
// sigma-gov — Government official tools (RTI, GFR, GeM, eOffice)
//
// CLI:
//   sigma-gov rti draft --ministry "MoHFW" --info-sought "vaccine cost"
//   sigma-gov gem order --category "IT Equipment" --gem-bid-id 12345
//   sigma-gov roster check --department "Education" --post "Teacher"
//   sigma-gov gfr procurement --amount 500000 --method LTE
//   sigma-gov pfms uc --scheme "PM-POSHAN" --amount 100000

#include <stdint.h>
#include <string.h>
#include <stdio.h>

// ── RTI Application Generator ─────────────────────────────────────────────

void sigma_gov_rti_draft(const char *ministry, const char *info_sought) {
    printf("RTI Application — Right to Information Act 2005\n\n");
    printf("To,\nThe Central Public Information Officer (CPIO)\n");
    printf("Ministry/Department: %s\n\n", ministry);
    printf("Subject: Application under Section 6(1) of RTI Act 2005\n\n");
    printf("I, [Applicant Name], citizen of India, hereby request the\n");
    printf("following information under the RTI Act 2005:\n\n");
    printf("Information sought: %s\n\n", info_sought);
    printf("Period: [Specify period]\n");
    printf("Format: Certified copy / Inspection of records\n\n");
    printf("I am enclosing ₹10/- as application fee via:\n");
    printf("  □ Demand Draft  □ IPO  □ Court Fee Stamp\n\n");
    printf("Reply Timeline: 30 days (§7 RTI Act)\n");
    printf("Appeal: First Appellate Authority within 30 days (§19)\n");
    printf("Second appeal: CIC/SIC within 90 days of FA order\n\n");
    printf("Portal: rtionline.gov.in | Helpline: 011-24623005\n");
}

// ── GeM (Government e-Marketplace) ───────────────────────────────────────

void sigma_gov_gem_order(const char *category, const char *bid_id) {
    printf("GeM Order — Government e-Marketplace\n");
    printf("Category:  %s\n", category);
    printf("Bid ID:    %s\n", bid_id);
    printf("\nProcurement route:\n");
    printf("  ≤ ₹25,000:    Direct purchase (L1 from GeM)\n");
    printf("  ≤ ₹5,00,000:  Bid on GeM (minimum 3 sellers)\n");
    printf("  > ₹5,00,000:  Custom bid with RA (Reverse Auction)\n");
    printf("\nRequired: GEM Buyer registration + DDO credentials\n");
    printf("Payment: PFMS integration (SNA/Treasury)\n");
    printf("Portal: gem.gov.in | Helpline: 1800-419-3436\n");
    printf("Compliance: Rule 149(ii) GFR 2017\n");
}

// ── Reservation Roster ────────────────────────────────────────────────────

void sigma_gov_roster_check(const char *dept, const char *post) {
    printf("Reservation Roster — %s | Post: %s\n", dept, post);
    printf("\n40-point roster (DoPT OM dated 02.07.1997):\n");
    printf("  SC:  15%%  |  ST: 7.5%%  |  OBC: 27%%  |  EWS: 10%%\n");
    printf("\nRoster sequence (first 13 vacancies):\n");
    printf("  1-UR  2-SC  3-UR  4-ST  5-OBC  6-UR  7-UR  8-OBC\n");
    printf("  9-SC  10-OBC  11-UR  12-ST  13-OBC\n");
    printf("\nSub-reservation: PwBD 4%% (horizontal), Ex-SM 10%% (horizontal)\n");
    printf("EWS: 10%% (as per Constitution 103rd Amendment Act 2019)\n");
    printf("Verify: dopt.gov.in → Reservation Roster\n");
}

// ── GFR Procurement ───────────────────────────────────────────────────────

void sigma_gov_gfr_procurement(uint64_t amount, const char *method) {
    printf("GFR 2017 Procurement Analysis\n");
    printf("Amount: ₹%llu | Method: %s\n",
           (unsigned long long)amount, method);
    printf("\nApplicable Rule:\n");
    if (amount <= 25000)
        printf("  Rule 154(i): Direct purchase without quotation ≤ ₹25,000\n");
    else if (amount <= 250000)
        printf("  Rule 154(ii): ≥ 3 quotations required\n");
    else if (amount <= 1000000)
        printf("  Rule 158: Limited tender enquiry (LTE) with ≥ 3 firms\n");
    else
        printf("  Rule 166: Open tender / e-tender mandatory\n");
    printf("Approval: As per Delegation of Financial Powers Rules 2024\n");
    printf("Audit trail: PFMS + UC within 12 months of expenditure\n");
}
