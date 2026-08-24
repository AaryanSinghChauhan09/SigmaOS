// India Stack - Professional Utility Engines for Indian Krishi, Vyapaar, and Kanoon
// Core calculators for Indian farmers, CAs/retailers, medical practitioners, structural engineers, and legal advocates

// #![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

/// Lightweight Newton-Raphson square root estimation for bare-metal `#![no_std]` environments
fn sovereign_sqrt(x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    let mut guess = x / 2.0;
    for _ in 0..10 {
        guess = 0.5 * (guess + x / guess);
    }
    guess
}

// =========================================================================
// 1. KRISHI CALCULATOR (FOR INDIAN FARMERS & AGRICULTURE)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndianCrop {
    Paddy,
    Wheat,
    Sugarcane,
    Cotton,
    Ragi,
    Maize,
}

pub struct KrishiCalculator;

impl KrishiCalculator {
    /// Calculate Minimum Support Price (MSP) using the Swaminathan Commission A2+FL or C2 formula (C2 + 50%)
    pub fn calculate_msp(_crop: IndianCrop, cost_of_production_paise: u64) -> u64 {
        // MSP is at least 1.5 times (50% return) the cost of production (C2 + 50%)
        let msp = (cost_of_production_paise as f64 * 1.5) as u64;
        msp
    }

    /// Calculate recommended N-P-K (Nitrogen, Phosphorus, Potassium) fertilizer dose in Kilograms
    /// inputs: field_area_bigha (1 Bigha ~ 0.25 Hectares), crop type
    pub fn calculate_fertilizer_req(crop: IndianCrop, field_area_bigha: f64) -> (f64, f64, f64) {
        // N-P-K ratio per bigha based on Indian Council of Agricultural Research (ICAR) guidelines
        let (n_ratio, p_ratio, k_ratio) = match crop {
            IndianCrop::Paddy => (20.0, 10.0, 10.0), // 20:10:10 kg/bigha
            IndianCrop::Wheat => (24.0, 12.0, 8.0),  // 24:12:8 kg/bigha
            IndianCrop::Sugarcane => (30.0, 15.0, 15.0), // 30:15:15 kg/bigha
            IndianCrop::Cotton => (16.0, 8.0, 8.0),  // 16:8:8 kg/bigha
            IndianCrop::Ragi => (12.0, 6.0, 6.0),    // 12:6:6 kg/bigha
            IndianCrop::Maize => (18.0, 9.0, 9.0),   // 18:9:9 kg/bigha
        };

        (
            n_ratio * field_area_bigha,
            p_ratio * field_area_bigha,
            k_ratio * field_area_bigha,
        )
    }
}

// =========================================================================
// 2. VYAPAAR CALCULATOR (FOR KIRANA OWNERS, CAs, & RETAILERS)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetClass {
    Computers,       // 40% depreciation under Income Tax Act
    Machinery,       // 15% depreciation
    OfficeBuildings, // 10% depreciation
    Furniture,       // 10% depreciation
}

pub struct VyapaarCalculator;

impl VyapaarCalculator {
    /// Calculate asset depreciation under the Written Down Value (WDV) method of the Indian Income Tax Act, 1961
    pub fn calculate_wdv_depreciation(
        asset: AssetClass,
        purchase_cost_paise: u64,
        years: u32,
    ) -> Vec<u64> {
        let rate = match asset {
            AssetClass::Computers => 0.40,
            AssetClass::Machinery => 0.15,
            AssetClass::OfficeBuildings => 0.10,
            AssetClass::Furniture => 0.10,
        };

        let mut values = Vec::new();
        let mut current_value = purchase_cost_paise as f64;

        for _ in 0..years {
            let dep = current_value * rate;
            current_value -= dep;
            values.push(current_value as u64);
        }

        values
    }

    /// Calculate interest on late tax payment under Section 234A/B/C of the Income Tax Act
    /// (Simple interest of 1% per month or part of a month on unpaid tax)
    pub fn calculate_section_234_interest(unpaid_tax_paise: u64, delay_months: u32) -> u64 {
        let monthly_rate = 0.01; // 1% per month
        let interest = unpaid_tax_paise as f64 * monthly_rate * delay_months as f64;
        interest as u64
    }

    /// Calculate MSME Equated Monthly Installments (EMI)
    /// formula: (Principal * r * (1+r)^n) / ((1+r)^n - 1)
    pub fn calculate_msme_emi(principal_paise: u64, annual_rate: f64, months: u32) -> u64 {
        let r = annual_rate / 12.0 / 100.0; // monthly rate fraction
        let _n = months as f64;

        // Power calculation (1+r)^n
        let mut pow = 1.0;
        for _ in 0..months {
            pow *= 1.0 + r;
        }

        let emi = (principal_paise as f64 * r * pow) / (pow - 1.0);
        emi as u64
    }
}

// =========================================================================
// 3. KANOON CALCULATOR (FOR INDIAN LEGAL ADVOCATES)
// =========================================================================

// =========================================================================
// 3. CHIKITSHAK CALCULATOR (FOR MEDICAL DOCTORS & CLINICAL UTILITIES)
// =========================================================================

pub struct ChikitshakCalculator;

impl ChikitshakCalculator {
    /// Calculate Body Surface Area (BSA) in square meters using Mosteller's formula: sqrt( (height_cm * weight_kg) / 3600 )
    pub fn calculate_mosteller_bsa(height_cm: f64, weight_kg: f64) -> f64 {
        let index = (height_cm * weight_kg) / 3600.0;
        sovereign_sqrt(index)
    }

    /// Calculate Creatinine Clearance (CrCl) in mL/min using the Cockcroft-Gault formula
    /// formula: ((140 - age) * weight_kg) / (72 * serum_creatinine)
    pub fn calculate_creatinine_clearance(age: u32, weight_kg: f64, serum_creatinine: f64, is_female: bool) -> f64 {
        if serum_creatinine <= 0.0 {
            return 0.0;
        }
        let baseline = ((140.0 - age as f64) * weight_kg) / (72.0 * serum_creatinine);
        if is_female {
            baseline * 0.85 // Female factor adjustment
        } else {
            baseline
        }
    }
}

// =========================================================================
// 4. ABHIYANTA CALCULATOR (FOR STRUCTURAL ENGINEERS)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConcreteGrade {
    M15, // 1:2:4 ratio
    M20, // 1:1.5:3 ratio
    M25, // 1:1:2 ratio
}

pub struct AbhiyantaCalculator;

impl AbhiyantaCalculator {
    /// Calculate concrete mixing aggregate weight ratios (Cement, Sand, Coarse Aggregate) in Kilograms
    pub fn get_concrete_mix_ratio(grade: ConcreteGrade, total_volume_m3: f64) -> (f64, f64, f64) {
        // Density of dry ingredients per cubic meter
        let cement_density = 1440.0; // kg/m3
        let sand_density = 1600.0;
        let aggregate_density = 1500.0;

        // Ratio parts: (cement, sand, aggregate)
        let (c_part, s_part, a_part) = match grade {
            ConcreteGrade::M15 => (1.0, 2.0, 4.0),
            ConcreteGrade::M20 => (1.0, 1.5, 3.0),
            ConcreteGrade::M25 => (1.0, 1.0, 2.0),
        };

        let total_parts = c_part + s_part + a_part;
        let cement_vol = (total_volume_m3 * c_part) / total_parts;
        let sand_vol = (total_volume_m3 * s_part) / total_parts;
        let aggregate_vol = (total_volume_m3 * a_part) / total_parts;

        (
            cement_vol * cement_density,
            sand_vol * sand_density,
            aggregate_vol * aggregate_density,
        )
    }

    /// Calculate beam deflection at center for simply supported beam under single-point load
    /// formula: (Load * L^3) / (48 * E * I)
    pub fn calculate_beam_deflection(load_newtons: f64, length_m: f64, elasticity_pa: f64, inertia_m4: f64) -> f64 {
        let l3 = length_m * length_m * length_m;
        let denominator = 48.0 * elasticity_pa * inertia_m4;
        if denominator == 0.0 {
            return 0.0;
        }
        (load_newtons * l3) / denominator
    }
}

// =========================================================================
// 5. KANOON CALCULATOR (FOR INDIAN LEGAL ADVOCATES)
// =========================================================================

pub struct KanoonCalculator;

impl KanoonCalculator {
    /// Calculate Court Fees under the Court Fees Act, 1870 based on claim/suit value
    pub fn calculate_civil_court_fee(claim_value_paise: u64) -> u64 {
        // Average ad-valorem state court fee structure:
        // - Up to ₹50,000 (5,000,000 paise): 2.5%
        // - ₹50,001 to ₹2,00,000: ₹1,250 + 5% on exceeding amount
        // - Above ₹2,00,000: ₹8,750 + 7.5% on exceeding amount
        let value = claim_value_paise as f64;
        let fee = if value <= 5_000_000.0 {
            value * 0.025
        } else if value <= 20_000_000.0 {
            125_000.0 + (value - 5_000_000.0) * 0.05
        } else {
            875_000.0 + (value - 20_000_000.0) * 0.075
        };
        fee as u64
    }

    /// Check if a suit can be filed under the Limitation Act, 1963
    /// inputs: elapsed_months, limitation_type
    pub fn is_within_limitation_period(elapsed_months: u32, suit_type: LimitationType) -> bool {
        let max_months = match suit_type {
            LimitationType::BreachOfContract => 36,             // 3 years
            LimitationType::RecoveryOfMoney => 36,              // 3 years
            LimitationType::RecoveryOfImmovableProperty => 144, // 12 years
            LimitationType::ExecutionOfDecree => 144,           // 12 years
            LimitationType::ForeclosureOfMortgage => 360,       // 30 years
        };

        elapsed_months <= max_months
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitationType {
    BreachOfContract,
    RecoveryOfMoney,
    RecoveryOfImmovableProperty,
    ExecutionOfDecree,
    ForeclosureOfMortgage,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_krishi_msp_calculation() {
        let cost = 100_000; // ₹1,000 cost of production
        let msp = KrishiCalculator::calculate_msp(IndianCrop::Paddy, cost);
        assert_eq!(msp, 150_000); // ₹1,500 MSP
    }

    #[test]
    fn test_fertilizer_req() {
        let area = 5.0; // 5 Bighas
        let (n, p, k) = KrishiCalculator::calculate_fertilizer_req(IndianCrop::Wheat, area);
        assert_eq!(n, 120.0);
        assert_eq!(p, 60.0);
        assert_eq!(k, 40.0);
    }

    #[test]
    fn test_vyapaar_depreciation() {
        let cost = 10_000_000; // ₹1 Lakh computer
        let balance = VyapaarCalculator::calculate_wdv_depreciation(AssetClass::Computers, cost, 2);
        assert_eq!(balance.len(), 2);
        assert_eq!(balance[0], 6_000_000); // 40% depreciation in Year 1 -> ₹60,000 remaining
        assert_eq!(balance[1], 3_600_000); // 40% depreciation in Year 2 -> ₹36,000 remaining
    }

    #[test]
    fn test_vyapaar_section_234_interest() {
        let unpaid = 100_000; // ₹1,000 unpaid tax
        let interest = VyapaarCalculator::calculate_section_234_interest(unpaid, 3);
        assert_eq!(interest, 3_000); // 1% per month for 3 months = ₹30
    }

    #[test]
    fn test_kanoon_court_fee() {
        let claim = 10_000_000; // ₹1 Lakh claim
        let fee = KanoonCalculator::calculate_civil_court_fee(claim);
        assert_eq!(fee, 125_000 + 250_000); // ₹1,250 + 5% on exceeding ₹50,000 = ₹3,750
    }

    #[test]
    fn test_kanoon_limitation() {
        assert!(KanoonCalculator::is_within_limitation_period(
            24,
            LimitationType::BreachOfContract
        ));
        assert!(!KanoonCalculator::is_within_limitation_period(
            48,
            LimitationType::BreachOfContract
        ));
    }

    #[test]
    fn test_doctor_bsa_and_creatinine_clearance() {
        // BSA of 180cm, 80kg male
        let bsa = ChikitshakCalculator::calculate_mosteller_bsa(180.0, 80.0);
        let expected_bsa = sovereign_sqrt((180.0 * 80.0) / 3600.0);
        assert_eq!(bsa, expected_bsa);

        // CrCl for 45yr, 70kg male with 1.2 mg/dL creatinine
        let crcl_male = ChikitshakCalculator::calculate_creatinine_clearance(45, 70.0, 1.2, false);
        assert_eq!(crcl_male, ((140.0 - 45.0) * 70.0) / (72.0 * 1.2));

        // CrCl for female
        let crcl_female = ChikitshakCalculator::calculate_creatinine_clearance(45, 70.0, 1.2, true);
        assert_eq!(crcl_female, crcl_male * 0.85);
    }

    #[test]
    fn test_engineer_mix_design_and_beam_deflection() {
        // M20 Mix ratios for 5 cubic meters
        let (cement, _sand, _agg) = AbhiyantaCalculator::get_concrete_mix_ratio(ConcreteGrade::M20, 5.0);
        // M20 parts: 1:1.5:3. total = 5.5. cement = 1/5.5, sand = 1.5/5.5, agg = 3/5.5.
        let expected_cement = (5.0 * 1.0) / 5.5 * 1440.0;
        assert_eq!(cement, expected_cement);

        // Deflection for load 5000N, length 4m, E = 200GPa, I = 0.0001 m4
        let deflection = AbhiyantaCalculator::calculate_beam_deflection(5000.0, 4.0, 200_000_000_000.0, 0.0001);
        assert_eq!(deflection, (5000.0 * 64.0) / (48.0 * 200_000_000_000.0 * 0.0001));
    }

    #[test]
    fn test_accountant_msme_emi() {
        // Principal 1 Lakh, annual rate 12%, 12 months. Monthly EMI should be approx 8884.
        let emi = VyapaarCalculator::calculate_msme_emi(100_000_00, 12.0, 12);
        assert_eq!(emi, 888487); // ₹8,884.87
    }
}
