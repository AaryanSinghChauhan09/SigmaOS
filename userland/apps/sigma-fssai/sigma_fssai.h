// SPDX-License-Identifier: GPL-2.0-only
// sigma_fssai.h — SigmaOS Food Safety & Restaurant Management App
// Regulator: FSSAI / Food Safety and Standards Act 2006
//            Food Safety and Standards (Licensing and Registration) Regulations 2011

#pragma once
#include <sigma_indiastack.h>

#define SIGMA_FSSAI_API        "https://foscos.fssai.gov.in/api"
#define SIGMA_FSSAI_SAC_CODE   "996331"  // Restaurant services SAC
#define SIGMA_FSSAI_GST_RATE_AC   5      // 5% GST for AC restaurants
#define SIGMA_FSSAI_GST_RATE_NON  5      // 5% GST for non-AC
#define SIGMA_FSSAI_GST_RATE_STAR 18     // 18% for starred hotels

typedef enum {
    SIGMA_FSSAI_BASIC_REG   = 1,   // Turnover < ₹12 lakh
    SIGMA_FSSAI_STATE_LIC   = 2,   // Turnover ₹12L – ₹20Cr
    SIGMA_FSSAI_CENTRAL_LIC = 3,   // Turnover > ₹20Cr / multi-state / importer
} sigma_fssai_license_type_t;

typedef struct {
    char   licence_no[18];         // 14-digit FSSAI licence number
    sigma_fssai_license_type_t type;
    char   business_name[128];
    char   fbo_name[128];          // Food Business Operator
    char   address[256];
    char   state[32];
    time_t issue_date;
    time_t expiry_date;
    bool   renewal_applied;
    char   kind_of_business[64];   // "Petty retailer", "Restaurant", "Manufacturer"
    double annual_turnover;
    int    hygiene_rating;         // 0-5 stars (Eat Right India)
    time_t last_inspection;
    time_t next_inspection;
} sigma_fssai_licence_t;

// Menu item with allergen declaration (mandatory from 2024)
typedef struct {
    char   item_code[16];
    char   item_name[64];
    double price;
    char   category[32];           // "Starter", "Main", "Dessert", "Beverage"
    bool   is_vegetarian;
    bool   is_vegan;
    bool   is_jain;
    bool   is_gluten_free;
    // Allergens (FSSAI 2024 mandatory declaration)
    bool   contains_gluten;
    bool   contains_crustaceans;
    bool   contains_eggs;
    bool   contains_fish;
    bool   contains_peanuts;
    bool   contains_soybeans;
    bool   contains_milk;
    bool   contains_nuts;
    bool   contains_sesame;
    bool   contains_sulphites;
    char   allergen_note[256];
    double portion_grams;
    double calories_kcal;
    char   hsn_sac[8];
    double gst_rate_pct;
} sigma_fssai_menu_item_t;

// HACCP — Critical Control Point monitoring
typedef struct {
    char   ccp_id[16];             // CCP-001, CCP-002...
    char   process_step[64];       // "Cooking", "Cold storage", "Reheating"
    char   hazard[128];            // What could go wrong
    double critical_limit_min;     // Minimum safe value
    double critical_limit_max;     // Maximum safe value
    char   unit[16];               // "°C", "pH", "minutes"
    double monitored_value;        // Actual reading
    bool   within_limits;
    time_t monitoring_time;
    char   monitored_by[64];
    char   corrective_action[256]; // If out of limits
    char   iot_sensor_id[32];      // If auto-monitored by IoT sensor
} sigma_fssai_ccp_t;

// Temperature log (cold chain)
typedef struct {
    char   zone[32];               // "Kitchen", "Cold room", "Freezer", "Display"
    double temperature_c;
    double target_temp_c;
    bool   in_range;
    time_t logged_at;
    char   logged_by[64];
    char   iot_sensor_id[32];
    char   corrective_action[256]; // If out of range
} sigma_fssai_temp_log_t;

// Order management (POS)
typedef struct {
    char   order_id[32];
    char   table_no[8];
    char   waiter_id[16];
    time_t order_time;
    time_t served_time;
    char   items[32][16];          // Item codes
    int    quantities[32];
    int    item_count;
    double subtotal;
    double cgst_amt;
    double sgst_amt;
    double total;
    char   payment_mode[16];       // "Cash", "UPI", "Card"
    char   upi_ref[32];
    bool   bill_issued;
    char   gstin_customer[16];     // If B2B — customer GSTIN
} sigma_fssai_order_t;

// Aggregator integration (Swiggy / Zomato / ONDC)
typedef struct {
    char   platform[16];           // "swiggy", "zomato", "ondc"
    char   restaurant_id[32];      // Platform's ID for the restaurant
    double avg_rating;
    int    total_reviews;
    double online_sales_month;
    double commission_pct;
    int    avg_delivery_time_min;
    bool   menu_synced;
    time_t last_sync;
} sigma_fssai_aggregator_t;

// API
int sigma_fssai_licence_verify(const char *licence_no,
                                sigma_fssai_licence_t *out);
int sigma_fssai_licence_type_determine(double annual_turnover,
                                        bool multi_state,
                                        sigma_fssai_license_type_t *type);
int sigma_fssai_menu_allergen_check(sigma_fssai_menu_item_t *items,
                                     int count, bool *compliant,
                                     char *violation_out);
int sigma_fssai_haccp_log(sigma_fssai_ccp_t *ccp);
int sigma_fssai_temp_log(sigma_fssai_temp_log_t *log);
int sigma_fssai_order_create(sigma_fssai_order_t *order,
                               const char *bill_pdf_out);
int sigma_fssai_recall_report(const char *product_name,
                               const char *batch_no,
                               const char *reason);
// CLI: sigma-fssai licence check --turnover 5000000
//      sigma-fssai haccp temperature log --zone Kitchen --temp 4
//      sigma-fssai hygiene audit --rating-checklist
//      sigma-fssai allergen check --menu menu.json
