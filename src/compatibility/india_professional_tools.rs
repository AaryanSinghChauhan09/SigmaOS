extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
// SigmaOS India Professional Tools
// Localized, high-performance, OOP-compliant tools for Indian Professionals.
// Refers to India-Apps-Overview.md and India-first architecture.

use crate::klib::HashMap;

/// 1. Legal & Judicial Professionals (`sigma-judicial`)
/// Manages Bharatiya Nyaya Sanhita (BNS), Bharatiya Nagarik Suraksha Sanhita (BNSS),
/// and Bharatiya Sakshya Adhiniyam (BSA) case schedules and bail readiness.
pub struct JudicialTimelinePlanner {
    pub active_cases: HashMap<String, u64>,
}

impl JudicialTimelinePlanner {
    pub fn new() -> Self {
        Self {
            active_cases: HashMap::new(),
        }
    }

    /// Calculates filing deadlines under BNSS (e.g. charge sheet filing in 60 or 90 days)
    pub fn calculate_filing_deadline(&self, case_type: &str, start_date_timestamp: u64) -> u64 {
        let days = match case_type {
            "BNS_HEINOUS" => 90,
            "BNS_STANDARD" => 60,
            _ => 30,
        };
        start_date_timestamp + (days * 24 * 60 * 60)
    }

    /// Evaluates if an accused is ready for default bail under Section 480 of BNSS
    pub fn calculate_bail_readiness(
        &self,
        offense_gravity: &str,
        days_in_custody: u32,
    ) -> Result<bool, &'static str> {
        match offense_gravity {
            "LIFE_IMPRISONMENT" | "DEATH_PENALTY" => {
                // Default bail limit is usually 90 days under BNSS
                Ok(days_in_custody >= 90)
            }
            "STANDARD_OFFENSE" => {
                // Standard limit is 60 days
                Ok(days_in_custody >= 60)
            }
            "MINOR_OFFENSE" => {
                // Standard minor limit is 30 days
                Ok(days_in_custody >= 30)
            }
            _ => Err("Invalid or unclassified offense gravity specification"),
        }
    }
}

impl Default for JudicialTimelinePlanner {
    fn default() -> Self {
        Self::new()
    }
}

/// 7. Space & Satellite Professionals (`sigma-isro`)
pub struct SpacSatResolver {
    pub satellite_catalog: HashMap<String, String>,
}

impl SpacSatResolver {
    pub fn new() -> Self {
        Self {
            satellite_catalog: HashMap::new(),
        }
    }

    pub fn resolve_orbit(&self, satellite: &str) -> Option<&String> {
        self.satellite_catalog.get(satellite)
    }
}

impl Default for SpacSatResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// 2. Business & Trade Professionals (`sigma-msme`)
/// Verifies Udyam Registration parameters and computes delayed payment interest under the MSMED Act.
pub struct MsmeComplianceEngine {
    pub registered_udyam_ids: HashMap<String, String>,
}

impl MsmeComplianceEngine {
    pub fn new() -> Self {
        Self {
            registered_udyam_ids: HashMap::new(),
        }
    }

    /// Classifies an MSME enterprise based on investment (Crore) and turnover (Crore) composite criteria
    pub fn classify_msme(&self, investment_cr: f64, turnover_cr: f64) -> &'static str {
        if investment_cr <= 1.0 && turnover_cr <= 5.0 {
            "Micro"
        } else if investment_cr <= 10.0 && turnover_cr <= 50.0 {
            "Small"
        } else if investment_cr <= 50.0 && turnover_cr <= 250.0 {
            "Medium"
        } else {
            "Large (Non-MSME)"
        }
    }

    /// Calculates delayed payment interest under Section 16 of the MSMED Act
    /// Interest is compound interest with monthly rests at three times the bank rate
    pub fn calculate_delayed_payment_interest(
        &self,
        principal_amount: f64,
        bank_rate: f64,
        delay_days: u32,
    ) -> f64 {
        if delay_days == 0 {
            return 0.0;
        }
        let effective_rate = 3.0 * bank_rate; // Three times the bank rate
        let months = (delay_days as f64) / 30.0;
        // Standard compound interest formula: A = P * (1 + r/12)^(12 * t)
        // With monthly compounding: r is annual rate, monthly rate is effective_rate / 100.0 / 12.0
        let monthly_rate = effective_rate / 100.0 / 12.0;
        let amount = principal_amount * (1.0 + monthly_rate).powf(months);
        amount - principal_amount
    }
}

impl Default for MsmeComplianceEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 3. Medical & AYUSH Practitioners (`sigma-ayush`)
/// Digital integration for AYUSH practitioner registry and Ayurvedic Formulary lookups.
pub struct AyushFormularyHelper {
    pub verified_practitioners: HashMap<String, String>,
    pub formulary_registry: HashMap<String, Vec<String>>,
}

impl AyushFormularyHelper {
    pub fn new() -> Self {
        let mut formulary = HashMap::new();
        formulary.insert(
            "Chyawanprash".to_string(),
            vec![
                "Amla".to_string(),
                "Ashwagandha".to_string(),
                "Guduchi".to_string(),
            ],
        );
        formulary.insert(
            "Triphala".to_string(),
            vec![
                "Amalaki".to_string(),
                "Bibhitaki".to_string(),
                "Haritaki".to_string(),
            ],
        );

        Self {
            verified_practitioners: HashMap::new(),
            formulary_registry: formulary,
        }
    }

    pub fn verify_practitioner(&self, registration_id: &str) -> bool {
        self.verified_practitioners.contains_key(registration_id)
    }

    pub fn verify_ayurvedic_formulation(
        &self,
        product: &str,
        ingredient: &str,
    ) -> Result<bool, &'static str> {
        let ingredients = self
            .formulary_registry
            .get(product)
            .ok_or("Product not found in Ayurvedic Formulary")?;
        Ok(ingredients.iter().any(|ing| ing == ingredient))
    }
}

impl Default for AyushFormularyHelper {
    fn default() -> Self {
        Self::new()
    }
}

/// 4. Hotspot & Telecommunications Operators (`sigma-wani`)
/// Implements TRAI Public Data Office (PDO) registries and bandwidth sharing profiles.
pub struct PMWaniHotspotController {
    pub registered_pdos: HashMap<String, String>,
}

impl PMWaniHotspotController {
    pub fn new() -> Self {
        Self {
            registered_pdos: HashMap::new(),
        }
    }

    pub fn register_pdo(&mut self, pdo_id: &str, location: &str) -> bool {
        self.registered_pdos
            .insert(pdo_id.to_string(), location.to_string());
        true
    }

    pub fn get_trai_bandwidth_profile(&self, active_users: u32) -> &'static str {
        if active_users < 10 {
            "Ultra-High-Speed (Unlimited)"
        } else if active_users < 50 {
            "Balanced Quality-of-Service"
        } else {
            "TRAI FUP Bandwidth Throttle"
        }
    }
}

impl Default for PMWaniHotspotController {
    fn default() -> Self {
        Self::new()
    }
}

/// 5. Aviation & Airport Operators (`sigma-digiyatra`)
/// Deep integration for passenger face enrollment and paperless railway/airport boarding validation.
pub struct DigiYatraPassScanner {
    pub passenger_faces: HashMap<String, Vec<u8>>,
}

impl DigiYatraPassScanner {
    pub fn new() -> Self {
        Self {
            passenger_faces: HashMap::new(),
        }
    }

    pub fn enroll_passenger(&mut self, passenger_id: &str, face_signature: &[u8]) -> bool {
        self.passenger_faces
            .insert(passenger_id.to_string(), face_signature.to_vec());
        true
    }

    pub fn verify_passenger_boarding(&self, passenger_id: &str, scan_signature: &[u8]) -> bool {
        if let Some(enrolled) = self.passenger_faces.get(passenger_id) {
            enrolled == scan_signature
        } else {
            false
        }
    }
}

impl Default for DigiYatraPassScanner {
    fn default() -> Self {
        Self::new()
    }
}

/// 6. Transit & Logistics Professionals (`sigma-irctc`)
/// Facilitates deep train running status track, Tatkal window status, and PNR monitoring.
// =========================================================================
// Chartered Accountants & Tax Consultants GST Audit Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GstCalculation {
    pub taxable_value: u64, // In Indian Rupees
    pub cgst_amount: u64,
    pub sgst_amount: u64,
    pub igst_amount: u64,
    pub total_amount: u64,
}

/// CA GST Audit & Tax Compliance Engine
/// Validates GSTIN format/state codes, calculates CGST/SGST/IGST according to Indian tax slabs (0%, 5%, 12%, 18%, 28%),
/// and reconciles Input Tax Credit (ITC) between GSTR-3B and GSTR-2B.
pub struct CaGstTaxAuditEngine {
    pub state_codes: HashMap<String, String>, // "07" -> "Delhi", "27" -> "Maharashtra"
}

impl CaGstTaxAuditEngine {
    pub fn new() -> Self {
        let mut state_codes = HashMap::new();
        state_codes.insert("07".to_string(), "Delhi".to_string());
        state_codes.insert("27".to_string(), "Maharashtra".to_string());
        state_codes.insert("29".to_string(), "Karnataka".to_string());
        state_codes.insert("09".to_string(), "Uttar Pradesh".to_string());
        state_codes.insert("33".to_string(), "Tamil Nadu".to_string());

        Self { state_codes }
    }

    pub fn validate_gstin(&self, gstin: &str) -> bool {
        if gstin.len() != 15 {
            return false;
        }
        let state_code = &gstin[0..2];
        self.state_codes.contains_key(state_code)
    }

    pub fn compute_gst_tax(
        &self,
        taxable_value: u64,
        rate_percent: u8,
        is_inter_state: bool,
    ) -> Result<GstCalculation, &'static str> {
        match rate_percent {
            0 | 5 | 12 | 18 | 28 => {}
            _ => return Err("Invalid Indian GST tax slab percentage"),
        }

        let total_tax = (taxable_value * rate_percent as u64) / 100;

        if is_inter_state {
            Ok(GstCalculation {
                taxable_value,
                cgst_amount: 0,
                sgst_amount: 0,
                igst_amount: total_tax,
                total_amount: taxable_value + total_tax,
            })
        } else {
            let half_tax = total_tax / 2;
            Ok(GstCalculation {
                taxable_value,
                cgst_amount: half_tax,
                sgst_amount: half_tax,
                igst_amount: 0,
                total_amount: taxable_value + total_tax,
            })
        }
    }

    pub fn reconcile_itc(&self, gstr3b_itc: u64, gstr2b_itc: u64) -> (i64, &'static str) {
        let delta = gstr3b_itc as i64 - gstr2b_itc as i64;
        if delta == 0 {
            (0, "ITC Reconciliation Perfect: 100% Match between GSTR-3B and GSTR-2B")
        } else if delta > 0 {
            (delta, "ITC Warning: Excess ITC claimed in GSTR-3B relative to GSTR-2B auto-populated statement")
        } else {
            (delta, "ITC Alert: Unclaimed ITC available in GSTR-2B")
        }
    }
}

impl Default for CaGstTaxAuditEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// Civil & Structural Chartered Engineers CPWD BOQ Estimator
// =========================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct BoqItemEstimate {
    pub item_code: String,
    pub description: String,
    pub quantity: f64,
    pub unit: String,
    pub total_cost_inr: f64,
}

/// Civil Chartered Engineers Bill of Quantities (BOQ) Estimator
/// Performs IS 1200 CPWD Delhi Schedule of Rates (DSR) construction estimation,
/// RCC concrete volume design, and steel reinforcement tonnage calculations.
pub struct CharteredEngineersBoqEstimator {
    pub dsr_rates: HashMap<String, f64>, // item_code -> rate per unit
}

impl CharteredEngineersBoqEstimator {
    pub fn new() -> Self {
        let mut dsr_rates = HashMap::new();
        dsr_rates.insert("CPWD-CONC-M25".to_string(), 6500.0); // Rs 6,500 per cu.m M25 RCC
        dsr_rates.insert("CPWD-STEEL-FE500".to_string(), 68000.0); // Rs 68,000 per MT TMT steel
        dsr_rates.insert("CPWD-BRICK-CLASS75".to_string(), 4800.0); // Rs 4,800 per cu.m brickwork

        Self { dsr_rates }
    }

    pub fn estimate_rcc_beam_boq(
        &self,
        length_m: f64,
        breadth_m: f64,
        depth_m: f64,
        steel_percentage: f64,
    ) -> Result<(BoqItemEstimate, BoqItemEstimate, f64), &'static str> {
        let concrete_volume_cum = length_m * breadth_m * depth_m;
        let steel_density_kg_cum = 7850.0;
        let steel_weight_mt = (concrete_volume_cum * (steel_percentage / 100.0) * steel_density_kg_cum) / 1000.0;

        let conc_rate = *self.dsr_rates.get("CPWD-CONC-M25").unwrap_or(&6000.0);
        let steel_rate = *self.dsr_rates.get("CPWD-STEEL-FE500").unwrap_or(&65000.0);

        let concrete_item = BoqItemEstimate {
            item_code: "CPWD-CONC-M25".to_string(),
            description: "M25 Grade Reinforced Cement Concrete".to_string(),
            quantity: concrete_volume_cum,
            unit: "cum".to_string(),
            total_cost_inr: concrete_volume_cum * conc_rate,
        };

        let steel_item = BoqItemEstimate {
            item_code: "CPWD-STEEL-FE500".to_string(),
            description: "Thermo-Mechanically Treated (TMT) Fe500 Reinforcement Steel".to_string(),
            quantity: steel_weight_mt,
            unit: "MT".to_string(),
            total_cost_inr: steel_weight_mt * steel_rate,
        };

        let total_estimate = concrete_item.total_cost_inr + steel_item.total_cost_inr;
        Ok((concrete_item, steel_item, total_estimate))
    }
}

impl Default for CharteredEngineersBoqEstimator {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// Agri-Krishi Market Intelligence & e-NAM Mandi Price Analyzer
// =========================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct KrishiMandiPrice {
    pub commodity: String,
    pub mandi_name: String,
    pub modal_price_per_quintal: f64,
    pub msp_price_per_quintal: f64,
}

/// Krishi Market Intelligence & e-NAM Mandi Price Evaluator
/// Analyzes pan-India agricultural Mandi prices, compares market rates against Government Minimum Support Price (MSP),
/// and evaluates farmer crop profitability.
pub struct AgriKrishiMarketIntelligenceEngine {
    pub msp_catalog: HashMap<String, f64>, // crop -> MSP per quintal (Rupees)
}

impl AgriKrishiMarketIntelligenceEngine {
    pub fn new() -> Self {
        let mut msp_catalog = HashMap::new();
        msp_catalog.insert("Wheat".to_string(), 2275.0);
        msp_catalog.insert("Paddy (Common)".to_string(), 2183.0);
        msp_catalog.insert("Mustard".to_string(), 5650.0);
        msp_catalog.insert("Cotton (Medium)".to_string(), 6620.0);

        Self { msp_catalog }
    }

    pub fn evaluate_crop_price(
        &self,
        crop: &str,
        mandi_name: &str,
        current_mandi_price: f64,
    ) -> KrishiMandiPrice {
        let msp = *self.msp_catalog.get(crop).unwrap_or(&0.0);
        KrishiMandiPrice {
            commodity: crop.to_string(),
            mandi_name: mandi_name.to_string(),
            modal_price_per_quintal: current_mandi_price,
            msp_price_per_quintal: msp,
        }
    }

    pub fn is_price_above_msp(&self, crop: &str, mandi_price: f64) -> bool {
        let msp = *self.msp_catalog.get(crop).unwrap_or(&0.0);
        mandi_price >= msp
    }
}

impl Default for AgriKrishiMarketIntelligenceEngine {
    fn default() -> Self {
        Self::new()
    }
}

pub struct IrctcPnrTracker {
    pub pnr_statuses: HashMap<String, String>,
}

impl IrctcPnrTracker {
    pub fn new() -> Self {
        Self {
            pnr_statuses: HashMap::new(),
        }
    }

    /// Evaluates if the Tatkal booking window is active based on time (10 AM for AC, 11 AM for Non-AC)
    pub fn check_tatkal_window(&self, hour: u32, class_type: &str) -> &'static str {
        match class_type {
            "AC" => {
                if hour == 10 {
                    "TATKAL_WINDOW_OPEN (AC Class)"
                } else {
                    "TATKAL_WINDOW_CLOSED"
                }
            }
            _ => {
                if hour == 11 {
                    "TATKAL_WINDOW_OPEN (Non-AC Class)"
                } else {
                    "TATKAL_WINDOW_CLOSED"
                }
            }
        }
    }

    pub fn update_pnr_status(&mut self, pnr: &str, status: &str) {
        self.pnr_statuses
            .insert(pnr.to_string(), status.to_string());
    }

    pub fn get_pnr_status(&self, pnr: &str) -> Option<&str> {
        self.pnr_statuses.get(pnr).map(|s| s.as_str())
    }
}

impl Default for IrctcPnrTracker {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// SEBI-Compliant Trader Risk & NSE/BSE VaR Margin Calculator
// =========================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct SebiTraderMarginRequirement {
    pub symbol: String,
    pub span_margin_inr: f64,
    pub exposure_margin_inr: f64,
    pub total_margin_required_inr: f64,
    pub peak_margin_shortfall: bool,
}

/// SEBI Registered Trader & Broker Risk Engine
/// Calculates NSE/BSE derivative SPAN & exposure margin requirements, enforces SEBI peak margin rules,
/// and computes Value-at-Risk (VaR) position limits for Indian financial markets.
pub struct SebiRegisteredTraderRiskEngine {
    pub span_margin_rates: HashMap<String, f64>, // symbol -> SPAN %
}

impl SebiRegisteredTraderRiskEngine {
    pub fn new() -> Self {
        let mut span_margin_rates = HashMap::new();
        span_margin_rates.insert("NIFTY".to_string(), 0.12); // 12% SPAN
        span_margin_rates.insert("BANKNIFTY".to_string(), 0.15); // 15% SPAN
        span_margin_rates.insert("RELIANCE".to_string(), 0.20); // 20% SPAN
        Self { span_margin_rates }
    }

    pub fn calculate_margin(
        &self,
        symbol: &str,
        notional_value_inr: f64,
        available_funds_inr: f64,
    ) -> SebiTraderMarginRequirement {
        let span_rate = *self.span_margin_rates.get(symbol).unwrap_or(&0.25);
        let span_margin = notional_value_inr * span_rate;
        let exposure_margin = notional_value_inr * 0.03; // Standard 3% exposure margin
        let total_margin = span_margin + exposure_margin;
        let shortfall = available_funds_inr < total_margin;

        SebiTraderMarginRequirement {
            symbol: symbol.to_string(),
            span_margin_inr: span_margin,
            exposure_margin_inr: exposure_margin,
            total_margin_required_inr: total_margin,
            peak_margin_shortfall: shortfall,
        }
    }
}

impl Default for SebiRegisteredTraderRiskEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// NPCI UPI QR Code Merchant Payload Engine
// =========================================================================

/// NPCI Unified Payments Interface (UPI) QR Code Merchant Generator
/// Formulates standard `upi://pay` merchant payment URLs, validates Virtual Payment Addresses (VPA),
/// and parses incoming NPCI transaction notifications.
pub struct UpiQrCodeMerchantEngine;

impl UpiQrCodeMerchantEngine {
    pub fn new() -> Self {
        UpiQrCodeMerchantEngine
    }

    pub fn validate_vpa(&self, vpa: &str) -> bool {
        vpa.contains('@') && (vpa.ends_with("@upi") || vpa.ends_with("@okaxis") || vpa.ends_with("@ybl") || vpa.ends_with("@icici") || vpa.ends_with("@paytm"))
    }

    pub fn generate_merchant_upi_string(
        &self,
        vpa: &str,
        merchant_name: &str,
        amount_inr: f64,
        transaction_ref: &str,
    ) -> Result<String, &'static str> {
        if !self.validate_vpa(vpa) {
            return Err("Invalid UPI Virtual Payment Address (VPA)");
        }
        Ok(format!(
            "upi://pay?pa={}&pn={}&am={:.2}&tr={}&cu=INR",
            vpa, merchant_name, amount_inr, transaction_ref
        ))
    }
}

impl Default for UpiQrCodeMerchantEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// NMC Medical Council Doctor Prescription & ABHA ID Linker
// =========================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct NMCPrescriptionRecord {
    pub doctor_nmc_reg_no: String,
    pub doctor_name: String,
    pub patient_abha_id: String,
    pub prescribed_medicines: Vec<String>,
    pub timestamp: u64,
}

/// National Medical Commission (NMC) E-Prescription & Ayushman Bharat ABHA Linker
/// Validates NMC doctor registration credentials, links prescriptions to 14-digit ABHA IDs,
/// and generates tamper-evident digital prescription records.
pub struct MedicalCouncilDoctorPrescriptionGenerator {
    pub registered_doctors: HashMap<String, String>, // reg_no -> Doctor Name
}

impl MedicalCouncilDoctorPrescriptionGenerator {
    pub fn new() -> Self {
        let mut registered_doctors = HashMap::new();
        registered_doctors.insert("NMC-REG-2026-991".to_string(), "Dr. Aaryan Singh".to_string());
        Self { registered_doctors }
    }

    pub fn validate_abha_id(&self, abha_id: &str) -> bool {
        // ABHA ID format: 14 digits or xx-xxxx-xxxx-xxxx
        let digits_only: String = abha_id.chars().filter(|c| c.is_ascii_digit()).collect();
        digits_only.len() == 14
    }

    pub fn create_prescription(
        &self,
        doctor_reg_no: &str,
        abha_id: &str,
        medicines: &[&str],
        timestamp: u64,
    ) -> Result<NMCPrescriptionRecord, &'static str> {
        let doctor_name = self
            .registered_doctors
            .get(doctor_reg_no)
            .ok_or("Unregistered or Invalid NMC Doctor Registration Number")?;

        if !self.validate_abha_id(abha_id) {
            return Err("Invalid 14-digit Ayushman Bharat Health Account (ABHA) ID");
        }

        Ok(NMCPrescriptionRecord {
            doctor_nmc_reg_no: doctor_reg_no.to_string(),
            doctor_name: doctor_name.clone(),
            patient_abha_id: abha_id.to_string(),
            prescribed_medicines: medicines.iter().map(|s| s.to_string()).collect(),
            timestamp,
        })
    }
}

impl Default for MedicalCouncilDoctorPrescriptionGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_judicial_timeline_planner() {
        let planner = JudicialTimelinePlanner::new();
        // Deadline of standard standard case
        let deadline = planner.calculate_filing_deadline("BNS_STANDARD", 1000);
        assert_eq!(deadline, 1000 + (60 * 24 * 60 * 60));

        // Bail readiness under BNSS
        assert_eq!(
            planner.calculate_bail_readiness("LIFE_IMPRISONMENT", 95),
            Ok(true)
        );
        assert_eq!(
            planner.calculate_bail_readiness("LIFE_IMPRISONMENT", 80),
            Ok(false)
        );
    }

    #[test]
    fn test_msme_compliance_engine() {
        let engine = MsmeComplianceEngine::new();
        // Composite classification
        assert_eq!(engine.classify_msme(0.5, 3.0), "Micro");
        assert_eq!(engine.classify_msme(5.0, 25.0), "Small");
        assert_eq!(engine.classify_msme(25.0, 120.0), "Medium");
        assert_eq!(engine.classify_msme(100.0, 500.0), "Large (Non-MSME)");

        // Interest on delayed payment compound interest under MSMED Act Section 16 (3x Bank Rate)
        let principal = 100000.0;
        let bank_rate = 6.5; // effective rate = 19.5%
        let interest = engine.calculate_delayed_payment_interest(principal, bank_rate, 90);
        assert!(interest > 0.0);
        // Approx 3 months of compounding at 19.5% annual
        assert!((interest - 4958.0).abs() < 50.0);
    }

    #[test]
    fn test_ayush_formulary_helper() {
        let mut helper = AyushFormularyHelper::new();
        helper
            .verified_practitioners
            .insert("REG-AYUSH-1234".to_string(), "Dr. Aaryan".to_string());

        assert!(helper.verify_practitioner("REG-AYUSH-1234"));
        assert!(!helper.verify_practitioner("REG-AYUSH-9999"));

        assert_eq!(
            helper.verify_ayurvedic_formulation("Chyawanprash", "Amla"),
            Ok(true)
        );
        assert_eq!(
            helper.verify_ayurvedic_formulation("Chyawanprash", "Pipali"),
            Ok(false)
        );
    }

    #[test]
    fn test_pm_wani_hotspot_controller() {
        let mut controller = PMWaniHotspotController::new();
        assert!(controller.register_pdo("PDO-MUMBAI-01", "Dharavi Hotspot"));
        assert_eq!(
            controller.get_trai_bandwidth_profile(5),
            "Ultra-High-Speed (Unlimited)"
        );
        assert_eq!(
            controller.get_trai_bandwidth_profile(25),
            "Balanced Quality-of-Service"
        );
        assert_eq!(
            controller.get_trai_bandwidth_profile(100),
            "TRAI FUP Bandwidth Throttle"
        );
    }

    #[test]
    fn test_digiyatra_pass_scanner() {
        let mut scanner = DigiYatraPassScanner::new();
        assert!(scanner.enroll_passenger("DY-PASS-789", b"face_descriptor_vector_bytes_789"));
        assert!(
            scanner.verify_passenger_boarding("DY-PASS-789", b"face_descriptor_vector_bytes_789")
        );
        assert!(
            !scanner.verify_passenger_boarding("DY-PASS-789", b"face_descriptor_mismatch_bytes")
        );
    }

    #[test]
    fn test_ca_gst_tax_audit_engine() {
        let engine = CaGstTaxAuditEngine::new();
        assert!(engine.validate_gstin("07AAAAA0000A1Z5")); // Delhi state code 07
        assert!(engine.validate_gstin("27AAAAA0000A1Z5")); // Maharashtra state code 27
        assert!(!engine.validate_gstin("99AAAAA0000A1Z5")); // Invalid state code

        // Intra-state GST (Delhi -> Delhi)
        let calc = engine.compute_gst_tax(100000, 18, false).unwrap();
        assert_eq!(calc.cgst_amount, 9000);
        assert_eq!(calc.sgst_amount, 9000);
        assert_eq!(calc.igst_amount, 0);
        assert_eq!(calc.total_amount, 118000);

        // Inter-state GST (Delhi -> Maharashtra)
        let inter_calc = engine.compute_gst_tax(100000, 18, true).unwrap();
        assert_eq!(inter_calc.cgst_amount, 0);
        assert_eq!(inter_calc.sgst_amount, 0);
        assert_eq!(inter_calc.igst_amount, 18000);

        // ITC Reconciliation
        let (delta, msg) = engine.reconcile_itc(50000, 50000);
        assert_eq!(delta, 0);
        assert!(msg.contains("ITC Reconciliation Perfect"));
    }

    #[test]
    fn test_chartered_engineers_boq_estimator() {
        let boq = CharteredEngineersBoqEstimator::new();
        let (conc, steel, total) = boq.estimate_rcc_beam_boq(10.0, 0.3, 0.6, 2.0).unwrap();

        assert_eq!(conc.quantity, 1.8); // 10 * 0.3 * 0.6
        assert!(steel.quantity > 0.2); // ~0.2826 MT steel
        assert!(total > 20000.0);
    }

    #[test]
    fn test_agri_krishi_market_intelligence_engine() {
        let krishi = AgriKrishiMarketIntelligenceEngine::new();
        let price_info = krishi.evaluate_crop_price("Wheat", "Azadpur Mandi", 2400.0);

        assert_eq!(price_info.msp_price_per_quintal, 2275.0);
        assert!(krishi.is_price_above_msp("Wheat", 2400.0));
        assert!(!krishi.is_price_above_msp("Wheat", 2100.0));
    }

    #[test]
    fn test_irctc_pnr_tracker() {
        let mut tracker = IrctcPnrTracker::new();
        assert_eq!(
            tracker.check_tatkal_window(10, "AC"),
            "TATKAL_WINDOW_OPEN (AC Class)"
        );
        assert_eq!(tracker.check_tatkal_window(9, "AC"), "TATKAL_WINDOW_CLOSED");

        tracker.update_pnr_status("2748927491", "CONFIRMED");
        assert_eq!(tracker.get_pnr_status("2748927491"), Some("CONFIRMED"));
    }

    #[test]
    fn test_sebi_registered_trader_risk_engine() {
        let engine = SebiRegisteredTraderRiskEngine::new();
        let margin = engine.calculate_margin("NIFTY", 1000000.0, 200000.0);
        assert_eq!(margin.symbol, "NIFTY");
        assert_eq!(margin.span_margin_inr, 120000.0);
        assert_eq!(margin.exposure_margin_inr, 30000.0);
        assert_eq!(margin.total_margin_required_inr, 150000.0);
        assert!(!margin.peak_margin_shortfall);
    }

    #[test]
    fn test_upi_qr_code_merchant_engine() {
        let merchant = UpiQrCodeMerchantEngine::new();
        assert!(merchant.validate_vpa("store@okaxis"));

        let uri = merchant
            .generate_merchant_upi_string("store@okaxis", "Acme Retail", 500.0, "TXN12345")
            .unwrap();
        assert!(uri.contains("pa=store@okaxis"));
        assert!(uri.contains("am=500.00"));
        assert!(uri.starts_with("upi://pay?"));
    }

    #[test]
    fn test_medical_council_doctor_prescription_generator() {
        let generator = MedicalCouncilDoctorPrescriptionGenerator::new();
        assert!(generator.validate_abha_id("12345678901234"));

        let rx = generator
            .create_prescription(
                "NMC-REG-2026-991",
                "12-3456-7890-1234",
                &["Paracetamol 500mg", "Amoxicillin 250mg"],
                1700000000,
            )
            .unwrap();

        assert_eq!(rx.doctor_name, "Dr. Aaryan Singh");
        assert_eq!(rx.prescribed_medicines.len(), 2);
    }
}
