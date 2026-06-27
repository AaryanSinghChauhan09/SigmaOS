// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_pos.h — Point-of-Sale terminal for Indian retail
 *
 * Target: ₹5,000–₹15,000 Android tablets and basic PCs (WASM browser mode)
 *
 * Features:
 *   - Barcode scan via phone/tablet camera (WebRTC + WASM barcode decoder)
 *   - UPI QR on every invoice (no payment gateway fees)
 *   - WhatsApp invoice sending (customer gets PDF on WhatsApp)
 *   - GST auto-calculation from HSN-mapped items
 *   - Offline mode — works without internet, syncs when back online
 *   - Integrates with sigma-inventory for stock deduction
 *   - Integrates with sigma-accounts for automatic voucher posting
 *   - GSTIN-based B2B invoicing with e-Invoice IRN generation
 *
 * sigma-bus integration:
 *   Sale posted → sigma.Accounts.VoucherPosted (auto-updates books)
 *   Sale posted → sigma.Inventory.StockDeducted (auto-updates stock)
 *   Day close   → sigma.Accounts.DayClose (Z-report generated)
 */
#include <sigma_kernel_types.h>
#include <userland/apps/sigma-accounts/sigma_accounts.h>
#include <stdbool.h>

/* ── Item in the POS catalogue ───────────────────────────────────────────── */
typedef struct {
    sigma_u32  id;
    char       barcode[32];
    char       name[128];
    char       hsn[8];
    double     gst_rate;          /* 0, 5, 12, 18, 28                      */
    sigma_s64  price_paise;       /* MRP in paise (₹1 = 100 paise)         */
    sigma_s64  stock_qty;         /* current stock (deducted on sale)       */
    char       unit[16];          /* "pcs", "kg", "litre"                   */
} sigma_pos_item_t;

/* ── Cart line ───────────────────────────────────────────────────────────── */
typedef struct {
    sigma_pos_item_t item;
    sigma_s64        qty_hundredths; /* qty × 100 (avoids float for kg etc.) */
    sigma_s64        line_total_paise;
    sigma_s64        discount_paise;
    sigma_s64        cgst_paise;
    sigma_s64        sgst_paise;
    sigma_s64        igst_paise;
} sigma_pos_line_t;

/* ── Payment method ──────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_PAY_CASH     = 1,
    SIGMA_PAY_UPI      = 2,   /* UPI QR code — Paytm, PhonePe, GPay         */
    SIGMA_PAY_CARD     = 3,   /* Debit/Credit card via POS terminal          */
    SIGMA_PAY_CREDIT   = 4,   /* Credit sale (B2B)                           */
    SIGMA_PAY_BARTER   = 5,   /* Exchange (e.g. old item for new)            */
} sigma_pos_payment_t;

/* ── Sale transaction ────────────────────────────────────────────────────── */
typedef struct {
    sigma_u32          id;
    char               invoice_no[32];   /* "INV/2024-25/0042"              */
    sigma_u64          timestamp_ns;
    char               customer_name[128];
    char               customer_phone[16];
    char               customer_gstin[16]; /* for B2B invoicing             */
    sigma_pos_line_t   lines[64];
    int                n_lines;
    sigma_s64          subtotal_paise;
    sigma_s64          total_discount_paise;
    sigma_s64          total_cgst_paise;
    sigma_s64          total_sgst_paise;
    sigma_s64          total_igst_paise;
    sigma_s64          grand_total_paise;
    sigma_pos_payment_t payment_method;
    sigma_s64          cash_tendered_paise; /* for cash payment             */
    sigma_s64          change_paise;
    char               upi_txn_id[64];    /* UPI transaction reference      */
    char               irn[64];           /* e-Invoice IRN (if B2B)         */
    bool               whatsapp_sent;
    bool               printed;
} sigma_pos_sale_t;

/* ── UPI QR data ─────────────────────────────────────────────────────────── */
typedef struct {
    char upi_id[64];          /* "merchant@upi"                              */
    char merchant_name[64];
    sigma_s64 amount_paise;   /* 0 = open amount (customer enters amount)    */
    char txn_ref[64];         /* invoice number for reconciliation           */
    char qr_string[512];      /* UPI deep link string for QR generation      */
} sigma_pos_upi_qr_t;

/* ── API ─────────────────────────────────────────────────────────────────── */

/* Look up item by barcode. */
int sigma_pos_lookup_barcode(const char *barcode, sigma_pos_item_t *out);

/* Add item to cart by barcode or ID. */
int sigma_pos_cart_add(sigma_pos_sale_t *sale, const char *barcode,
                        sigma_s64 qty_hundredths);

/* Apply discount (amount or percent). */
int sigma_pos_cart_discount(sigma_pos_sale_t *sale, int line_index,
                             sigma_s64 discount_paise);

/* Calculate totals + GST for entire cart. */
int sigma_pos_calculate(sigma_pos_sale_t *sale);

/* Generate UPI QR code string for the sale total. */
int sigma_pos_upi_qr(const sigma_pos_sale_t *sale,
                      const char *upi_id, sigma_pos_upi_qr_t *out);

/* Complete the sale: post to accounts, deduct stock, generate invoice. */
int sigma_pos_complete(sigma_pos_sale_t *sale);

/* Send invoice via WhatsApp (uses sigma-bus → WhatsApp bridge). */
int sigma_pos_whatsapp_send(const sigma_pos_sale_t *sale,
                             const char *phone_number);

/* Print receipt to thermal printer. */
int sigma_pos_print(const sigma_pos_sale_t *sale);

/* Generate daily Z-report (end-of-day summary). */
int sigma_pos_z_report(sigma_u64 date_epoch, char *json_out, size_t max_len);

/* Void a completed sale (creates credit note in sigma-accounts). */
int sigma_pos_void(sigma_u32 sale_id, const char *reason);
