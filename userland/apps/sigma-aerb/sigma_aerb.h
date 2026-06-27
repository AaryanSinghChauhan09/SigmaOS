// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_aerb.h — Nuclear & radiation professionals (AERB, DAE, BARC)
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

typedef struct {
    char   facility_name[128];
    char   license_no[32];
    char   facility_type[32]; /* "X-ray", "CT", "LINAC", "PET"          */
    sigma_u64 license_expiry_epoch;
    bool   rpo_appointed;     /* Radiation Protection Officer            */
    char   rpo_name[128];
} sigma_aerb_facility_t;

typedef struct {
    char   worker_id[16];
    char   worker_name[128];
    sigma_u32 month_yyyymm;
    double tld_dose_msv;      /* TLD badge reading                       */
    double cumulative_dose_msv;
    double annual_limit_msv;  /* AERB: 20 mSv/year occupational         */
    bool   dose_exceeded;
} sigma_radiation_dose_t;

typedef struct {
    char   equipment_id[32];
    char   equipment_type[32];
    sigma_u64 test_date_epoch;
    double kvp_accuracy_pct;       /* ≤ 5% for diagnostic                */
    double mas_accuracy_pct;
    double hvl_mm_al;              /* Half-Value Layer                   */
    bool   all_tests_passed;
    char   tested_by[128];
} sigma_xray_qa_t;

int sigma_aerb_facility_register(const sigma_aerb_facility_t *f,
                                  char *application_no_out, size_t max);
int sigma_aerb_dose_log(const sigma_radiation_dose_t *dose);
int sigma_aerb_xray_qa(const sigma_xray_qa_t *qa);
int sigma_aerb_isotope_log(const char *isotope, double activity_mbq,
                            const char *purpose, sigma_u64 receipt_epoch);
