// SPDX-License-Identifier: GPL-2.0-only
// sigma_cs.h — SigmaOS Company Secretary App
// Regulator: ICSI / MCA / SEBI LODR / Companies Act 2013 / FEMA

#pragma once
#include <sigma_indiastack.h>

// Secretarial Standards
typedef enum {
    SIGMA_CS_SS1_BOARD_MEETINGS   = 1,
    SIGMA_CS_SS2_GENERAL_MEETINGS = 2,
    SIGMA_CS_SS4_BOARD_REPORT     = 4,
} sigma_cs_standard_t;

typedef struct {
    char   cin[22];               // Corporate Identity Number
    char   company_name[128];
    char   cs_membership[12];     // ICSI membership number
    char   icsi_cp_no[16];        // Certificate of Practice number
    bool   listed;                // Listed on stock exchange?
    char   stock_exchange[8];     // "BSE", "NSE", "BOTH"
    char   isin[14];
} sigma_cs_company_t;

// ROC Filings
typedef struct {
    char   form_type[16];         // "MGT-7", "AOC-4", "DIR-12", etc.
    char   cin[22];
    char   financial_year[8];     // "2025-26"
    time_t due_date;
    bool   filed;
    time_t filed_date;
    char   srn[16];               // Service Request Number (MCA21)
    double late_fee;              // ₹ late fee if applicable
    char   attachment_path[256];
} sigma_cs_roc_filing_t;

// Board Meeting
typedef struct {
    char   meeting_id[32];
    char   cin[22];
    char   meeting_type[32];      // "Board", "AGM", "EGM", "Postal Ballot"
    time_t notice_date;           // SS-1: minimum 7 days notice for board (21 for GM)
    time_t meeting_date;
    char   venue[256];
    char   agenda_items[16][128]; // Agenda points
    int    agenda_count;
    bool   quorum_met;
    int    directors_present;
    int    total_directors;
    char   minutes_path[256];     // Path to signed minutes
    bool   minutes_signed;
    char   e_voting_platform[64]; // NSDL, CDSL, Karvy
} sigma_cs_board_meeting_t;

// SEBI LODR compliance
typedef struct {
    char   company[128];
    char   quarter[8];            // "Q1-FY26", etc.
    bool   board_meeting_intimated;      // 2 working days before
    bool   financial_results_filed;      // 45 days post quarter
    bool   shareholding_pattern_filed;   // 21 days post quarter
    bool   corporate_governance_filed;   // Annual
    bool   rpt_disclosed;                // Related Party Transactions
    bool   insider_trading_policy_reviewed;
    bool   whistle_blower_policy_reviewed;
    double compliance_score;             // % complete
} sigma_cs_lodr_compliance_t;

// FEMA
typedef struct {
    char   transaction_type[32];  // "FC-GPR", "FC-TRS", "APR"
    char   cin[22];
    double foreign_investment_inr;
    char   investor_country[4];   // ISO 3166
    char   fdi_sector[64];
    double fdi_cap_pct;           // Sectoral cap
    bool   approval_required;     // RBI/Govt route
    bool   filed;
    char   ebiz_srn[32];
} sigma_cs_fema_t;

int sigma_cs_roc_file(sigma_cs_roc_filing_t *filing);
int sigma_cs_board_meeting_notice(sigma_cs_board_meeting_t *meeting,
                                   const char *output_pdf);
int sigma_cs_lodr_compliance_check(const char *cin, const char *quarter,
                                    sigma_cs_lodr_compliance_t *out);
int sigma_cs_fema_report(sigma_cs_fema_t *fema);
int sigma_cs_due_date_calendar(const char *cin, const char *fy,
                                const char *output_json);
// CLI: sigma-cs roc file MGT-7 --cin U12345MH2020PTC123456
//      sigma-cs board-meeting agenda --company "XYZ Ltd" --date 2026-07-15
//      sigma-cs sebi lodr quarterly-compliance --quarter Q1-FY27
