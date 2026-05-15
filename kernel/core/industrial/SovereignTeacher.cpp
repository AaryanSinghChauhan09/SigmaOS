#include "../../../include/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Indian Teacher / Educator Shard (S-TEACH)
 * Purpose: Professional tools for Indian school teachers, college lecturers, and academic administrators.
 * Standards: NEP 2020, CBSE/ICSE/State Board norms, UGC Act 1956, NAAC Assessment Framework,
 *            RTE Act 2009, NCTE Act 1993.
 * Features: Grade calculator (CBSE 10-point scale), Attendance percentage, UGC pay matrix,
 *           NAAC criterion score, RTE pupil-teacher ratio checker.
 */

namespace SigmaOS {
namespace Kernel {
namespace Academic {

// CBSE grading scale (marks → grade → grade point)
struct CBSEGrade {
    sigma_u32 min_marks;
    const char* grade;
    sigma_u32 grade_point; // *10 for precision (A1 = 10.0 → 100)
};

static const CBSEGrade CBSE_GRADE_TABLE[] = {
    {91, "A1", 100},
    {81, "A2",  90},
    {71,  "B1",  80},
    {61,  "B2",  70},
    {51,  "C1",  60},
    {41,  "C2",  50},
    {33,   "D",  40},
    { 0,   "E",   0},  // Fail
};
static const sigma_u32 CBSE_LEN = sizeof(CBSE_GRADE_TABLE) / sizeof(CBSE_GRADE_TABLE[0]);

// UGC 7th Pay Commission Academic Pay Level (monthly basic in paise)
struct UGCPayLevel {
    sigma_u32 level;
    const char* designation;
    sigma_u64 basic_paise;
};

static const UGCPayLevel UGC_PAY[] = {
    {10, "Assistant Professor",  5700000ULL * 100},   // ₹57,700
    {11, "Assistant Professor (Sr Scale)", 6830000ULL * 100},
    {12, "Assistant Professor (Selection Grade)", 7940000ULL * 100},
    {13, "Associate Professor", 13131400ULL * 100},
    {14, "Professor",           14400000ULL * 100},
    {15, "Professor (HAG)", 18200000ULL * 100},
};
static const sigma_u32 UGC_LEN = sizeof(UGC_PAY) / sizeof(UGC_PAY[0]);

class SovereignTeacher : public SigmaOS::SigmaObject {
public:
    static SovereignTeacher& getInstance() {
        static SovereignTeacher instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignTeacher"; }

    void init() {
        sigma_log_info("[S-TEACH] Initializing Indian Education Compliance Nexus...");
        sigma_log_info("[S-TEACH] Standards: NEP 2020 | CBSE | UGC Act | RTE Act 2009 | NAAC");
    }

    /**
     * CBSE 10-point grade calculator for a student's marks out of 100.
     * Returns grade point * 10 (100 = A1 = 10.0).
     */
    sigma_u32 cbseGrade(sigma_u32 marks) {
        for (sigma_u32 i = 0; i < CBSE_LEN; ++i) {
            if (marks >= CBSE_GRADE_TABLE[i].min_marks) {
                sigma_log_info("[S-TEACH] CBSE | Marks: %u/100 | Grade: %s | GP: %u.%u",
                               marks, CBSE_GRADE_TABLE[i].grade,
                               CBSE_GRADE_TABLE[i].grade_point / 10,
                               CBSE_GRADE_TABLE[i].grade_point % 10);
                return CBSE_GRADE_TABLE[i].grade_point;
            }
        }
        return 0;
    }

    /**
     * CGPA calculator (average of best 5 subjects' grade points).
     * @param gp  Array of grade_points * 10 (e.g. {100, 90, 80, 70, 60})
     * @param n   Number of subjects (max 6, best 5 counted for CBSE)
     */
    void calcCGPA(sigma_u32* gp, sigma_u32 n) {
        if (n == 0) { sigma_log_err("[S-TEACH] No subjects provided."); return; }
        // Sort descending, pick best 5
        for (sigma_u32 i = 0; i < n - 1; ++i)
            for (sigma_u32 j = i + 1; j < n; ++j)
                if (gp[j] > gp[i]) { sigma_u32 t = gp[i]; gp[i] = gp[j]; gp[j] = t; }
        sigma_u32 count = (n > 5) ? 5 : n;
        sigma_u64 sum = 0;
        for (sigma_u32 i = 0; i < count; ++i) sum += gp[i];
        sigma_u64 cgpa_x10 = sum / count; // already *10
        sigma_log_info("[S-TEACH] CBSE CGPA: %llu.%llu (Best %u subjects)", cgpa_x10/10, cgpa_x10%10, count);
    }

    /**
     * Attendance calculator per UGC / University norms.
     * Minimum 75% attendance required for exam eligibility (most universities).
     * @param present  Classes attended
     * @param total    Total classes conducted
     */
    void calcAttendance(sigma_u32 present, sigma_u32 total) {
        if (total == 0) { sigma_log_err("[S-TEACH] Total classes cannot be zero."); return; }
        sigma_u32 pct_x10 = (present * 1000) / total;
        bool eligible = pct_x10 >= 750;
        sigma_log_info("[S-TEACH] Attendance | %u/%u = %u.%u%% | Exam Eligibility: %s",
                       present, total, pct_x10/10, pct_x10%10,
                       eligible ? "✅ ELIGIBLE (≥75%)" : "🚫 DETAINED (< 75%)");
    }

    /**
     * UGC 7th Pay Commission — look up pay level for a designation.
     */
    void ugcPayLookup(sigma_u32 level) {
        for (sigma_u32 i = 0; i < UGC_LEN; ++i) {
            if (UGC_PAY[i].level == level) {
                sigma_log_info("[S-TEACH] UGC Pay Level %u | %s | Basic: ₹%llu",
                               level, UGC_PAY[i].designation, UGC_PAY[i].basic_paise / 100);
                return;
            }
        }
        sigma_log_err("[S-TEACH] UGC pay level %u not found.", level);
    }

    /**
     * RTE Act 2009 — Pupil-Teacher Ratio (PTR) checker.
     * Sec 25: PTR ≤ 30:1 (Class I–V), ≤ 35:1 (Class VI–VIII).
     */
    void rtePtr(sigma_u32 students, sigma_u32 teachers, bool primary) {
        sigma_u32 limit = primary ? 30 : 35;
        sigma_u32 ratio = (teachers > 0) ? students / teachers : 9999;
        bool compliant = ratio <= limit;
        sigma_log_info("[S-TEACH] RTE PTR | %u students / %u teachers = %u:1 | Limit %u:1 | %s",
                       students, teachers, ratio, limit,
                       compliant ? "✅ COMPLIANT" : "🚫 NON-COMPLIANT — appoint additional teachers");
    }
};

} // namespace Academic
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void teach_init() {
    SigmaOS::Kernel::Academic::SovereignTeacher::getInstance().init();
}

sigma_u32 teach_cbse(sigma_u32 marks) {
    return SigmaOS::Kernel::Academic::SovereignTeacher::getInstance().cbseGrade(marks);
}

void teach_attendance(sigma_u32 p, sigma_u32 t) {
    SigmaOS::Kernel::Academic::SovereignTeacher::getInstance().calcAttendance(p, t);
}

void teach_ugc_pay(sigma_u32 level) {
    SigmaOS::Kernel::Academic::SovereignTeacher::getInstance().ugcPayLookup(level);
}

void teach_rte_ptr(sigma_u32 s, sigma_u32 t, bool primary) {
    SigmaOS::Kernel::Academic::SovereignTeacher::getInstance().rtePtr(s, t, primary);
}

} // extern "C"
