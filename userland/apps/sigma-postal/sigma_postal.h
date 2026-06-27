// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_postal.h — Postal & courier (India Post, Indian Post Office Act 2023)
 * India Post = world's largest postal network
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

typedef enum {
    SIGMA_POST_SPEED       = 1,   /* Speed Post                          */
    SIGMA_POST_REGISTERED  = 2,   /* Registered Post                     */
    SIGMA_POST_PARCEL      = 3,
    SIGMA_POST_EXPRESS     = 4,   /* Express Parcel Post                 */
    SIGMA_POST_COURIER_DHL = 5,   /* DHL / BlueDart / Delhivery          */
    SIGMA_POST_COURIER_BD  = 6,
    SIGMA_POST_COURIER_DEL = 7,
} sigma_post_type_t;

typedef struct {
    char   awb[30];                /* Air/Waybill number                  */
    char   sender_name[128];
    char   sender_pin[7];
    char   recipient_name[128];
    char   recipient_pin[7];
    char   recipient_phone[16];
    double weight_kg;
    sigma_post_type_t service_type;
    sigma_s64 postage_paise;
    sigma_s64 gst_paise;           /* 18% on courier services            */
    bool   cod;                    /* Cash on Delivery                   */
    sigma_s64 cod_amount_paise;
    char   current_status[64];    /* "In Transit", "Out for Delivery"   */
    char   current_location[64];
    sigma_u64 dispatched_epoch;
    sigma_u64 estimated_delivery_epoch;
    bool   delivered;
    sigma_u64 delivered_epoch;
} sigma_shipment_t;

/* Postage rate calculator */
typedef struct {
    char   from_pin[7];
    char   to_pin[7];
    double weight_grams;
    sigma_post_type_t service;
    /* Output */
    sigma_s64 base_rate_paise;
    sigma_s64 gst_paise;
    sigma_s64 total_paise;
    sigma_u32 estimated_days;
} sigma_postage_rate_t;

int sigma_postal_track(const char *awb, sigma_shipment_t *out);
int sigma_postal_rate(sigma_postage_rate_t *calc);
int sigma_postal_cod_reconcile(sigma_u64 date_epoch,
                                sigma_s64 *collected_paise_out,
                                sigma_s64 *pending_paise_out);
int sigma_postal_ippb_balance(const char *account_no,
                               sigma_s64 *balance_paise_out);
int sigma_postal_pin_lookup(const char *pincode,
                             char *district_out, char *state_out,
                             char *delivery_office_out, size_t max);
