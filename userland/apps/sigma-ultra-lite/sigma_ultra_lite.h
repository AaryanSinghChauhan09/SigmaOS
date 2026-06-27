// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_ultra_lite.h — Sigma Ultra-Lite: 16MB RAM, feature phones, USSD
 *
 * 300 million Indians still use feature phones (JioPhone, Nokia 3310-era).
 * They cannot run a full OS. sigma-ultra-lite meets them where they are.
 *
 * Three access modes:
 *   1. USSD (*99# style menus) — works on any 2G phone, no data needed
 *   2. SMS — structured responses via SMS
 *   3. WAP/GPRS — ultra-light HTML for GPRS browsers (Nokia S40)
 *   4. sigma-thin — diskless PXE netboot on ₹5,000 basic PCs (16MB RAM)
 *
 * Available features in ultra-lite mode:
 *   - GST return filing (GSTR-3B simplified) via USSD
 *   - Bank balance check (UPI via USSD *99#)
 *   - Mandi prices (eNAM) via SMS
 *   - PMKISAN status via SMS
 *   - MGNREGA wages check via USSD
 *   - Weather alerts (MOSDAC) via SMS
 *   - Aadhaar authentication (OTP) via SMS
 *   - sigma-pos receipt via SMS to customer
 *
 * USSD session format (*369# for SigmaOS services):
 *   *369*1# → Balance
 *   *369*2# → GST
 *   *369*3# → Mandi prices
 *   *369*4# → MGNREGA
 *   *369*5# → Weather
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── USSD menu item ──────────────────────────────────────────────────────── */
typedef struct {
    char   code[8];          /* "1", "2", "1*2" etc.                       */
    char   label[64];        /* shown in USSD menu                         */
    char   handler[32];      /* which sigma service handles this           */
} sigma_ussd_item_t;

/* ── USSD session ────────────────────────────────────────────────────────── */
typedef struct {
    char   session_id[32];
    char   msisdn[16];       /* caller's mobile number                     */
    char   input[256];       /* current USSD input                         */
    int    depth;            /* menu depth                                 */
    bool   active;
} sigma_ussd_session_t;

/* ── SMS message ──────────────────────────────────────────────────────────── */
typedef struct {
    char   from_msisdn[16];
    char   to_msisdn[16];
    char   body[160];        /* 160 chars max per SMS                      */
    sigma_u64 timestamp_ns;
} sigma_sms_t;

/* ── Ultra-lite system requirements ─────────────────────────────────────── */
#define SIGMA_ULTRALITE_MIN_RAM_MB    16
#define SIGMA_ULTRALITE_MIN_FLASH_MB  64
#define SIGMA_ULTRALITE_KERNEL_SIZE_KB 512  /* entire kernel < 512KB       */

/* ── API ─────────────────────────────────────────────────────────────────── */

/* Handle incoming USSD request (*369# flow). */
int sigma_ussd_handle(const sigma_ussd_session_t *session,
                       char *response_out, size_t max_len,
                       bool *end_session_out);

/* Send an SMS via sigma-commnet or carrier API. */
int sigma_sms_send(const sigma_sms_t *msg);

/* Get mandi price via SMS-compatible response. */
int sigma_ultralite_mandi_price(const char *commodity,
                                 const char *district,
                                 char *sms_response_out, size_t max_len);

/* UPI balance via USSD (*99# NPCI integration). */
int sigma_ultralite_upi_balance(const char *vpa,
                                 char *ussd_response_out, size_t max_len);

/* GST 3B simplified via USSD. */
int sigma_ultralite_gstr3b_simple(const char *gstin,
                                   sigma_u32 month_yyyymm,
                                   sigma_s64 taxable_paise,
                                   char *ussd_response_out, size_t max_len);

/* Send invoice as SMS to customer. */
int sigma_ultralite_sms_invoice(const char *customer_mobile,
                                 const char *merchant_name,
                                 sigma_s64 amount_paise,
                                 const char *items_brief);
