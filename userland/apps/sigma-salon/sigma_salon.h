// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_salon.h — Beauty & wellness salons (Shop & Establishment, GST 18%)
 * ₹90,000 crore beauty industry
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

typedef struct {
    sigma_u32  id;
    char       name[128];
    char       phone[16];
    char       service_history[8][64]; /* last 8 services                 */
    int        n_history;
    char       color_formula[256];     /* saved hair color formula         */
    char       treatment_notes[512];
    sigma_s64  loyalty_points;
    sigma_u64  last_visit_epoch;
} sigma_salon_client_t;

typedef struct {
    sigma_u32  stylist_id;
    char       name[128];
    sigma_s64  monthly_revenue_paise;
    double     commission_pct;
    sigma_s64  commission_paise;
    sigma_u64  period_start_epoch;
    sigma_u64  period_end_epoch;
} sigma_stylist_commission_t;

typedef struct {
    char       service_name[64];
    sigma_s64  price_paise;
    double     gst_rate;           /* 18% on salon services               */
    sigma_u32  duration_min;
    char       stylist_name[64];
    sigma_u64  appointment_epoch;
    bool       completed;
} sigma_appointment_t;

int sigma_salon_client_create(const sigma_salon_client_t *c);
int sigma_salon_appointment_book(const sigma_appointment_t *a);
int sigma_salon_commission_calc(sigma_stylist_commission_t *c);
int sigma_salon_gst_invoice(sigma_u32 client_id,
                             const sigma_appointment_t *services,
                             int n_services, char *invoice_json_out, size_t max);
int sigma_salon_inventory_alert(const char *product_name,
                                 sigma_u32 current_stock,
                                 sigma_u32 reorder_level,
                                 bool *alert_out);
