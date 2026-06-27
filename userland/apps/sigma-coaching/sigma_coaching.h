// SPDX-License-Identifier: GPL-2.0-only
// sigma_coaching.h — SigmaOS Coaching Institute Management App
// Regulator: UGC (Mandatory Disclosure) / Ministry of Education / State Coaching Acts
//            Rajasthan Coaching Institutions (Control & Regulation) Act 2023
//            Consumer Protection Act 2019 / GST (18% on educational services)
// Purpose  : Admission management, mandatory UGC disclosure, GST invoicing,
//            student wellbeing monitoring (PHQ-9/GAD-7), refund policy compliance,
//            NEET/JEE/UPSC batch management, faculty payroll (EPFO/TDS).

#pragma once
#include <sigma_indiastack.h>
#include <sigma_bus.h>

// ---------------------------------------------------------------------------
// Regulatory Constants
// ---------------------------------------------------------------------------
#define SIGMA_COACH_GST_RATE_PCT          18    // GST on coaching services
#define SIGMA_COACH_TDS_RATE_PCT           10   // TDS 194J on professional fees
#define SIGMA_COACH_REFUND_WINDOW_DAYS     15   // Consumer Protection Act refund window
#define SIGMA_COACH_SAC_CODE               "999293"  // SAT coaching services SAC
#define SIGMA_COACH_DISCLOSURE_MANDATORY   1    // UGC mandatory disclosure
#define SIGMA_COACH_PHQ9_THRESHOLD_MILD    5    // PHQ-9 mild depression threshold
#define SIGMA_COACH_PHQ9_THRESHOLD_MOD     10   // PHQ-9 moderate depression threshold
#define SIGMA_COACH_PHQ9_THRESHOLD_SEVERE  15   // PHQ-9 severe depression threshold

// ---------------------------------------------------------------------------
// Course & Exam Types
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_COACH_EXAM_JEE_MAIN       = 1,
    SIGMA_COACH_EXAM_JEE_ADVANCED   = 2,
    SIGMA_COACH_EXAM_NEET_UG        = 3,
    SIGMA_COACH_EXAM_NEET_PG        = 4,
    SIGMA_COACH_EXAM_UPSC_CSE       = 5,
    SIGMA_COACH_EXAM_UPSC_ESE       = 6,   // Engineering Services
    SIGMA_COACH_EXAM_CAT            = 7,
    SIGMA_COACH_EXAM_GATE           = 8,
    SIGMA_COACH_EXAM_CLAT           = 9,
    SIGMA_COACH_EXAM_CA_FOUNDATION  = 10,
    SIGMA_COACH_EXAM_CA_INTER       = 11,
    SIGMA_COACH_EXAM_CA_FINAL       = 12,
    SIGMA_COACH_EXAM_STATE_PSC      = 13,
    SIGMA_COACH_EXAM_SSC_CGL        = 14,
    SIGMA_COACH_EXAM_BANKING        = 15,  // IBPS/SBI PO/Clerk
    SIGMA_COACH_EXAM_CUET           = 16,  // Common University Entrance Test
    SIGMA_COACH_EXAM_OTHER          = 99,
} sigma_coach_exam_type_t;

typedef enum {
    SIGMA_COACH_BATCH_CLASSROOM     = 1,
    SIGMA_COACH_BATCH_ONLINE        = 2,
    SIGMA_COACH_BATCH_HYBRID        = 3,
    SIGMA_COACH_BATCH_DISTANCE      = 4,
    SIGMA_COACH_BATCH_CRASH_COURSE  = 5,
} sigma_coach_batch_mode_t;

// ---------------------------------------------------------------------------
// Student Registration & Admission
// ---------------------------------------------------------------------------

typedef struct {
    char     student_id[32];         // Internal student ID
    char     name[128];
    char     mobile[12];
    char     parent_mobile[12];
    char     email[128];
    char     aadhaar_hash[64];       // HMAC of Aadhaar
    char     address[256];
    char     city[64];
    char     state[32];
    char     qualification[64];      // Current class / degree
    char     school_college[128];
    sigma_coach_exam_type_t target_exam;
    char     batch_id[32];
    time_t   admission_date;
    double   fee_total;              // Total fee (inclusive of GST)
    double   fee_gst;                // GST component (18%)
    double   fee_paid;
    double   fee_outstanding;
    char     invoice_number[32];     // GST invoice number
    bool     scholarship;
    double   scholarship_amount;
    char     scholarship_scheme[64]; // PM Scholarship, state scheme, etc.
    time_t   course_start_date;
    time_t   course_end_date;
    bool     consent_form_signed;    // Mandatory under state coaching acts
    bool     ugc_disclosure_provided; // UGC mandatory disclosure given to student
} sigma_coach_student_t;

// ---------------------------------------------------------------------------
// UGC Mandatory Disclosure (as per UGC Regulations & state acts)
// ---------------------------------------------------------------------------

typedef struct {
    char     institute_name[128];
    char     reg_number[64];         // State registration number
    char     address[256];
    char     gstin[16];
    char     pan[12];
    char     owner_name[128];
    char     contact_email[128];
    char     contact_phone[12];
    char     website[128];
    // Fee disclosure (mandatory)
    double   fee_per_course_min;     // Minimum fee across all courses
    double   fee_per_course_max;     // Maximum fee
    char     fee_structure_url[256]; // Public URL of fee schedule
    // Refund policy disclosure
    int      refund_window_days;     // Days within which refund is available
    double   refund_pct_within_window; // % refund if withdrawn before course starts
    char     refund_policy_text[512];
    // Faculty disclosure
    int      total_faculty;
    int      faculty_with_pg;        // PG qualified faculty
    int      faculty_with_phd;
    // Result disclosure (last 3 years)
    int      students_appeared_yr1;
    int      students_qualified_yr1;
    int      students_appeared_yr2;
    int      students_qualified_yr2;
    int      students_appeared_yr3;
    int      students_qualified_yr3;
    time_t   disclosure_last_updated;
} sigma_coach_ugc_disclosure_t;

// ---------------------------------------------------------------------------
// Batch Management
// ---------------------------------------------------------------------------

typedef struct {
    char     batch_id[32];
    char     batch_name[128];
    sigma_coach_exam_type_t exam;
    sigma_coach_batch_mode_t mode;
    char     faculty_primary[128];
    char     faculty_id[32];
    int      capacity;
    int      enrolled;
    time_t   start_date;
    time_t   end_date;
    int      total_classes;
    int      classes_conducted;
    char     schedule_json[512];     // JSON schedule (days/times)
    char     syllabus_url[256];
    bool     recording_available;
    bool     test_series_included;
    double   fee_per_student;
} sigma_coach_batch_t;

// ---------------------------------------------------------------------------
// Student Wellbeing — Mental Health Monitoring
// (Required under Rajasthan Coaching Act 2023 and best practice)
// ---------------------------------------------------------------------------

typedef struct {
    char     student_id[32];
    time_t   assessment_date;
    // PHQ-9 (Patient Health Questionnaire — Depression)
    int      phq9_q1;   // Little interest or pleasure in doing things (0-3)
    int      phq9_q2;   // Feeling down, depressed, or hopeless
    int      phq9_q3;   // Trouble falling or staying asleep / sleeping too much
    int      phq9_q4;   // Feeling tired or having little energy
    int      phq9_q5;   // Poor appetite or overeating
    int      phq9_q6;   // Feeling bad about yourself
    int      phq9_q7;   // Trouble concentrating
    int      phq9_q8;   // Moving/speaking slowly or being fidgety/restless
    int      phq9_q9;   // Thoughts of being better off dead or self-harm
    int      phq9_total; // 0-27 total score
    // GAD-7 (Generalized Anxiety Disorder)
    int      gad7_q1;   // Feeling nervous, anxious, or on edge
    int      gad7_q2;   // Not able to stop or control worrying
    int      gad7_q3;   // Worrying too much about different things
    int      gad7_q4;   // Trouble relaxing
    int      gad7_q5;   // Being so restless it is hard to sit still
    int      gad7_q6;   // Becoming easily annoyed or irritable
    int      gad7_q7;   // Feeling afraid as if something awful might happen
    int      gad7_total; // 0-21 total score
    bool     counselor_referred;
    char     counselor_name[128];
    time_t   counselor_appointment;
    bool     crisis_protocol_activated; // PHQ-9 Q9 > 0 triggers immediate protocol
} sigma_coach_wellbeing_t;

// ---------------------------------------------------------------------------
// Fee & GST Invoice
// ---------------------------------------------------------------------------

typedef struct {
    char     invoice_no[32];          // GST invoice number (format per GST rules)
    char     invoice_date[12];        // YYYY-MM-DD
    char     student_id[32];
    char     student_name[128];
    char     student_gstin[16];       // If business education (optional)
    char     institute_gstin[16];
    char     place_of_supply[4];      // State code (e.g., "08" for Rajasthan)
    double   taxable_value;
    double   cgst_9pct;               // 9% CGST (for intra-state)
    double   sgst_9pct;               // 9% SGST (for intra-state)
    double   igst_18pct;              // 18% IGST (for inter-state)
    double   total_amount;
    char     payment_mode[32];        // UPI, NEFT, cash, etc.
    char     upi_ref[32];
    bool     e_invoice_generated;
    char     irn[64];                 // Invoice Reference Number (if applicable)
    char     qr_code_data[256];
} sigma_coach_gst_invoice_t;

// ---------------------------------------------------------------------------
// Refund Management (Consumer Protection Act 2019)
// ---------------------------------------------------------------------------

typedef struct {
    char     refund_id[32];
    char     student_id[32];
    char     invoice_no[32];
    time_t   request_date;
    time_t   course_start_date;
    double   fee_paid;
    double   refund_amount;
    double   deduction_amount;
    char     deduction_reason[128];
    bool     within_refund_window;
    char     status[32];              // PENDING, PROCESSED, REJECTED
    time_t   processed_date;
    char     bank_ref[64];
    // Compliance: refund must be processed within 30 days (Consumer Protection)
    bool     within_30_days;
} sigma_coach_refund_t;

// ---------------------------------------------------------------------------
// API Functions
// ---------------------------------------------------------------------------

// Student Management
int sigma_coach_student_admit(sigma_coach_student_t *student);
int sigma_coach_student_get(const char *student_id, sigma_coach_student_t *out);
int sigma_coach_attendance_mark(const char *student_id,
                                 const char *batch_id,
                                 time_t date, bool present);

// UGC Disclosure
int sigma_coach_ugc_disclosure_generate(sigma_coach_ugc_disclosure_t *disc,
                                         const char *output_html_path);
bool sigma_coach_ugc_disclosure_compliant(const sigma_coach_ugc_disclosure_t *disc);

// Batch Management
int sigma_coach_batch_create(sigma_coach_batch_t *batch);
int sigma_coach_batch_get(const char *batch_id, sigma_coach_batch_t *out);
int sigma_coach_class_mark_conducted(const char *batch_id, time_t date,
                                      const char *topic);

// Student Wellbeing
int sigma_coach_wellbeing_assess(sigma_coach_wellbeing_t *assessment);
bool sigma_coach_wellbeing_crisis_flag(const sigma_coach_wellbeing_t *a);
int sigma_coach_wellbeing_refer_counselor(const char *student_id,
                                           const char *counselor_name,
                                           time_t appointment);

// Invoicing
int sigma_coach_invoice_generate(sigma_coach_gst_invoice_t *inv);
int sigma_coach_invoice_e_invoice(sigma_coach_gst_invoice_t *inv);

// Refunds
int sigma_coach_refund_process(sigma_coach_refund_t *refund);
bool sigma_coach_refund_policy_check(const sigma_coach_refund_t *r,
                                      const sigma_coach_ugc_disclosure_t *policy);

// ---------------------------------------------------------------------------
// CLI Entry Points
// ---------------------------------------------------------------------------
// sigma-coaching student admit --name <> --exam JEE_MAIN --batch <id>
// sigma-coaching disclosure generate --output disclosure.html
// sigma-coaching wellbeing assess --student <id>
// sigma-coaching invoice generate --student <id> --amount <fee>
// sigma-coaching refund process --student <id> --invoice <no>
// sigma-coaching batch report --batch <id> --month 2026-06
