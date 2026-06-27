// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_forest.h — Forest & wildlife officers
 * Indian Forest Act 1927, Forest Rights Act 2006, Wildlife Protection Act 1972
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── Wildlife schedule ───────────────────────────────────────────────────── */
typedef enum {
    SIGMA_WL_SCHEDULE_I   = 1,  /* Absolute protection (Tiger, Elephant)   */
    SIGMA_WL_SCHEDULE_II  = 2,  /* High protection                         */
    SIGMA_WL_SCHEDULE_III = 3,
    SIGMA_WL_SCHEDULE_IV  = 4,
    SIGMA_WL_SCHEDULE_V   = 5,  /* Vermin (may be hunted)                  */
    SIGMA_WL_SCHEDULE_VI  = 6,  /* Protected plants                        */
} sigma_wl_schedule_t;

/* ── Forest Rights Claim ─────────────────────────────────────────────────── */
typedef struct {
    sigma_u32  id;
    char       village_name[128];
    char       district[64];
    char       state[3];
    char       claimant_name[128];
    char       community_name[128];
    double     area_hectares;
    char       claim_type[32];   /* "individual", "community", "CFR"       */
    sigma_u64  filing_epoch;
    char       status[32];       /* "filed", "verified", "granted", "rejected" */
    char       gram_sabha_resolution[256];
} sigma_frc_claim_t;

/* ── Fire incident ───────────────────────────────────────────────────────── */
typedef struct {
    sigma_u64  detected_epoch;
    char       district[64];
    char       forest_division[64];
    double     lat, lon;
    char       severity[16];     /* "low", "medium", "high", "extreme"     */
    double     area_affected_ha;
    bool       firms_alert;      /* from NASA FIRMS (VIIRS/MODIS)           */
    char       action_taken[512];
    sigma_u64  contained_epoch;
} sigma_forest_fire_t;

/* ── Species encounter ───────────────────────────────────────────────────── */
typedef struct {
    char   species_name[128];
    char   scientific_name[128];
    sigma_wl_schedule_t schedule;
    sigma_u64 sighting_epoch;
    double lat, lon;
    char   division[64];
    char   observed_by[128];
    char   notes[512];
    bool   photo_evidence;
    bool   cites_listed;
} sigma_wildlife_sighting_t;

/* ── API ─────────────────────────────────────────────────────────────────── */
int sigma_forest_frc_file(const sigma_frc_claim_t *claim);
int sigma_forest_fire_alert(const sigma_forest_fire_t *fire);
int sigma_forest_wildlife_sighting(const sigma_wildlife_sighting_t *sighting);
int sigma_forest_species_schedule(const char *species_name,
                                   sigma_wl_schedule_t *sched_out);
int sigma_forest_campa_report(const char *division,
                               sigma_u32 fy_start, char *json_out, size_t max);
int sigma_forest_patrol_log(const char *officer_name, const char *beat,
                              sigma_u64 date_epoch, double km_covered,
                              const char *observations);
