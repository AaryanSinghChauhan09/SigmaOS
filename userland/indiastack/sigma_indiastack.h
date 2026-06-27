// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_indiastack.h — IndiaStack native integration
 *
 * IndiaStack = the world's largest open API platform for digital public goods.
 * Every Indian citizen interacts with it daily (UPI, Aadhaar, DigiLocker).
 * SigmaOS is the first OS where IndiaStack is a first-class OS primitive.
 *
 * Integrated components:
 *   UPI 2.0          — Unified Payments Interface (NPCI)
 *   ONDC             — Open Network for Digital Commerce
 *   OCEN             — Open Credit Enablement Network (lending APIs)
 *   Account Aggregator — RBI's consent-based financial data sharing
 *   DigiLocker       — Official document storage (driving licence, marks)
 *   Aadhaar Auth     — Offline XML, OTP, biometric (UIDAI APIs)
 *   e-RUPI           — Prepaid digital vouchers (welfare delivery)
 *   ABDM             — Ayushman Bharat Digital Mission (health IDs)
 *   GSTN APIs        — GST Network (GSTR filing, e-Invoice, eWay Bill)
 *   MCA APIs         — Ministry of Corporate Affairs (ROC filings)
 *   eCourts          — National court case management
 *   PARIVAHAN        — Vehicle registration + driving licence
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── UPI 2.0 ─────────────────────────────────────────────────────────────── */
typedef struct {
    char   vpa[64];             /* Virtual Payment Address: "user@upi"      */
    char   merchant_name[128];
    sigma_s64 amount_paise;    /* 0 = collect (customer enters amount)      */
    char   txn_ref[64];        /* merchant reference                        */
    char   remarks[128];
    /* Output after payment */
    char   upi_txn_id[64];
    bool   success;
    char   error_msg[256];
} sigma_upi_t;

int sigma_upi_generate_qr(const sigma_upi_t *req, char *qr_string_out, size_t max);
int sigma_upi_collect(const sigma_upi_t *req, char *payment_link_out, size_t max);
int sigma_upi_verify_txn(const char *txn_id, sigma_upi_t *status_out);

/* ── ONDC (Open Network for Digital Commerce) ────────────────────────────── */
typedef struct {
    char   network_id[64];    /* ONDC registry network participant ID       */
    char   city_code[8];      /* "std:080" = Bangalore                     */
    char   category[32];      /* "F&B", "Grocery", "Electronics"           */
    char   provider_id[64];
} sigma_ondc_t;

int sigma_ondc_search(const sigma_ondc_t *ctx, const char *query,
                       char *results_json_out, size_t max_len);
int sigma_ondc_order(const sigma_ondc_t *ctx, const char *item_id,
                      sigma_s64 qty, char *order_json_out, size_t max_len);

/* ── Account Aggregator (RBI) ────────────────────────────────────────────── */
typedef struct {
    char   aa_handle[64];      /* AA handle: "user@onemoney"               */
    char   fip_id[32];         /* Financial Information Provider ID        */
    char   consent_id[64];     /* consent artefact ID                      */
    sigma_u64 consent_from_epoch;
    sigma_u64 consent_to_epoch;
    char   fi_type[16];        /* "DEPOSIT", "MUTUAL_FUNDS", "INSURANCE"  */
} sigma_aa_t;

int sigma_aa_consent_request(const sigma_aa_t *req, char *consent_url_out, size_t max);
int sigma_aa_fetch_data(const sigma_aa_t *req, char *fi_json_out, size_t max_len);

/* ── DigiLocker ──────────────────────────────────────────────────────────── */
int sigma_digilocker_get_document(const char *aadhaar_no,
                                   const char *doc_type,   /* "DRVLC", "VHRCR" */
                                   const char *doc_id,
                                   char *xml_out, size_t max_len);

/* ── Aadhaar Auth ────────────────────────────────────────────────────────── */
int sigma_aadhaar_offline_xml(const char *aadhaar_no,
                               const char *share_code,     /* 4-digit         */
                               char *xml_out, size_t max_len);

/* ── e-RUPI voucher ──────────────────────────────────────────────────────── */
typedef struct {
    char   beneficiary_mobile[16];
    char   purpose[64];          /* "healthcare", "fertilizer", "education" */
    sigma_s64 amount_paise;
    sigma_u64 expiry_epoch;
    char   voucher_code[32];     /* output: QR code data                    */
} sigma_erupi_t;

int sigma_erupi_create(const sigma_erupi_t *voucher);
int sigma_erupi_redeem(const char *voucher_code, char *status_out, size_t max);

/* ── ABDM Health ID ──────────────────────────────────────────────────────── */
int sigma_abdm_create_abha(const char *aadhaar_no, const char *mobile,
                             char *abha_number_out, size_t max_len);
int sigma_abdm_link_record(const char *abha_no, const char *hip_id,
                             char *health_id_out, size_t max_len);
