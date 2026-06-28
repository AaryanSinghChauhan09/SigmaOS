// SPDX-License-Identifier: GPL-2.0-only
// sigma_commnet.h — SigmaOS Community Internet Infrastructure
// Purpose: Village/colony-owned ISP — one upstream connection shared fairly
//          across community nodes. DID-based access control, local content
//          caching, fair-share QoS, TRAI community Wi-Fi rule compliant.
//
// Architecture:
//   [BSNL/Jio/Starlink upstream]
//          |
//   [sigma-commnet Gateway Node] (SigmaOS machine, 2 NICs)
//     |          |          |          |
//  House 1    House 2    House 3    School
//  (Node)     (Node)     (Node)     (Node)
//
// TRAI compliance: cost-sharing (not reselling) — permitted under
// Telecom Commercial Communications Customer Preference Regulations

#pragma once
#include <stdint.h>
#include <stdbool.h>
#include <time.h>

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

#define SIGMA_COMMNET_MAX_MEMBERS        256
#define SIGMA_COMMNET_MAX_CACHE_DOMAINS  1024
#define SIGMA_COMMNET_DEFAULT_QOS_FAIR   1     // Fair-share (HTB qdisc)
#define SIGMA_COMMNET_TRAI_MAX_NODES     20    // TRAI community Wi-Fi limit
#define SIGMA_COMMNET_CACHE_DIR          "/var/sigma-commnet/cache"
#define SIGMA_COMMNET_VERSION            "1.0.0"

// TRAI Community Wi-Fi Compliance:
// - Maximum 20 users per hotspot (TRAI guidelines)
// - Only cost-sharing — no profit margin allowed
// - Must maintain access logs (6 months per DoT rules)
// - ISP T&C compliance — no resale

// ---------------------------------------------------------------------------
// Gateway Configuration
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_COMMNET_UPSTREAM_WIRED   = 1,  // Ethernet/fiber from ISP
    SIGMA_COMMNET_UPSTREAM_LTE     = 2,  // 4G/5G dongle
    SIGMA_COMMNET_UPSTREAM_STARLINK = 3, // Starlink satellite
    SIGMA_COMMNET_UPSTREAM_BSNL    = 4,  // BSNL broadband
} sigma_commnet_upstream_type_t;

typedef enum {
    SIGMA_COMMNET_MESH_WIFI_24     = 1,  // 2.4 GHz (longer range)
    SIGMA_COMMNET_MESH_WIFI_50     = 2,  // 5.0 GHz (higher speed)
    SIGMA_COMMNET_MESH_WIFI_60     = 3,  // 60 GHz (short range, high speed)
    SIGMA_COMMNET_MESH_ETHERNET    = 4,  // Wired mesh (PoE switches)
    SIGMA_COMMNET_MESH_POWERLINE   = 5,  // Powerline networking
} sigma_commnet_mesh_type_t;

typedef struct {
    char     gateway_id[32];           // Unique gateway DID
    char     community_name[128];      // Village/colony name
    char     district[64];
    char     state[32];
    double   latitude;
    double   longitude;
    // Upstream connection
    sigma_commnet_upstream_type_t upstream_type;
    char     upstream_interface[16];   // e.g. "eth0"
    double   upstream_bandwidth_mbps;  // Contracted ISP bandwidth
    double   upstream_cost_monthly;    // Monthly ISP cost (₹)
    char     isp_name[64];
    char     isp_account[64];
    // Mesh network
    sigma_commnet_mesh_type_t mesh_type;
    char     mesh_interface[16];       // e.g. "wlan0"
    char     mesh_ssid[32];
    bool     mesh_encrypted;           // WPA3 mandatory
    // Billing
    double   cost_per_member_monthly;  // Calculated: ISP cost / enrolled members
    bool     billing_enabled;
    char     upi_collection_id[64];    // Community UPI VPA for fee collection
    // TRAI compliance
    uint8_t  max_members;              // ≤ 20 per TRAI guidelines
    bool     trai_compliant;
    bool     access_log_enabled;       // Mandatory DoT requirement
    time_t   created_at;
    bool     active;
} sigma_commnet_gateway_t;

// ---------------------------------------------------------------------------
// Member (Enrolled Household / Institution)
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_COMMNET_MEMBER_HOUSEHOLD  = 1,
    SIGMA_COMMNET_MEMBER_SCHOOL     = 2,
    SIGMA_COMMNET_MEMBER_CLINIC     = 3,
    SIGMA_COMMNET_MEMBER_SHOP       = 4,
    SIGMA_COMMNET_MEMBER_PANCHAYAT  = 5,
} sigma_commnet_member_type_t;

typedef struct {
    char     member_id[32];
    char     name[128];
    char     aadhaar_hash[64];         // Head of household (DID enrollment)
    char     did[128];                 // DID for access control
    char     mobile[12];
    sigma_commnet_member_type_t type;
    char     mac_addresses[8][18];     // Up to 8 devices per member
    int      mac_count;
    double   fair_share_mbps;          // Allocated bandwidth (auto-calculated)
    double   hard_cap_mbps;            // Max burst (2x fair share)
    bool     priority_member;          // Schools/clinics get QoS priority
    time_t   enrolled_at;
    bool     active;
    bool     fee_paid_this_month;
    time_t   last_payment;
    double   outstanding_balance;
    // Current session stats
    double   bytes_down_today_mb;
    double   bytes_up_today_mb;
    double   bytes_down_month_mb;
    double   bytes_up_month_mb;
} sigma_commnet_member_t;

// ---------------------------------------------------------------------------
// QoS (Fair-Share Bandwidth Management)
// ---------------------------------------------------------------------------

typedef struct {
    double   total_bandwidth_mbps;     // Upstream capacity
    uint8_t  active_members;           // Currently connected
    double   fair_share_per_member;    // total / active_members
    double   priority_reserve_mbps;    // For schools/clinics
    double   cache_hit_ratio;          // % of traffic served from local cache
    double   effective_per_member;     // After cache hit savings
    // tc qdisc HTB parameters
    char     qdisc_handle[16];         // e.g. "1:0"
    char     htb_root_rate[16];        // e.g. "100mbit"
    char     htb_default_class[8];     // Default class for unmatched traffic
} sigma_commnet_qos_config_t;

// Fair-share algorithm:
// IF all members active simultaneously: each gets (total / count) Mbps
// IF some idle: active members get proportionally more (HTB borrow)
// ALWAYS: priority members (schools, clinics) guaranteed minimum
int sigma_commnet_qos_calculate(sigma_commnet_gateway_t *gw,
                                 sigma_commnet_member_t *members, int count,
                                 sigma_commnet_qos_config_t *out);
int sigma_commnet_qos_apply(const sigma_commnet_qos_config_t *qos,
                              const char *interface);

// ---------------------------------------------------------------------------
// Local Content Cache
// ---------------------------------------------------------------------------

typedef struct {
    char     domain[256];              // e.g. "ncert.nic.in"
    char     description[128];         // "NCERT textbooks — education"
    uint64_t cache_size_mb;
    time_t   last_synced;
    time_t   sync_interval_h;         // Sync frequency in hours
    bool     enabled;
    uint32_t cache_hits_today;
    double   bandwidth_saved_mb_today;
    // Categories: Government sites, NCERT, eNAM, health, education
    char     category[32];
} sigma_commnet_cache_entry_t;

// Pre-seeded domains for Indian villages:
// ncert.nic.in         - School textbooks (offline learning)
// enam.gov.in          - Mandi prices (farmers)
// pmkisan.gov.in       - PM-KISAN status
// nhp.gov.in           - National Health Portal
// igrs.ap.gov.in       - Land records (state-specific)
// digilocker.gov.in    - Document access
// uidai.gov.in         - Aadhaar services
// cowin.gov.in         - Vaccination records
// epfindia.gov.in      - EPF status
// india.gov.in         - Government services portal

int sigma_commnet_cache_add(const char *domain, const char *description,
                             const char *category);
int sigma_commnet_cache_sync(const char *domain);
int sigma_commnet_cache_sync_all(void);
int sigma_commnet_cache_stats(sigma_commnet_cache_entry_t *entries, int *count);

// ---------------------------------------------------------------------------
// Offline Mode
// ---------------------------------------------------------------------------

typedef struct {
    bool     upstream_alive;
    bool     offline_mode_active;
    time_t   offline_since;
    double   offline_duration_s;
    // Local services available offline:
    bool     cache_serving;           // Cached govt websites available
    bool     local_health_available;  // sigma-health local records
    bool     local_edu_available;     // NCERT cached content
    bool     local_gov_available;     // Panchayat records (sigma-gram)
    bool     local_market_available;  // Last-known eNAM prices
    char     offline_notice_text[256]; // Shown to community dashboard
} sigma_commnet_offline_status_t;

int sigma_commnet_upstream_check(bool *alive, double *latency_ms);
int sigma_commnet_offline_status(sigma_commnet_offline_status_t *out);

// ---------------------------------------------------------------------------
// Bandwidth Reporting (Community Dashboard)
// ---------------------------------------------------------------------------

typedef struct {
    char     period[16];              // "today", "week", "month"
    double   total_down_gb;
    double   total_up_gb;
    double   peak_bandwidth_mbps;
    time_t   peak_time;
    double   cache_saved_gb;          // Bandwidth saved by local cache
    double   cost_this_period;        // ₹ cost for the period
    double   cost_per_member;         // ₹ cost per enrolled member
    // Per-member breakdown (sorted by usage)
    struct {
        char   member_id[32];
        char   name[64];
        double down_gb;
        double up_gb;
        double share_pct;
    } members[SIGMA_COMMNET_MAX_MEMBERS];
    int      member_count;
} sigma_commnet_bandwidth_report_t;

int sigma_commnet_bandwidth_report(const char *period,
                                    sigma_commnet_bandwidth_report_t *out);
int sigma_commnet_bandwidth_report_export(const char *period,
                                           const char *format, // "html", "csv", "pdf"
                                           const char *output_path);

// ---------------------------------------------------------------------------
// Access Control (DID-based)
// ---------------------------------------------------------------------------

// Only enrolled community members can access
// Enrollment: head of household submits DID → gateway admin approves
// Access log: every connection logged (MAC + DID + timestamp) per DoT rules

int sigma_commnet_member_enroll(sigma_commnet_member_t *member);
int sigma_commnet_member_mac_authorize(const char *mac_address,
                                        bool *authorized,
                                        char *member_id_out);
int sigma_commnet_access_log_query(time_t from, time_t to,
                                    const char *output_csv);
int sigma_commnet_member_suspend(const char *member_id,
                                  const char *reason);

// ---------------------------------------------------------------------------
// Billing (Cost-Share — No Profit)
// ---------------------------------------------------------------------------

typedef struct {
    char     member_id[32];
    char     month[8];               // "YYYY-MM"
    double   isp_cost_total;
    double   member_share;           // isp_cost / enrolled_members
    double   usage_share;            // Optional: weighted by usage
    bool     paid;
    time_t   paid_at;
    char     upi_ref[64];
    char     payment_mode[32];
} sigma_commnet_bill_t;

int sigma_commnet_bill_generate(const char *month,
                                 sigma_commnet_bill_t *bills, int *count);
int sigma_commnet_bill_mark_paid(const char *member_id, const char *month,
                                  const char *upi_ref);
int sigma_commnet_bill_export(const char *month, const char *output_csv);

// ---------------------------------------------------------------------------
// Setup CLI Helpers
// ---------------------------------------------------------------------------

// sigma-commnet setup --gateway eth0 --mesh wlan0 --members 20
// sigma-commnet member add --name "Ramesh Kumar" --aadhaar <hash> --mac AA:BB:CC:DD:EE:FF
// sigma-commnet bandwidth report --week last
// sigma-commnet cache add --url "ncert.nic.in" --category education
// sigma-commnet cache sync --all
// sigma-commnet status
// sigma-commnet qos show
// sigma-commnet bill generate --month 2026-07

int sigma_commnet_setup(const char *gateway_iface, const char *mesh_iface,
                         int max_members, sigma_commnet_gateway_t *out);
int sigma_commnet_status_print(void);
