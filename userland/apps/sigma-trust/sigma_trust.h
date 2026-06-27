// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_trust.h — Religious trusts, NGOs, FCRA, 80G compliance
 * Public Trust Act (state-wise), FCRA 2010, Income Tax 12A/80G
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

typedef enum {
    SIGMA_TRUST_PUBLIC_CHARITABLE = 1,
    SIGMA_TRUST_RELIGIOUS         = 2,
    SIGMA_TRUST_WAQF              = 3,
    SIGMA_TRUST_PRIVATE           = 4,
    SIGMA_TRUST_NGO_SECTION8      = 5,  /* Companies Act Section 8         */
} sigma_trust_type_t;

typedef struct {
    sigma_u32  id;
    char       name[128];
    char       registration_no[32];
    sigma_trust_type_t type;
    char       state[3];           /* "MH", "DL" etc.                      */
    char       pan[11];
    char       fcra_no[16];        /* FCRA registration (if applicable)    */
    bool       fcra_registered;
    bool       tax_12a;            /* 12A income tax exemption             */
    bool       tax_80g;            /* 80G donor deduction                  */
    sigma_u64  registration_epoch;
    sigma_u64  fcra_renewal_epoch;
} sigma_trust_t;

/* ── Donation record ─────────────────────────────────────────────────────── */
typedef struct {
    sigma_u32  id;
    sigma_u32  trust_id;
    char       donor_name[128];
    char       donor_pan[11];
    char       donor_phone[16];
    char       donor_email[128];
    sigma_s64  amount_paise;
    sigma_u64  date_epoch;
    char       mode[16];           /* "cash", "UPI", "NEFT", "cheque"      */
    char       utr[32];            /* bank reference                        */
    bool       foreign_donor;      /* FCRA required if foreign             */
    bool       receipt_issued;
    char       receipt_no[32];
    bool       form10be_issued;    /* mandatory for 80G donors             */
} sigma_donation_t;

/* ── 80G certificate ─────────────────────────────────────────────────────── */
typedef struct {
    char   trust_name[128];
    char   trust_pan[11];
    char   trust_80g_no[32];       /* ITBA 80G unique number               */
    char   donor_name[128];
    char   donor_pan[11];
    sigma_s64 donation_paise;
    sigma_s64 deductible_paise;    /* 50% or 100% depending on category   */
    sigma_u32 fy_start_year;
    char   certificate_no[32];
    sigma_u64 issue_epoch;
} sigma_80g_cert_t;

/* ── API ─────────────────────────────────────────────────────────────────── */
int sigma_trust_create(const sigma_trust_t *trust);
int sigma_trust_donation_add(const sigma_donation_t *don);
int sigma_trust_80g_cert_generate(const sigma_donation_t *don,
                                   const sigma_trust_t *trust,
                                   sigma_80g_cert_t *cert_out);
int sigma_trust_form10bd(sigma_u32 trust_id, sigma_u32 fy_start,
                          char *csv_out, size_t max_len);
int sigma_trust_fcra_return(sigma_u32 trust_id, sigma_u32 fy_start,
                             char *json_out, size_t max_len);
int sigma_trust_hundi_count(sigma_u32 trust_id, sigma_u64 date_epoch,
                             sigma_s64 amount_paise);
