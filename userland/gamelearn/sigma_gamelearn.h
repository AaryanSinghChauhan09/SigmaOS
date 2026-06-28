// SPDX-License-Identifier: GPL-2.0-only
// sigma_gamelearn.h — SigmaOS GameLearn Platform
// Purpose: Learn OS skills and digital literacy through games in Indian
//          languages. DID-signed completion certificates. School-credit
//          integration. Real skills unlocked after game completion.

#pragma once
#include <stdint.h>
#include <stdbool.h>
#include <time.h>

// ---------------------------------------------------------------------------
// Game Modules
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_GL_MODULE_DIGITAL_DUKAAN  = 1,  // Digital shopkeeper — sigma-accounts
    SIGMA_GL_MODULE_KISAN_KHEL      = 2,  // Farmer's game — sigma-agri
    SIGMA_GL_MODULE_SHASAN_GYAAN    = 3,  // Governance — sigma-gram
    SIGMA_GL_MODULE_SURAKSHA_CHAMP  = 4,  // Security champion — sigma-sec
    SIGMA_GL_MODULE_KANOON_RAKSHAK  = 5,  // Law guardian — sigma-legal
    SIGMA_GL_MODULE_SEHAT_HERO      = 6,  // Health hero — sigma-health
    SIGMA_GL_MODULE_PAISA_SAMAJH    = 7,  // Financial literacy — sigma-accounts
    SIGMA_GL_MODULE_TECH_EXPLORER   = 8,  // OS exploration — general
} sigma_gl_module_t;

typedef struct {
    sigma_gl_module_t module;
    char     module_name_en[64];
    char     module_name_hi[64];        // हिंदी name
    char     description_en[256];
    char     description_hi[256];
    char     real_skill_unlocked[128];  // What the player can actually do after
    char     sigma_app_trained[32];     // Which sigma-* app this trains
    uint32_t levels;
    uint32_t questions_per_level;
    uint32_t min_score_to_pass;         // Out of 100
    char     certificate_title[128];    // DID cert title on completion
    char     languages[8][8];           // Available languages (ISO 639)
    int      language_count;
} sigma_gl_module_info_t;

// Module descriptions:
// 1. Digital Dukaan — Play shopkeeper, make invoices, earn in-game money
//    Real skill: can create GST invoice in sigma-accounts in 10 minutes
// 2. Kisan Ka Khel — Farmer game: check MSP, file PMFBY, use eNAM
//    Real skill: sigma-agri fully understood
// 3. Shasan Gyaan — Sarpanch game: Gram Sabha, MGNREGS, certificates
//    Real skill: sigma-gram operations
// 4. Suraksha Champion — Attack/defend: protect your DID identity
//    Real skill: sigma-sec best practices
// 5. Kanoon ka Rakshak — Solve cases with BNS/BNSS, draft notices
//    Real skill: sigma-legal basics

// ---------------------------------------------------------------------------
// Player Progress
// ---------------------------------------------------------------------------

typedef struct {
    char     player_did[128];
    sigma_gl_module_t module;
    uint32_t current_level;
    uint32_t score;                  // Total score so far
    uint32_t high_score;
    bool     completed;
    time_t   started_at;
    time_t   completed_at;
    uint32_t time_spent_s;
    char     preferred_language[8]; // ISO 639 code
    // Achievements
    uint32_t badges_earned;
    char     badges[16][32];        // Badge names
    // Certificate
    bool     certificate_issued;
    char     certificate_did[128];  // DID-signed certificate
    char     certificate_hash[64];
    time_t   certificate_date;
} sigma_gl_progress_t;

// ---------------------------------------------------------------------------
// Question/Scenario Engine
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_GL_Q_MULTIPLE_CHOICE  = 1,
    SIGMA_GL_Q_FILL_BLANK       = 2,
    SIGMA_GL_Q_SIMULATION       = 3, // Actually perform action in sigma app
    SIGMA_GL_Q_SCENARIO         = 4, // Narrative choice (like a visual novel)
} sigma_gl_question_type_t;

typedef struct {
    uint32_t question_id;
    sigma_gl_question_type_t type;
    char     text_en[512];
    char     text_hi[512];           // Hindi translation
    char     text_local[512];        // Player's selected language
    char     options[4][128];        // For MCQ
    uint8_t  correct_option;         // 0-3
    char     explanation_en[256];    // Why this is correct
    char     explanation_hi[256];
    uint32_t points;
    char     hint[128];
    // For SIMULATION type: what action to perform
    char     sigma_command[128];     // e.g. "sigma-accounts invoice new"
    char     expected_output[256];
} sigma_gl_question_t;

// ---------------------------------------------------------------------------
// Reward System
// ---------------------------------------------------------------------------

typedef struct {
    char     badge_id[32];
    char     badge_name_en[64];
    char     badge_name_hi[64];
    char     description[128];
    char     module[32];
    uint32_t points_value;
    bool     sigma_certified;        // Appears on DID profile
    char     icon_emoji[8];
} sigma_gl_badge_t;

// Sample badges:
// 🏪 "Dukaan Master"        — Complete Digital Dukaan all levels
// 🌾 "Kisan Samrat"         — Complete Kisan Ka Khel
// ⚖️  "Kanoon Gyaani"        — Complete Kanoon ka Rakshak
// 🛡️  "Cyber Surakshak"      — Complete Suraksha Champion
// 🏆 "Sigma Certified"      — Complete ALL modules
// ⭐ "Leaderboard Top 10"   — State-level top scorer

// ---------------------------------------------------------------------------
// Leaderboard
// ---------------------------------------------------------------------------

typedef struct {
    uint32_t rank;
    char     player_name[64];        // First name only (privacy)
    char     district[64];
    char     state[32];
    uint32_t total_score;
    uint32_t modules_completed;
    bool     sigma_certified;
} sigma_gl_leaderboard_entry_t;

// ---------------------------------------------------------------------------
// School Integration
// ---------------------------------------------------------------------------

typedef struct {
    char     school_id[32];          // UDISE code
    char     school_name[128];
    char     district[64];
    char     state[32];
    uint32_t enrolled_students;
    uint32_t active_this_month;
    double   avg_completion_pct;
    uint32_t certificates_issued;
    // School can award marks for completion
    bool     marks_integration;      // School decided to give marks
    uint8_t  marks_per_module;       // e.g. 5 marks per completed module
    char     academic_year[8];
} sigma_gl_school_t;

// ---------------------------------------------------------------------------
// API
// ---------------------------------------------------------------------------

int sigma_gl_module_list(sigma_gl_module_info_t *modules, int *count);
int sigma_gl_start(sigma_gl_module_t module, const char *player_did,
                    const char *language, sigma_gl_progress_t *out);
int sigma_gl_get_question(const char *progress_id, uint32_t level,
                            sigma_gl_question_t *out);
int sigma_gl_answer(const char *progress_id, uint32_t question_id,
                     uint8_t answer, bool *correct, uint32_t *points_earned);
int sigma_gl_complete_module(const char *progress_id,
                               char *certificate_did_out);
int sigma_gl_leaderboard(const char *state, const char *district,
                           sigma_gl_leaderboard_entry_t *entries, int *count);
int sigma_gl_progress_get(const char *player_did, sigma_gl_module_t module,
                            sigma_gl_progress_t *out);

// School APIs
int sigma_gl_school_register(sigma_gl_school_t *school);
int sigma_gl_school_report(const char *school_id,
                             const char *academic_year,
                             const char *output_pdf);

// CLI:
// sigma-gamelearn list
// sigma-gamelearn start --module digital-dukaan --language Hindi
// sigma-gamelearn progress --module kisan-khel
// sigma-gamelearn leaderboard --state Maharashtra --district Pune
// sigma-gamelearn certificate --did <player-did> --module suraksha-champion
