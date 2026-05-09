#include "core/sigma_types.h"
/*
 * =============================================================================
 * Σ SIGMAOS SHELL: SOVEREIGN CALCULATOR SHARD (v2.0)
 * =============================================================================
 * Modules: NCERT Physics/Math/Chemistry & Indian Law Timelines/Fines.
 * Comprehensive Support for Indian Legal Bare Texts.
 * =============================================================================
 */
#include "core/sigma_kernel_types.h"

/* =============================================================================
 * NCERT CALCULATORS
 * ============================================================================= */

/* --- Physics (Class 11 & 12) --- */
sigma_u64 ncert_calc_force(sigma_u64 mass, sigma_u64 acceleration) {
    return mass * acceleration; /* F = ma */
}

sigma_u64 ncert_calc_kinetic_energy(sigma_u64 mass, sigma_u64 velocity) {
    return (mass * velocity * velocity) / 2; /* KE = 1/2 mv^2 */
}

sigma_u64 ncert_calc_potential_energy(sigma_u64 mass, sigma_u64 height) {
    sigma_u64 g = 10; // Approx 9.8
    return mass * g * height; /* PE = mgh */
}

sigma_u64 ncert_calc_momentum(sigma_u64 mass, sigma_u64 velocity) {
    return mass * velocity; /* p = mv */
}

sigma_u64 ncert_calc_work_done(sigma_u64 force, sigma_u64 displacement) {
    return force * displacement; /* W = F * s (assuming cos 0) */
}

sigma_u64 ncert_calc_rest_mass_energy(sigma_u64 mass) {
    /* E = mc^2 placeholder (Simplified for integer arithmetic) */
    sigma_u64 c = 299792458;
    return mass * c * c;
}

sigma_u64 ncert_calc_ohms_law_voltage(sigma_u64 current, sigma_u64 resistance) {
    return current * resistance; /* V = IR */
}

sigma_u64 ncert_calc_electrical_power(sigma_u64 voltage, sigma_u64 current) {
    return voltage * current; /* P = VI */
}

/* --- Mathematics (Class 10, 11, 12) --- */
sigma_u64 ncert_calc_arithmetic_progression_nth(sigma_u64 a, sigma_u64 n, sigma_u64 d) {
    return a + (n - 1) * d; /* an = a + (n-1)d */
}

sigma_u64 ncert_calc_arithmetic_progression_sum(sigma_u64 n, sigma_u64 a, sigma_u64 l) {
    return (n * (a + l)) / 2; /* Sn = n/2(a+l) */
}

sigma_u64 ncert_calc_circle_area(sigma_u64 radius) {
    // Approx pi as 22/7
    return (22 * radius * radius) / 7; 
}

/* --- Chemistry (Mole Concept) --- */
sigma_u64 ncert_calc_moles(sigma_u64 given_mass, sigma_u64 molar_mass) {
    if (molar_mass == 0) return 0;
    return given_mass / molar_mass;
}


/* =============================================================================
 * INDIAN LEGAL BARE TEXT CALCULATORS
 * ============================================================================= */

/* --- 1. BNSS (Bharatiya Nagarik Suraksha Sanhita, 2023) --- */
// Default bail timeline under Section 187 BNSS (previously 167 CrPC)
sigma_u32 bnss_calc_default_bail_days(bool is_death_or_life_imprisonment, bool is_min_10_years) {
    if (is_death_or_life_imprisonment || is_min_10_years) {
        return 90; // 90 days for serious offences
    }
    return 60; // 60 days for other offences
}

// Maximum period of police custody under BNSS Section 187
sigma_u32 bnss_calc_max_police_custody_days() {
    return 15; // Can be requested in parts over first 40/60 days
}

/* --- 2. BNS (Bharatiya Nyaya Sanhita, 2023) --- */
// Community service timeline calculator (Section 4)
sigma_u32 bns_calc_community_service_hours(sigma_u32 severity_level) {
    // New concept in BNS for petty offences
    return severity_level * 10; 
}

/* --- 3. CPC (Civil Procedure Code, 1908) --- */
// Written statement filing timeline (Order VIII Rule 1)
sigma_u32 cpc_calc_written_statement_deadline(sigma_u32 summons_served_day) {
    return summons_served_day + 30; // Normal deadline: 30 days
}

sigma_u32 cpc_calc_written_statement_max_deadline(sigma_u32 summons_served_day, bool is_commercial_suit) {
    if (is_commercial_suit) {
        return summons_served_day + 120; // 120 days for Commercial Suits
    }
    return summons_served_day + 90; // 90 days with court permission
}

/* --- 4. The Limitation Act, 1963 --- */
// Time limit for filing appeals
sigma_u32 limitation_calc_appeal_high_court(sigma_u32 decree_day) {
    return decree_day + 90; // 90 days to High Court
}

sigma_u32 limitation_calc_appeal_other_court(sigma_u32 decree_day) {
    return decree_day + 30; // 30 days to other courts
}

sigma_u32 limitation_calc_suit_recovery_money(sigma_u32 cause_of_action_day) {
    return cause_of_action_day + (3 * 365); // 3 years
}

/* --- 5. Negotiable Instruments Act, 1881 (Sec 138 - Cheque Bounce) --- */
// Notice period calculations
sigma_u32 ni_act_calc_notice_deadline(sigma_u32 bank_return_memo_day) {
    return bank_return_memo_day + 30; // Notice must be sent within 30 days
}

sigma_u32 ni_act_calc_payment_wait_period(sigma_u32 notice_receipt_day) {
    return notice_receipt_day + 15; // Drawer has 15 days to pay
}

sigma_u32 ni_act_calc_complaint_filing_deadline(sigma_u32 payment_wait_expiry_day) {
    return payment_wait_expiry_day + 30; // Complaint to be filed within 30 days of cause of action
}

/* --- 6. Insolvency and Bankruptcy Code (IBC), 2016 --- */
// CIRP (Corporate Insolvency Resolution Process) timelines
sigma_u32 ibc_calc_cirp_standard_deadline(sigma_u32 admission_day) {
    return admission_day + 180; // 180 days standard
}

sigma_u32 ibc_calc_cirp_max_deadline(sigma_u32 admission_day) {
    return admission_day + 330; // 330 days maximum including litigation
}

/* --- 7. Motor Vehicles Act, 1988 (Amended 2019) --- */
// Traffic fine calculator
sigma_u32 mv_act_calc_speeding_fine(bool is_lcv_or_passenger_vehicle) {
    if (is_lcv_or_passenger_vehicle) {
        return 2000; // Rs 2000 to 4000
    }
    return 1000; // Rs 1000 to 2000 for LMV
}

sigma_u32 mv_act_calc_drunk_driving_fine(bool is_repeat_offence) {
    if (is_repeat_offence) {
        return 15000; // Rs 15000 or up to 2 yrs imprisonment
    }
    return 10000; // Rs 10000 or up to 6 months imprisonment
}

/* --- 8. POCSO Act, 2012 --- */
// Investigation timeline
sigma_u32 pocso_calc_investigation_deadline(sigma_u32 fir_day) {
    return fir_day + 60; // 2 months from FIR
}

// Trial timeline
sigma_u32 pocso_calc_trial_deadline(sigma_u32 cognizance_day) {
    return cognizance_day + 365; // 1 year from taking cognizance
}

/* --- 9. RERA (Real Estate Regulation and Development Act, 2016) --- */
sigma_u32 rera_calc_builder_possession_grace_period() {
    return 365; // Usually up to 1 year grace period standard in agreements
}

sigma_u32 rera_calc_appeal_to_tribunal(sigma_u32 order_receipt_day) {
    return order_receipt_day + 60; // 60 days to appeal to REAT
}

/* --- 10. Consumer Protection Act, 2019 --- */
sigma_u32 consumer_calc_filing_limitation(sigma_u32 cause_of_action_day) {
    return cause_of_action_day + (2 * 365); // 2 years from cause of action
}

sigma_u32 consumer_calc_appeal_state_commission(sigma_u32 district_order_day) {
    return district_order_day + 45; // 45 days to State Commission
}

/* --- General Timelines Engine --- */
sigma_u32 law_calc_filing_deadline(sigma_u32 incident_day, sigma_u32 limit_days) {
    return incident_day + limit_days;
}

void sigma_calc_init() {
    kprintf("Σ [CALC]: Sovereign Calculator Shard active.\n");
    kprintf("Σ [NCERT]: Physics, Math, and Chemistry primitives loaded.\n");
    kprintf("Σ [LAW]: Indian Legal Calculators active for: BNSS, BNS, CPC, Limitation Act, IBC, NI Act, POCSO, RERA, Consumer Protection.\n");
}
