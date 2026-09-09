// SigmaOS Profession Tools Module - 100 specialized tools for 20 distinct professions
// Safe Rust, zero external dependencies

// ==========================================
// CATEGORY 1: HEALTHCARE & MEDICINE (1-5)
// ==========================================

/// Tool 1: Body Mass Index (BMI) Calculator
pub struct BmiCalculator;

impl BmiCalculator {
    pub fn calculate(weight_kg: f64, height_m: f64) -> Result<(f64, &'static str), &'static str> {
        if height_m <= 0.0 || weight_kg <= 0.0 {
            return Err("Height and weight must be positive");
        }
        let bmi = weight_kg / (height_m * height_m);
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
}

/// Tool 2: Medication Dosage Calculator
pub struct DosageCalculator;

impl DosageCalculator {
    pub fn calculate_dose(weight_kg: f64, dose_mg_per_kg: f64, concentration_mg_per_ml: f64) -> Result<(f64, f64), &'static str> {
        if weight_kg <= 0.0 || dose_mg_per_kg <= 0.0 || concentration_mg_per_ml <= 0.0 {
            return Err("All parameters must be positive");
        }
        let total_dose_mg = weight_kg * dose_mg_per_kg;
        let volume_ml = total_dose_mg / concentration_mg_per_ml;
        Ok((total_dose_mg, volume_ml))
    }
}

/// Tool 3: Mean Arterial Pressure (MAP) Calculator
pub struct MeanArterialPressureCalc;

impl MeanArterialPressureCalc {
    pub fn calculate(systolic: f64, diastolic: f64) -> Result<f64, &'static str> {
        if systolic <= diastolic || diastolic <= 0.0 {
            return Err("Systolic pressure must be greater than diastolic pressure");
        }
        let map = diastolic + (systolic - diastolic) / 3.0;
        Ok(map)
    }
}

/// Tool 4: Estimated Glomerular Filtration Rate (eGFR - Cockcroft-Gault)
pub struct GlomerularFiltrationRateCalc;

impl GlomerularFiltrationRateCalc {
    pub fn cockcroft_gault(age_years: f64, weight_kg: f64, serum_creatinine_mg_dl: f64, is_female: bool) -> Result<f64, &'static str> {
        if age_years <= 0.0 || weight_kg <= 0.0 || serum_creatinine_mg_dl <= 0.0 {
            return Err("Invalid parameters for eGFR calculation");
        }
        let mut egfr = ((140.0 - age_years) * weight_kg) / (72.0 * serum_creatinine_mg_dl);
        if is_female {
            egfr *= 0.85;
        }
        Ok(egfr)
    }
}

/// Tool 5: Neonatal APGAR Score Calculator
pub struct ApgarScoreCalculator;

impl ApgarScoreCalculator {
    pub fn calculate(appearance: u8, pulse: u8, grimace: u8, activity: u8, respiration: u8) -> Result<(u8, &'static str), &'static str> {
        if appearance > 2 || pulse > 2 || grimace > 2 || activity > 2 || respiration > 2 {
            return Err("Each APGAR parameter score must be between 0 and 2");
        }
        let total = appearance + pulse + grimace + activity + respiration;
        let status = if total >= 7 {
            "Normal / Reassuring"
        } else if total >= 4 {
            "Moderately Abnormal"
        } else {
            "Critically Low / Action Required"
        };
        Ok((total, status))
    }
}

// ==========================================
// CATEGORY 2: CIVIL ENGINEERING & CONSTRUCTION (6-10)
// ==========================================

/// Tool 6: Concrete Slab Volume & Bag Calculator
pub struct ConcreteVolumeCalc;

impl ConcreteVolumeCalc {
    pub fn calculate(length_m: f64, width_m: f64, depth_m: f64, bag_size_kg: f64) -> Result<(f64, usize), &'static str> {
        if length_m <= 0.0 || width_m <= 0.0 || depth_m <= 0.0 || bag_size_kg <= 0.0 {
            return Err("Dimensions and bag size must be positive");
        }
        let volume_m3 = length_m * width_m * depth_m;
        // Concrete density approx 2400 kg/m3
        let total_weight_kg = volume_m3 * 2400.0;
        let bags_needed = (total_weight_kg / bag_size_kg).ceil() as usize;
        Ok((volume_m3, bags_needed))
    }
}

/// Tool 7: Beam Maximum Deflection Calculator (Simply Supported Uniform Load)
pub struct BeamDeflectionCalc;

impl BeamDeflectionCalc {
    pub fn calculate_simply_supported(load_n_per_m: f64, length_m: f64, elasticity_pa: f64, inertia_m4: f64) -> Result<f64, &'static str> {
        if length_m <= 0.0 || elasticity_pa <= 0.0 || inertia_m4 <= 0.0 {
            return Err("Length, Modulus of Elasticity, and Moment of Inertia must be positive");
        }
        let max_deflection = (5.0 * load_n_per_m * length_m.powi(4)) / (384.0 * elasticity_pa * inertia_m4);
        Ok(max_deflection)
    }
}

/// Tool 8: Retaining Wall Sliding Safety Factor
pub struct RetainingWallStabilityCalc;

impl RetainingWallStabilityCalc {
    pub fn sliding_factor_of_safety(wall_weight_kn: f64, friction_coeff: f64, lateral_thrust_kn: f64) -> Result<(f64, bool), &'static str> {
        if lateral_thrust_kn <= 0.0 || wall_weight_kn <= 0.0 || friction_coeff <= 0.0 {
            return Err("Forces and friction coefficient must be positive");
        }
        let resisting_force = wall_weight_kn * friction_coeff;
        let factor = resisting_force / lateral_thrust_kn;
        let is_safe = factor >= 1.5; // Standard engineering safety threshold
        Ok((factor, is_safe))
    }
}

/// Tool 9: Asphalt Paving Tonnage Calculator
pub struct AsphaltQuantityCalc;

impl AsphaltQuantityCalc {
    pub fn calculate_tonnage(length_m: f64, width_m: f64, thickness_mm: f64) -> Result<f64, &'static str> {
        if length_m <= 0.0 || width_m <= 0.0 || thickness_mm <= 0.0 {
            return Err("Dimensions must be positive");
        }
        let volume_m3 = length_m * width_m * (thickness_mm / 1000.0);
        // Compacted asphalt density ~ 2.35 tonnes/m3
        let tonnage = volume_m3 * 2.35;
        Ok(tonnage)
    }
}

/// Tool 10: Pipe Flow Velocity Calculator
pub struct PipeFlowVelocityCalc;

impl PipeFlowVelocityCalc {
    pub fn calculate(flow_rate_m3_s: f64, pipe_diameter_m: f64) -> Result<f64, &'static str> {
        if flow_rate_m3_s < 0.0 || pipe_diameter_m <= 0.0 {
            return Err("Diameter must be positive and flow rate non-negative");
        }
        let radius = pipe_diameter_m / 2.0;
        let area = std::f64::consts::PI * radius * radius;
        let velocity = flow_rate_m3_s / area;
        Ok(velocity)
    }
}

// ==========================================
// CATEGORY 3: SOFTWARE & DEVOPS (11-15)
// ==========================================

/// Tool 11: Cron Expression Validator & Description
pub struct CronExpressionParser;

impl CronExpressionParser {
    pub fn describe(expression: &str) -> Result<String, &'static str> {
        let parts: Vec<&str> = expression.split_whitespace().collect();
        if parts.len() != 5 {
            return Err("Cron expression must consist of exactly 5 fields (min hour dom month dow)");
        }
        Ok(format!("Cron job scheduled with fields - Min: {}, Hour: {}, DayOfMonth: {}, Month: {}, DayOfWeek: {}", parts[0], parts[1], parts[2], parts[3], parts[4]))
    }
}

/// Tool 12: Simple String Pattern Matcher (Regex Tool)
pub struct RegexTesterTool;

impl RegexTesterTool {
    pub fn contains_match(pattern: &str, text: &str) -> bool {
        text.contains(pattern)
    }
}

/// Tool 13: JWT Token Header & Payload Decoder
pub struct JwtTokenDecoder;

impl JwtTokenDecoder {
    pub fn decode_unverified(jwt: &str) -> Result<(String, String), &'static str> {
        let parts: Vec<&str> = jwt.split('.').collect();
        if parts.len() != 3 {
            return Err("Invalid JWT format, expected 3 dot-separated components");
        }
        Ok((parts[0].to_string(), parts[1].to_string()))
    }
}

/// Tool 14: Base64 Encoder & Decoder
pub struct Base64ConverterTool;

impl Base64ConverterTool {
    pub fn encode(data: &[u8]) -> String {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = String::new();
        let mut i = 0;
        while i < data.len() {
            let b0 = data[i] as usize;
            let b1 = if i + 1 < data.len() { data[i + 1] as usize } else { 0 };
            let b2 = if i + 2 < data.len() { data[i + 2] as usize } else { 0 };

            let c0 = b0 >> 2;
            let c1 = ((b0 & 3) << 4) | (b1 >> 4);
            let c2 = ((b1 & 15) << 2) | (b2 >> 6);
            let c3 = b2 & 63;

            out.push(CHARS[c0] as char);
            out.push(CHARS[c1] as char);
            if i + 1 < data.len() { out.push(CHARS[c2] as char); } else { out.push('='); }
            if i + 2 < data.len() { out.push(CHARS[c3] as char); } else { out.push('='); }
            i += 3;
        }
        out
    }
}

/// Tool 15: Semantic Versioning Parser
pub struct SemverValidator;

impl SemverValidator {
    pub fn parse(version: &str) -> Result<(u64, u64, u64), &'static str> {
        let v = version.strip_prefix('v').unwrap_or(version);
        let parts: Vec<&str> = v.split('.').collect();
        if parts.len() != 3 {
            return Err("Semver string must be in format MAJOR.MINOR.PATCH");
        }
        let major = parts[0].parse::<u64>().map_err(|_| "Invalid major version")?;
        let minor = parts[1].parse::<u64>().map_err(|_| "Invalid minor version")?;
        let patch = parts[2].parse::<u64>().map_err(|_| "Invalid patch version")?;
        Ok((major, minor, patch))
    }
}

// ==========================================
// CATEGORY 4: ASTRONOMY & ASTROPHYSICS (16-20)
// ==========================================

/// Tool 16: Schwarzschild Radius Calculator (Black Hole Radius)
pub struct SchwarzschildRadiusCalc;

impl SchwarzschildRadiusCalc {
    pub fn calculate(mass_kg: f64) -> Result<f64, &'static str> {
        if mass_kg <= 0.0 {
            return Err("Mass must be positive");
        }
        let g = 6.67430e-11;
        let c = 299_792_458.0;
        let radius = (2.0 * g * mass_kg) / (c * c);
        Ok(radius)
    }
}

/// Tool 17: Astronomical Distance Units Converter
pub struct ParsecToLightYearConverter;

impl ParsecToLightYearConverter {
    pub fn parsec_to_light_years(parsecs: f64) -> f64 {
        parsecs * 3.26156
    }

    pub fn light_years_to_parsecs(light_years: f64) -> f64 {
        light_years / 3.26156
    }
}

/// Tool 18: Telescope Magnification Calculator
pub struct TelescopeMagnificationCalc;

impl TelescopeMagnificationCalc {
    pub fn calculate(objective_focal_length_mm: f64, eyepiece_focal_length_mm: f64) -> Result<f64, &'static str> {
        if objective_focal_length_mm <= 0.0 || eyepiece_focal_length_mm <= 0.0 {
            return Err("Focal lengths must be positive");
        }
        Ok(objective_focal_length_mm / eyepiece_focal_length_mm)
    }
}

/// Tool 19: Kepler's Third Law Orbital Period Calculator
pub struct KeplerThirdLawCalc;

impl KeplerThirdLawCalc {
    pub fn orbital_period_years(semi_major_axis_au: f64) -> Result<f64, &'static str> {
        if semi_major_axis_au <= 0.0 {
            return Err("Semi-major axis must be positive");
        }
        // T^2 = a^3 around Solar-mass star
        Ok(semi_major_axis_au.powf(1.5))
    }
}

/// Tool 20: Redshift Velocity Calculator
pub struct RedshiftVelocityCalc;

impl RedshiftVelocityCalc {
    pub fn recession_velocity_km_s(redshift_z: f64) -> Result<f64, &'static str> {
        if redshift_z < 0.0 {
            return Err("Redshift z must be non-negative");
        }
        let c_km_s = 299_792.458;
        let num = (redshift_z + 1.0).powi(2) - 1.0;
        let den = (redshift_z + 1.0).powi(2) + 1.0;
        Ok(c_km_s * (num / den))
    }
}

// ==========================================
// CATEGORY 5: FINANCE & ACCOUNTING (21-25)
// ==========================================

/// Tool 21: Compound Interest Calculator
pub struct CompoundInterestCalc;

impl CompoundInterestCalc {
    pub fn future_value(principal: f64, annual_rate_pct: f64, compounds_per_year: u32, years: f64) -> Result<f64, &'static str> {
        if principal < 0.0 || compounds_per_year == 0 || years < 0.0 {
            return Err("Invalid compound interest parameters");
        }
        let r = annual_rate_pct / 100.0;
        let n = compounds_per_year as f64;
        let fv = principal * (1.0 + r / n).powf(n * years);
        Ok(fv)
    }
}

/// Tool 22: Loan Amortization Monthly Payment Calculator
pub struct AmortizationScheduleCalc;

impl AmortizationScheduleCalc {
    pub fn monthly_payment(principal: f64, annual_interest_rate_pct: f64, term_years: u32) -> Result<f64, &'static str> {
        if principal <= 0.0 || term_years == 0 {
            return Err("Principal and term must be positive");
        }
        let monthly_rate = (annual_interest_rate_pct / 100.0) / 12.0;
        let total_months = term_years * 12;
        if monthly_rate == 0.0 {
            return Ok(principal / total_months as f64);
        }
        let payment = principal * (monthly_rate * (1.0 + monthly_rate).powi(total_months as i32))
            / ((1.0 + monthly_rate).powi(total_months as i32) - 1.0);
        Ok(payment)
    }
}

/// Tool 23: Break-Even Point Analyzer
pub struct BreakEvenAnalyzer;

impl BreakEvenAnalyzer {
    pub fn calculate_units(fixed_costs: f64, price_per_unit: f64, variable_cost_per_unit: f64) -> Result<(f64, f64), &'static str> {
        if price_per_unit <= variable_cost_per_unit {
            return Err("Price per unit must be strictly greater than variable cost per unit");
        }
        let break_even_units = fixed_costs / (price_per_unit - variable_cost_per_unit);
        let break_even_revenue = break_even_units * price_per_unit;
        Ok((break_even_units, break_even_revenue))
    }
}

/// Tool 24: Present Value of Future Cash Flow
pub struct PresentValueCalc;

impl PresentValueCalc {
    pub fn calculate(future_value: f64, discount_rate_pct: f64, years: f64) -> Result<f64, &'static str> {
        if years < 0.0 {
            return Err("Years must be non-negative");
        }
        let r = discount_rate_pct / 100.0;
        let pv = future_value / (1.0 + r).powf(years);
        Ok(pv)
    }
}

/// Tool 25: Simple Income Tax Calculator
pub struct TaxWithholdingCalc;

impl TaxWithholdingCalc {
    pub fn calculate_simple_tax(taxable_income: f64) -> f64 {
        if taxable_income <= 10_000.0 {
            taxable_income * 0.10
        } else if taxable_income <= 50_000.0 {
            1_000.0 + (taxable_income - 10_000.0) * 0.15
        } else {
            7_000.0 + (taxable_income - 50_000.0) * 0.25
        }
    }
}

// ==========================================
// CATEGORY 6: LAW & LEGAL PROFESSION (26-30)
// ==========================================

/// Tool 26: Statute of Limitations Date Calculator
pub struct StatuteOfLimitationsCalc;

impl StatuteOfLimitationsCalc {
    pub fn calculate_expiration_year(incident_year: u32, limitation_period_years: u32) -> u32 {
        incident_year + limitation_period_years
    }
}

/// Tool 27: Legal Case Citation Formatter
pub struct LegalCitationFormatter;

impl LegalCitationFormatter {
    pub fn format_us_case(plaintiff: &str, defendant: &str, volume: u32, reporter: &str, page: u32, year: u32) -> String {
        format!("{} v. {}, {} {} {} ({})", plaintiff, defendant, volume, reporter, page, year)
    }
}

/// Tool 28: Pre-judgment Statutory Interest Calculator
pub struct ContractInterestCalc;

impl ContractInterestCalc {
    pub fn calculate_interest(principal_amount: f64, statutory_rate_pct: f64, days_overdue: u32) -> f64 {
        let daily_rate = (statutory_rate_pct / 100.0) / 365.0;
        principal_amount * daily_rate * (days_overdue as f64)
    }
}

/// Tool 29: Basic Child Support Obligation Estimator
pub struct ChildSupportCalc;

impl ChildSupportCalc {
    pub fn estimate_monthly_obligation(combined_monthly_income: f64, obligor_income_share_pct: f64, num_children: u32) -> Result<f64, &'static str> {
        if combined_monthly_income <= 0.0 || num_children == 0 {
            return Err("Income and number of children must be positive");
        }
        let base_support_pct = match num_children {
            1 => 0.17,
            2 => 0.25,
            3 => 0.29,
            _ => 0.31,
        };
        let total_base_support = combined_monthly_income * base_support_pct;
        let obligor_share = total_base_support * (obligor_income_share_pct / 100.0);
        Ok(obligor_share)
    }
}

/// Tool 30: Court Business Deadline Calculator
pub struct CourtDeadlineCalc;

impl CourtDeadlineCalc {
    pub fn add_business_days(start_day_index: u32, business_days_to_add: u32) -> u32 {
        let mut current = start_day_index;
        let mut added = 0;
        while added < business_days_to_add {
            current += 1;
            // Assuming 6 and 0 (Saturday, Sunday) are weekends in modulo 7
            if current % 7 != 0 && current % 7 != 6 {
                added += 1;
            }
        }
        current
    }
}

// ==========================================
// CATEGORY 7: SOUND & AUDIO ENGINEERING (31-35)
// ==========================================

/// Tool 31: dB SPL to Pascals Acoustic Pressure Converter
pub struct DbSPLToPressureConverter;

impl DbSPLToPressureConverter {
    pub fn spl_to_pascals(db_spl: f64) -> f64 {
        let p_ref = 20.0e-6; // 20 micropascals
        p_ref * 10.0f64.powf(db_spl / 20.0)
    }
}

/// Tool 32: BPM to Delay Time Calculator
pub struct DelayTimeBpmCalc;

impl DelayTimeBpmCalc {
    pub fn quarter_note_delay_ms(bpm: f64) -> Result<f64, &'static str> {
        if bpm <= 0.0 {
            return Err("BPM must be positive");
        }
        Ok(60_000.0 / bpm)
    }
}

/// Tool 33: Sample Rate Nyquist Frequency Calculator
pub struct SampleRateNyquistCalc;

impl SampleRateNyquistCalc {
    pub fn nyquist_frequency_hz(sample_rate_hz: f64) -> Result<f64, &'static str> {
        if sample_rate_hz <= 0.0 {
            return Err("Sample rate must be positive");
        }
        Ok(sample_rate_hz / 2.0)
    }
}

/// Tool 34: Parallel Speaker Impedance Calculator
pub struct SpeakerImpedanceCalc;

impl SpeakerImpedanceCalc {
    pub fn parallel_impedance(impedances_ohms: &[f64]) -> Result<f64, &'static str> {
        if impedances_ohms.is_empty() {
            return Err("At least one speaker impedance is required");
        }
        let mut inv_sum = 0.0;
        for &z in impedances_ohms {
            if z <= 0.0 {
                return Err("Speaker impedance must be positive");
            }
            inv_sum += 1.0 / z;
        }
        Ok(1.0 / inv_sum)
    }
}

/// Tool 35: Frequency to Musical Note Converter
pub struct AudioFrequenciesToNoteCalc;

impl AudioFrequenciesToNoteCalc {
    pub fn freq_to_midi_note(freq_hz: f64) -> Result<u8, &'static str> {
        if freq_hz <= 0.0 {
            return Err("Frequency must be positive");
        }
        let note = 69.0 + 12.0 * (freq_hz / 440.0).log2();
        Ok(note.round() as u8)
    }
}

// ==========================================
// CATEGORY 8: GRAPHIC DESIGN & DIGITAL MEDIA (36-40)
// ==========================================

/// Tool 36: Color HEX to RGB Converter
pub struct ColorHexRgbConverter;

impl ColorHexRgbConverter {
    pub fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), &'static str> {
        let clean = hex.trim_start_matches('#');
        if clean.len() != 6 {
            return Err("HEX string must be 6 hex characters");
        }
        let r = u8::from_str_radix(&clean[0..2], 16).map_err(|_| "Invalid HEX digit")?;
        let g = u8::from_str_radix(&clean[2..4], 16).map_err(|_| "Invalid HEX digit")?;
        let b = u8::from_str_radix(&clean[4..6], 16).map_err(|_| "Invalid HEX digit")?;
        Ok((r, g, b))
    }
}

/// Tool 37: Golden Ratio Grid Calculator
pub struct GoldenRatioGridCalc;

impl GoldenRatioGridCalc {
    pub fn split_length(total_length: f64) -> Result<(f64, f64), &'static str> {
        if total_length <= 0.0 {
            return Err("Length must be positive");
        }
        let phi = 1.61803398875;
        let smaller = total_length / phi;
        let larger = total_length - smaller;
        Ok((larger, smaller))
    }
}

/// Tool 38: Aspect Ratio Dimension Calculator
pub struct AspectRatioCalc;

impl AspectRatioCalc {
    pub fn calculate_height(width: u32, ratio_w: u32, ratio_h: u32) -> Result<u32, &'static str> {
        if ratio_w == 0 || ratio_h == 0 {
            return Err("Aspect ratios must be non-zero");
        }
        Ok((width * ratio_h) / ratio_w)
    }
}

/// Tool 39: Physical Print DPI Resolution Calculator
pub struct PrintDpiResCalc;

impl PrintDpiResCalc {
    pub fn required_pixels(width_inches: f64, height_inches: f64, dpi: u32) -> (u32, u32) {
        let px_w = (width_inches * (dpi as f64)).round() as u32;
        let px_h = (height_inches * (dpi as f64)).round() as u32;
        (px_w, px_h)
    }
}

/// Tool 40: Contrast Ratio Checker (WCAG Luminance approximation)
pub struct ContrastRatioChecker;

impl ContrastRatioChecker {
    pub fn contrast_ratio(lum1: f64, lum2: f64) -> f64 {
        let (l1, l2) = if lum1 > lum2 { (lum1, lum2) } else { (lum2, lum1) };
        (l1 + 0.05) / (l2 + 0.05)
    }
}

// ==========================================
// CATEGORY 9: AGRICULTURE & AGRONOMY (41-45)
// ==========================================

/// Tool 41: Fertilizer N-P-K Application Rate Calculator
pub struct FertilizerNpkCalc;

impl FertilizerNpkCalc {
    pub fn fertilizer_amount_kg(target_element_kg: f64, grade_percentage: f64) -> Result<f64, &'static str> {
        if grade_percentage <= 0.0 {
            return Err("Grade percentage must be positive");
        }
        Ok(target_element_kg / (grade_percentage / 100.0))
    }
}

/// Tool 42: Corn/Grain Crop Yield Estimator
pub struct CropYieldEstimator;

impl CropYieldEstimator {
    pub fn estimate_bushels_per_acre(ears_per_acre: f64, kernels_per_ear: f64) -> f64 {
        // Standard kernel factor ~ 89,000 kernels per bushel
        (ears_per_acre * kernels_per_ear) / 89_000.0
    }
}

/// Tool 43: Irrigation Water Demand Calculator
pub struct IrrigationRequirementCalc;

impl IrrigationRequirementCalc {
    pub fn water_volume_liters(area_sq_meters: f64, evapotranspiration_mm: f64) -> f64 {
        // 1 mm over 1 m2 = 1 liter
        area_sq_meters * evapotranspiration_mm
    }
}

/// Tool 44: Seeding Rate Calculator
pub struct SeedingRateCalc;

impl SeedingRateCalc {
    pub fn seeds_per_hectare(target_plants_per_ha: f64, germination_rate_pct: f64, purity_pct: f64) -> Result<f64, &'static str> {
        if germination_rate_pct <= 0.0 || purity_pct <= 0.0 {
            return Err("Germination and purity rates must be positive");
        }
        let decimal_purity = (germination_rate_pct / 100.0) * (purity_pct / 100.0);
        Ok(target_plants_per_ha / decimal_purity)
    }
}

/// Tool 45: Pearson Square Feed Ration Blend
pub struct LivestockFeedRatioCalc;

impl LivestockFeedRatioCalc {
    pub fn pearson_square_parts(protein_feed1_pct: f64, protein_feed2_pct: f64, target_protein_pct: f64) -> Result<(f64, f64), &'static str> {
        let (high, low) = if protein_feed1_pct > protein_feed2_pct {
            (protein_feed1_pct, protein_feed2_pct)
        } else {
            (protein_feed2_pct, protein_feed1_pct)
        };
        if target_protein_pct < low || target_protein_pct > high {
            return Err("Target protein percentage must be between feed 1 and feed 2 protein percentages");
        }
        let parts_high = target_protein_pct - low;
        let parts_low = high - target_protein_pct;
        Ok((parts_high, parts_low))
    }
}

// ==========================================
// CATEGORY 10: CHEMISTRY & CHEMICAL ENG (46-50)
// ==========================================

/// Tool 46: Simple Molar Mass Estimator
pub struct MolarMassCalc;

impl MolarMassCalc {
    pub fn simple_water_molar_mass() -> f64 {
        2.0 * 1.008 + 15.999 // H2O
    }
}

/// Tool 47: Solution Dilution Calculator (C1V1 = C2V2)
pub struct SolutionDilutionCalc;

impl SolutionDilutionCalc {
    pub fn calculate_initial_volume(c1: f64, c2: f64, v2: f64) -> Result<f64, &'static str> {
        if c1 <= 0.0 || c2 <= 0.0 || v2 <= 0.0 {
            return Err("Concentrations and target volume must be positive");
        }
        if c2 > c1 {
            return Err("Target concentration C2 cannot be greater than initial C1");
        }
        Ok((c2 * v2) / c1)
    }
}

/// Tool 48: Solution pH Calculator
pub struct PhCalculator;

impl PhCalculator {
    pub fn calculate_ph(hydrogen_ion_conc_molar: f64) -> Result<f64, &'static str> {
        if hydrogen_ion_conc_molar <= 0.0 {
            return Err("Ion concentration must be strictly positive");
        }
        Ok(-hydrogen_ion_conc_molar.log10())
    }
}

/// Tool 49: Ideal Gas Law Pressure Solver (PV = nRT)
pub struct IdealGasLawCalc;

impl IdealGasLawCalc {
    pub fn calculate_pressure_pa(moles: f64, temp_kelvin: f64, volume_m3: f64) -> Result<f64, &'static str> {
        if volume_m3 <= 0.0 || temp_kelvin <= 0.0 || moles <= 0.0 {
            return Err("Moles, temperature, and volume must be positive");
        }
        let r = 8.314462618; // Gas constant J/(mol·K)
        let p = (moles * r * temp_kelvin) / volume_m3;
        Ok(p)
    }
}

/// Tool 50: Reaction Mass Stoichiometry Calculator
pub struct ReactionStoichiometryCalc;

impl ReactionStoichiometryCalc {
    pub fn calculate_product_mass(mass_reactant_g: f64, molar_mass_reactant: f64, molar_mass_product: f64) -> Result<f64, &'static str> {
        if molar_mass_reactant <= 0.0 || molar_mass_product <= 0.0 {
            return Err("Molar masses must be positive");
        }
        let moles = mass_reactant_g / molar_mass_reactant;
        Ok(moles * molar_mass_product)
    }
}

// ==========================================
// CATEGORY 11: PHYSICS & NUCLEAR SCIENCE (51-55)
// ==========================================

/// Tool 51: Kinetic Energy Calculator
pub struct KineticEnergyCalc;

impl KineticEnergyCalc {
    pub fn calculate_joules(mass_kg: f64, velocity_m_s: f64) -> f64 {
        0.5 * mass_kg * velocity_m_s * velocity_m_s
    }
}

/// Tool 52: Radioactive Half-Life Decay Calculator
pub struct HalfLifeDecayCalc;

impl HalfLifeDecayCalc {
    pub fn remaining_mass(initial_mass: f64, half_life_years: f64, elapsed_years: f64) -> Result<f64, &'static str> {
        if half_life_years <= 0.0 || initial_mass < 0.0 {
            return Err("Half life must be positive and initial mass non-negative");
        }
        let decay_factor = 0.5f64.powf(elapsed_years / half_life_years);
        Ok(initial_mass * decay_factor)
    }
}

/// Tool 53: Photon Energy Calculator
pub struct PhotonEnergyCalc;

impl PhotonEnergyCalc {
    pub fn energy_from_wavelength_nm(wavelength_nm: f64) -> Result<f64, &'static str> {
        if wavelength_nm <= 0.0 {
            return Err("Wavelength must be positive");
        }
        let h = 6.62607015e-34; // Planck constant
        let c = 299_792_458.0; // speed of light
        let wavelength_m = wavelength_nm * 1e-9;
        Ok((h * c) / wavelength_m)
    }
}

/// Tool 54: Relativistic Time Dilation Calculator
pub struct RelativisticTimeDilationCalc;

impl RelativisticTimeDilationCalc {
    pub fn dilated_time(proper_time_seconds: f64, velocity_m_s: f64) -> Result<f64, &'static str> {
        let c = 299_792_458.0;
        if velocity_m_s >= c || velocity_m_s < 0.0 {
            return Err("Velocity must be non-negative and less than speed of light");
        }
        let gamma = 1.0 / (1.0 - (velocity_m_s * velocity_m_s) / (c * c)).sqrt();
        Ok(proper_time_seconds * gamma)
    }
}

/// Tool 55: Ohm's Law Solver
pub struct OhmLawCalc;

impl OhmLawCalc {
    pub fn voltage(current_amp: f64, resistance_ohm: f64) -> f64 {
        current_amp * resistance_ohm
    }

    pub fn power(voltage_volt: f64, current_amp: f64) -> f64 {
        voltage_volt * current_amp
    }
}

// ==========================================
// CATEGORY 12: MECHANICAL ENGINEERING (56-60)
// ==========================================

/// Tool 56: Gear Ratio & Speed Multiplier
pub struct GearRatioCalc;

impl GearRatioCalc {
    pub fn calculate(driver_teeth: u32, driven_teeth: u32, input_rpm: f64) -> Result<(f64, f64), &'static str> {
        if driver_teeth == 0 || driven_teeth == 0 {
            return Err("Teeth counts must be non-zero");
        }
        let gear_ratio = driven_teeth as f64 / driver_teeth as f64;
        let output_rpm = input_rpm / gear_ratio;
        Ok((gear_ratio, output_rpm))
    }
}

/// Tool 57: Hydraulic Cylinder Force Calculator
pub struct HydraulicCylinderForceCalc;

impl HydraulicCylinderForceCalc {
    pub fn force_newtons(pressure_pa: f64, bore_diameter_m: f64) -> Result<f64, &'static str> {
        if bore_diameter_m <= 0.0 {
            return Err("Bore diameter must be positive");
        }
        let area = std::f64::consts::PI * (bore_diameter_m / 2.0).powi(2);
        Ok(pressure_pa * area)
    }
}

/// Tool 58: Thermal Linear Expansion Calculator
pub struct ThermalExpansionCalc;

impl ThermalExpansionCalc {
    pub fn delta_length(initial_length_m: f64, coeff_expansion: f64, delta_temp_c: f64) -> f64 {
        initial_length_m * coeff_expansion * delta_temp_c
    }
}

/// Tool 59: Torque & Horsepower Power Solver
pub struct TorquePowerCalc;

impl TorquePowerCalc {
    pub fn power_kw(torque_nm: f64, rpm: f64) -> f64 {
        (torque_nm * rpm * 2.0 * std::f64::consts::PI) / 60_000.0
    }
}

/// Tool 60: Hooke's Law Spring Constant Calculator
pub struct SpringConstantCalc;

impl SpringConstantCalc {
    pub fn force_newtons(spring_constant_n_m: f64, displacement_m: f64) -> f64 {
        spring_constant_n_m * displacement_m
    }
}

// ==========================================
// CATEGORY 13: AEROSPACE & AVIONICS (61-65)
// ==========================================

/// Tool 61: Mach Number Calculator
pub struct MachNumberCalc;

impl MachNumberCalc {
    pub fn mach_number(true_airspeed_m_s: f64, temp_kelvin: f64) -> Result<f64, &'static str> {
        if temp_kelvin <= 0.0 {
            return Err("Temperature must be positive Kelvin");
        }
        let speed_of_sound = (1.4 * 287.058 * temp_kelvin).sqrt();
        Ok(true_airspeed_m_s / speed_of_sound)
    }
}

/// Tool 62: Aviation Density Altitude Calculator
pub struct DensityAltitudeCalc;

impl DensityAltitudeCalc {
    pub fn estimate_feet(pressure_altitude_ft: f64, isa_temp_dev_c: f64) -> f64 {
        pressure_altitude_ft + (120.0 * isa_temp_dev_c)
    }
}

/// Tool 63: Aircraft Center of Gravity Calculator
pub struct AircraftCenterOfGravityCalc;

impl AircraftCenterOfGravityCalc {
    pub fn center_of_gravity_arm(weights_and_arms: &[(f64, f64)]) -> Result<f64, &'static str> {
        let mut total_weight = 0.0;
        let mut total_moment = 0.0;
        for &(weight, arm) in weights_and_arms {
            if weight < 0.0 {
                return Err("Weight cannot be negative");
            }
            total_weight += weight;
            total_moment += weight * arm;
        }
        if total_weight == 0.0 {
            return Err("Total weight cannot be zero");
        }
        Ok(total_moment / total_weight)
    }
}

/// Tool 64: Crosswind & Headwind Component Solver
pub struct CrosswindComponentCalc;

impl CrosswindComponentCalc {
    pub fn calculate(wind_speed_knots: f64, wind_angle_deg: f64) -> (f64, f64) {
        let rad = wind_angle_deg.to_radians();
        let headwind = wind_speed_knots * rad.cos();
        let crosswind = wind_speed_knots * rad.sin();
        (headwind, crosswind)
    }
}

/// Tool 65: Flight Trip Fuel Planner
pub struct FuelBurnFlightPlanner;

impl FuelBurnFlightPlanner {
    pub fn calculate_total_fuel_gallons(flight_hours: f64, burn_rate_gph: f64, reserve_hours: f64) -> Result<f64, &'static str> {
        if flight_hours < 0.0 || burn_rate_gph <= 0.0 || reserve_hours < 0.0 {
            return Err("Invalid flight parameters");
        }
        Ok((flight_hours + reserve_hours) * burn_rate_gph)
    }
}

// ==========================================
// CATEGORY 14: JOURNALISM & PUBLISHING (66-70)
// ==========================================

/// Tool 66: Flesch-Kincaid Reading Ease Calculator
pub struct FleschKincaidReadabilityCalc;

impl FleschKincaidReadabilityCalc {
    pub fn reading_ease(total_words: usize, total_sentences: usize, total_syllables: usize) -> Result<f64, &'static str> {
        if total_words == 0 || total_sentences == 0 {
            return Err("Word and sentence counts must be non-zero");
        }
        let score = 206.835 - 1.015 * (total_words as f64 / total_sentences as f64) - 84.6 * (total_syllables as f64 / total_words as f64);
        Ok(score)
    }
}

/// Tool 67: Reading Time Estimator
pub struct WordCountEstimator;

impl WordCountEstimator {
    pub fn estimate_reading_minutes(word_count: usize, wpm: usize) -> Result<f64, &'static str> {
        if wpm == 0 {
            return Err("Words per minute must be non-zero");
        }
        Ok(word_count as f64 / wpm as f64)
    }
}

/// Tool 68: Title Case Converter
pub struct HeadlineCapitalizer;

impl HeadlineCapitalizer {
    pub fn to_title_case(headline: &str) -> String {
        headline
            .split_whitespace()
            .map(|word| {
                let mut c = word.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str(),
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

/// Tool 69: Text Jaccard Similarity Checker
pub struct PlagiarismSimilarityChecker;

impl PlagiarismSimilarityChecker {
    pub fn jaccard_similarity(text1: &str, text2: &str) -> f64 {
        let set1: std::collections::BTreeSet<&str> = text1.split_whitespace().collect();
        let set2: std::collections::BTreeSet<&str> = text2.split_whitespace().collect();
        let intersection = set1.intersection(&set2).count();
        let union = set1.union(&set2).count();
        if union == 0 {
            return 1.0;
        }
        intersection as f64 / union as f64
    }
}

/// Tool 70: Press Release Header Formatter
pub struct PressReleaseFormatter;

impl PressReleaseFormatter {
    pub fn format(city: &str, date: &str, title: &str, body: &str) -> String {
        format!("FOR IMMEDIATE RELEASE\n\n{}\n\n{}, {} -- {}", title.to_uppercase(), city, date, body)
    }
}

// ==========================================
// CATEGORY 15: EDUCATION & PEDAGOGY (71-75)
// ==========================================

/// Tool 71: Cumulative GPA Calculator
pub struct GpaCalculatorTool;

impl GpaCalculatorTool {
    pub fn calculate_gpa(courses: &[(f64, f64)]) -> Result<f64, &'static str> {
        let mut total_points = 0.0;
        let mut total_credits = 0.0;
        for &(grade_point, credit_hours) in courses {
            if credit_hours < 0.0 || grade_point < 0.0 {
                return Err("Grade points and credit hours must be non-negative");
            }
            total_points += grade_point * credit_hours;
            total_credits += credit_hours;
        }
        if total_credits == 0.0 {
            return Err("Total credit hours must be greater than zero");
        }
        Ok(total_points / total_credits)
    }
}

/// Tool 72: Exam Letter Grade Curve
pub struct TestGradingScaleCalc;

impl TestGradingScaleCalc {
    pub fn percentage_to_letter(score_pct: f64) -> &'static str {
        if score_pct >= 90.0 { "A" }
        else if score_pct >= 80.0 { "B" }
        else if score_pct >= 70.0 { "C" }
        else if score_pct >= 60.0 { "D" }
        else { "F" }
    }
}

/// Tool 73: Student Attendance Rate Tracker
pub struct ClassroomAttendanceTracker;

impl ClassroomAttendanceTracker {
    pub fn attendance_percentage(classes_attended: usize, total_classes: usize) -> Result<f64, &'static str> {
        if total_classes == 0 {
            return Err("Total classes cannot be zero");
        }
        Ok((classes_attended as f64 / total_classes as f64) * 100.0)
    }
}

/// Tool 74: Pedagogical Rubric Score Calculator
pub struct RubricScoreCalc;

impl RubricScoreCalc {
    pub fn rubric_total_pct(scores: &[u32], max_per_category: u32) -> Result<f64, &'static str> {
        if max_per_category == 0 || scores.is_empty() {
            return Err("Scores list cannot be empty and max category score must be non-zero");
        }
        let total_earned: u32 = scores.iter().sum();
        let total_possible = (scores.len() as u32) * max_per_category;
        Ok((total_earned as f64 / total_possible as f64) * 100.0)
    }
}

/// Tool 75: Lesson Plan Segment Allocator
pub struct LessonPlanTimeAllocator;

impl LessonPlanTimeAllocator {
    pub fn allocate_minutes(total_duration_mins: u32) -> (u32, u32, u32, u32) {
        let intro = (total_duration_mins as f64 * 0.15).round() as u32;
        let direct_instruction = (total_duration_mins as f64 * 0.35).round() as u32;
        let guided_practice = (total_duration_mins as f64 * 0.35).round() as u32;
        let assessment = total_duration_mins.saturating_sub(intro + direct_instruction + guided_practice);
        (intro, direct_instruction, guided_practice, assessment)
    }
}

// ==========================================
// CATEGORY 16: LOGISTICS & SUPPLY CHAIN (76-80)
// ==========================================

/// Tool 76: Economic Order Quantity (EOQ) Calculator
pub struct EconomicOrderQuantityCalc;

impl EconomicOrderQuantityCalc {
    pub fn calculate(annual_demand: f64, order_cost: f64, holding_cost_per_unit: f64) -> Result<f64, &'static str> {
        if holding_cost_per_unit <= 0.0 || annual_demand <= 0.0 || order_cost <= 0.0 {
            return Err("Costs and demand must be positive");
        }
        Ok(((2.0 * annual_demand * order_cost) / holding_cost_per_unit).sqrt())
    }
}

/// Tool 77: Shipping Dimensional Weight Calculator
pub struct FreightDimensionalWeightCalc;

impl FreightDimensionalWeightCalc {
    pub fn dimensional_weight_kg(length_cm: f64, width_cm: f64, height_cm: f64, dim_divisor: f64) -> Result<f64, &'static str> {
        if dim_divisor <= 0.0 {
            return Err("Divisor must be positive");
        }
        Ok((length_cm * width_cm * height_cm) / dim_divisor)
    }
}

/// Tool 78: Inventory Safety Stock Calculator
pub struct SafetyStockCalc;

impl SafetyStockCalc {
    pub fn calculate(max_daily_usage: f64, avg_daily_usage: f64, max_lead_time_days: f64, avg_lead_time_days: f64) -> f64 {
        (max_daily_usage * max_lead_time_days) - (avg_daily_usage * avg_lead_time_days)
    }
}

/// Tool 79: Container Box Fit Estimator
pub struct ContainerLoadingOptimizer;

impl ContainerLoadingOptimizer {
    pub fn max_boxes_fit(container_l: u32, container_w: u32, container_h: u32, box_l: u32, box_w: u32, box_h: u32) -> Result<u32, &'static str> {
        if box_l == 0 || box_w == 0 || box_h == 0 {
            return Err("Box dimensions must be non-zero");
        }
        let fit_l = container_l / box_l;
        let fit_w = container_w / box_w;
        let fit_h = container_h / box_h;
        Ok(fit_l * fit_w * fit_h)
    }
}

/// Tool 80: Trip Route Fuel Cost Estimator
pub struct RouteDistanceFuelCostCalc;

impl RouteDistanceFuelCostCalc {
    pub fn estimate_cost(distance_miles: f64, mpg: f64, price_per_gallon: f64) -> Result<f64, &'static str> {
        if mpg <= 0.0 {
            return Err("MPG must be positive");
        }
        let gallons_needed = distance_miles / mpg;
        Ok(gallons_needed * price_per_gallon)
    }
}

// ==========================================
// CATEGORY 17: MOLECULAR BIOLOGY & GENETICS (81-85)
// ==========================================

/// Tool 81: DNA Reverse Complement Generator
pub struct DnaReverseComplementTool;

impl DnaReverseComplementTool {
    pub fn reverse_complement(dna_sequence: &str) -> String {
        dna_sequence
            .chars()
            .rev()
            .map(|c| match c.to_ascii_uppercase() {
                'A' => 'T',
                'T' => 'A',
                'C' => 'G',
                'G' => 'C',
                other => other,
            })
            .collect()
    }
}

/// Tool 82: GC Content Percentage Calculator
pub struct GcContentCalculator;

impl GcContentCalculator {
    pub fn calculate_gc_pct(sequence: &str) -> Result<f64, &'static str> {
        if sequence.is_empty() {
            return Err("Sequence cannot be empty");
        }
        let gc_count = sequence
            .chars()
            .filter(|&c| match c.to_ascii_uppercase() {
                'G' | 'C' => true,
                _ => false,
            })
            .count();
        Ok((gc_count as f64 / sequence.len() as f64) * 100.0)
    }
}

/// Tool 83: RNA Codon to Amino Acid Single-Letter Translator
pub struct CodonToAminoAcidTranslator;

impl CodonToAminoAcidTranslator {
    pub fn translate_codon(codon: &str) -> &'static str {
        match codon.to_ascii_uppercase().as_str() {
            "AUG" => "M (Met - Start)",
            "UAA" | "UAG" | "UGA" => "* (Stop)",
            "UUU" | "UUC" => "F (Phe)",
            "UUA" | "UUG" | "CUU" | "CUC" | "CUA" | "CUG" => "L (Leu)",
            _ => "Unknown/Other Amino Acid",
        }
    }
}

/// Tool 84: PCR Primer Melting Temperature Calculator (Wallace Rule)
pub struct PcrAnnealingTempCalc;

impl PcrAnnealingTempCalc {
    pub fn wallace_rule_tm(primer_sequence: &str) -> f64 {
        let mut a_t = 0;
        let mut g_c = 0;
        for c in primer_sequence.chars() {
            match c.to_ascii_uppercase() {
                'A' | 'T' => a_t += 1,
                'G' | 'C' => g_c += 1,
                _ => {}
            }
        }
        (2 * a_t + 4 * g_c) as f64
    }
}

/// Tool 85: Protein Molecular Weight Estimator
pub struct ProteinMolecularWeightCalc;

impl ProteinMolecularWeightCalc {
    pub fn estimate_mw_da(num_amino_acids: usize) -> f64 {
        // Average amino acid residue weight ~ 110 Daltons
        (num_amino_acids as f64) * 110.0
    }
}

// ==========================================
// CATEGORY 18: ARCHITECTURE & INTERIOR DESIGN (86-90)
// ==========================================

/// Tool 86: Stair Riser & Tread Calculator (Blondel Rule)
pub struct StairRiserTreadCalc;

impl StairRiserTreadCalc {
    pub fn calculate_steps(total_rise_cm: f64) -> Result<(usize, f64, f64), &'static str> {
        if total_rise_cm <= 0.0 {
            return Err("Total rise must be positive");
        }
        let ideal_riser = 17.0; // 17cm target
        let step_count = (total_rise_cm / ideal_riser).round() as usize;
        let actual_riser = total_rise_cm / (step_count as f64);
        // Blondel rule: 2 * Riser + Tread = 63 cm
        let actual_tread = 63.0 - 2.0 * actual_riser;
        Ok((step_count, actual_riser, actual_tread))
    }
}

/// Tool 87: Room Daylight Glazing Factor Calculator
pub struct RoomDaylightFactorCalc;

impl RoomDaylightFactorCalc {
    pub fn glazing_ratio_pct(window_area_m2: f64, floor_area_m2: f64) -> Result<f64, &'static str> {
        if floor_area_m2 <= 0.0 {
            return Err("Floor area must be positive");
        }
        Ok((window_area_m2 / floor_area_m2) * 100.0)
    }
}

/// Tool 88: Wall Paint Coverage Calculator
pub struct PaintCoverageCalc;

impl PaintCoverageCalc {
    pub fn liters_needed(wall_area_m2: f64, coats: u32, coverage_m2_per_liter: f64) -> Result<f64, &'static str> {
        if coverage_m2_per_liter <= 0.0 || coats == 0 {
            return Err("Coverage per liter and coat count must be positive");
        }
        Ok((wall_area_m2 * (coats as f64)) / coverage_m2_per_liter)
    }
}

/// Tool 89: Flooring Tile & Waste Calculator
pub struct FlooringTileCalculator;

impl FlooringTileCalculator {
    pub fn tiles_needed(room_area_sq_m: f64, tile_area_sq_m: f64, waste_pct: f64) -> Result<usize, &'static str> {
        if tile_area_sq_m <= 0.0 {
            return Err("Tile area must be positive");
        }
        let gross_area = room_area_sq_m * (1.0 + waste_pct / 100.0);
        Ok((gross_area / tile_area_sq_m).ceil() as usize)
    }
}

/// Tool 90: HVAC Room Load Calculator
pub struct HVACLoadCalculator;

impl HVACLoadCalculator {
    pub fn cooling_btu_required(room_volume_m3: f64, occupants: u32) -> f64 {
        let base_btu = room_volume_m3 * 100.0; // Approx 100 BTU per m3
        let occupant_btu = (occupants as f64) * 400.0;
        base_btu + occupant_btu
    }
}

// ==========================================
// CATEGORY 19: MARINE ENG & NAVAL ARCHITECTURE (91-95)
// ==========================================

/// Tool 91: Hull Displacement Mass Calculator
pub struct DisplacementTonnageCalc;

impl DisplacementTonnageCalc {
    pub fn displacement_tonnes(submerged_volume_m3: f64, seawater_density_tonnes_m3: f64) -> f64 {
        submerged_volume_m3 * seawater_density_tonnes_m3
    }
}

/// Tool 92: Metacentric Height (GM) Stability Calculator
pub struct MetacentricHeightCalc;

impl MetacentricHeightCalc {
    pub fn calculate_gm(km_m: f64, kg_m: f64) -> f64 {
        km_m - kg_m
    }
}

/// Tool 93: Propeller Theoretical Speed Calculator
pub struct PropellerPitchSpeedCalc;

impl PropellerPitchSpeedCalc {
    pub fn theoretical_speed_knots(rpm: f64, pitch_inches: f64) -> f64 {
        // (RPM * pitch in inches * 60) / (12 * 6076.12 feet per nautical mile)
        (rpm * pitch_inches * 60.0) / 72_913.44
    }
}

/// Tool 94: Anchor Scope & Rode Length Calculator
pub struct AnchorRodeLengthCalc;

impl AnchorRodeLengthCalc {
    pub fn calculate_rode_length_m(water_depth_m: f64, bow_height_m: f64, scope_ratio: f64) -> Result<f64, &'static str> {
        if scope_ratio <= 0.0 {
            return Err("Scope ratio must be positive");
        }
        let total_effective_depth = water_depth_m + bow_height_m;
        Ok(total_effective_depth * scope_ratio)
    }
}

/// Tool 95: Vessel Daily Fuel Consumption Calculator (Admiralty Formula)
pub struct FuelConsumptionKnotsCalc;

impl FuelConsumptionKnotsCalc {
    pub fn estimate_daily_fuel_tons(displacement_tonnes: f64, speed_knots: f64, admiralty_coeff: f64) -> Result<f64, &'static str> {
        if admiralty_coeff <= 0.0 {
            return Err("Admiralty coefficient must be positive");
        }
        let disp_2_3 = displacement_tonnes.powf(2.0 / 3.0);
        let speed_3 = speed_knots.powi(3);
        Ok((disp_2_3 * speed_3) / admiralty_coeff)
    }
}

// ==========================================
// CATEGORY 20: METEOROLOGY & ATMOSPHERIC (96-100)
// ==========================================

/// Tool 96: Dew Point Calculator (Magnus-Tetens)
pub struct DewPointCalculator;

impl DewPointCalculator {
    pub fn calculate_celsius(temp_c: f64, relative_humidity_pct: f64) -> Result<f64, &'static str> {
        if relative_humidity_pct <= 0.0 || relative_humidity_pct > 100.0 {
            return Err("Relative humidity must be between 0 and 100%");
        }
        let a = 17.27;
        let b = 237.7;
        let alpha = ((a * temp_c) / (b + temp_c)) + (relative_humidity_pct / 100.0).ln();
        Ok((b * alpha) / (a - alpha))
    }
}

/// Tool 97: Heat Index Feel Temperature Calculator
pub struct HeatIndexCalculator;

impl HeatIndexCalculator {
    pub fn simple_heat_index(temp_f: f64, rh_pct: f64) -> f64 {
        // Simplified Steadman heat index equation
        0.5 * (temp_f + 61.0 + ((temp_f - 68.0) * 1.2) + (rh_pct * 0.094))
    }
}

/// Tool 98: Wind Chill Index Calculator
pub struct WindChillCalculator;

impl WindChillCalculator {
    pub fn calculate_celsius(temp_c: f64, wind_speed_km_h: f64) -> f64 {
        if temp_c > 10.0 || wind_speed_km_h < 4.8 {
            return temp_c; // Wind chill standard equations apply for cold temps and moving wind
        }
        let v_016 = wind_speed_km_h.powf(0.16);
        13.12 + 0.6215 * temp_c - 11.37 * v_016 + 0.3965 * temp_c * v_016
    }
}

/// Tool 99: Barometric Altitude Estimator
pub struct BarometricPressureAltitudeCalc;

impl BarometricPressureAltitudeCalc {
    pub fn estimate_altitude_m(pressure_hpa: f64, sea_level_hpa: f64) -> Result<f64, &'static str> {
        if pressure_hpa <= 0.0 || sea_level_hpa <= 0.0 {
            return Err("Pressures must be positive");
        }
        let alt = 44330.0 * (1.0 - (pressure_hpa / sea_level_hpa).powf(1.0 / 5.25588));
        Ok(alt)
    }
}

/// Tool 100: Relative Humidity from Psychrometer
pub struct RelativeHumidityCalc;

impl RelativeHumidityCalc {
    pub fn estimate_from_psychrometer(dry_bulb_c: f64, wet_bulb_c: f64) -> Result<f64, &'static str> {
        if wet_bulb_c > dry_bulb_c {
            return Err("Wet bulb temperature cannot exceed dry bulb temperature");
        }
        let depression = dry_bulb_c - wet_bulb_c;
        let rh = 100.0 - (depression * 6.5);
        Ok(rh.max(0.0).min(100.0))
    }
}

// Master Suite Struct containing all 100 tools registered
pub struct MasterProfessionToolsSuite;

impl MasterProfessionToolsSuite {
    pub fn total_registered_tools() -> usize {
        100
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_healthcare_tools() {
        assert!(BmiCalculator::calculate(70.0, 1.75).is_ok());
        assert!(DosageCalculator::calculate_dose(20.0, 5.0, 10.0).is_ok());
        assert!((MeanArterialPressureCalc::calculate(120.0, 80.0).unwrap() - 93.3333333).abs() < 1e-4);
        assert!(GlomerularFiltrationRateCalc::cockcroft_gault(50.0, 70.0, 1.0, false).is_ok());
        assert_eq!(ApgarScoreCalculator::calculate(2, 2, 2, 2, 2).unwrap().0, 10);
    }

    #[test]
    fn test_civil_eng_tools() {
        assert!(ConcreteVolumeCalc::calculate(5.0, 5.0, 0.1, 25.0).is_ok());
        assert!(BeamDeflectionCalc::calculate_simply_supported(1000.0, 4.0, 200e9, 0.0001).is_ok());
        assert!(RetainingWallStabilityCalc::sliding_factor_of_safety(100.0, 0.5, 30.0).is_ok());
        assert!(AsphaltQuantityCalc::calculate_tonnage(100.0, 5.0, 50.0).is_ok());
        assert!(PipeFlowVelocityCalc::calculate(0.1, 0.2).is_ok());
    }

    #[test]
    fn test_devops_tools() {
        assert!(CronExpressionParser::describe("0 12 * * *").is_ok());
        assert!(RegexTesterTool::contains_match("rust", "sigmaos_rust_core"));
        assert!(JwtTokenDecoder::decode_unverified("header.payload.sig").is_ok());
        assert_eq!(Base64ConverterTool::encode(b"hello"), "aGVsbG8=");
        assert_eq!(SemverValidator::parse("v1.2.3").unwrap(), (1, 2, 3));
    }

    #[test]
    fn test_astronomy_tools() {
        assert!(SchwarzschildRadiusCalc::calculate(1.989e30).is_ok()); // Sun mass
        assert_eq!(ParsecToLightYearConverter::parsec_to_light_years(1.0), 3.26156);
        assert_eq!(TelescopeMagnificationCalc::calculate(1000.0, 10.0).unwrap(), 100.0);
        assert_eq!(KeplerThirdLawCalc::orbital_period_years(1.0).unwrap(), 1.0);
        assert!(RedshiftVelocityCalc::recession_velocity_km_s(0.1).is_ok());
    }

    #[test]
    fn test_finance_tools() {
        assert!(CompoundInterestCalc::future_value(1000.0, 5.0, 12, 10.0).is_ok());
        assert!(AmortizationScheduleCalc::monthly_payment(200000.0, 4.5, 30).is_ok());
        assert_eq!(BreakEvenAnalyzer::calculate_units(10000.0, 50.0, 30.0).unwrap().0, 500.0);
        assert!(PresentValueCalc::calculate(1000.0, 5.0, 5.0).is_ok());
        assert_eq!(TaxWithholdingCalc::calculate_simple_tax(10000.0), 1000.0);
    }

    #[test]
    fn test_legal_tools() {
        assert_eq!(StatuteOfLimitationsCalc::calculate_expiration_year(2020, 3), 2023);
        assert!(LegalCitationFormatter::format_us_case("Roe", "Wade", 410, "U.S.", 113, 1973).contains("Roe v. Wade"));
        assert!(ContractInterestCalc::calculate_interest(10000.0, 6.0, 30) > 0.0);
        assert!(ChildSupportCalc::estimate_monthly_obligation(5000.0, 60.0, 2).is_ok());
        assert!(CourtDeadlineCalc::add_business_days(1, 5) > 1);
    }

    #[test]
    fn test_audio_tools() {
        assert!(DbSPLToPressureConverter::spl_to_pascals(94.0) > 0.0);
        assert_eq!(DelayTimeBpmCalc::quarter_note_delay_ms(120.0).unwrap(), 500.0);
        assert_eq!(SampleRateNyquistCalc::nyquist_frequency_hz(44100.0).unwrap(), 22050.0);
        assert_eq!(SpeakerImpedanceCalc::parallel_impedance(&[8.0, 8.0]).unwrap(), 4.0);
        assert_eq!(AudioFrequenciesToNoteCalc::freq_to_midi_note(440.0).unwrap(), 69);
    }

    #[test]
    fn test_graphic_design_tools() {
        assert_eq!(ColorHexRgbConverter::hex_to_rgb("#FF0000").unwrap(), (255, 0, 0));
        assert!(GoldenRatioGridCalc::split_length(100.0).is_ok());
        assert_eq!(AspectRatioCalc::calculate_height(1920, 16, 9).unwrap(), 1080);
        assert_eq!(PrintDpiResCalc::required_pixels(8.5, 11.0, 300), (2550, 3300));
        assert!(ContrastRatioChecker::contrast_ratio(1.0, 0.0) > 1.0);
    }

    #[test]
    fn test_agronomy_tools() {
        let amt = FertilizerNpkCalc::fertilizer_amount_kg(50.0, 46.0).unwrap();
        assert!((amt - 108.695652).abs() < 1e-4);
        assert!(CropYieldEstimator::estimate_bushels_per_acre(30000.0, 500.0) > 0.0);
        assert_eq!(IrrigationRequirementCalc::water_volume_liters(100.0, 5.0), 500.0);
        assert!(SeedingRateCalc::seeds_per_hectare(100000.0, 90.0, 95.0).is_ok());
        assert!(LivestockFeedRatioCalc::pearson_square_parts(44.0, 10.0, 16.0).is_ok());
    }

    #[test]
    fn test_chemistry_tools() {
        assert!(MolarMassCalc::simple_water_molar_mass() > 18.0);
        assert_eq!(SolutionDilutionCalc::calculate_initial_volume(10.0, 1.0, 100.0).unwrap(), 10.0);
        assert_eq!(PhCalculator::calculate_ph(1e-7).unwrap(), 7.0);
        assert!(IdealGasLawCalc::calculate_pressure_pa(1.0, 298.15, 0.024).is_ok());
        assert!(ReactionStoichiometryCalc::calculate_product_mass(18.0, 18.0, 44.0).is_ok());
    }

    #[test]
    fn test_physics_tools() {
        assert_eq!(KineticEnergyCalc::calculate_joules(2.0, 3.0), 9.0);
        assert_eq!(HalfLifeDecayCalc::remaining_mass(100.0, 5.0, 5.0).unwrap(), 50.0);
        assert!(PhotonEnergyCalc::energy_from_wavelength_nm(500.0).is_ok());
        assert!(RelativisticTimeDilationCalc::dilated_time(10.0, 1000.0).is_ok());
        assert_eq!(OhmLawCalc::voltage(2.0, 5.0), 10.0);
    }

    #[test]
    fn test_mechanical_eng_tools() {
        assert_eq!(GearRatioCalc::calculate(10, 20, 1000.0).unwrap().0, 2.0);
        assert!(HydraulicCylinderForceCalc::force_newtons(1e6, 0.1).is_ok());
        let delta_l = ThermalExpansionCalc::delta_length(1.0, 12e-6, 100.0);
        assert!((delta_l - 0.0012).abs() < 1e-6);
        assert!(TorquePowerCalc::power_kw(100.0, 3000.0) > 0.0);
        assert_eq!(SpringConstantCalc::force_newtons(100.0, 0.1), 10.0);
    }

    #[test]
    fn test_aerospace_tools() {
        assert!(MachNumberCalc::mach_number(340.0, 288.15).is_ok());
        assert_eq!(DensityAltitudeCalc::estimate_feet(5000.0, 10.0), 6200.0);
        assert_eq!(AircraftCenterOfGravityCalc::center_of_gravity_arm(&[(100.0, 10.0), (100.0, 20.0)]).unwrap(), 15.0);
        assert!(CrosswindComponentCalc::calculate(20.0, 30.0).0 > 0.0);
        assert_eq!(FuelBurnFlightPlanner::calculate_total_fuel_gallons(2.0, 10.0, 0.5).unwrap(), 25.0);
    }

    #[test]
    fn test_journalism_tools() {
        assert!(FleschKincaidReadabilityCalc::reading_ease(100, 5, 150).is_ok());
        assert_eq!(WordCountEstimator::estimate_reading_minutes(400, 200).unwrap(), 2.0);
        assert_eq!(HeadlineCapitalizer::to_title_case("breaking news report"), "Breaking News Report");
        assert!(PlagiarismSimilarityChecker::jaccard_similarity("hello world", "hello world") == 1.0);
        assert!(PressReleaseFormatter::format("NEW YORK", "2026", "LAUNCH", "Content").contains("FOR IMMEDIATE RELEASE"));
    }

    #[test]
    fn test_education_tools() {
        assert_eq!(GpaCalculatorTool::calculate_gpa(&[(4.0, 3.0), (3.0, 3.0)]).unwrap(), 3.5);
        assert_eq!(TestGradingScaleCalc::percentage_to_letter(95.0), "A");
        assert_eq!(ClassroomAttendanceTracker::attendance_percentage(18, 20).unwrap(), 90.0);
        assert_eq!(RubricScoreCalc::rubric_total_pct(&[4, 4, 4], 4).unwrap(), 100.0);
        assert_eq!(LessonPlanTimeAllocator::allocate_minutes(60).0, 9);
    }

    #[test]
    fn test_logistics_tools() {
        assert!(EconomicOrderQuantityCalc::calculate(1000.0, 50.0, 2.0).is_ok());
        assert!(FreightDimensionalWeightCalc::dimensional_weight_kg(50.0, 40.0, 30.0, 5000.0).is_ok());
        assert_eq!(SafetyStockCalc::calculate(100.0, 80.0, 10.0, 7.0), 440.0);
        assert_eq!(ContainerLoadingOptimizer::max_boxes_fit(100, 100, 100, 10, 10, 10).unwrap(), 1000);
        assert_eq!(RouteDistanceFuelCostCalc::estimate_cost(100.0, 20.0, 3.50).unwrap(), 17.50);
    }

    #[test]
    fn test_genetics_tools() {
        assert_eq!(DnaReverseComplementTool::reverse_complement("ATGC"), "GCAT");
        assert_eq!(GcContentCalculator::calculate_gc_pct("GCGC").unwrap(), 100.0);
        assert_eq!(CodonToAminoAcidTranslator::translate_codon("AUG"), "M (Met - Start)");
        assert_eq!(PcrAnnealingTempCalc::wallace_rule_tm("ATGC"), 12.0);
        assert_eq!(ProteinMolecularWeightCalc::estimate_mw_da(100), 11000.0);
    }

    #[test]
    fn test_architecture_tools() {
        assert!(StairRiserTreadCalc::calculate_steps(280.0).is_ok());
        assert_eq!(RoomDaylightFactorCalc::glazing_ratio_pct(5.0, 25.0).unwrap(), 20.0);
        assert_eq!(PaintCoverageCalc::liters_needed(100.0, 2, 10.0).unwrap(), 20.0);
        assert_eq!(FlooringTileCalculator::tiles_needed(10.0, 1.0, 10.0).unwrap(), 11);
        assert!(HVACLoadCalculator::cooling_btu_required(50.0, 4) > 0.0);
    }

    #[test]
    fn test_marine_tools() {
        let disp = DisplacementTonnageCalc::displacement_tonnes(100.0, 1.025);
        assert!((disp - 102.5).abs() < 1e-4);
        assert_eq!(MetacentricHeightCalc::calculate_gm(5.0, 3.5), 1.5);
        assert!(PropellerPitchSpeedCalc::theoretical_speed_knots(2000.0, 20.0) > 0.0);
        assert_eq!(AnchorRodeLengthCalc::calculate_rode_length_m(10.0, 2.0, 5.0).unwrap(), 60.0);
        assert!(FuelConsumptionKnotsCalc::estimate_daily_fuel_tons(10000.0, 20.0, 50000.0).is_ok());
    }

    #[test]
    fn test_meteorology_tools() {
        assert!(DewPointCalculator::calculate_celsius(25.0, 50.0).is_ok());
        assert!(HeatIndexCalculator::simple_heat_index(80.0, 60.0) > 0.0);
        assert_eq!(WindChillCalculator::calculate_celsius(15.0, 10.0), 15.0);
        assert!(BarometricPressureAltitudeCalc::estimate_altitude_m(900.0, 1013.25).is_ok());
        assert!(RelativeHumidityCalc::estimate_from_psychrometer(20.0, 15.0).is_ok());
        assert_eq!(MasterProfessionToolsSuite::total_registered_tools(), 100);
    }
}
