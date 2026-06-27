// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_telecom.h — Telecom engineers (TRAI, DoT, WPC, EMF compliance)
 * ₹4 lakh crore Indian telecom sector
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

typedef struct {
    char     tower_id[32];
    double   lat, lon;
    double   height_m;
    char     operator_name[64];
    char     band[16];          /* "700MHz", "2100MHz", "3500MHz"           */
    double   max_eirp_dbm;
    sigma_u64 last_audit_epoch;
    bool     sacfa_clearance;   /* Standing Advisory Committee on Freq.    */
    bool     nocc_clearance;    /* No Objection from Civil Aviation        */
} sigma_bts_site_t;

typedef struct {
    char   site_id[32];
    double distance_from_tower_m;
    double measured_power_density_mWm2;
    double limit_mWm2;         /* DoT: 1/10th of ICNIRP = ~4.5 mW/m²     */
    bool   compliant;
    char   measurement_date[16];
} sigma_emf_measurement_t;

typedef struct {
    char   circle[32];         /* "MH", "DL", "TN" — TRAI circles         */
    char   period[8];          /* "2026-Q1"                                */
    double call_drop_rate_pct; /* TRAI threshold: < 2%                    */
    double voice_quality_mos;  /* Mean Opinion Score                      */
    double data_speed_mbps;    /* TRAI: 2 Mbps minimum 4G                */
    bool   compliant;
} sigma_qos_report_t;

int sigma_telecom_bts_register(const sigma_bts_site_t *site);
int sigma_telecom_emf_check(const sigma_emf_measurement_t *m);
int sigma_telecom_qos_report(const sigma_qos_report_t *r,
                              char *trai_upload_json_out, size_t max);
int sigma_telecom_wpc_license(const char *band, const char *operator_name,
                               double power_dbm, char *license_no_out, size_t max);
int sigma_telecom_ip_alloc(const char *asn, const char *prefix,
                            char *allocation_json_out, size_t max);
