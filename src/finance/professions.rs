#![allow(unused_variables)]
// SigmaOS Scientific & Professional Calculators
// OOP-compliant zero-dependency calculators for Engineers, Accountants, Cashiers, Tax Officials, and Medical Doctors.

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// =========================================================================
// 1. ENGINEERS CALCULATORS (MECHANICAL, ELECTRICAL, CIVIL, SOFTWARE)
// =========================================================================

pub struct EngineeringCalculators;

impl EngineeringCalculators {
    /// Ohm's Law and Power: Electrical Engineering
    /// Returns (Voltage, Current, Resistance, Power) based on what inputs are provided
    pub fn calculate_ohms_law(
        v: Option<f64>,
        i: Option<f64>,
        r: Option<f64>,
    ) -> Result<(f64, f64, f64, f64), &'static str> {
        match (v, i, r) {
            (None, Some(current), Some(resistance)) => {
                let voltage = current * resistance;
                let power = voltage * current;
                Ok((voltage, current, resistance, power))
            }
            (Some(voltage), None, Some(resistance)) => {
                let current = voltage / resistance;
                let power = voltage * current;
                Ok((voltage, current, resistance, power))
            }
            (Some(voltage), Some(current), None) => {
                let resistance = voltage / current;
                let power = voltage * current;
                Ok((voltage, current, resistance, power))
            }
            _ => Err("Invalid parameters: Provide exactly two of (Voltage V, Current I, Resistance R)"),
        }
    }

    /// Beam Deflection & Stress: Structural / Civil Engineering
    /// Simple cantilever beam deflection under point load at the free end: d = (P * L^3) / (3 * E * I)
    /// Returns deflection in meters
    pub fn calculate_beam_deflection(
        load_newtons: f64,
        length_meters: f64,
        elastic_modulus_gpa: f64,
        moment_of_inertia_m4: f64,
    ) -> f64 {
        let e_pa = elastic_modulus_gpa * 1e9;
        (load_newtons * length_meters.powi(3)) / (3.0 * e_pa * moment_of_inertia_m4)
    }

    /// Thermal Expansion: Mechanical Engineering
    /// dL = L0 * alpha * dT
    pub fn calculate_thermal_expansion(
        initial_length_meters: f64,
        expansion_coeff: f64,
        temp_change_celsius: f64,
    ) -> f64 {
        initial_length_meters * expansion_coeff * temp_change_celsius
    }

    /// Network Bandwidth & Latency: Software / Network Engineering
    /// Computes TCP throughput bounds based on bandwidth-delay product: Max Throughput = Buffer Size / RTT
    pub fn calculate_max_tcp_throughput_bps(buffer_size_bytes: u64, rtt_ms: f64) -> f64 {
        let rtt_sec = rtt_ms / 1000.0;
        (buffer_size_bytes * 8) as f64 / rtt_sec
    }
}

// =========================================================================
// 2. ACCOUNTANT, CASHIER & TAX OFFICIAL CALCULATORS
// =========================================================================

pub struct FinancialProfessionCalculators;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepreciationMethod {
    StraightLine,
    WrittenDownValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaxRegime {
    OldRegime,
    NewRegime2026,
}

impl FinancialProfessionCalculators {
    /// Straight-line and WDV Depreciation calculation for Corporate Accountants
    pub fn calculate_depreciation(
        asset_value_paise: u64,
        salvage_value_paise: u64,
        useful_life_years: u32,
        rate: f64,
        method: DepreciationMethod,
    ) -> Vec<u64> {
        let mut schedules = Vec::new();
        let mut current_val = asset_value_paise;

        for _ in 0..useful_life_years {
            match method {
                DepreciationMethod::StraightLine => {
                    let dep = (((asset_value_paise - salvage_value_paise) as f64) / (useful_life_years as f64)) as u64;
                    current_val = current_val.saturating_sub(dep);
                    schedules.push(current_val);
                }
                DepreciationMethod::WrittenDownValue => {
                    let dep = (current_val as f64 * rate) as u64;
                    current_val = current_val.saturating_sub(dep);
                    schedules.push(current_val);
                }
            }
        }
        schedules
    }

    /// Cashier Cash-Drawer Registry & Denomination Audit tool
    pub fn audit_cash_drawer(
        denominations: &[(u32, u32)], // Array of (Denomination value, Count of notes/coins)
        expected_total_paise: u64,
    ) -> (u64, i64) {
        let mut actual_total_paise = 0u64;
        for &(value, count) in denominations {
            actual_total_paise += (value as u64) * (count as u64) * 100; // to paise
        }
        let variance = (actual_total_paise as i64) - (expected_total_paise as i64);
        (actual_total_paise, variance)
    }

    /// Progressive Income Tax Calculator for Tax Officers and Auditing
    pub fn calculate_income_tax(
        taxable_income_paise: u64,
        regime: TaxRegime,
    ) -> u64 {
        let income = taxable_income_paise / 100; // Compute in base currency units for simplicity
        let tax = match regime {
            TaxRegime::NewRegime2026 => {
                // Simplified progressive brackets for New Tax Regime
                if income <= 300_000 {
                    0
                } else if income <= 700_000 {
                    (income - 300_000) * 5 / 100
                } else if income <= 1_000_000 {
                    20_000 + (income - 700_000) * 10 / 100
                } else if income <= 1_500_000 {
                    50_000 + (income - 1_000_000) * 15 / 100
                } else {
                    125_000 + (income - 1_500_000) * 30 / 100
                }
            }
            TaxRegime::OldRegime => {
                // Brackets for Traditional Tax Regime
                if income <= 250_000 {
                    0
                } else if income <= 500_000 {
                    (income - 250_000) * 5 / 100
                } else if income <= 1_000_000 {
                    12_500 + (income - 500_000) * 20 / 100
                } else {
                    112_500 + (income - 1_000_000) * 30 / 100
                }
            }
        };
        tax * 100 // return in paise
    }
}

// =========================================================================
// 3. MEDICAL & CLINICAL DOCTORS CALCULATORS
// =========================================================================

pub struct MedicalDoctorCalculators;

impl MedicalDoctorCalculators {
    /// Body Mass Index (BMI) & Classification
    pub fn calculate_bmi(weight_kg: f64, height_meters: f64) -> Result<(f64, &'static str), &'static str> {
        if height_meters <= 0.0 || weight_kg <= 0.0 {
            return Err("Height and weight must be positive numbers");
        }
        let bmi = weight_kg / height_meters.powi(2);
        let category = if bmi < 18.5 {
            "Underweight"
        } else if bmi < 25.0 {
            "Normal weight"
        } else if bmi < 30.0 {
            "Overweight"
        } else {
            "Obese"
        };
        Ok((bmi, category))
    }

    /// Pediatric Dosage Calculator (Clark's Rule)
    /// dosage = (Weight of child in lbs / 150) * Adult dose
    pub fn calculate_pediatric_dose_clarks_rule(
        child_weight_kg: f64,
        adult_dose_mg: f64,
    ) -> f64 {
        let child_weight_lbs = child_weight_kg * 2.20462;
        (child_weight_lbs / 150.0) * adult_dose_mg
    }

    /// Glomerular Filtration Rate (GFR) estimation - Cockcroft-Gault Equation
    /// Returns GFR in mL/min
    pub fn calculate_creatinine_clearance(
        age_years: u32,
        weight_kg: f64,
        serum_creatinine_mg_dl: f64,
        is_female: bool,
    ) -> Result<f64, &'static str> {
        if serum_creatinine_mg_dl <= 0.0 {
            return Err("Serum creatinine must be a positive value");
        }
        let mut cl_cr = ((140 - age_years) as f64 * weight_kg) / (72.0 * serum_creatinine_mg_dl);
        if is_female {
            cl_cr *= 0.85;
        }
        Ok(cl_cr)
    }
}
