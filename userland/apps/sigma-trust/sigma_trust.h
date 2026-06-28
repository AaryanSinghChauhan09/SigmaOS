// SPDX-License-Identifier: GPL-2.0-only
// sigma_trust.h — SigmaOS Religious Institutions & NGO/Trust Management App
// Regulator: Charity Commissioner (state) / FCRA / IT Dept (12A/80G) / Waqf Board

#pragma once
#include <sigma_indiastack.h>

typedef enum {
    SIGMA_TRUST_TYPE_HINDU_TEMPLE  = 1,
    SIGMA_TRUST_TYPE_MASJID        = 2,
    SIGMA_TRUST_TYPE_CHURCH        = 3,
    SIGMA_TRUST_TYPE_GURUDWARA     = 4,
    SIGMA_TRUST_TYPE_CHARITABLE    = 5,  // Section 8 company or Trust
    SIGMA_TRUST_TYPE_NGO           = 6,
    SIGMA_TRUST_TYPE_WAQF          = 7,
} sigma_trust_type_t;

typedef struct {
    char   trust_name[128];
    sigma_trust_type_t type;
    char   registration_no[32];    // Charity Commissioner reg
    char   state[32];
    char   pan[12];
    char   gstin[16];              // If GST registered
    bool   twelve_a_registered;   // IT exemption on income
    char   twelve_a_cert[32];
    bool   eighty_g_registered;   // Donors get tax deduction
    char   eighty_g_cert[32];
    bool   fcra_registered;        // Foreign Contribution Regulation Act
    char   fcra_reg_no[16];        // If FCRA registered
    bool   csr_eligible;           // Can receive CSR funds
    char   darpan_id[32];          // NGO Darpan portal ID
} sigma_trust_registration_t;

// Donations tracker
typedef struct {
    char   receipt_no[32];
    char   donor_name[128];
    char   donor_pan[12];
    char   donor_mobile[12];
    double amount;
    char   payment_mode[32];       // Cash, UPI, Cheque, NEFT
    char   upi_ref[32];
    bool   eighty_g_eligible;
    char   form_10be_no[32];       // Certificate to donor
    time_t donation_date;
    char   purpose[128];
    bool   fcra_donation;          // Foreign contribution flag
    char   fcra_donor_country[4];  // If foreign
} sigma_trust_donation_t;

// 80G receipts — mandatory for donor deductions
typedef struct {
    char   form_10be_no[32];       // Auto-generated serial
    char   donor_pan[12];
    char   donor_name[128];
    double donation_amount;
    double eligible_amount;        // 50% / 100% depending on trust category
    double deduction_pct;          // 50 or 100
    char   eighty_g_cert_no[32];
    char   fy[8];
    time_t donation_date;
    bool   form_10bd_included;     // Mandatory aggregation return (May 31)
} sigma_trust_eighty_g_receipt_t;

// FCRA compliance
typedef struct {
    char   fcra_reg_no[16];
    double fc_received_inr;        // Total foreign contribution this year
    double fc_utilised_inr;        // Amount spent
    double fc_balance_inr;
    char   fc_bank_sbi_ac[20];     // MANDATORY: FCRA funds only through SBI NDLS
    bool   fc4_return_filed;       // Annual return due Sept 30
    char   fc4_ack_no[32];
    time_t fc4_filed_date;
    bool   audit_done;             // CA audit mandatory
    char   auditor_pan[12];
} sigma_trust_fcra_t;

// Hundi (donation box) counting
typedef struct {
    char   hundi_id[16];
    char   location[64];           // Which temple/location
    time_t opening_time;
    time_t counting_date;
    int    counting_persons;       // Need ≥ 2 persons for trust records
    double cash_amount;
    uint32_t coin_count;
    char   witness_1[64];
    char   witness_2[64];
    char   remarks[256];
} sigma_trust_hundi_count_t;

int sigma_trust_register(sigma_trust_registration_t *trust);
int sigma_trust_donation_receipt(sigma_trust_donation_t *donation,
                                  const char *pdf_output);
int sigma_trust_80g_issue(sigma_trust_eighty_g_receipt_t *cert,
                           const char *pdf_output);
int sigma_trust_form_10bd_file(const char *trust_pan, const char *fy,
                                sigma_trust_donation_t *donations, int count);
int sigma_trust_fcra_annual_return(sigma_trust_fcra_t *fcra,
                                    const char *output_pdf);
int sigma_trust_hundi_log(sigma_trust_hundi_count_t *count);
// CLI: sigma-trust fcra return FC4 --fy 2025-26
//      sigma-trust 80g receipt --donor "Ram Sharma" --amount 51000
//      sigma-trust donations hundi-count --date today --count 15000
