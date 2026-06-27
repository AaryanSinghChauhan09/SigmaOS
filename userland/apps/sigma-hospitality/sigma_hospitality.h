// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_hospitality.h — Hotels, tourism, travel agents (MOT, HRACC, FRRO)
 * ₹15.24 lakh crore tourism sector
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

typedef enum {
    SIGMA_HOTEL_1STAR = 1, SIGMA_HOTEL_2STAR = 2, SIGMA_HOTEL_3STAR = 3,
    SIGMA_HOTEL_4STAR = 4, SIGMA_HOTEL_5STAR = 5, SIGMA_HOTEL_BUDGET = 6,
} sigma_hotel_class_t;

typedef struct {
    sigma_u32  room_no;
    char       room_type[32];    /* "Deluxe", "Suite", "Standard"           */
    sigma_s64  rack_rate_paise;  /* per night MRP                           */
    bool       occupied;
    char       guest_name[128];
    char       guest_passport[16]; /* mandatory for foreign guests          */
    char       guest_country[3];
    sigma_u64  checkin_epoch;
    sigma_u64  checkout_epoch;
    bool       form_c_submitted; /* mandatory within 24h for foreign guests */
} sigma_room_t;

/* GST on hotel rooms (FY 2024-25 rates) */
static inline double sigma_hotel_gst_rate(sigma_s64 room_rent_paise) {
    if (room_rent_paise <= 100000) return 0.0;    /* ≤ ₹1,000: exempt       */
    if (room_rent_paise <= 750000) return 12.0;   /* ₹1,001–₹7,500: 12%     */
    return 18.0;                                   /* > ₹7,500: 18%          */
}

int sigma_hotel_checkin(sigma_room_t *room);
int sigma_hotel_checkout(sigma_u32 room_no, sigma_s64 *total_paise_out);
int sigma_hotel_form_c_submit(sigma_u32 room_no);
int sigma_hotel_gst_invoice(sigma_u32 room_no, char *json_out, size_t max);
int sigma_hotel_irctc_search(const char *from, const char *to,
                               sigma_u64 date_epoch, char *trains_json_out, size_t max);
int sigma_hotel_mot_classify(sigma_hotel_class_t class_type,
                               char *hracc_checklist_json_out, size_t max);
