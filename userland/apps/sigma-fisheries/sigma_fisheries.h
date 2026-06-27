// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_fisheries.h — Fishermen & aquaculture (PMMSY, MPEDA, Marine Fishing Regulation)
 * 28 million fishing community in India
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

typedef struct {
    char   vessel_reg[16];   /* Fishing vessel registration number         */
    char   owner_name[128];
    char   port[64];
    double length_m;
    bool   deep_sea;         /* >12 nautical miles from coast               */
    sigma_u64 license_expiry_epoch;
    bool   biometric_card;   /* Matsya Seva Kendra card issued              */
} sigma_vessel_reg_t;

typedef struct {
    char   vessel_reg[16];
    char   species[64];      /* "Pomfret", "Rohu", "Prawn", "Tuna"        */
    double weight_kg;
    char   landing_port[64];
    sigma_u64 catch_date_epoch;
    sigma_s64 price_per_kg_paise;
    char   export_cert[32];  /* MPEDA health certificate number            */
} sigma_catch_log_t;

/* State-wise fishing ban periods (monsoon ban — 61 days coastal, longer mechanised) */
typedef struct {
    char   state[3];
    sigma_u32 ban_start_yday; /* day of year ban starts (typically ~130)   */
    sigma_u32 ban_end_yday;
    bool   trawler_ban;
    bool   gillnet_ban;
} sigma_fishing_ban_t;

int sigma_fisheries_vessel_register(const sigma_vessel_reg_t *v);
int sigma_fisheries_catch_log(const sigma_catch_log_t *c);
int sigma_fisheries_ban_check(const char *state, sigma_u64 date_epoch,
                               bool *in_ban_period, char *details_out, size_t max);
int sigma_fisheries_pmmsy_apply(const char *scheme, sigma_s64 project_cost_paise,
                                 bool sc_st_women, sigma_s64 *subsidy_paise_out);
int sigma_fisheries_weather_alert(const char *district, char *alert_out, size_t max);
int sigma_fisheries_mpeda_register(const char *exporter_name,
                                    char *mpeda_reg_no_out, size_t max);
