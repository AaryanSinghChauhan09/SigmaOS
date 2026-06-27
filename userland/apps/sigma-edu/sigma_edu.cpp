// SPDX-License-Identifier: GPL-2.0-or-later
// sigma-edu — Education tools for teachers, educators, and institutions
// Covers: NEP 2020, CBSE/ICSE/State boards, UGC, NAAC, DIKSHA
//
// CLI:
//   sigma-edu nep outcomes --class 5 --subject math
//   sigma-edu question-paper --board CBSE --class 10 --subject science
//   sigma-edu ugc naac --criteria 1 --criterion-weight 100
//   sigma-edu curriculum --board ICSE --class 12 --subject physics

#include <stdint.h>
#include <string.h>
#include <stdio.h>

// ── NEP 2020 Competency Framework (FLN + NIPUN Bharat) ────────────────────────

struct NEPOutcome {
    uint8_t     class_num;
    const char *subject;
    const char *domain;
    const char *outcome;
    const char *nipun_target;  // Grade-3 foundational target
};

static const NEPOutcome nep_outcomes[] = {
    // Foundational Stage (Classes 1-2)
    { 1, "math",    "Numbers",   "Count, read, write 1-99; addition/subtraction within 9",
      "Count 1-120, solve addition/subtraction problems within 99" },
    { 2, "math",    "Numbers",   "Count, read, write up to 999; multiplication tables 2-5",
      "Understand place value to 999" },
    // Preparatory Stage (Classes 3-5)
    { 3, "math",    "Numbers",   "4-digit numbers; multiplication of 2-digit numbers; fractions",
      "Solve 2-digit multiplication; understand fractions 1/2, 1/4, 1/3" },
    { 4, "math",    "Geometry",  "Identify 2D/3D shapes; perimeter of squares and rectangles",
      "Understand angles; area of rectangles" },
    { 5, "math",    "Numbers",   "Operations on fractions/decimals; LCM/HCF",
      "Multiply/divide fractions; percentage basics" },
    { 5, "science", "Environment","Food chains; plant parts and functions; water cycle",
      "Understand adaptation; basic ecosystem" },
    // Middle Stage (Classes 6-8)
    { 6, "math",    "Algebra",   "Introduction to algebra; linear equations in one variable",
      "Understand variables; simple equations" },
    { 7, "science", "Physics",   "Heat and temperature; motion and time; electric current",
      "Apply Ohm's law; understand speed/velocity" },
    { 8, "math",    "Mensuration","Area of quadrilaterals; volume of cuboids and cylinders",
      "Apply Heron's formula" },
    { 9, "math",    "Coordinate","Cartesian plane; linear equations in two variables",
      "Distance formula; section formula" },
    { 10,"science", "Chemistry", "Chemical reactions; acids/bases/salts; metals/non-metals",
      "Balance equations; understand pH scale" },
    { 0, NULL, NULL, NULL, NULL }
};

void sigma_edu_nep_outcomes(uint8_t class_num, const char *subject) {
    printf("NEP 2020 Learning Outcomes — Class %d, %s\n", class_num, subject);
    printf("Framework: NCF 2023 + NIPUN Bharat (FLN)\n\n");
    int found = 0;
    for (int i = 0; nep_outcomes[i].subject; i++) {
        if (nep_outcomes[i].class_num == class_num &&
            strcmp(nep_outcomes[i].subject, subject) == 0) {
            const NEPOutcome *o = &nep_outcomes[i];
            printf("Domain: %s\n  Outcome: %s\n", o->domain, o->outcome);
            if (o->nipun_target)
                printf("  NIPUN Target: %s\n", o->nipun_target);
            printf("\n");
            found = 1;
        }
    }
    if (!found) printf("No outcomes found for class %d subject '%s'\n",
                       class_num, subject);
}

// ── Question Paper Generator (Bloom's Taxonomy) ───────────────────────────────

struct BloomLevel {
    uint8_t     level;
    const char *name;
    const char *verbs;
    uint8_t     marks_pct;  // suggested % of marks at this level
};

static const BloomLevel bloom_levels[] = {
    { 1, "Remembering",   "define, list, recall, state",          20 },
    { 2, "Understanding", "explain, describe, summarise",          25 },
    { 3, "Applying",      "solve, demonstrate, use, calculate",    25 },
    { 4, "Analysing",     "compare, differentiate, examine",       15 },
    { 5, "Evaluating",    "judge, assess, critique, justify",      10 },
    { 6, "Creating",      "design, construct, formulate, propose",  5 },
    { 0, NULL, NULL, 0 }
};

void sigma_edu_question_paper(const char *board, uint8_t class_num,
                               const char *subject, uint32_t total_marks) {
    printf("Question Paper Design — %s | Class %d | %s | %u marks\n",
           board, class_num, subject, total_marks);
    printf("Based on CBSE Blueprint 2024-25\n\n");
    printf("Section-wise distribution:\n");
    printf("  Section A (MCQ/1-mark):    %u marks\n", total_marks * 20 / 100);
    printf("  Section B (2-mark):        %u marks\n", total_marks * 20 / 100);
    printf("  Section C (3-mark):        %u marks\n", total_marks * 30 / 100);
    printf("  Section D (5-mark/case):   %u marks\n", total_marks * 30 / 100);
    printf("\nBloom's Taxonomy distribution:\n");
    for (int i = 0; bloom_levels[i].name; i++) {
        printf("  L%d %-15s (%2d%%): %u marks | Verbs: %s\n",
               bloom_levels[i].level, bloom_levels[i].name,
               bloom_levels[i].marks_pct,
               total_marks * bloom_levels[i].marks_pct / 100,
               bloom_levels[i].verbs);
    }
    printf("\nCompliance: %s syllabus | 3-hour duration | Inclusive design\n", board);
}

// ── UGC / NAAC AQAR ───────────────────────────────────────────────────────────

struct NAACCriteria {
    uint8_t     num;
    const char *name;
    uint32_t    weight;  // out of 1000
    const char *key_indicator;
};

static const NAACCriteria naac_criteria[] = {
    { 1, "Curricular Aspects",                    150, "1.1 Curriculum Design and Development" },
    { 2, "Teaching-Learning and Evaluation",       200, "2.1 Student Enrollment and Profile" },
    { 3, "Research, Innovations and Extension",   250, "3.1 Resource Mobilisation for Research" },
    { 4, "Infrastructure and Learning Resources",  100, "4.1 Physical Facilities" },
    { 5, "Student Support and Progression",        100, "5.1 Student Support" },
    { 6, "Governance, Leadership and Management",  100, "6.1 Institutional Vision and Leadership" },
    { 7, "Institutional Values and Best Practices", 100,"7.1 Institutional Values and Social Responsibilities" },
    { 0, NULL, 0, NULL }
};

void sigma_edu_naac(uint8_t criteria_num, uint32_t weight) {
    printf("NAAC Self-Study Report (SSR) — Criteria %d\n", criteria_num);
    for (int i = 0; naac_criteria[i].name; i++) {
        if (naac_criteria[i].num == criteria_num) {
            printf("  Name:   %s\n", naac_criteria[i].name);
            printf("  Weight: %u/1000 (%.1f%%)\n",
                   naac_criteria[i].weight,
                   naac_criteria[i].weight / 10.0f);
            printf("  Key Indicator: %s\n", naac_criteria[i].key_indicator);
            printf("  Evidence: Upload via sigma-digilocker or local PDF\n");
            printf("  Portal: naac.gov.in/iiqa\n");
            return;
        }
    }
    printf("Total NAAC weight: 1000 | Grade thresholds:\n");
    printf("  A++ ≥ 3.76 | A+ 3.51-3.75 | A 3.26-3.50\n");
    printf("  B++ 3.01-3.25 | B+ 2.76-3.00 | B 2.51-2.75 | C ≥ 2.01\n");
    (void)weight;
}

// ── DIKSHA Platform Integration ───────────────────────────────────────────────

void sigma_edu_diksha_content(const char *subject, uint8_t class_num) {
    printf("DIKSHA Content — Class %d, %s\n", class_num, subject);
    printf("Access: diksha.gov.in | App: DIKSHA (iOS/Android)\n");
    printf("QR code scanning: Use ENERGISED textbook QR codes\n");
    printf("Teacher training: NISHTHA 3.0 online modules\n");
    printf("  Module 1: NEP 2020 and Learning Outcomes\n");
    printf("  Module 2: FLN — Foundational Literacy and Numeracy\n");
    printf("  Module 3: Inclusive Education\n");
    printf("Offline sync: sigma-net sync --app diksha --district offline\n");
}
