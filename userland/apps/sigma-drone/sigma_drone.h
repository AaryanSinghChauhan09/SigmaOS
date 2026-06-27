// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_drone.h — DGCA RPAS compliance & drone AI payloads
 *
 * India's Drone Rules 2021 + DGCA RPAS (Remotely Piloted Aircraft System)
 * India has one of the world's most permissive drone regulatory frameworks.
 *
 * Use cases:
 *   - Agriculture: crop health monitoring, precision spraying (Kisan Drones)
 *   - Survey: SVAMITVA property mapping, disaster assessment
 *   - Delivery: pharma (remote areas), e-commerce last-mile
 *   - Security: border surveillance, event management
 *   - Infrastructure: power line inspection, pipeline monitoring
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── RPAS class (Drone Rules 2021) ──────────────────────────────────────── */
typedef enum {
    SIGMA_RPAS_NANO   = 1,  /* < 250g — no permission needed for non-commercial */
    SIGMA_RPAS_MICRO  = 2,  /* 250g–2kg */
    SIGMA_RPAS_SMALL  = 3,  /* 2–25kg */
    SIGMA_RPAS_MEDIUM = 4,  /* 25–150kg */
    SIGMA_RPAS_LARGE  = 5,  /* > 150kg */
} sigma_rpas_class_t;

/* ── Drone registration ─────────────────────────────────────────────────── */
typedef struct {
    char   uin[20];               /* Unique Identification Number (DGCA)   */
    char   manufacturer[64];
    char   model[64];
    sigma_rpas_class_t class_type;
    double max_takeoff_weight_kg;
    char   owner_name[128];
    char   owner_aadhaar[12];
    char   owner_pilot_cert[32];  /* DGCA Remote Pilot Certificate         */
    sigma_u64 registration_epoch;
    sigma_u64 cert_expiry_epoch;
    bool   registered;
    bool   uti_approved;          /* Unmanned Traffic Integration          */
} sigma_drone_reg_t;

/* ── Flight plan ─────────────────────────────────────────────────────────── */
typedef struct {
    char       drone_uin[20];
    double     origin_lat, origin_lon;
    double     destination_lat, destination_lon;
    double     max_altitude_m;     /* AGL — above ground level             */
    sigma_u64  planned_start_epoch;
    sigma_u64  planned_end_epoch;
    char       purpose[32];        /* "agri", "survey", "delivery", "photo" */
    char       no_fly_checks[16][64]; /* results of NFZ checks             */
    int        n_checks;
    bool       dgca_permission;    /* Permission Artefact from DigitalSky  */
    char       permission_id[64];
} sigma_drone_flight_plan_t;

/* ── AI payload: crop health ──────────────────────────────────────────────── */
typedef struct {
    double   lat, lon;
    double   ndvi;              /* Normalized Difference Vegetation Index  */
    double   ndre;              /* Red Edge index (nitrogen stress)        */
    double   ndwi;              /* Water index (soil moisture)             */
    char     crop_type[32];
    double   estimated_yield_t_per_ha;
    char     advisory[512];    /* "Apply urea in NW quadrant of field"    */
    char     disease_detected[128]; /* if any, from ML model              */
    double   disease_confidence;
} sigma_drone_crop_health_t;

/* ── API ─────────────────────────────────────────────────────────────────── */
int sigma_drone_register(const sigma_drone_reg_t *reg,
                          char *uin_out, size_t max_len);
int sigma_drone_no_fly_check(double lat, double lon, double alt_m,
                               bool *in_no_fly_zone, char *zone_name_out);
int sigma_drone_flight_permission(sigma_drone_flight_plan_t *plan);
int sigma_drone_log_flight(const char *uin, double lat, double lon,
                             double alt_m, sigma_u64 ts_ns);
int sigma_drone_crop_health(const sigma_u8 *multispectral_image,
                              sigma_u32 width, sigma_u32 height,
                              double lat, double lon,
                              sigma_drone_crop_health_t *out);
