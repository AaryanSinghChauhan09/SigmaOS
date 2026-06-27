// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_accounts.h — Double-entry accounting engine for SigmaOS
 *
 * Inspired by: GnuCash, Tally ERP, Odoo Accounting, Akaunting
 *
 * Features:
 *   - Double-entry bookkeeping (every debit has an equal credit)
 *   - Chart of accounts (assets, liabilities, equity, income, expenses)
 *   - GST-aware vouchers (GSTR-1, GSTR-3B auto-population)
 *   - e-Invoice IRN generation (GSTN API integration)
 *   - eWay Bill generation from sales vouchers
 *   - DID-signed audit trail (every voucher signed by owner's DID)
 *   - Multi-currency support (₹ INR default, configurable)
 *   - Financial year support (Indian FY: April 1 – March 31)
 *   - Tally XML import/export compatibility
 *
 * Architecture:
 *   sigma-accounts app → sigma-bus → sigmad/accounts daemon
 *   sigmad/accounts → SQLite ledger at /sigma/var/accounts/<company>.db
 *   Audit trail signed → sigma-trustd (DID signatures)
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── Account types ───────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_ACCT_ASSET       = 1,  /* Current/fixed assets              */
    SIGMA_ACCT_LIABILITY   = 2,  /* Current/long-term liabilities     */
    SIGMA_ACCT_EQUITY      = 3,  /* Owner's equity, retained earnings */
    SIGMA_ACCT_INCOME      = 4,  /* Sales, service income             */
    SIGMA_ACCT_EXPENSE     = 5,  /* Cost of goods, operating expenses */
    SIGMA_ACCT_GST_OUTPUT  = 6,  /* CGST/SGST/IGST output liability   */
    SIGMA_ACCT_GST_INPUT   = 7,  /* CGST/SGST/IGST input credit       */
} sigma_acct_type_t;

/* ── Account ─────────────────────────────────────────────────────────────── */
typedef struct {
    sigma_u32       id;
    char            name[128];
    char            code[16];       /* e.g. "1010" cash, "4010" sales */
    sigma_acct_type_t type;
    sigma_u32       parent_id;      /* 0 = top-level account          */
    char            gstin[16];      /* for party accounts             */
    char            pan[11];
    char            hsn[8];         /* for stock/service accounts     */
    bool            active;
    /* Balance (in paise — no floating point for money) */
    sigma_s64       balance_paise;  /* negative = credit balance      */
} sigma_account_t;

/* ── Voucher types ───────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_VCH_SALES     = 1,  /* Sales invoice                        */
    SIGMA_VCH_PURCHASE  = 2,  /* Purchase invoice                     */
    SIGMA_VCH_PAYMENT   = 3,  /* Payment to creditor                  */
    SIGMA_VCH_RECEIPT   = 4,  /* Receipt from debtor                  */
    SIGMA_VCH_JOURNAL   = 5,  /* General journal entry                */
    SIGMA_VCH_CONTRA    = 6,  /* Bank↔Cash transfer                   */
    SIGMA_VCH_CREDIT_NOTE = 7,
    SIGMA_VCH_DEBIT_NOTE  = 8,
} sigma_vch_type_t;

/* ── Ledger entry (one line of a voucher) ────────────────────────────────── */
typedef struct {
    sigma_u32  account_id;
    sigma_s64  amount_paise;  /* positive = debit, negative = credit  */
    char       narration[256];
    char       hsn[8];
    double     gst_rate;      /* 0.0, 5.0, 12.0, 18.0, 28.0          */
    sigma_s64  cgst_paise;
    sigma_s64  sgst_paise;
    sigma_s64  igst_paise;
} sigma_ledger_entry_t;

/* ── Voucher ──────────────────────────────────────────────────────────────── */
typedef struct {
    sigma_u32          id;
    char               number[32];    /* "INV/2024-25/0042"            */
    sigma_vch_type_t   type;
    sigma_u64          date_epoch;    /* Unix timestamp (IST)          */
    char               party_name[128];
    char               party_gstin[16];
    sigma_ledger_entry_t entries[32];
    int                n_entries;
    sigma_s64          total_paise;   /* sum of debit entries          */
    char               narration[512];
    /* GST fields */
    char               irn[64];       /* e-Invoice IRN (64 hex chars)  */
    char               eway_bill[13]; /* eWay Bill number              */
    char               qr_code[512];  /* QR code data for e-Invoice    */
    /* Audit */
    char               did_signature[128]; /* DID signature hex        */
    bool               verified;
} sigma_voucher_t;

/* ── API ─────────────────────────────────────────────────────────────────── */

/* Create a new account in the chart of accounts. */
int sigma_accounts_create(const sigma_account_t *acct);

/* Post a voucher (validates double-entry before accepting). */
int sigma_accounts_post(const sigma_voucher_t *vch);

/* Get account balance as of a given date. */
sigma_s64 sigma_accounts_balance(sigma_u32 acct_id, sigma_u64 as_of_epoch);

/* Generate GSTR-1 report for a period (returns JSON). */
int sigma_accounts_gstr1(sigma_u64 from_epoch, sigma_u64 to_epoch,
                          char *json_out, size_t max_len);

/* Generate GSTR-3B liability summary. */
int sigma_accounts_gstr3b(sigma_u64 from_epoch, sigma_u64 to_epoch,
                           char *json_out, size_t max_len);

/* Generate e-Invoice IRN via GSTN API. */
int sigma_accounts_generate_irn(sigma_voucher_t *vch);

/* Generate eWay Bill from a sales voucher. */
int sigma_accounts_generate_eway(sigma_voucher_t *vch);

/* Import from Tally XML export. */
int sigma_accounts_import_tally(const char *xml_path,
                                 int *vouchers_imported,
                                 int *accounts_imported);

/* Export to Tally-compatible XML. */
int sigma_accounts_export_tally(const char *xml_path,
                                 sigma_u64 from_epoch, sigma_u64 to_epoch);

/* Sign a voucher with the owner's DID (stored in sigma-vault). */
int sigma_accounts_sign_voucher(sigma_voucher_t *vch);

/* Verify a voucher's DID signature. */
bool sigma_accounts_verify_voucher(const sigma_voucher_t *vch);
