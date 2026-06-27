// SPDX-License-Identifier: GPL-2.0-or-later
// sigma-bank — Banking & Finance professional tools
// Covers: RBI norms, KYC/AML, IBC, NPA, PMLA, EMI, MCLR, SARFAESI
//
// CLI:
//   sigma-bank npa classify --account ACC001 --dpd 95
//   sigma-bank kyc verify --pan ABCDE1234F --aadhar-last4 1234
//   sigma-bank emi --principal 5000000 --rate 8.5 --tenure 240
//   sigma-bank mclr --bank SBI --reset-date 2026-06-01
//   sigma-bank ibc cirp --company ABCDE --filing-date 2026-01-15

#include <stdint.h>
#include <string.h>
#include <stdio.h>
#include <math.h>

// ── NPA Classification (RBI Prudential Norms, 90-day rule) ───────────────────

typedef enum {
    ASSET_STANDARD    = 0,
    ASSET_SMA_0       = 1,   // Special Mention Account — 0-30 DPD
    ASSET_SMA_1       = 2,   // 31-60 DPD
    ASSET_SMA_2       = 3,   // 61-90 DPD
    ASSET_SUB_STANDARD= 4,   // >90 DPD ≤ 12 months
    ASSET_DOUBTFUL_1  = 5,   // Sub-standard >12 months ≤ 24 months
    ASSET_DOUBTFUL_2  = 6,   // 24-36 months
    ASSET_DOUBTFUL_3  = 7,   // >36 months
    ASSET_LOSS        = 8,   // Loss asset
} AssetClassification;

static const char *asset_names[] = {
    "Standard",
    "SMA-0 (0-30 DPD)",
    "SMA-1 (31-60 DPD)",
    "SMA-2 (61-90 DPD)",
    "Sub-Standard NPA",
    "Doubtful-1 NPA",
    "Doubtful-2 NPA",
    "Doubtful-3 NPA",
    "Loss Asset NPA",
};

static const float provision_pct[] = {
    0.40f,   // Standard secured
    0.0f,    // SMA-0
    0.0f,    // SMA-1
    0.0f,    // SMA-2
    15.0f,   // Sub-standard
    25.0f,   // Doubtful-1
    40.0f,   // Doubtful-2
    100.0f,  // Doubtful-3
    100.0f,  // Loss
};

void sigma_bank_npa_classify(const char *account, uint32_t dpd) {
    AssetClassification cls;
    if (dpd <= 30)       cls = ASSET_SMA_0;
    else if (dpd <= 60)  cls = ASSET_SMA_1;
    else if (dpd <= 90)  cls = ASSET_SMA_2;
    else if (dpd <= 450) cls = ASSET_SUB_STANDARD;
    else if (dpd <= 810) cls = ASSET_DOUBTFUL_1;
    else if (dpd <= 1170)cls = ASSET_DOUBTFUL_2;
    else                 cls = ASSET_DOUBTFUL_3;

    printf("NPA Classification — Account: %s | DPD: %u\n", account, dpd);
    printf("  Classification: %s\n", asset_names[cls]);
    printf("  Provisioning required: %.1f%%\n", provision_pct[cls]);
    printf("  RBI Ref: Master Circular — Prudential norms on Income Recognition\n");
    if (dpd > 90) {
        printf("  Action: Initiate recovery per SARFAESI Act 2002\n");
        printf("  sigma-bank sarfaesi notice --account %s --type 13-2\n", account);
    }
}

// ── EMI Calculator (Reducing Balance) ─────────────────────────────────────────

void sigma_bank_emi(uint64_t principal, float rate_pct, uint32_t tenure_months) {
    // EMI = P × r × (1+r)^n / ((1+r)^n - 1)
    float r   = rate_pct / 12.0f / 100.0f;
    float pow_val = powf(1.0f + r, (float)tenure_months);
    float emi = (float)principal * r * pow_val / (pow_val - 1.0f);
    float total_payment = emi * tenure_months;
    float total_interest = total_payment - (float)principal;

    printf("EMI Calculation (Reducing Balance)\n");
    printf("  Principal:       ₹%lu (₹%.2f lakh)\n",
           (unsigned long)principal, (float)principal / 100000.0f);
    printf("  Interest rate:   %.2f%% p.a.\n", rate_pct);
    printf("  Tenure:          %u months (%u years)\n",
           tenure_months, tenure_months / 12);
    printf("  Monthly EMI:     ₹%.0f\n", emi);
    printf("  Total payment:   ₹%.0f\n", total_payment);
    printf("  Total interest:  ₹%.0f\n", total_interest);
    printf("  Interest-to-principal ratio: %.2f\n",
           total_interest / (float)principal);
    printf("\n  Amortisation schedule (first 3 months):\n");
    printf("  %-5s %-10s %-10s %-12s\n", "Month", "EMI(₹)", "Interest", "Principal");
    float outstanding = (float)principal;
    for (int m = 1; m <= 3 && m <= (int)tenure_months; m++) {
        float int_comp  = outstanding * r;
        float prin_comp = emi - int_comp;
        printf("  %-5d %-10.0f %-10.0f %-12.0f\n",
               m, emi, int_comp, prin_comp);
        outstanding -= prin_comp;
    }
    printf("  ... [use sigma-bank emi-schedule for full table]\n");
}

// ── MCLR (Marginal Cost of Funds Based Lending Rate) ─────────────────────────

struct MCLREntry {
    const char *bank;
    const char *tenor;
    float       rate;
    const char *effective_date;
};

static const MCLREntry mclr_table[] = {
    { "SBI",   "Overnight", 8.10f, "2026-06-01" },
    { "SBI",   "1-Month",   8.35f, "2026-06-01" },
    { "SBI",   "3-Month",   8.40f, "2026-06-01" },
    { "SBI",   "6-Month",   8.75f, "2026-06-01" },
    { "SBI",   "1-Year",    8.85f, "2026-06-01" },
    { "HDFC",  "1-Year",    9.10f, "2026-06-01" },
    { "ICICI", "1-Year",    9.10f, "2026-06-01" },
    { "PNB",   "1-Year",    8.95f, "2026-06-01" },
    { NULL, NULL, 0.0f, NULL }
};

void sigma_bank_mclr(const char *bank) {
    printf("MCLR Rates — %s\n", bank);
    printf("%-10s %-10s %-12s\n", "Bank", "Tenor", "MCLR");
    for (int i = 0; mclr_table[i].bank; i++) {
        if (strcmp(mclr_table[i].bank, bank) == 0) {
            printf("%-10s %-10s %.2f%% (w.e.f. %s)\n",
                   mclr_table[i].bank, mclr_table[i].tenor,
                   mclr_table[i].rate, mclr_table[i].effective_date);
        }
    }
    printf("RBI Ref: RBI/2015-16/418 — MCLR Framework\n");
    printf("Spread over MCLR: as per bank's internal policy\n");
}

// ── IBC CIRP Timeline Tracker ────────────────────────────────────────────────

void sigma_bank_ibc_cirp(const char *company, const char *filing_date) {
    printf("IBC 2016 — CIRP Timeline: %s\n", company);
    printf("CIRP Filing Date: %s\n", filing_date);
    printf("\nKey deadlines (IBC §§ 7/9/10):\n");
    printf("  Day  1: Admission of application by NCLT\n");
    printf("  Day  3: Appoint IRP (Interim Resolution Professional)\n");
    printf("  Day 14: Public announcement by IRP\n");
    printf("  Day 30: First CoC (Committee of Creditors) meeting\n");
    printf("  Day 67: Submit IM (Information Memorandum) to CoC\n");
    printf("  Day 75: Submit Resolution Plan by applicants\n");
    printf("  Day 90: CoC approve/reject plan (75%% voting)\n");
    printf("  Day 180: CIRP completion deadline (extendable to 270 days)\n");
    printf("  Day 270: Final deadline — else liquidation order\n");
    printf("\nKey parties:\n");
    printf("  IRP → RP: sigma-bank ibc appoint-rp\n");
    printf("  Claims registry: sigma-bank ibc claims --company %s\n", company);
    printf("  NCLT e-filing: nclt.gov.in\n");
}

// ── KYC / PEP Check ──────────────────────────────────────────────────────────

void sigma_bank_kyc_verify(const char *pan, const char *aadhar_suffix) {
    printf("KYC Verification\n");
    printf("  PAN:           %s\n", pan);
    printf("  Aadhaar (last 4): %s\n", aadhar_suffix);
    printf("  KYC Mode:      C-KYC (Central KYC Registry)\n");
    printf("  PEP Check:     Verify against MHA/UN sanctions list\n");
    printf("  CIBIL pull:    Requires signed consent form (Form 17-A)\n");
    printf("  PMLA CTR:      Cash transactions ≥ ₹10 lakh → auto-report\n");
    printf("  STR:           Suspicious pattern → STR to FIU-IND (fiuindia.gov.in)\n");
    printf("  VKYC:          sigma-bank vkyc --pan %s (RBI Master Direction)\n", pan);
    printf("  CERSAI:        sigma-bank cersai check --pan %s\n", pan);
}
