// SPDX-License-Identifier: GPL-2.0-or-later
// sigma-realty — Real estate professional tools (RERA, stamp duty, TDS-194IA)
//
// CLI:
//   sigma-realty stamp-duty --state MH --value 5000000 --type residential
//   sigma-realty rera verify --project-id P52100012345
//   sigma-realty tds property --sale-value 8500000
//   sigma-realty registration charges --state DL --value 3000000

#include <stdint.h>
#include <string.h>
#include <stdio.h>

// ── Stamp Duty Rates (state-wise, FY 2025-26) ─────────────────────────────

struct StampRate {
    const char *state;
    float       residential_pct;
    float       commercial_pct;
    float       registration_pct;
    const char *notes;
};

static const StampRate stamp_rates[] = {
    { "MH", 5.0f, 5.0f, 1.0f, "Male buyer; female buyer: 4%" },
    { "DL", 4.0f, 6.0f, 1.0f, "Female: 4%; both: 4%" },
    { "KA", 5.0f, 5.6f, 1.0f, "Bengaluru BDA: 5% stamp" },
    { "TN", 7.0f, 7.0f, 4.0f, "One of highest in India" },
    { "UP", 7.0f, 7.0f, 1.0f, "Female: 6%" },
    { "GJ", 4.9f, 4.9f, 1.0f, "Affordable housing: 3.5%" },
    { "PB", 7.0f, 7.0f, 1.0f, "Includes rural development fee" },
    { "HR", 5.0f, 5.0f, 1.0f, "Female/joint: 3%; male: 5%" },
    { "RJ", 5.0f, 5.0f, 1.0f, "Jaipur MC area: 5%" },
    { "WB", 5.0f, 7.0f, 1.0f, "Urban: 5%; municipal area: +1%" },
    { NULL, 0, 0, 0, NULL }
};

void sigma_realty_stamp_duty(const char *state, uint64_t value,
                              const char *prop_type) {
    printf("Stamp Duty Calculator — %s | ₹%llu | %s\n",
           state, (unsigned long long)value, prop_type);

    for (int i = 0; stamp_rates[i].state; i++) {
        if (strcmp(stamp_rates[i].state, state) == 0) {
            const StampRate *r = &stamp_rates[i];
            float rate = strcmp(prop_type, "commercial") == 0
                         ? r->commercial_pct : r->residential_pct;
            uint64_t stamp    = (uint64_t)((float)value * rate / 100.0f);
            uint64_t reg      = (uint64_t)((float)value * r->registration_pct / 100.0f);
            uint64_t total    = stamp + reg;

            printf("  Stamp duty (%g%%):         ₹%llu\n",
                   rate, (unsigned long long)stamp);
            printf("  Registration (%.0f%%):      ₹%llu\n",
                   r->registration_pct, (unsigned long long)reg);
            printf("  Total stamp+reg charges:  ₹%llu\n",
                   (unsigned long long)total);
            printf("  Note: %s\n", r->notes);
            printf("  e-Stamp: SHCIL (shcilestamp.com)\n");
            printf("  SRO registration: book.igrs.ap.gov.in (state-specific)\n");
            return;
        }
    }
    printf("State '%s' not in database. Check state SRO portal.\n", state);
}

// ── RERA Project Verification ─────────────────────────────────────────────

void sigma_realty_rera_verify(const char *project_id) {
    printf("RERA Project Verification — %s\n", project_id);
    printf("Check: maharera.mahaonline.gov.in (MH)\n");
    printf("       rera.delhi.gov.in (DL)\n");
    printf("       rera.karnataka.gov.in (KA)\n");
    printf("National: RERA portal state-wise (MoHUA)\n\n");
    printf("Key checks:\n");
    printf("  ✓ Project registration valid?\n");
    printf("  ✓ Completion date vs promised date\n");
    printf("  ✓ Escrow account balance (70%% rule)\n");
    printf("  ✓ Any complaints pending\n");
    printf("  ✓ Carpet area vs super built-up area ratio\n");
    printf("  ✓ Promoter registration valid\n\n");
    printf("sigma-digilocker pull --doc rera-certificate --id %s\n", project_id);
}

// ── TDS on Property Purchase (Section 194-IA) ─────────────────────────────

void sigma_realty_tds_property(uint64_t sale_value) {
    printf("TDS on Property Purchase — Section 194-IA\n");
    printf("Sale consideration: ₹%llu\n", (unsigned long long)sale_value);

    if (sale_value < 5000000) {
        printf("  TDS not applicable (below ₹50 lakh threshold)\n");
        return;
    }

    uint64_t tds_amt = sale_value * 1 / 100;  // 1% TDS
    printf("  TDS rate:      1%%\n");
    printf("  TDS amount:    ₹%llu\n", (unsigned long long)tds_amt);
    printf("  Net to seller: ₹%llu\n",
           (unsigned long long)(sale_value - tds_amt));
    printf("\nDeposit via Form 26QB (online):\n");
    printf("  tin.tin.nsdl.com → e-payment → TDS on property\n");
    printf("  Deadline: within 30 days from end of month of deduction\n");
    printf("  TDS certificate: Form 16B (download from TRACES within 15 days)\n");
    printf("  PAN of seller required; TAN not required for buyer\n");
    printf("\nPenalty for non-deduction: ₹200/day u/s 234E + 1%% pm interest\n");
}
