// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_police.h — Police & law enforcement (BNS/BNSS 2024, CCTNS)
 * BNS replaces IPC, BNSS replaces CrPC, BSA replaces Indian Evidence Act
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── FIR ─────────────────────────────────────────────────────────────────── */
typedef struct {
    sigma_u32  fir_no;
    char       police_station[128];
    char       district[64];
    char       state[3];
    sigma_u64  date_of_occurrence_epoch;
    sigma_u64  date_of_report_epoch;
    char       complainant_name[128];
    char       complainant_phone[16];
    char       accused_name[256];         /* may be unknown                */
    char       bns_sections[16][16];      /* e.g. "BNS-101", "BNS-318"    */
    int        n_sections;
    char       brief_facts[2048];
    bool       zero_fir;                  /* BNSS: can file at any station */
    char       transfer_to_station[128];  /* if zero FIR                   */
    bool       victim_notified;           /* BNSS: SMS within 24h         */
    sigma_u64  victim_notified_epoch;
} sigma_fir_t;

/* ── IPC to BNS section mapping (key ones) ──────────────────────────────── */
typedef struct {
    const char *ipc;
    const char *bns;
    const char *description;
} sigma_ipc_bns_map_t;

static const sigma_ipc_bns_map_t IPC_BNS_MAP[] = {
    { "IPC-302", "BNS-101",  "Murder"                         },
    { "IPC-307", "BNS-109",  "Attempt to murder"              },
    { "IPC-376", "BNS-63",   "Rape"                           },
    { "IPC-420", "BNS-318",  "Cheating"                       },
    { "IPC-406", "BNS-316",  "Criminal breach of trust"       },
    { "IPC-379", "BNS-303",  "Theft"                          },
    { "IPC-392", "BNS-309",  "Robbery"                        },
    { "IPC-323", "BNS-115",  "Voluntarily causing hurt"       },
    { "IPC-498A","BNS-85",   "Cruelty by husband/relatives"   },
    { "IPC-354", "BNS-74",   "Assault with intent to outrage" },
    { "IPC-363", "BNS-137",  "Kidnapping"                     },
    { "IPC-120B","BNS-61",   "Criminal conspiracy"            },
    { nullptr,   nullptr,    nullptr                           },
};

/* ── Traffic fine (Motor Vehicles Act 2019) ──────────────────────────────── */
typedef struct {
    const char *offense;
    sigma_s64  fine_paise;
} sigma_traffic_fine_t;

static const sigma_traffic_fine_t TRAFFIC_FINES[] = {
    { "drunken-driving",       1000000 },  /* ₹10,000 first offence        */
    { "over-speeding",          200000 },  /* ₹2,000                       */
    { "mobile-phone",           500000 },  /* ₹5,000                       */
    { "no-helmet",               100000},  /* ₹1,000                       */
    { "no-seatbelt",             100000},
    { "signal-jumping",          100000},
    { "wrong-side",              100000},
    { "without-license",         500000},
    { "without-insurance",       200000},  /* ₹2,000                       */
    { nullptr, 0 },
};

/* ── API ─────────────────────────────────────────────────────────────────── */
int sigma_police_fir_create(const sigma_fir_t *fir, char *fir_no_out, size_t max);
int sigma_police_ipc_to_bns(const char *ipc_section, char *bns_out, size_t max);
int sigma_police_traffic_fine(const char *offense, sigma_s64 *fine_paise_out);
int sigma_police_echallan(const char *vehicle_no, const char *offense,
                           char *challan_no_out, size_t max);
int sigma_police_cctns_lookup(const char *name_or_uid,
                               char *records_json_out, size_t max);
int sigma_police_vahan_verify(const char *vehicle_no,
                               char *owner_json_out, size_t max);
