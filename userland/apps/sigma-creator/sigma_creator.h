// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_creator.h — Content creators & influencers (ASCI, SEBI, tax 44ADA)
 * 80 million+ content creators in India
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* Disclosure check (ASCI Influencer Guidelines 2021) */
typedef struct {
    char   content_type[32];   /* "review", "ad", "sponsored", "gifted"   */
    bool   paid;
    bool   gifted;
    bool   affiliate_link;
    bool   financial_content;  /* SEBI rules apply                        */
    /* Required disclosures */
    char   required_label[32]; /* "#Ad", "#Sponsored", "#Collab"          */
    bool   prominent_placement;/* must be in first 3 lines / 3 seconds    */
    bool   sebi_disclaimer_needed;
} sigma_creator_disclosure_t;

/* Creator tax (44ADA presumptive taxation) */
typedef struct {
    sigma_s64  gross_receipts_paise;
    bool       opted_44ada;       /* 50% deemed expense if opted          */
    sigma_s64  taxable_income_paise;
    sigma_s64  tax_payable_paise;
    sigma_s64  tds_deducted_paise; /* from brands (194J/194C)             */
    sigma_s64  net_tax_paise;
    bool       gst_registration_required; /* if > ₹20L receipts          */
    bool       foreign_income;    /* FEMA reporting needed               */
} sigma_creator_tax_t;

/* Brand deal invoice */
typedef struct {
    char   brand_name[128];
    char   brand_gstin[16];
    char   creator_name[128];
    char   creator_gstin[16];
    char   deliverables[512];
    sigma_s64  fee_paise;
    double gst_rate;             /* 18% on creator services               */
    sigma_s64  gst_paise;
    double tds_pct;              /* typically 10% (194J) or 1% (194C)    */
    sigma_s64  tds_paise;
    sigma_s64  net_paise;
    sigma_u64  due_epoch;
} sigma_creator_invoice_t;

int sigma_creator_disclosure_check(const sigma_creator_disclosure_t *d,
                                    char *compliance_out, size_t max);
int sigma_creator_tax_44ada(sigma_creator_tax_t *tax);
int sigma_creator_invoice_generate(const sigma_creator_invoice_t *inv,
                                    char *pdf_path_out, size_t max);
int sigma_creator_gst_register_check(sigma_s64 annual_receipts_paise,
                                      bool *registration_required);
