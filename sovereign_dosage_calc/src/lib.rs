// SovereignDosageCalc - Medical Dosage Calculation System
// Implements Telemedicine Guidelines & Drugs Act compliance
// No external dependencies - implements from first principles

use std::fmt;

/// Drug schedule per Drugs Act
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrugSchedule {
    H1,  // Narcotic drugs
    H,   // Psychotropic substances
    X,   // Restricted drugs
    None, // Non-scheduled
}

impl DrugSchedule {
    pub fn as_str(&self) -> &'static str {
        match self {
            DrugSchedule::H1 => "H1",
            DrugSchedule::H => "H",
            DrugSchedule::X => "X",
            DrugSchedule::None => "None",
        }
    }
    
    pub fn requires_prescription(&self) -> bool {
        matches!(self, DrugSchedule::H1 | DrugSchedule::H | DrugSchedule::X)
    }
    
    pub fn max_quantity_limit(&self) -> u64 {
        // Maximum quantity in units per prescription
        match self {
            DrugSchedule::H1 => 30,   // Strict limit
            DrugSchedule::H => 100,  // Moderate limit
            DrugSchedule::X => 50,   // Restricted limit
            DrugSchedule::None => 1000, // No limit
        }
    }
}

/// Dosage unit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DosageUnit {
    Mg,
    G,
    Ml,
    Units,
    Drops,
}

impl DosageUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            DosageUnit::Mg => "mg",
            DosageUnit::G => "g",
            DosageUnit::Ml => "ml",
            DosageUnit::Units => "units",
            DosageUnit::Drops => "drops",
        }
    }
    
    pub fn to_base(&self) -> f64 {
        // Convert to base unit (mg)
        match self {
            DosageUnit::Mg => 1.0,
            DosageUnit::G => 1000.0,
            DosageUnit::Ml => 1.0, // Assuming 1mg/ml for simplicity
            DosageUnit::Units => 1.0,
            DosageUnit::Drops => 0.05, // Approx 20 drops = 1ml
        }
    }
}

/// Frequency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Frequency {
    Once,
    TwiceDaily,
    ThreeTimesDaily,
    FourTimesDaily,
    Every6Hours,
    Every8Hours,
    Every12Hours,
    AsNeeded,
}

impl Frequency {
    pub fn as_str(&self) -> &'static str {
        match self {
            Frequency::Once => "Once",
            Frequency::TwiceDaily => "Twice daily",
            Frequency::ThreeTimesDaily => "Three times daily",
            Frequency::FourTimesDaily => "Four times daily",
            Frequency::Every6Hours => "Every 6 hours",
            Frequency::Every8Hours => "Every 8 hours",
            Frequency::Every12Hours => "Every 12 hours",
            Frequency::AsNeeded => "As needed",
        }
    }
    
    pub fn daily_multiplier(&self) -> f64 {
        match self {
            Frequency::Once => 1.0,
            Frequency::TwiceDaily => 2.0,
            Frequency::ThreeTimesDaily => 3.0,
            Frequency::FourTimesDaily => 4.0,
            Frequency::Every6Hours => 4.0,
            Frequency::Every8Hours => 3.0,
            Frequency::Every12Hours => 2.0,
            Frequency::AsNeeded => 1.0,
        }
    }
}

/// Duration
#[derive(Debug, Clone)]
pub struct Duration {
    pub value: u64,
    pub unit: DurationUnit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurationUnit {
    Days,
    Weeks,
    Months,
}

impl Duration {
    pub fn new(value: u64, unit: DurationUnit) -> Self {
        Duration { value, unit }
    }
    
    pub fn as_days(&self) -> u64 {
        match self.unit {
            DurationUnit::Days => self.value,
            DurationUnit::Weeks => self.value * 7,
            DurationUnit::Months => self.value * 30, // Approximate
        }
    }
}

/// Dosage
#[derive(Debug, Clone)]
pub struct Dosage {
    pub amount: f64,
    pub unit: DosageUnit,
}

impl Dosage {
    pub fn new(amount: f64, unit: DosageUnit) -> Self {
        Dosage { amount, unit }
    }
    
    pub fn in_base_units(&self) -> f64 {
        self.amount * self.unit.to_base()
    }
}

/// Drug information
#[derive(Debug, Clone)]
pub struct DrugInfo {
    pub drug_id: String,
    pub generic_name: String,
    pub brand_names: Vec<String>,
    pub schedule: DrugSchedule,
    pub standard_dosage: Dosage,
    pub max_dosage: Dosage,
}

impl DrugInfo {
    pub fn new(
        drug_id: String,
        generic_name: String,
        brand_names: Vec<String>,
        schedule: DrugSchedule,
        standard_dosage: Dosage,
        max_dosage: Dosage,
    ) -> Self {
        DrugInfo {
            drug_id,
            generic_name,
            brand_names,
            schedule,
            standard_dosage,
            max_dosage,
        }
    }
}

/// Drug interaction
#[derive(Debug, Clone)]
pub struct Interaction {
    pub severity: InteractionSeverity,
    pub description: String,
    pub recommendation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionSeverity {
    Mild,
    Moderate,
    Severe,
    Contraindicated,
}

impl Interaction {
    pub fn new(severity: InteractionSeverity, description: String, recommendation: String) -> Self {
        Interaction {
            severity,
            description,
            recommendation,
        }
    }
}

/// Medication
#[derive(Debug, Clone)]
pub struct Medication {
    pub drug_id: String,
    pub brand_name: String,
    pub generic_name: String,
    pub dosage: Dosage,
    pub frequency: Frequency,
    pub duration: Duration,
    pub schedule: DrugSchedule,
    pub interactions: Vec<Interaction>,
}

impl Medication {
    pub fn new(
        drug_id: String,
        brand_name: String,
        generic_name: String,
        dosage: Dosage,
        frequency: Frequency,
        duration: Duration,
        schedule: DrugSchedule,
    ) -> Self {
        Medication {
            drug_id,
            brand_name,
            generic_name,
            dosage,
            frequency,
            duration,
            schedule,
            interactions: Vec::new(),
        }
    }
    
    pub fn add_interaction(&mut self, interaction: Interaction) {
        self.interactions.push(interaction);
    }
    
    pub fn total_quantity(&self) -> f64 {
        let daily_dose = self.dosage.in_base_units() * self.frequency.daily_multiplier();
        let total_days = self.duration.as_days();
        daily_dose * total_days as f64
    }
    
    pub fn daily_dose(&self) -> f64 {
        self.dosage.in_base_units() * self.frequency.daily_multiplier()
    }
}

/// Patient parameters
#[derive(Debug, Clone)]
pub struct PatientParams {
    pub weight_kg: f64,
    pub age_years: u64,
    pub renal_function: RenalFunction,
    pub liver_function: LiverFunction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenalFunction {
    Normal,
    MildImpairment,
    ModerateImpairment,
    SevereImpairment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiverFunction {
    Normal,
    MildImpairment,
    ModerateImpairment,
    SevereImpairment,
}

impl PatientParams {
    pub fn new(weight_kg: f64, age_years: u64) -> Self {
        PatientParams {
            weight_kg,
            age_years,
            renal_function: RenalFunction::Normal,
            liver_function: LiverFunction::Normal,
        }
    }
    
    pub fn with_renal_function(mut self, renal_function: RenalFunction) -> Self {
        self.renal_function = renal_function;
        self
    }
    
    pub fn with_liver_function(mut self, liver_function: LiverFunction) -> Self {
        self.liver_function = liver_function;
        self
    }
    
    pub fn get_dose_adjustment_factor(&self) -> f64 {
        let mut factor = 1.0;
        
        // Renal function adjustment
        factor *= match self.renal_function {
            RenalFunction::Normal => 1.0,
            RenalFunction::MildImpairment => 0.75,
            RenalFunction::ModerateImpairment => 0.5,
            RenalFunction::SevereImpairment => 0.25,
        };
        
        // Liver function adjustment
        factor *= match self.liver_function {
            LiverFunction::Normal => 1.0,
            LiverFunction::MildImpairment => 0.8,
            LiverFunction::ModerateImpairment => 0.6,
            LiverFunction::SevereImpairment => 0.4,
        };
        
        factor
    }
}

/// Prescription
#[derive(Debug, Clone)]
pub struct Prescription {
    pub rx_id: [u8; 32],
    pub patient_id: [u8; 32],
    pub doctor_id: [u8; 32],
    pub medications: Vec<Medication>,
    pub diagnosis: String,
    pub created_at: u64,
    pub expires_at: u64,
}

impl Prescription {
    pub fn new(patient_id: [u8; 32], doctor_id: [u8; 32], diagnosis: String) -> Self {
        let rx_id = Self::generate_rx_id(&patient_id, &diagnosis);
        let created_at = Self::current_timestamp();
        let expires_at = created_at + (30 * 24 * 60 * 60); // 30 days validity
        
        Prescription {
            rx_id,
            patient_id,
            doctor_id,
            medications: Vec::new(),
            diagnosis,
            created_at,
            expires_at,
        }
    }
    
    fn generate_rx_id(patient_id: &[u8; 32], diagnosis: &str) -> [u8; 32] {
        // Placeholder for actual BLAKE3 hash
        let mut hash = [0u8; 32];
        for (i, &byte) in patient_id.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(byte);
        }
        let diag_bytes = diagnosis.as_bytes();
        for (i, &byte) in diag_bytes.iter().enumerate() {
            hash[(i + 16) % 32] = hash[(i + 16) % 32].wrapping_add(byte);
        }
        hash
    }
    
    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
    
    pub fn add_medication(&mut self, medication: Medication) {
        self.medications.push(medication);
    }
    
    pub fn get_rx_id(&self) -> String {
        self.rx_id.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }
    
    pub fn is_valid(&self) -> bool {
        let now = Self::current_timestamp();
        now >= self.created_at && now <= self.expires_at
    }
}

impl fmt::Display for Prescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Prescription\n\
             ID: {}\n\
             Diagnosis: {}\n\
             Created: {}\n\
             Expires: {}\n\
             Valid: {}\n\
             Medications ({}):",
            self.get_rx_id(),
            self.diagnosis,
            self.created_at,
            self.expires_at,
            self.is_valid(),
            self.medications.len()
        )?;
        
        for (i, med) in self.medications.iter().enumerate() {
            writeln!(
                f,
                "\n  {}. {} ({}) - {} {} {}x",
                i + 1,
                med.brand_name,
                med.generic_name,
                med.dosage.amount,
                med.dosage.unit.as_str(),
                med.frequency.as_str(),
                med.frequency.daily_multiplier()
            )?;
        }
        
        Ok(())
    }
}

/// Dosage calculator
pub struct DosageCalculator {
    drug_database: Vec<DrugInfo>,
}

impl DosageCalculator {
    pub fn new() -> Self {
        DosageCalculator {
            drug_database: Vec::new(),
        }
    }
    
    /// Calculate weight-based dosage
    pub fn calculate_weight_based_dosage(
        &self,
        standard_dosage_mg: f64,
        patient_weight_kg: f64,
        dose_per_kg: f64,
    ) -> f64 {
        standard_dosage_mg * (patient_weight_kg * dose_per_kg / 70.0)
    }
    
    /// Calculate age-based dosage
    pub fn calculate_age_based_dosage(
        &self,
        adult_dosage_mg: f64,
        patient_age_years: u64,
    ) -> f64 {
        // Simplified age-based calculation
        if patient_age_years < 2 {
            adult_dosage_mg * 0.125
        } else if patient_age_years < 12 {
            adult_dosage_mg * 0.5
        } else if patient_age_years < 18 {
            adult_dosage_mg * 0.75
        } else {
            adult_dosage_mg
        }
    }
    
    /// Calculate adjusted dosage based on patient parameters
    pub fn calculate_adjusted_dosage(
        &self,
        base_dosage_mg: f64,
        patient_params: &PatientParams,
    ) -> f64 {
        base_dosage_mg * patient_params.get_dose_adjustment_factor()
    }
    
    /// Check drug interactions
    pub fn check_interactions(&self, medications: &[Medication]) -> Vec<Interaction> {
        let mut interactions = Vec::new();
        
        // Simplified interaction checking
        for (i, med1) in medications.iter().enumerate() {
            for med2 in medications.iter().skip(i + 1) {
                // Check for known interactions (placeholder logic)
                if self.has_interaction(&med1.drug_id, &med2.drug_id) {
                    interactions.push(Interaction::new(
                        InteractionSeverity::Moderate,
                        format!("Potential interaction between {} and {}", med1.generic_name, med2.generic_name),
                        "Monitor patient closely".to_string(),
                    ));
                }
            }
        }
        
        interactions
    }
    
    fn has_interaction(&self, drug1: &str, drug2: &str) -> bool {
        // Placeholder for actual interaction database
        // In production, this would query a comprehensive interaction database
        false
    }
    
    /// Get drug information
    pub fn get_drug_info(&self, drug_id: &str) -> Option<&DrugInfo> {
        self.drug_database
            .iter()
            .find(|d| d.drug_id == drug_id)
    }
    
    /// Add drug to database
    pub fn add_drug(&mut self, drug_info: DrugInfo) {
        self.drug_database.push(drug_info);
    }
    
    /// Verify dosage against limits
    pub fn verify_dosage(&self, medication: &Medication) -> Result<bool, String> {
        let daily_dose = medication.daily_dose();
        let max_limit = medication.schedule.max_quantity_limit() as f64;
        
        if daily_dose > max_limit {
            Err(format!("Daily dose {} exceeds limit {} for schedule {}",
                daily_dose, max_limit, medication.schedule.as_str()))
        } else {
            Ok(true)
        }
    }
    
    /// Check contraindications
    pub fn check_contraindications(
        &self,
        drug_id: &str,
        patient_history: &[String],
    ) -> Vec<String> {
        let mut warnings = Vec::new();
        
        // Check for known contraindications
        for condition in patient_history {
            if self.is_contraindicated(drug_id, condition) {
                warnings.push(format!("Drug is contraindicated for: {}", condition));
            }
        }
        
        warnings
    }
    
    fn is_contraindicated(&self, drug_id: &str, condition: &str) -> bool {
        // Placeholder for actual contraindication database
        false
    }
}

impl Default for DosageCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_weight_based_dosage() {
        let calculator = DosageCalculator::new();
        
        let dose = calculator.calculate_weight_based_dosage(100.0, 70.0, 1.0);
        assert!((dose - 100.0).abs() < 0.1);
        
        let dose_child = calculator.calculate_weight_based_dosage(100.0, 35.0, 1.0);
        assert!((dose_child - 50.0).abs() < 0.1);
    }
    
    #[test]
    fn test_age_based_dosage() {
        let calculator = DosageCalculator::new();
        
        let adult_dose = calculator.calculate_age_based_dosage(100.0, 25);
        assert!((adult_dose - 100.0).abs() < 0.1);
        
        let child_dose = calculator.calculate_age_based_dosage(100.0, 10);
        assert!((child_dose - 50.0).abs() < 0.1);
    }
    
    #[test]
    fn test_dose_adjustment() {
        let calculator = DosageCalculator::new();
        
        let normal_params = PatientParams::new(70.0, 25);
        let adjusted = calculator.calculate_adjusted_dosage(100.0, &normal_params);
        assert!((adjusted - 100.0).abs() < 0.1);
        
        let impaired_params = PatientParams::new(70.0, 25)
            .with_renal_function(RenalFunction::ModerateImpairment);
        let adjusted_impaired = calculator.calculate_adjusted_dosage(100.0, &impaired_params);
        assert!((adjusted_impaired - 50.0).abs() < 0.1);
    }
    
    #[test]
    fn test_prescription_creation() {
        let patient_id = [0u8; 32];
        let doctor_id = [1u8; 32];
        
        let prescription = Prescription::new(patient_id, doctor_id, "Hypertension".to_string());
        
        assert_eq!(prescription.diagnosis, "Hypertension");
        assert!(prescription.is_valid());
    }
    
    #[test]
    fn test_medication_quantity() {
        let dosage = Dosage::new(500.0, DosageUnit::Mg);
        let frequency = Frequency::TwiceDaily;
        let duration = Duration::new(7, DurationUnit::Days);
        
        let medication = Medication::new(
            "drug001".to_string(),
            "BrandName".to_string(),
            "Generic".to_string(),
            dosage,
            frequency,
            duration,
            DrugSchedule::None,
        );
        
        let total = medication.total_quantity();
        assert!((total - 7000.0).abs() < 0.1); // 500mg * 2 * 7 = 7000mg
    }
    
    #[test]
    fn test_dosage_limits() {
        let calculator = DosageCalculator::new();
        
        let dosage = Dosage::new(50.0, DosageUnit::Mg);
        let medication = Medication::new(
            "drug001".to_string(),
            "BrandName".to_string(),
            "Generic".to_string(),
            dosage,
            Frequency::TwiceDaily,
            Duration::new(7, DurationUnit::Days),
            DrugSchedule::H1,
        );
        
        let result = calculator.verify_dosage(&medication);
        assert!(result.is_ok());
    }
}
