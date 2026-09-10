// SigmaOS Indian Profession Tools Module - Specialized tools for Indian professionals
// Safe Rust, zero external dependencies

// ==========================================
// 1. INDIAN CHARTERED ACCOUNTANTS & TAX CONSULTANTS
// ==========================================

/// Tool 1: Indian GST & HSN Tax Calculator
pub struct IndianGstCalculator;

impl IndianGstCalculator {
    pub fn calculate_gst(taxable_value_inr: f64, gst_rate_pct: f64, is_inter_state: bool) -> Result<(f64, f64, f64, f64), &'static str> {
        if taxable_value_inr < 0.0 || gst_rate_pct < 0.0 {
            return Err("Value and rate must be non-negative");
        }
        let total_gst = taxable_value_inr * (gst_rate_pct / 100.0);
        let gross_total = taxable_value_inr + total_gst;

        if is_inter_state {
            // IGST
            Ok((0.0, 0.0, total_gst, gross_total))
        } else {
            // CGST + SGST (split equally)
            let cgst = total_gst / 2.0;
            let sgst = total_gst / 2.0;
            Ok((cgst, sgst, 0.0, gross_total))
        }
    }
}

/// Tool 2: Indian Income Tax New vs Old Regime Calculator (FY 2024-25)
pub struct IndianIncomeTaxRegimeCalculator;

impl IndianIncomeTaxRegimeCalculator {
    pub fn calculate_new_regime_tax(taxable_income_inr: f64) -> f64 {
        // Standard deduction of 75,000 INR
        let net_income = (taxable_income_inr - 75_000.0).max(0.0);
        if net_income <= 700_000.0 {
            return 0.0; // Section 87A rebate for income up to 7 Lakhs
        }

        let mut tax = 0.0;
        // Slab 1: 0 - 3L: Nil
        // Slab 2: 3L - 7L: 5%
        if net_income > 300_000.0 {
            let slab_income = (net_income.min(700_000.0) - 300_000.0).max(0.0);
            tax += slab_income * 0.05;
        }
        // Slab 3: 7L - 10L: 10%
        if net_income > 700_000.0 {
            let slab_income = (net_income.min(1_000_000.0) - 700_000.0).max(0.0);
            tax += slab_income * 0.10;
        }
        // Slab 4: 10L - 12L: 15%
        if net_income > 1_000_000.0 {
            let slab_income = (net_income.min(1_200_000.0) - 1_000_000.0).max(0.0);
            tax += slab_income * 0.15;
        }
        // Slab 5: 12L - 15L: 20%
        if net_income > 1_200_000.0 {
            let slab_income = (net_income.min(1_500_000.0) - 1_200_000.0).max(0.0);
            tax += slab_income * 0.20;
        }
        // Slab 6: Above 15L: 30%
        if net_income > 1_500_000.0 {
            let slab_income = net_income - 1_500_000.0;
            tax += slab_income * 0.30;
        }

        // Health & Education Cess: 4%
        tax * 1.04
    }
}

/// Tool 3: Indian Tax Deducted at Source (TDS) Section Estimator
pub struct IndianTdsCalculator;

impl IndianTdsCalculator {
    pub fn calculate_tds(section_code: &str, payment_amount_inr: f64) -> Result<(f64, f64), &'static str> {
        let (rate_pct, threshold_inr) = match section_code {
            "194C" => (1.0, 30_000.0),  // Contractor
            "194J" => (10.0, 30_000.0), // Professional Fees
            "194I" => (10.0, 240_000.0),// Rent
            "194H" => (5.0, 15_000.0),  // Commission
            _ => return Err("Unsupported TDS Section"),
        };

        if payment_amount_inr <= threshold_inr {
            Ok((0.0, payment_amount_inr))
        } else {
            let tds_amount = payment_amount_inr * (rate_pct / 100.0);
            let net_payable = payment_amount_inr - tds_amount;
            Ok((tds_amount, net_payable))
        }
    }
}

// ==========================================
// 2. INDIAN FARMERS & AGRICULTURAL SCIENTISTS
// ==========================================

/// Tool 4: PM-Kisan & Minimum Support Price (MSP) Revenue Estimator
pub struct IndianMspYieldCalculator;

impl IndianMspYieldCalculator {
    pub fn estimate_crop_msp_value(crop_name: &str, yield_quintals: f64) -> Result<f64, &'static str> {
        if yield_quintals < 0.0 {
            return Err("Yield quintals must be non-negative");
        }
        // Approximate official Govt MSP rate per quintal in INR
        let msp_rate_per_quintal = match crop_name.to_ascii_lowercase().as_str() {
            "paddy" | "rice" => 2300.0,
            "wheat" => 2275.0,
            "cotton" => 6620.0,
            "mustard" | "sarson" => 5650.0,
            "sugarcane" => 315.0,
            "maize" | "makka" => 2090.0,
            _ => return Err("Unsupported crop for MSP lookup"),
        };

        Ok(yield_quintals * msp_rate_per_quintal)
    }
}

/// Tool 5: Indian Mandi Price & Market Fee Estimator
pub struct IndianMandiFeeCalculator;

impl IndianMandiFeeCalculator {
    pub fn calculate_mandi_net(sale_amount_inr: f64, state_mandi_tax_pct: f64) -> Result<(f64, f64), &'static str> {
        if sale_amount_inr < 0.0 || state_mandi_tax_pct < 0.0 {
            return Err("Amount and tax rate must be non-negative");
        }
        let tax_amount = sale_amount_inr * (state_mandi_tax_pct / 100.0);
        let net_payout = sale_amount_inr - tax_amount;
        Ok((tax_amount, net_payout))
    }
}

// ==========================================
// 3. INDIAN CIVIL ENGINEERS & ARCHITECTS
// ==========================================

/// Tool 6: IS 456 Concrete Mix Design Proportions (Indian Standard)
pub struct IndianConcreteGradeIs456;

impl IndianConcreteGradeIs456 {
    pub fn get_mix_ratio(grade: &str) -> Result<(&'static str, f64), &'static str> {
        match grade.to_ascii_uppercase().as_str() {
            "M5" => Ok(("1 : 5 : 10 (Cement : Sand : Aggregate)", 5.0)),
            "M7.5" => Ok(("1 : 4 : 8", 7.5)),
            "M10" => Ok(("1 : 3 : 6", 10.0)),
            "M15" => Ok(("1 : 2 : 4", 15.0)),
            "M20" => Ok(("1 : 1.5 : 3", 20.0)),
            "M25" => Ok(("1 : 1 : 2", 25.0)),
            _ => Err("Invalid concrete grade under IS 456"),
        }
    }
}

/// Tool 7: Traditional Vaastu Shastra Directional Orientation Evaluator
pub struct IndianVaastuOrientationEvaluator;

impl IndianVaastuOrientationEvaluator {
    pub fn evaluate_room_vaastu(room_type: &str, facing_direction: &str) -> Result<(&'static str, &'static str), &'static str> {
        let dir = facing_direction.to_ascii_uppercase();
        match room_type.to_ascii_lowercase().as_str() {
            "kitchen" => {
                if dir == "SOUTHEAST" || dir == "SE" {
                    Ok(("Highly Auspicious (Agneya Corner)", "ideal"))
                } else if dir == "NORTHWEST" || dir == "NW" {
                    Ok(("Acceptable Secondary Choice", "good"))
                } else {
                    Ok(("Inauspicious: Avoid placing kitchen in North/Southwest", "avoid"))
                }
            }
            "master_bedroom" => {
                if dir == "SOUTHWEST" || dir == "SW" {
                    Ok(("Highly Auspicious (Nairrutya Corner)", "ideal"))
                } else {
                    Ok(("Inauspicious for head of family", "avoid"))
                }
            }
            "pooja_room" | "temple" => {
                if dir == "NORTHEAST" || dir == "NE" {
                    Ok(("Highly Auspicious (Ishan Corner)", "ideal"))
                } else {
                    Ok(("Avoid placing pooja room in South/Southwest", "avoid"))
                }
            }
            _ => Err("Unsupported room type for Vaastu evaluation"),
        }
    }
}

// ==========================================
// 4. INDIAN AYURVEDIC PRACTITIONERS & PHARMACISTS
// ==========================================

/// Tool 8: Tridosha Prakriti Assessment Calculator (Vata, Pitta, Kapha)
pub struct IndianAyurvedaDoshaCalculator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DoshaScore {
    pub vata: u32,
    pub pitta: u32,
    pub kapha: u32,
}

impl IndianAyurvedaDoshaCalculator {
    pub fn dominant_prakriti(score: DoshaScore) -> &'static str {
        if score.vata >= score.pitta && score.vata >= score.kapha {
            "Vata Predominant (Air & Ether Element)"
        } else if score.pitta >= score.vata && score.pitta >= score.kapha {
            "Pitta Predominant (Fire & Water Element)"
        } else {
            "Kapha Predominant (Earth & Water Element)"
        }
    }
}

/// Tool 9: Ayurvedic Rasa (Taste) & Virya Potency Evaluator
pub struct IndianAyurvedicHerbalDravyaguna;

impl IndianAyurvedicHerbalDravyaguna {
    pub fn get_herb_properties(herb_name: &str) -> Result<(&'static str, &'static str, &'static str), &'static str> {
        match herb_name.to_ascii_lowercase().as_str() {
            "ashwagandha" => Ok(("Tikta/Katu (Bitter/Pungent)", "Ushna (Heating)", "Vata-Kapha Hara")),
            "tulsi" => Ok(("Katu/Tikta (Pungent/Bitter)", "Ushna (Heating)", "Kapha-Vata Hara")),
            "triphala" | "amla" => Ok(("Kashaya/Madhura (Astringent/Sweet)", "Sheeta (Cooling)", "Tridosha Samana")),
            "shatavari" => Ok(("Madhura/Tikta (Sweet/Bitter)", "Sheeta (Cooling)", "Pitta-Vata Hara")),
            "turmeric" | "haldi" => Ok(("Tikta/Katu (Bitter/Pungent)", "Ushna (Heating)", "Tridoshakara")),
            _ => Err("Herb profile not found in Dravyaguna database"),
        }
    }
}

// ==========================================
// 5. INDIAN LEGAL PROFESSIONALS & ADVOCATES
// ==========================================

/// Tool 10: Indian Penal Code (IPC) to Bharatiya Nyaya Sanhita (BNS) Section Mapper
pub struct IndianPenalCodeBnsMapper;

impl IndianPenalCodeBnsMapper {
    pub fn map_ipc_to_bns(ipc_section: u32) -> Result<(u32, &'static str), &'static str> {
        match ipc_section {
            302 => Ok((101, "Murder")),
            304 => Ok((105, "Culpable Homicide Not Amounting to Murder")),
            420 => Ok((318, "Cheating & Dishonestly Inducing Delivery of Property")),
            376 => Ok((63, "Rape")),
            124 => Ok((152, "Act Endangering Sovereignty, Unity and Integrity of India")),
            498 => Ok((85, "Cruelty by Husband or Relatives")),
            307 => Ok((109, "Attempt to Murder")),
            _ => Err("IPC Section mapping to BNS not found"),
        }
    }
}

/// Tool 11: Indian Court Case Citation Formatter (Supreme Court / High Court)
pub struct IndianLegalCitationFormatter;

impl IndianLegalCitationFormatter {
    pub fn format_sc_citation(petitioner: &str, respondent: &str, air_year: u32, page: u32) -> String {
        format!("{} v. {}, AIR {} SC {}", petitioner, respondent, air_year, page)
    }
}

// ==========================================
// 6. INDIAN TEACHERS & EDUCATORS
// ==========================================

/// Tool 12: CBSE/ICSE Percentage to CGPA & Grade Point Converter
pub struct IndianSchoolGradeConverter;

impl IndianSchoolGradeConverter {
    pub fn percentage_to_cbse_grade(percentage: f64) -> Result<(&'static str, f64), &'static str> {
        if percentage < 0.0 || percentage > 100.0 {
            return Err("Percentage must be between 0 and 100");
        }
        let cgpa = percentage / 9.5;
        let grade = if percentage >= 91.0 { "A1" }
        else if percentage >= 81.0 { "A2" }
        else if percentage >= 71.0 { "B1" }
        else if percentage >= 61.0 { "B2" }
        else if percentage >= 51.0 { "C1" }
        else if percentage >= 41.0 { "C2" }
        else if percentage >= 33.0 { "D" }
        else { "E (Needs Improvement)" };

        Ok((grade, cgpa))
    }
}

// ==========================================
// 7. INDIAN ARTISANS & WEAVERS
// ==========================================

/// Tool 13: Indian Handloom Thread Count & Khadi Fabric Density Estimator
pub struct IndianHandloomKhadiCalculator;

impl IndianHandloomKhadiCalculator {
    pub fn estimate_khadi_density(warp_count: u32, weft_count: u32, ends_per_inch: u32) -> f64 {
        let avg_count = (warp_count + weft_count) as f64 / 2.0;
        (ends_per_inch as f64) / avg_count.sqrt()
    }
}

// ==========================================
// 8. INDIAN LOGISTICS & TRANSPORT OPERATORS
// ==========================================

/// Tool 14: Indian E-Way Bill Validity & FASTag Toll Estimator
pub struct IndianEwayBillTollCalculator;

impl IndianEwayBillTollCalculator {
    pub fn calculate_eway_bill_validity_days(distance_km: f64, is_over_dimensional_cargo: bool) -> Result<u32, &'static str> {
        if distance_km <= 0.0 {
            return Err("Distance must be strictly positive");
        }
        let km_per_day = if is_over_dimensional_cargo {
            20.0
        } else {
            200.0
        };
        let days = (distance_km / km_per_day).ceil() as u32;
        Ok(days.max(1))
    }
}

// Master Suite for Indian Profession Tools
pub struct MasterIndianProfessionToolsSuite;

impl MasterIndianProfessionToolsSuite {
    pub fn total_tools_count() -> usize {
        14
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indian_gst_calculator() {
        let (cgst, sgst, igst, total) = IndianGstCalculator::calculate_gst(1000.0, 18.0, false).unwrap();
        assert_eq!(cgst, 90.0);
        assert_eq!(sgst, 90.0);
        assert_eq!(igst, 0.0);
        assert_eq!(total, 1180.0);

        let (_, _, igst_inter, _) = IndianGstCalculator::calculate_gst(1000.0, 18.0, true).unwrap();
        assert_eq!(igst_inter, 180.0);
    }

    #[test]
    fn test_income_tax_calculator() {
        let tax = IndianIncomeTaxRegimeCalculator::calculate_new_regime_tax(800_000.0);
        assert!(tax > 0.0);
        let tax_nil = IndianIncomeTaxRegimeCalculator::calculate_new_regime_tax(600_000.0);
        assert_eq!(tax_nil, 0.0);
    }

    #[test]
    fn test_tds_calculator() {
        let (tds, net) = IndianTdsCalculator::calculate_tds("194J", 50_000.0).unwrap();
        assert_eq!(tds, 5000.0);
        assert_eq!(net, 45000.0);
    }

    #[test]
    fn test_msp_calculator() {
        let val = IndianMspYieldCalculator::estimate_crop_msp_value("wheat", 100.0).unwrap();
        assert_eq!(val, 227500.0);
    }

    #[test]
    fn test_mandi_calculator() {
        let (tax, payout) = IndianMandiFeeCalculator::calculate_mandi_net(100000.0, 2.0).unwrap();
        assert_eq!(tax, 2000.0);
        assert_eq!(payout, 98000.0);
    }

    #[test]
    fn test_concrete_is456() {
        let (ratio, fck) = IndianConcreteGradeIs456::get_mix_ratio("M20").unwrap();
        assert!(ratio.contains("1.5"));
        assert_eq!(fck, 20.0);
    }

    #[test]
    fn test_vaastu_evaluator() {
        let (eval, status) = IndianVaastuOrientationEvaluator::evaluate_room_vaastu("kitchen", "SE").unwrap();
        assert_eq!(status, "ideal");
        assert!(eval.contains("Agneya"));
    }

    #[test]
    fn test_ayurveda_dosha() {
        let score = DoshaScore { vata: 10, pitta: 5, kapha: 2 };
        let res = IndianAyurvedaDoshaCalculator::dominant_prakriti(score);
        assert!(res.contains("Vata Predominant"));
    }

    #[test]
    fn test_dravyaguna_herbs() {
        let (rasa, virya, _) = IndianAyurvedicHerbalDravyaguna::get_herb_properties("ashwagandha").unwrap();
        assert!(rasa.contains("Bitter"));
        assert_eq!(virya, "Ushna (Heating)");
    }

    #[test]
    fn test_ipc_bns_mapper() {
        let (bns, name) = IndianPenalCodeBnsMapper::map_ipc_to_bns(302).unwrap();
        assert_eq!(bns, 101);
        assert_eq!(name, "Murder");
    }

    #[test]
    fn test_citation_formatter() {
        let cit = IndianLegalCitationFormatter::format_sc_citation("Kesavananda Bharati", "State of Kerala", 1973, 1461);
        assert!(cit.contains("AIR 1973 SC 1461"));
    }

    #[test]
    fn test_cbse_converter() {
        let (grade, cgpa) = IndianSchoolGradeConverter::percentage_to_cbse_grade(95.0).unwrap();
        assert_eq!(grade, "A1");
        assert_eq!(cgpa, 10.0);
    }

    #[test]
    fn test_khadi_density() {
        let density = IndianHandloomKhadiCalculator::estimate_khadi_density(40, 40, 60);
        assert!(density > 0.0);
    }

    #[test]
    fn test_eway_bill_validity() {
        let days = IndianEwayBillTollCalculator::calculate_eway_bill_validity_days(350.0, false).unwrap();
        assert_eq!(days, 2);
        assert_eq!(MasterIndianProfessionToolsSuite::total_tools_count(), 14);
    }
}
