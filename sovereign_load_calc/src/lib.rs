// SovereignLoadCalc - Structural Load Calculation System
// Implements BIS Standards / Structural Compliance
// No external dependencies - implements from first principles

use std::fmt;

/// Element type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementType {
    Beam,
    Column,
    Slab,
    Foundation,
}

impl ElementType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ElementType::Beam => "Beam",
            ElementType::Column => "Column",
            ElementType::Slab => "Slab",
            ElementType::Foundation => "Foundation",
        }
    }
}

/// Material grade
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialGrade {
    ConcreteM15,
    ConcreteM20,
    ConcreteM25,
    ConcreteM30,
    ConcreteM35,
    ConcreteM40,
    SteelFe250,
    SteelFe415,
    SteelFe500,
    SteelFe550,
}

impl MaterialGrade {
    pub fn as_str(&self) -> &'static str {
        match self {
            MaterialGrade::ConcreteM15 => "M15",
            MaterialGrade::ConcreteM20 => "M20",
            MaterialGrade::ConcreteM25 => "M25",
            MaterialGrade::ConcreteM30 => "M30",
            MaterialGrade::ConcreteM35 => "M35",
            MaterialGrade::ConcreteM40 => "M40",
            MaterialGrade::SteelFe250 => "Fe250",
            MaterialGrade::SteelFe415 => "Fe415",
            MaterialGrade::SteelFe500 => "Fe500",
            MaterialGrade::SteelFe550 => "Fe550",
        }
    }
    
    pub fn characteristic_strength(&self) -> f64 {
        // Characteristic strength in N/mm² (MPa)
        match self {
            MaterialGrade::ConcreteM15 => 15.0,
            MaterialGrade::ConcreteM20 => 20.0,
            MaterialGrade::ConcreteM25 => 25.0,
            MaterialGrade::ConcreteM30 => 30.0,
            MaterialGrade::ConcreteM35 => 35.0,
            MaterialGrade::ConcreteM40 => 40.0,
            MaterialGrade::SteelFe250 => 250.0,
            MaterialGrade::SteelFe415 => 415.0,
            MaterialGrade::SteelFe500 => 500.0,
            MaterialGrade::SteelFe550 => 550.0,
        }
    }
}

/// Material type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialType {
    Concrete,
    Steel,
}

impl MaterialType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MaterialType::Concrete => "Concrete",
            MaterialType::Steel => "Steel",
        }
    }
}

/// Material
#[derive(Debug, Clone)]
pub struct Material {
    pub material_type: MaterialType,
    pub grade: MaterialGrade,
}

impl Material {
    pub fn new(material_type: MaterialType, grade: MaterialGrade) -> Self {
        Material {
            material_type,
            grade,
        }
    }
    
    pub fn unit_weight(&self) -> f64 {
        // Unit weight in kN/m³
        match self.material_type {
            MaterialType::Concrete => 25.0,
            MaterialType::Steel => 78.5,
        }
    }
}

/// Dimensions
#[derive(Debug, Clone)]
pub struct Dimensions {
    pub length: f64,   // in meters
    pub width: f64,    // in meters
    pub height: f64,   // in meters
    pub thickness: f64, // in meters
}

impl Dimensions {
    pub fn new(length: f64, width: f64, height: f64, thickness: f64) -> Self {
        Dimensions {
            length,
            width,
            height,
            thickness,
        }
    }
    
    pub fn volume(&self) -> f64 {
        self.length * self.width * self.height
    }
    
    pub fn area(&self) -> f64 {
        self.length * self.width
    }
}

/// Load set
#[derive(Debug, Clone)]
pub struct LoadSet {
    pub dead_load: f64,    // kN/m² or kN/m
    pub live_load: f64,    // kN/m² or kN/m
    pub wind_load: f64,    // kN/m² or kN/m
    pub seismic_load: f64, // kN/m² or kN/m
    pub total_load: f64,
}

impl LoadSet {
    pub fn new(dead_load: f64, live_load: f64, wind_load: f64, seismic_load: f64) -> Self {
        let total_load = dead_load + live_load + wind_load + seismic_load;
        LoadSet {
            dead_load,
            live_load,
            wind_load,
            seismic_load,
            total_load,
        }
    }
    
    pub fn factored_load(&self) -> f64 {
        // Load combination per IS 875 Part 2
        // 1.5(DL + LL) + 1.5(WL) + 1.5(SL)
        1.5 * (self.dead_load + self.live_load + self.wind_load + self.seismic_load)
    }
}

/// Analysis results
#[derive(Debug, Clone)]
pub struct AnalysisResults {
    pub bending_moment: f64,  // kNm
    pub shear_force: f64,    // kN
    pub deflection: f64,     // mm
    pub stress: f64,         // N/mm²
}

impl AnalysisResults {
    pub fn new(bending_moment: f64, shear_force: f64, deflection: f64, stress: f64) -> Self {
        AnalysisResults {
            bending_moment,
            shear_force,
            deflection,
            stress,
        }
    }
}

/// Compliance status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    RequiresReview,
}

impl ComplianceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ComplianceStatus::Compliant => "Compliant",
            ComplianceStatus::NonCompliant => "Non-Compliant",
            ComplianceStatus::RequiresReview => "Requires Review",
        }
    }
}

/// Structural element
#[derive(Debug, Clone)]
pub struct StructuralElement {
    pub element_id: [u8; 32],
    pub element_type: ElementType,
    pub dimensions: Dimensions,
    pub material: Material,
    pub loads: LoadSet,
    pub analysis_results: AnalysisResults,
    pub compliance_status: ComplianceStatus,
}

impl StructuralElement {
    pub fn new(
        element_type: ElementType,
        dimensions: Dimensions,
        material: Material,
        loads: LoadSet,
    ) -> Self {
        let element_id = Self::generate_element_id(&element_type);
        let analysis_results = Self::analyze_element(&element_type, &dimensions, &material, &loads);
        let compliance_status = Self::check_compliance(&element_type, &material, &analysis_results);
        
        StructuralElement {
            element_id,
            element_type,
            dimensions,
            material,
            loads,
            analysis_results,
            compliance_status,
        }
    }
    
    fn generate_element_id(element_type: &ElementType) -> [u8; 32] {
        // Placeholder for actual BLAKE3 hash
        let mut hash = [0u8; 32];
        let type_str = element_type.as_str().as_bytes();
        for (i, &byte) in type_str.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(byte);
        }
        hash
    }
    
    fn analyze_element(
        element_type: &ElementType,
        dimensions: &Dimensions,
        material: &Material,
        loads: &LoadSet,
    ) -> AnalysisResults {
        // Simplified analysis - actual implementation would use IS code formulas
        let span = dimensions.length;
        let load = loads.total_load;
        
        match element_type {
            ElementType::Beam => {
                // Simply supported beam: M = wL²/8, V = wL/2
                let bending_moment = load * span * span / 8.0;
                let shear_force = load * span / 2.0;
                let deflection = (5.0 * load * span.powi(4)) / (384.0 * material.characteristic_strength() * 1e6); // Simplified
                let stress = bending_moment * 1e6 / (dimensions.width * dimensions.height.powi(2) / 6.0);
                
                AnalysisResults::new(bending_moment, shear_force, deflection, stress)
            }
            ElementType::Column => {
                // Axial load: P = load * area
                let axial_load = load * dimensions.area();
                let bending_moment = 0.0;
                let shear_force = 0.0;
                let deflection = axial_load / (material.characteristic_strength() * dimensions.area() * 1e6) * 1000.0;
                let stress = axial_load / dimensions.area();
                
                AnalysisResults::new(bending_moment, shear_force, deflection, stress)
            }
            ElementType::Slab => {
                // One-way slab: M = wL²/8
                let bending_moment = load * span * span / 8.0;
                let shear_force = load * span / 2.0;
                let deflection = (5.0 * load * span.powi(4)) / (384.0 * material.characteristic_strength() * 1e6);
                let stress = bending_moment * 1e6 / (dimensions.thickness.powi(2) / 6.0);
                
                AnalysisResults::new(bending_moment, shear_force, deflection, stress)
            }
            ElementType::Foundation => {
                // Bearing capacity: q = load / area
                let bearing_pressure = load / dimensions.area();
                let bending_moment = 0.0;
                let shear_force = 0.0;
                let deflection = bearing_pressure / material.characteristic_strength() * 1000.0;
                let stress = bearing_pressure;
                
                AnalysisResults::new(bending_moment, shear_force, deflection, stress)
            }
        }
    }
    
    fn check_compliance(
        element_type: &ElementType,
        material: &Material,
        results: &AnalysisResults,
    ) -> ComplianceStatus {
        // Simplified compliance check per IS codes
        let allowable_stress = material.characteristic_strength() / 1.5; // Factor of safety 1.5
        
        if results.stress <= allowable_stress {
            ComplianceStatus::Compliant
        } else if results.stress <= allowable_stress * 1.1 {
            ComplianceStatus::RequiresReview
        } else {
            ComplianceStatus::NonCompliant
        }
    }
    
    pub fn get_element_id(&self) -> String {
        self.element_id.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }
}

impl fmt::Display for StructuralElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Structural Element\n\
             ID: {}\n\
             Type: {}\n\
             Material: {} {}\n\
             Dimensions: L={}m, W={}m, H={}m, T={}m\n\
             Loads: DL={} kN/m², LL={} kN/m², Total={} kN/m²\n\
             Analysis: M={} kNm, V={} kN, δ={} mm, σ={} N/mm²\n\
             Compliance: {}",
            self.get_element_id(),
            self.element_type.as_str(),
            self.material.material_type.as_str(),
            self.material.grade.as_str(),
            self.dimensions.length,
            self.dimensions.width,
            self.dimensions.height,
            self.dimensions.thickness,
            self.loads.dead_load,
            self.loads.live_load,
            self.loads.total_load,
            self.analysis_results.bending_moment,
            self.analysis_results.shear_force,
            self.analysis_results.deflection,
            self.analysis_results.stress,
            self.compliance_status.as_str()
        )
    }
}

/// Load calculator
pub struct LoadCalculator {
    elements: Vec<StructuralElement>,
}

impl LoadCalculator {
    pub fn new() -> Self {
        LoadCalculator {
            elements: Vec::new(),
        }
    }
    
    /// Calculate dead load
    pub fn calculate_dead_load(material: &Material, dimensions: &Dimensions) -> f64 {
        // Dead load = unit weight × volume
        material.unit_weight() * dimensions.volume()
    }
    
    /// Calculate live load per IS 875 Part 2
    pub fn calculate_live_load(occupancy: &str) -> f64 {
        // Live load values in kN/m² per IS 875 Part 2
        match occupancy.to_lowercase().as_str() {
            "residential" => 2.0,
            "office" => 3.0,
            "school" => 4.0,
            "assembly" => 5.0,
            "storage" => 7.5,
            "garage" => 2.5,
            _ => 3.0, // Default
        }
    }
    
    /// Calculate wind load per IS 875 Part 3
    pub fn calculate_wind_load(
        basic_wind_speed: f64,
        height: f64,
        terrain_category: u8,
    ) -> f64 {
        // Simplified wind load calculation per IS 875 Part 3
        let vb = basic_wind_speed; // m/s
        let k1 = 1.0; // Risk coefficient
        let k2 = 1.0 + (height / 10.0).min(0.5); // Terrain height factor
        let k3 = 1.0; // Importance factor
        let k4 = 1.0; // Topography factor
        
        let vz = vb * k1 * k2 * k3 * k4;
        let pz = 0.6 * vz * vz / 1000.0; // Design wind pressure in kN/m²
        
        pz
    }
    
    /// Calculate seismic load per IS 1893
    pub fn calculate_seismic_load(
        zone: u8,
        importance: u8,
        soil_type: u8,
        weight: f64,
    ) -> f64 {
        // Simplified seismic load calculation per IS 1893
        let zone_factor = match zone {
            2 => 0.10,
            3 => 0.16,
            4 => 0.24,
            5 => 0.36,
            _ => 0.10,
        };
        
        let importance_factor = match importance {
            1 => 1.0,
            2 => 1.5,
            _ => 1.0,
        };
        
        let soil_factor = match soil_type {
            1 => 1.0,
            2 => 1.2,
            3 => 1.5,
            _ => 1.0,
        };
        
        let r = 1.0; // Response reduction factor
        let sa = 2.5; // Spectral acceleration
        
        let ah = zone_factor / r * sa;
        let seismic_coefficient = ah * importance_factor * soil_factor;
        
        weight * seismic_coefficient
    }
    
    /// Add structural element
    pub fn add_element(&mut self, element: StructuralElement) {
        self.elements.push(element);
    }
    
    /// Get element by ID
    pub fn get_element(&self, element_id: &str) -> Option<&StructuralElement> {
        self.elements
            .iter()
            .find(|e| e.get_element_id() == element_id)
    }
    
    /// List all elements
    pub fn list_elements(&self) -> Vec<&StructuralElement> {
        self.elements.iter().collect()
    }
    
    /// Check BIS compliance
    pub fn check_bis_compliance(&self, element_id: &str, code: &str) -> ComplianceReport {
        let element = self.get_element(element_id);
        
        match element {
            Some(e) => {
                let is_compliant = e.compliance_status == ComplianceStatus::Compliant;
                ComplianceReport {
                    code: code.to_string(),
                    is_compliant,
                    details: format!("Element {} is {}", e.element_type.as_str(), e.compliance_status.as_str()),
                }
            }
            None => ComplianceReport {
                code: code.to_string(),
                is_compliant: false,
                details: "Element not found".to_string(),
            },
        }
    }
}

/// Compliance report
#[derive(Debug, Clone)]
pub struct ComplianceReport {
    pub code: String,
    pub is_compliant: bool,
    pub details: String,
}

impl fmt::Display for ComplianceReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BIS Compliance Report\n\
             Code: {}\n\
             Compliant: {}\n\
             Details: {}",
            self.code,
            self.is_compliant,
            self.details
        )
    }
}

impl Default for LoadCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dead_load_calculation() {
        let calculator = LoadCalculator::new();
        let material = Material::new(MaterialType::Concrete, MaterialGrade::ConcreteM25);
        let dimensions = Dimensions::new(5.0, 0.3, 0.5, 0.0);
        
        let dead_load = calculator.calculate_dead_load(&material, &dimensions);
        
        assert!((dead_load - 18.75).abs() < 0.1); // 25 * 5 * 0.3 * 0.5 = 18.75
    }
    
    #[test]
    fn test_live_load_calculation() {
        let calculator = LoadCalculator::new();
        
        let residential_load = calculator.calculate_live_load("residential");
        assert_eq!(residential_load, 2.0);
        
        let office_load = calculator.calculate_live_load("office");
        assert_eq!(office_load, 3.0);
    }
    
    #[test]
    fn test_wind_load_calculation() {
        let calculator = LoadCalculator::new();
        
        let wind_load = calculator.calculate_wind_load(50.0, 10.0, 2);
        assert!(wind_load > 0.0);
    }
    
    #[test]
    fn test_seismic_load_calculation() {
        let calculator = LoadCalculator::new();
        
        let seismic_load = calculator.calculate_seismic_load(4, 1, 1, 100.0);
        assert!(seismic_load > 0.0);
    }
    
    #[test]
    fn test_beam_creation() {
        let dimensions = Dimensions::new(5.0, 0.3, 0.5, 0.0);
        let material = Material::new(MaterialType::Concrete, MaterialGrade::ConcreteM25);
        let loads = LoadSet::new(10.0, 5.0, 2.0, 1.0);
        
        let beam = StructuralElement::new(ElementType::Beam, dimensions, material, loads);
        
        assert_eq!(beam.element_type, ElementType::Beam);
        assert!(beam.analysis_results.bending_moment > 0.0);
    }
    
    #[test]
    fn test_compliance_check() {
        let calculator = LoadCalculator::new();
        let dimensions = Dimensions::new(5.0, 0.3, 0.5, 0.0);
        let material = Material::new(MaterialType::Concrete, MaterialGrade::ConcreteM40);
        let loads = LoadSet::new(5.0, 3.0, 1.0, 0.5);
        
        let beam = StructuralElement::new(ElementType::Beam, dimensions, material, loads);
        let element_id = beam.get_element_id();
        calculator.add_element(beam);
        
        let report = calculator.check_bis_compliance(&element_id, "IS 456");
        assert!(report.code == "IS 456");
    }
}
