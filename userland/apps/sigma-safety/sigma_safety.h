// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_safety.h — Industrial safety officers (Factories Act 1948, BOCW 1996)
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

typedef struct {
    char   factory_name[128];
    char   factory_address[256];
    char   factory_license[32];
    sigma_u32 workers_total;
    bool   hazardous_process;       /* Factories Act Section 41A         */
    bool   safety_committee;        /* mandatory if > 250 workers        */
    sigma_u64 license_expiry_epoch;
} sigma_factory_reg_t;

typedef struct {
    sigma_u64  drill_epoch;
    sigma_u32  participants;
    sigma_u32  duration_min;
    char       drill_type[32];      /* "fire evacuation", "chemical spill"*/
    char       observations[512];
    char       conducted_by[128];
    bool       all_areas_covered;
} sigma_fire_drill_t;

/* BOCW cess calculation: 1% of construction cost */
static inline sigma_s64 sigma_bocw_cess(sigma_s64 construction_cost_paise) {
    return construction_cost_paise / 100;  /* 1% */
}

/* Near miss report */
typedef struct {
    sigma_u64  incident_epoch;
    char       location[128];
    char       description[512];
    char       potential_consequences[256];
    char       immediate_action[256];
    char       root_cause[256];    /* 5-Why analysis                     */
    char       corrective_action[512];
    sigma_u64  completion_epoch;
    bool       closed;
} sigma_near_miss_t;

int sigma_safety_factory_register(const sigma_factory_reg_t *f,
                                   char *reg_no_out, size_t max);
int sigma_safety_fire_drill_log(const sigma_fire_drill_t *drill);
int sigma_safety_near_miss_report(const sigma_near_miss_t *nm);
int sigma_safety_fire_load_calc(double floor_area_sqm,
                                 const char *occupancy_type,
                                 double *fire_load_mjm2_out,
                                 const char **sprinkler_req_out);
int sigma_safety_accident_register(const char *factory_id,
                                    sigma_u64 date_epoch,
                                    const char *description,
                                    int fatalities, int injured,
                                    char *form26_json_out, size_t max);
