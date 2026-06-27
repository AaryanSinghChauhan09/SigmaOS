// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_waste.h — Waste management (SWM Rules 2016, E-Waste 2022, BMW 2016)
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

typedef enum {
    SIGMA_WASTE_BMW_YELLOW = 1,  /* Anatomical/pathological               */
    SIGMA_WASTE_BMW_RED    = 2,  /* Contaminated recyclable               */
    SIGMA_WASTE_BMW_WHITE  = 3,  /* Sharps translucent                    */
    SIGMA_WASTE_BMW_BLUE   = 4,  /* Glass metallic sharps                 */
} sigma_bmw_category_t;

typedef struct {
    sigma_bmw_category_t category;
    double   weight_kg;
    sigma_u64 date_epoch;
    char     facility_id[32];
    char     cbwtf_name[64];
    char     manifest_no[32];
} sigma_bmw_log_t;

typedef struct {
    char   producer_name[128];
    char   producer_gstin[16];
    double sales_units;
    double collection_target_units;  /* CPCB-set target                    */
    double collection_achieved_units;
    bool   target_met;
    sigma_u32 year;
} sigma_ewaste_epr_t;

int sigma_waste_bmw_log(const sigma_bmw_log_t *entry);
int sigma_waste_ewaste_epr(const sigma_ewaste_epr_t *epr,
                            char *cpcb_upload_json_out, size_t max);
int sigma_waste_plastic_check(double thickness_microns, bool *compliant_out);
int sigma_waste_hazardous_manifest(const char *waste_type, double qty_kg,
                                    const char *from_facility,
                                    const char *to_facility,
                                    char *manifest_no_out, size_t max);
