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
}
