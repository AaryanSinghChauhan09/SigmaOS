// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_mining.h — Mining professionals (DGMS, MMDR Act 2015)
 * Mines Act 1952, PESO explosive regulations, IBM reporting
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

typedef enum {
    SIGMA_MINE_OPENCAST      = 1,
    SIGMA_MINE_UNDERGROUND   = 2,
    SIGMA_MINE_BOTH          = 3,
} sigma_mine_type_t;

typedef enum {
    SIGMA_MIN_COAL     = 1,
    SIGMA_MIN_IRON_ORE = 2,
    SIGMA_MIN_LIMESTONE= 3,
    SIGMA_MIN_BAUXITE  = 4,
    SIGMA_MIN_GOLD     = 5,
    SIGMA_MIN_SAND     = 6,
    SIGMA_MIN_OTHER    = 99,
} sigma_mineral_type_t;

/* ── Accident report (mandatory within 2 hours to DGMS) ─────────────────── */
typedef struct {
    sigma_u64  incident_epoch;
    char       mine_name[128];
    char       mine_code[16];    /* DGMS unique code                        */
    char       location[128];    /* "Level 3, Face 7"                       */
    char       incident_type[32];/* "fatal", "serious", "dangerous-occurr"  */
    int        fatalities;
    int        seriously_injured;
    char       description[1024];
    char       probable_cause[512];
    bool       dgms_notified;    /* Form I filed within 2 hours             */
    sigma_u64  notified_epoch;
} sigma_mining_accident_t;

/* ── Mineral dispatch challan ────────────────────────────────────────────── */
typedef struct {
    char   challan_no[32];
    sigma_u64 date_epoch;
    char   mine_code[16];
    sigma_mineral_type_t mineral;
    double quantity_mt;          /* metric tonnes                            */
    char   vehicle_no[16];
    char   destination[128];
    char   buyer_gstin[16];
    sigma_s64 value_paise;
    sigma_s64 dmf_paise;        /* District Mineral Foundation levy          */
    sigma_s64 nmet_paise;       /* NMET levy                                 */
} sigma_mineral_challan_t;

/* ── HEMM maintenance log ────────────────────────────────────────────────── */
typedef struct {
    char   equipment_id[32];
    char   equipment_type[64];   /* "Dumper", "Shovel", "Drill", "Dozer"    */
    sigma_u64 maintenance_epoch;
    char   work_done[512];
    double hours_operated;
    double hours_total;
    bool   fit_for_use;
    char   certifying_person[128];
} sigma_hemm_log_t;

/* ── API ─────────────────────────────────────────────────────────────────── */
int sigma_mining_accident_report(const sigma_mining_accident_t *acc);
int sigma_mining_dispatch_challan(const sigma_mineral_challan_t *ch);
int sigma_mining_dgms_checklist(sigma_mine_type_t type, char *json_out, size_t max);
int sigma_mining_hemm_log(const sigma_hemm_log_t *entry);
int sigma_mining_production_report(const char *mine_code, sigma_u32 month_yyyymm,
                                    double quantity_mt, sigma_mineral_type_t mineral);
int sigma_mining_dmf_calc(sigma_mineral_type_t mineral, double qty_mt,
                           sigma_s64 value_paise, sigma_s64 *dmf_out, sigma_s64 *nmet_out);
