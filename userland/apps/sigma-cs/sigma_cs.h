// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_cs.h — Company Secretaries (ICSI compliance suite)
 * Companies Act 2013, Secretarial Standards SS-1/SS-2/SS-4, SEBI LODR
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── ROC Filing types ────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_ROC_MGT7   = 1,  /* Annual Return                       */
    SIGMA_ROC_MGT7A  = 2,  /* Abridged Annual Return (OPC/Small)  */
    SIGMA_ROC_AOC4   = 3,  /* Financial Statements                */
    SIGMA_ROC_AOC4XBRL = 4,/* XBRL Financial Statements           */
    SIGMA_ROC_DIR3KYC= 5,  /* Director KYC                        */
    SIGMA_ROC_DIR12  = 6,  /* Change of Directors                 */
    SIGMA_ROC_SH7    = 7,  /* Alteration of Share Capital         */
    SIGMA_ROC_INC22A = 8,  /* Active Company Tagging              */
    SIGMA_ROC_PAS3   = 9,  /* Return of Allotment                 */
} sigma_roc_form_t;

/* ── Board Meeting ───────────────────────────────────────────────────────── */
typedef struct {
    sigma_u32  id;
    char       company_cin[22];
    char       company_name[128];
    sigma_u64  meeting_date_epoch;
    char       venue[256];
    char       chairperson[128];
    char       directors[16][128];
    int        n_directors;
    int        quorum_required;   /* SS-1: 1/3 of total strength, min 2     */
    int        quorum_present;
    char       agenda_items[32][256];
    int        n_agenda;
    char       minutes_file[256]; /* path to signed minutes PDF             */
    bool       notice_sent;       /* SS-1: 21 days notice mandatory         */
    sigma_u64  notice_sent_epoch;
} sigma_board_meeting_t;

/* ── API ─────────────────────────────────────────────────────────────────── */

int  sigma_cs_roc_file(const char *cin, sigma_roc_form_t form,
                        const char *data_json, char *ack_out, size_t ack_max);

int  sigma_cs_board_meeting_notice(sigma_board_meeting_t *mtg);
int  sigma_cs_board_meeting_minutes(sigma_board_meeting_t *mtg,
                                     const char *resolutions_json);

int  sigma_cs_secretarial_audit(const char *cin, sigma_u32 fy_start,
                                  char *report_out, size_t report_max);

/* IPC ↔ BNS section mapper (useful cross-app; also used by sigma-police) */
int  sigma_cs_old_section_to_new(const char *act, const char *old_section,
                                   char *new_section_out, size_t max_len);
