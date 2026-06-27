// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_isro.h — ISRO integration: NavIC, Bhuvan, MOSDAC
 *
 * India's space infrastructure as OS-level primitives.
 * No Google Maps. No AWS weather. Entirely sovereign Indian data.
 *
 * NavIC  — India's own GPS (7 satellites, sub-10m accuracy in India)
 * Bhuvan — ISRO's geoportal (satellite imagery, GIS data)
 * MOSDAC — Met & Oceanographic Satellite Data Archival Centre (ISRO/SAC)
 * Cartosat — High-resolution Indian satellite imagery
 * ResourceSat — Land use, agriculture, forests
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── NavIC position ──────────────────────────────────────────────────────── */
typedef struct {
    double   latitude;           /* decimal degrees                         */
    double   longitude;
    double   altitude_m;
    double   accuracy_m;         /* horizontal accuracy in metres           */
    sigma_u64 timestamp_ns;
    sigma_u32 satellites_used;
    bool     navic_fix;          /* true = NavIC; false = GPS fallback      */
    bool     fix_valid;
} sigma_navic_fix_t;

int sigma_navic_get_position(sigma_navic_fix_t *out);
int sigma_navic_start(void (*cb)(const sigma_navic_fix_t *fix, void *ctx),
                       void *ctx);
void sigma_navic_stop(void);

/* ── Bhuvan maps ─────────────────────────────────────────────────────────── */
typedef struct {
    double   lat_min, lon_min;
    double   lat_max, lon_max;
    sigma_u32 zoom;              /* 1-18                                    */
    char     layer[32];          /* "cartosat", "liss4", "terrain", "admin" */
    char     format[8];          /* "png", "jpeg"                           */
} sigma_bhuvan_tile_req_t;

int sigma_bhuvan_tile(const sigma_bhuvan_tile_req_t *req,
                       sigma_u8 **tile_data_out, size_t *tile_len_out);

int sigma_bhuvan_reverse_geocode(double lat, double lon,
                                   char *address_out, size_t max_len);

/* ── MOSDAC weather ──────────────────────────────────────────────────────── */
typedef struct {
    double   lat, lon;
    char     district[64];
    char     state[3];
    /* Current conditions */
    double   temp_celsius;
    double   humidity_pct;
    double   rainfall_mm_24h;
    double   wind_speed_kmh;
    char     wind_direction[8]; /* "NE", "SW" etc.                         */
    /* Forecast (next 3 days) */
    double   forecast_temp[3];
    double   forecast_rain[3];
    char     forecast_desc[3][64];
    /* Agricultural advisory */
    char     agri_advisory[512]; /* "Delay sowing — rain expected in 48h"  */
} sigma_mosdac_weather_t;

int sigma_mosdac_weather(double lat, double lon,
                           sigma_mosdac_weather_t *out);

int sigma_mosdac_cyclone_alerts(char *alerts_json_out, size_t max_len);

/* ── Cartosat / ResourceSat imagery ─────────────────────────────────────── */
typedef struct {
    double   lat, lon;
    double   radius_km;
    char     satellite[16];  /* "cartosat-3", "resourcesat-2a"             */
    sigma_u64 image_date_epoch;
    char     purpose[32];    /* "agriculture", "urban", "disaster"         */
} sigma_isro_imagery_req_t;

int sigma_isro_request_imagery(const sigma_isro_imagery_req_t *req,
                                 char *download_url_out, size_t max_len);
