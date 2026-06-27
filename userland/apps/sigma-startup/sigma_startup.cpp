// SPDX-License-Identifier: GPL-2.0-or-later
// sigma-startup — Tools for entrepreneurs and startups
// Covers: DPIIT recognition, MCA incorporation, IP, angel tax, ESOP, funding
//
// CLI:
//   sigma-startup dpiit apply --entity "SigmaOS Technologies Pvt Ltd"
//   sigma-startup trademark file --mark "SIGMAOS" --class 9,42
//   sigma-startup angel-tax check --round-size 5000000 --fmv 4500000
//   sigma-startup esop pool --equity-percent 10 --vesting "4yr-1yr-cliff"
//   sigma-startup valuation dcf --revenue 1000000 --growth 40 --years 5

#include <stdint.h>
#include <string.h>
#include <stdio.h>
#include <math.h>

// ── DPIIT Startup India Recognition ─────────────────────────────────────────

void sigma_startup_dpiit_apply(const char *entity_name) {
    printf("DPIIT Startup India Recognition — %s\n", entity_name);
    printf("\nEligibility:\n");
    printf("  ✓ Incorporated/Registered < 10 years\n");
    printf("  ✓ Turnover < ₹100 crore in any preceding FY\n");
    printf("  ✓ Working towards innovation/scale\n");
    printf("  ✓ Not formed by splitting existing business\n");
    printf("\nBenefits:\n");
    printf("  • 3-year tax exemption (§ 80IAC — 3 consecutive years out of 10)\n");
    printf("  • Angel tax exemption (§ 56(2)(viib)) — DPIIT certified\n");
    printf("  • Self-certification for 6 labour laws + 3 environmental laws\n");
    printf("  • Fast-track patent examination (80%% fee concession)\n");
    printf("  • Access to ₹10,000 Cr Fund of Funds (SIDBI)\n");
    printf("\nApplication:\n");
    printf("  Portal: startupindia.gov.in\n");
    printf("  Form: Self-declaration + Incorporation certificate\n");
    printf("  Processing time: 7-10 working days\n");
    printf("  sigma-digilocker pull --doc mca-certificate --entity \"%s\"\n", entity_name);
}

// ── Angel Tax Calculator (§ 56(2)(viib)) ─────────────────────────────────────

void sigma_startup_angel_tax(uint64_t round_size, uint64_t fmv) {
    printf("Angel Tax Check (§ 56(2)(viib) Income Tax Act)\n");
    printf("  Round size (consideration):  ₹%llu\n", (unsigned long long)round_size);
    printf("  Fair Market Value (FMV):     ₹%llu\n", (unsigned long long)fmv);

    if (round_size <= fmv) {
        printf("  Result: NO angel tax applicable\n");
        printf("  (Consideration ≤ FMV — no excess consideration)\n");
    } else {
        uint64_t excess = round_size - fmv;
        uint64_t tax    = excess * 30 / 100;   // 30% + surcharge
        printf("  Excess consideration: ₹%llu\n", (unsigned long long)excess);
        printf("  Angel tax (@30%%):     ₹%llu (approx)\n", (unsigned long long)tax);
        printf("  EXEMPTION: Apply for DPIIT recognition → full exemption\n");
        printf("  Notification: CBDT Notification 13/2023 (DPIIT certified)\n");
    }
    printf("  FMV calculation methods: DCF or NAV (Rule 11UA)\n");
    printf("  sigma-startup valuation dcf (for DCF method)\n");
}

// ── Trademark Filing (Trade Marks Act 1999) ──────────────────────────────────

struct NiceClass {
    uint8_t     num;
    const char *description;
};

static const NiceClass nice_classes[] = {
    {  1, "Chemicals for industry, science, photography" },
    {  5, "Pharmaceuticals, medical preparations" },
    {  9, "Scientific/electrical apparatus; software; computers" },
    { 16, "Paper, printed materials, publications" },
    { 25, "Clothing, footwear, headgear" },
    { 35, "Advertising, business management, retail" },
    { 36, "Insurance, financial, banking services" },
    { 38, "Telecommunications" },
    { 41, "Education, entertainment, cultural services" },
    { 42, "Scientific services, IT/software, research" },
    { 45, "Legal services, security, personal/social services" },
    {  0, NULL }
};

void sigma_startup_trademark(const char *mark, const char *classes_csv) {
    printf("Trademark Filing — Mark: '%s'\n", mark);
    printf("Filing authority: Office of the Controller General of Patents,\n");
    printf("                  Designs and Trade Marks (IP India)\n");
    printf("Portal: ipindia.gov.in/trade-marks.htm\n");
    printf("\nSelected Nice Classes:\n");

    // Parse comma-separated class numbers
    char buf[64];
    strncpy(buf, classes_csv, sizeof(buf) - 1);
    char *tok = strtok(buf, ",");
    while (tok) {
        int cls = atoi(tok);
        for (int i = 0; nice_classes[i].description; i++) {
            if (nice_classes[i].num == cls) {
                printf("  Class %2d: %s\n", cls, nice_classes[i].description);
                break;
            }
        }
        tok = strtok(NULL, ",");
    }

    printf("\nFiling fees: ₹4,500/class (individual/startup online)\n");
    printf("Timeline: Examination 12-18 months | Registration 2-3 years\n");
    printf("TM Journal publication → 4-month opposition window\n");
    printf("search: sigma-startup trademark search --mark '%s'\n", mark);
}

// ── ESOP Pool Calculator (Companies Act 2013) ─────────────────────────────────

void sigma_startup_esop(float equity_pct, const char *vesting_schedule) {
    printf("ESOP Pool — %.1f%% equity | Schedule: %s\n", equity_pct, vesting_schedule);
    printf("\nCompliance (Companies Act 2013, Rule 12):\n");
    printf("  • Special Resolution required (shareholders)\n");
    printf("  • Board + Compensation Committee approval\n");
    printf("  • ESOP scheme filed with ROC (Form PAS-3)\n");
    printf("\nVesting schedule (%s):\n", vesting_schedule);
    if (strstr(vesting_schedule, "4yr")) {
        printf("  Year 1: 0%% (cliff) → Year 2: 25%% → Year 3: 50%% → Year 4: 75%%\n");
        printf("  Monthly after cliff: 25%%/36 = 0.69%% per month\n");
    }
    printf("\nTax treatment:\n");
    printf("  Grant:    No tax\n");
    printf("  Vest:     No tax (FMV - Exercise Price = perquisite at exercise)\n");
    printf("  Exercise: Perquisite taxed as salary (Form 12BA)\n");
    printf("  Sale:     Capital gains (STCG 15%% / LTCG 10%% after 24 months)\n");
    printf("\nValuation: Use sigma-startup valuation dcf for FMV\n");
    printf("409A equivalent: Rule 11UA valuation by SEBI-registered CA\n");
}

// ── DCF Valuation ─────────────────────────────────────────────────────────────

void sigma_startup_valuation_dcf(uint64_t revenue, float growth_pct,
                                  uint32_t years, float discount_pct) {
    printf("DCF Valuation\n");
    printf("  Base revenue: ₹%llu | Growth: %.0f%% | Years: %u | WACC: %.0f%%\n",
           (unsigned long long)revenue, growth_pct, years, discount_pct);

    float r = discount_pct / 100.0f;
    float g = growth_pct   / 100.0f;
    float terminal_growth = 0.03f; // 3% terminal growth
    float total_pv = 0.0f;
    float rev = (float)revenue;

    printf("\n  Year   Revenue(₹)    FCF(₹)      PV(₹)\n");
    for (uint32_t y = 1; y <= years; y++) {
        rev *= (1.0f + g);
        float fcf = rev * 0.20f;  // assume 20% FCF margin
        float pv  = fcf / powf(1.0f + r, (float)y);
        total_pv += pv;
        printf("  %4u   %10.0f    %8.0f    %8.0f\n", y, rev, fcf, pv);
    }

    // Terminal value
    float terminal_fcf = rev * 0.20f * (1.0f + terminal_growth);
    float terminal_val  = terminal_fcf / (r - terminal_growth);
    float terminal_pv   = terminal_val / powf(1.0f + r, (float)years);
    total_pv += terminal_pv;

    printf("  Terminal value PV: ₹%.0f\n", terminal_pv);
    printf("  ─────────────────────────────\n");
    printf("  Enterprise Value:  ₹%.0f\n", total_pv);
    printf("  Per share (10L shares): ₹%.2f\n", total_pv / 1000000.0f);
}

// ── Mudra Loan ────────────────────────────────────────────────────────────────

void sigma_startup_mudra(uint64_t amount) {
    printf("Pradhan Mantri MUDRA Yojana (PMMY)\n");
    if (amount <= 50000) {
        printf("  Category: SHISHU (up to ₹50,000)\n");
        printf("  Target: Micro enterprises, street vendors\n");
    } else if (amount <= 500000) {
        printf("  Category: KISHOR (₹50,001 – ₹5,00,000)\n");
        printf("  Target: Established businesses seeking expansion\n");
    } else if (amount <= 1000000) {
        printf("  Category: TARUN (₹5,00,001 – ₹10,00,000)\n");
        printf("  Target: Well-established MSMEs\n");
    } else {
        printf("  Amount ₹%llu exceeds MUDRA limit (₹10 lakh)\n",
               (unsigned long long)amount);
        printf("  Consider: CGTMSE / SIDBI schemes\n");
        return;
    }
    printf("  No collateral required\n");
    printf("  Apply via: any bank, MFI, or mudra.org.in\n");
    printf("  Processing: 7-14 working days\n");
}
