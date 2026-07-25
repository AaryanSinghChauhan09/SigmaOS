// India Stack - Professional Utility Engines for Indian Krishi, Vyapaar, and Kanoon
// Core calculators for Indian farmers, CAs/retailers, and legal advocates

#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

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
    pub fn calculate_msp(crop: IndianCrop, cost_of_production_paise: u64) -> u64 {
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
}

// =========================================================================
// 3. KANOON CALCULATOR (FOR INDIAN LEGAL ADVOCATES)
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
}
