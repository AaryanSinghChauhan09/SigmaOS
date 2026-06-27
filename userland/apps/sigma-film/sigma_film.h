// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_film.h — Film & entertainment (CBFC, IT Rules 2021, Copyright Act)
 * ₹2 lakh crore M&E industry
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

typedef enum {
    SIGMA_CERT_U    = 1,   /* Universal                                   */
    SIGMA_CERT_UA7  = 2,   /* Parental guidance — under 7                */
    SIGMA_CERT_UA13 = 3,
    SIGMA_CERT_UA16 = 4,
    SIGMA_CERT_A    = 5,   /* Adults only                                 */
    SIGMA_CERT_S    = 6,   /* Specialised audiences (medical/scientific)  */
} sigma_cbfc_cert_t;

typedef struct {
    char            title[128];
    char            language[32];
    sigma_u32       duration_min;
    sigma_cbfc_cert_t cert;
    char            cert_no[32];
    sigma_u64       cert_date_epoch;
    char            regional_board[32]; /* "Mumbai", "Chennai", "Delhi"   */
    bool            cut_list_cleared;
} sigma_film_cert_t;

typedef struct {
    char   scene_no[8];
    char   location[64];
    char   int_ext[4];     /* "INT" / "EXT"                               */
    char   day_night[8];
    char   cast[256];
    char   props[256];
    char   description[512];
    double est_duration_min;
} sigma_scene_breakdown_t;

typedef struct {
    char   content_id[32];
    char   title[128];
    char   content_type[32]; /* "film", "web-series", "short-film"        */
    sigma_cbfc_cert_t cert;
    bool   grievance_officer_appointed;
    char   grievance_officer_email[128];
    bool   monthly_report_filed;
    sigma_u64 last_report_epoch;
} sigma_ott_compliance_t;

int sigma_film_cbfc_apply(const sigma_film_cert_t *cert,
                           char *application_no_out, size_t max);
int sigma_film_scene_breakdown(const sigma_scene_breakdown_t *scenes,
                                int n_scenes, char *schedule_json_out, size_t max);
int sigma_film_ott_classify(sigma_ott_compliance_t *ott);
int sigma_film_iprs_register(const char *work_title, const char *composer,
                               char *registration_no_out, size_t max);
int sigma_film_copyright_register(const char *work_title, const char *author,
                                   char *cert_no_out, size_t max);
