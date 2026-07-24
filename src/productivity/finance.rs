// SigmaOS India-First Finance Module (sigma_finance)
// OOP-based GST, TDS, UPI, and Income Tax engine
// No external crate dependencies - fully self-contained

/// HSN code entry for GST lookup
#[derive(Debug, Clone)]
pub struct HsnEntry {
    pub code: u32,
    pub description: &'static str,
    pub gst_rate: f64,
}

/// GST slab table - India 2024-25 rates
const GST_SLABS: &[HsnEntry] = &[
    HsnEntry {
        code: 0,
        description: "Exempted goods/services",
        gst_rate: 0.0,
    },
    HsnEntry {
        code: 1001,
        description: "Wheat and meslin",
        gst_rate: 0.0,
    },
    HsnEntry {
        code: 1006,
        description: "Rice",
        gst_rate: 5.0,
    },
    HsnEntry {
        code: 2201,
        description: "Waters, including mineral waters",
        gst_rate: 12.0,
    },
    HsnEntry {
        code: 3004,
        description: "Medicaments (excluding Sch. I)",
        gst_rate: 12.0,
    },
    HsnEntry {
        code: 4901,
        description: "Books, brochures",
        gst_rate: 0.0,
    },
    HsnEntry {
        code: 8471,
        description: "Automatic data processing machines",
        gst_rate: 18.0,
    },
    HsnEntry {
        code: 8517,
        description: "Telephone sets; smartphones",
        gst_rate: 18.0,
    },
    HsnEntry {
        code: 8702,
        description: "Motor vehicles for transport",
        gst_rate: 28.0,
    },
    HsnEntry {
        code: 9983,
        description: "Software (IT services)",
        gst_rate: 18.0,
    },
    HsnEntry {
        code: 9984,
        description: "Telecom services",
        gst_rate: 18.0,
    },
    HsnEntry {
        code: 9997,
        description: "Other services",
        gst_rate: 18.0,
    },
];

/// GST transaction type - inter-state vs intra-state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GstTransactionType {
    IntraState, // CGST + SGST
    InterState, // IGST only
    Export,     // Zero-rated
}

/// Breakdown of GST components
#[derive(Debug, Clone)]
pub struct GstBreakdown {
    pub base_amount: f64,
    pub cgst_rate: f64,
    pub sgst_rate: f64,
    pub igst_rate: f64,
    pub cgst_amount: f64,
    pub sgst_amount: f64,
    pub igst_amount: f64,
    pub total_gst: f64,
    pub grand_total: f64,
    pub transaction_type: GstTransactionType,
}

impl GstBreakdown {
    pub fn summary(&self) -> String {
        match self.transaction_type {
            GstTransactionType::IntraState => {
                let mut s = String::from("Intra-State GST Breakdown:\n");
                s.push_str(&format!("  Base:  ₹{:.2}\n", self.base_amount));
                s.push_str(&format!(
                    "  CGST ({:.0}%): ₹{:.2}\n",
                    self.cgst_rate, self.cgst_amount
                ));
                s.push_str(&format!(
                    "  SGST ({:.0}%): ₹{:.2}\n",
                    self.sgst_rate, self.sgst_amount
                ));
                s.push_str(&format!("  Grand Total: ₹{:.2}", self.grand_total));
                s
            }
            GstTransactionType::InterState => {
                let mut s = String::from("Inter-State GST Breakdown:\n");
                s.push_str(&format!("  Base:  ₹{:.2}\n", self.base_amount));
                s.push_str(&format!(
                    "  IGST ({:.0}%): ₹{:.2}\n",
                    self.igst_rate, self.igst_amount
                ));
                s.push_str(&format!("  Grand Total: ₹{:.2}", self.grand_total));
                s
            }
            GstTransactionType::Export => {
                format!("Zero-Rated Export: ₹{:.2}", self.base_amount)
            }
        }
    }
}

/// OOP trait for GST calculators
pub trait GstCalculator {
    fn calculate(&self, amount: f64, rate: f64, txn_type: GstTransactionType) -> GstBreakdown;
    fn reverse_calculate(
        &self,
        inclusive_amount: f64,
        rate: f64,
        txn_type: GstTransactionType,
    ) -> GstBreakdown;
    fn lookup_hsn_rate(&self, hsn_code: u32) -> Option<f64>;
    fn name(&self) -> &'static str;
}

/// Standard GST calculator implementing GstCalculator trait
pub struct StandardGstCalculator;

impl StandardGstCalculator {
    pub fn new() -> Self {
        StandardGstCalculator
    }
}

impl Default for StandardGstCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl GstCalculator for StandardGstCalculator {
    fn calculate(&self, amount: f64, rate: f64, txn_type: GstTransactionType) -> GstBreakdown {
        match txn_type {
            GstTransactionType::IntraState => {
                let half_rate = rate / 2.0;
                let each = amount * half_rate / 100.0;
                GstBreakdown {
                    base_amount: amount,
                    cgst_rate: half_rate,
                    sgst_rate: half_rate,
                    igst_rate: 0.0,
                    cgst_amount: each,
                    sgst_amount: each,
                    igst_amount: 0.0,
                    total_gst: each * 2.0,
                    grand_total: amount + each * 2.0,
                    transaction_type: txn_type,
                }
            }
            GstTransactionType::InterState => {
                let igst = amount * rate / 100.0;
                GstBreakdown {
                    base_amount: amount,
                    cgst_rate: 0.0,
                    sgst_rate: 0.0,
                    igst_rate: rate,
                    cgst_amount: 0.0,
                    sgst_amount: 0.0,
                    igst_amount: igst,
                    total_gst: igst,
                    grand_total: amount + igst,
                    transaction_type: txn_type,
                }
            }
            GstTransactionType::Export => GstBreakdown {
                base_amount: amount,
                cgst_rate: 0.0,
                sgst_rate: 0.0,
                igst_rate: 0.0,
                cgst_amount: 0.0,
                sgst_amount: 0.0,
                igst_amount: 0.0,
                total_gst: 0.0,
                grand_total: amount,
                transaction_type: txn_type,
            },
        }
    }

    fn reverse_calculate(
        &self,
        inclusive_amount: f64,
        rate: f64,
        txn_type: GstTransactionType,
    ) -> GstBreakdown {
        let base = inclusive_amount / (1.0 + rate / 100.0);
        let gst_total = inclusive_amount - base;
        let breakdown = self.calculate(base, rate, txn_type);
        // Reconstruct with correct totals
        let ratio = if gst_total > 0.0 {
            gst_total / breakdown.total_gst.max(0.001)
        } else {
            0.0
        };
        GstBreakdown {
            base_amount: base,
            cgst_amount: breakdown.cgst_amount * ratio,
            sgst_amount: breakdown.sgst_amount * ratio,
            igst_amount: breakdown.igst_amount * ratio,
            total_gst: gst_total,
            grand_total: inclusive_amount,
            ..breakdown
        }
    }

    fn lookup_hsn_rate(&self, hsn_code: u32) -> Option<f64> {
        GST_SLABS
            .iter()
            .find(|e| e.code == hsn_code)
            .map(|e| e.gst_rate)
    }

    fn name(&self) -> &'static str {
        "StandardGstCalculator"
    }
}

// ─── TDS (Tax Deducted at Source) Engine ─────────────────────────────────────

/// Income Tax Act TDS sections
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TdsSection {
    Section192,   // Salary - slab rates
    Section194A,  // Interest (non-bank) - 10%
    Section194B,  // Lottery winnings - 30%
    Section194C,  // Contractor payments - 1% (individual), 2% (company)
    Section194D,  // Insurance commission - 5%
    Section194H,  // Commission/brokerage - 5%
    Section194I,  // Rent - 10% (land/building), 2% (plant/machinery)
    Section194J,  // Professional/technical fees - 10%
    Section194Q,  // Purchase of goods - 0.1%
    Section206AB, // Non-filers higher TDS - 5% or 2x, whichever higher
}

/// TDS deductee type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeducteeType {
    Individual,
    Company,
    HinduUndividedFamily,
}

/// TDS calculation result
#[derive(Debug, Clone)]
pub struct TdsResult {
    pub section: TdsSection,
    pub gross_payment: f64,
    pub tds_rate: f64,
    pub tds_amount: f64,
    pub surcharge: f64,
    pub health_education_cess: f64,
    pub total_deduction: f64,
    pub net_payment: f64,
}

/// OOP trait for TDS calculators
pub trait TdsCalculator {
    fn calculate_tds(&self, gross: f64, section: TdsSection, deductee: DeducteeType) -> TdsResult;
    fn threshold_exceeded(&self, gross: f64, section: TdsSection) -> bool;
    fn name(&self) -> &'static str;
}

/// India TDS calculator per Finance Act 2024
pub struct IndiaTdsCalculator {
    pub financial_year: u32,
}

impl IndiaTdsCalculator {
    pub fn new(fy: u32) -> Self {
        IndiaTdsCalculator { financial_year: fy }
    }

    fn get_rate(&self, section: TdsSection, deductee: DeducteeType) -> f64 {
        match section {
            TdsSection::Section192 => 0.0, // Slab-based; simplified to 0 here
            TdsSection::Section194A => 10.0,
            TdsSection::Section194B => 30.0,
            TdsSection::Section194C => match deductee {
                DeducteeType::Individual | DeducteeType::HinduUndividedFamily => 1.0,
                DeducteeType::Company => 2.0,
            },
            TdsSection::Section194D => 5.0,
            TdsSection::Section194H => 5.0,
            TdsSection::Section194I => 10.0,
            TdsSection::Section194J => 10.0,
            TdsSection::Section194Q => 0.1,
            TdsSection::Section206AB => 5.0,
        }
    }

    fn get_threshold(&self, section: TdsSection) -> f64 {
        match section {
            TdsSection::Section192 => 250000.0,
            TdsSection::Section194A => 40000.0,
            TdsSection::Section194B => 10000.0,
            TdsSection::Section194C => 30000.0, // per payment; 100000 aggregate
            TdsSection::Section194D => 15000.0,
            TdsSection::Section194H => 15000.0,
            TdsSection::Section194I => 240000.0,
            TdsSection::Section194J => 30000.0,
            TdsSection::Section194Q => 5000000.0, // 50 lakh
            TdsSection::Section206AB => 0.0,
        }
    }
}

impl TdsCalculator for IndiaTdsCalculator {
    fn calculate_tds(&self, gross: f64, section: TdsSection, deductee: DeducteeType) -> TdsResult {
        let rate = self.get_rate(section, deductee);
        let tds_amount = gross * rate / 100.0;
        let cess = tds_amount * 0.04; // 4% Health & Education Cess
        TdsResult {
            section,
            gross_payment: gross,
            tds_rate: rate,
            tds_amount,
            surcharge: 0.0,
            health_education_cess: cess,
            total_deduction: tds_amount + cess,
            net_payment: gross - tds_amount - cess,
        }
    }

    fn threshold_exceeded(&self, gross: f64, section: TdsSection) -> bool {
        gross > self.get_threshold(section)
    }

    fn name(&self) -> &'static str {
        "IndiaTdsCalculator"
    }
}

// ─── Income Tax Slab Calculator ───────────────────────────────────────────────

/// Tax regime option
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaxRegime {
    Old, // With deductions (80C, HRA, etc.)
    New, // Simplified slabs, fewer deductions (default since FY 2023-24)
}

/// Annual income tax result
#[derive(Debug, Clone)]
pub struct IncomeTaxResult {
    pub gross_income: f64,
    pub standard_deduction: f64,
    pub taxable_income: f64,
    pub basic_tax: f64,
    pub surcharge: f64,
    pub cess: f64,
    pub total_tax: f64,
    pub effective_rate: f64,
    pub regime: TaxRegime,
}

/// OOP trait for income tax calculators
pub trait IncomeTaxCalculator {
    fn calculate(&self, gross_income: f64, deductions: f64, regime: TaxRegime) -> IncomeTaxResult;
    fn rebate_87a(&self, taxable_income: f64, regime: TaxRegime) -> f64;
    fn name(&self) -> &'static str;
}

/// India Income Tax Calculator (FY 2024-25 / AY 2025-26)
pub struct IndiaIncomeTaxCalculator;

impl IndiaIncomeTaxCalculator {
    pub fn new() -> Self {
        IndiaIncomeTaxCalculator
    }

    fn new_regime_tax(&self, taxable: f64) -> f64 {
        // New Regime slabs FY 2024-25
        let mut tax = 0.0;
        let slabs = [
            (300000.0, 0.0),
            (400000.0, 5.0),
            (300000.0, 10.0),
            (300000.0, 15.0),
            (300000.0, 20.0),
            (f64::MAX, 30.0),
        ];
        let mut remaining = taxable;
        for (band, rate) in &slabs {
            if remaining <= 0.0 {
                break;
            }
            let chunk = remaining.min(*band);
            tax += chunk * rate / 100.0;
            remaining -= chunk;
        }
        tax
    }

    fn old_regime_tax(&self, taxable: f64) -> f64 {
        // Old Regime slabs
        let mut tax = 0.0;
        if taxable > 1000000.0 {
            tax = 112500.0 + (taxable - 1000000.0) * 0.30;
        } else if taxable > 500000.0 {
            tax = 12500.0 + (taxable - 500000.0) * 0.20;
        } else if taxable > 250000.0 {
            tax = (taxable - 250000.0) * 0.05;
        }
        tax
    }
}

impl Default for IndiaIncomeTaxCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl IncomeTaxCalculator for IndiaIncomeTaxCalculator {
    fn calculate(&self, gross_income: f64, deductions: f64, regime: TaxRegime) -> IncomeTaxResult {
        let std_deduction = match regime {
            TaxRegime::New => 75000.0_f64.min(gross_income),
            TaxRegime::Old => 50000.0_f64.min(gross_income),
        };

        let taxable_income = (gross_income - std_deduction - deductions).max(0.0);
        let basic_tax = match regime {
            TaxRegime::New => self.new_regime_tax(taxable_income),
            TaxRegime::Old => self.old_regime_tax(taxable_income),
        };

        // Rebate u/s 87A
        let rebate = self.rebate_87a(taxable_income, regime);
        let tax_after_rebate = (basic_tax - rebate).max(0.0);

        // Surcharge
        let surcharge = if taxable_income > 50000000.0 {
            tax_after_rebate * 0.37
        } else if taxable_income > 20000000.0 {
            tax_after_rebate * 0.25
        } else if taxable_income > 10000000.0 {
            tax_after_rebate * 0.15
        } else if taxable_income > 5000000.0 {
            tax_after_rebate * 0.10
        } else {
            0.0
        };

        let cess = (tax_after_rebate + surcharge) * 0.04;
        let total_tax = tax_after_rebate + surcharge + cess;
        let effective_rate = if gross_income > 0.0 {
            total_tax / gross_income * 100.0
        } else {
            0.0
        };

        IncomeTaxResult {
            gross_income,
            standard_deduction: std_deduction,
            taxable_income,
            basic_tax: tax_after_rebate,
            surcharge,
            cess,
            total_tax,
            effective_rate,
            regime,
        }
    }

    fn rebate_87a(&self, taxable_income: f64, regime: TaxRegime) -> f64 {
        let limit = match regime {
            TaxRegime::New => 700000.0,
            TaxRegime::Old => 500000.0,
        };
        if taxable_income <= limit {
            12500.0
        } else {
            0.0
        }
    }

    fn name(&self) -> &'static str {
        "IndiaIncomeTaxCalculator FY2024-25"
    }
}

// ─── UPI Payment Reference Generator ─────────────────────────────────────────

/// UPI Virtual Payment Address
#[derive(Debug, Clone)]
pub struct UpiVpa {
    pub username: String,
    pub bank_handle: String,
}

impl UpiVpa {
    pub fn new(username: &str, bank_handle: &str) -> Self {
        UpiVpa {
            username: username.to_string(),
            bank_handle: bank_handle.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}@{}", self.username, self.bank_handle)
    }
}

/// UPI payment request
#[derive(Debug, Clone)]
pub struct UpiPaymentRequest {
    pub payee_vpa: UpiVpa,
    pub payer_vpa: UpiVpa,
    pub amount: f64,
    pub currency: &'static str,
    pub transaction_note: String,
    pub reference_id: String,
    pub merchant_code: Option<String>,
}

impl UpiPaymentRequest {
    /// Generate UPI deep-link URI (BHIM/GPay/PhonePe compatible)
    pub fn to_upi_uri(&self) -> String {
        format!(
            "upi://pay?pa={}&pn={}&am={:.2}&cu={}&tn={}&tr={}",
            self.payee_vpa.to_string(),
            urlencoded(&self.payee_vpa.username),
            self.amount,
            self.currency,
            urlencoded(&self.transaction_note),
            self.reference_id,
        )
    }

    /// Check if amount is within NPCI daily limit (₹1 lakh standard)
    pub fn within_daily_limit(&self) -> bool {
        self.amount <= 100000.0
    }
}

fn urlencoded(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c {
            ' ' => vec!['+'],
            c if c.is_alphanumeric() || c == '-' || c == '_' => vec![c],
            _ => vec!['_'], // simplified encoding
        })
        .collect()
}

/// OOP interface for UPI integrations
pub trait UpiGateway {
    fn create_payment_request(
        &self,
        payee: UpiVpa,
        payer: UpiVpa,
        amount: f64,
        note: &str,
    ) -> UpiPaymentRequest;
    fn generate_reference_id(&self) -> String;
    fn name(&self) -> &'static str;
}

/// NPCI-compliant UPI gateway implementation
pub struct NpciUpiGateway {
    counter: core::sync::atomic::AtomicU64,
}

impl NpciUpiGateway {
    pub fn new() -> Self {
        NpciUpiGateway {
            counter: core::sync::atomic::AtomicU64::new(1),
        }
    }
}

impl Default for NpciUpiGateway {
    fn default() -> Self {
        Self::new()
    }
}

impl UpiGateway for NpciUpiGateway {
    fn create_payment_request(
        &self,
        payee: UpiVpa,
        payer: UpiVpa,
        amount: f64,
        note: &str,
    ) -> UpiPaymentRequest {
        let ref_id = self.generate_reference_id();
        UpiPaymentRequest {
            payee_vpa: payee,
            payer_vpa: payer,
            amount,
            currency: "INR",
            transaction_note: note.to_string(),
            reference_id: ref_id,
            merchant_code: None,
        }
    }

    fn generate_reference_id(&self) -> String {
        let n = self
            .counter
            .fetch_add(1, core::sync::atomic::Ordering::SeqCst);
        format!("SIGMA{:012}", n)
    }

    fn name(&self) -> &'static str {
        "NpciUpiGateway"
    }
}

// ─── Indic Localization ───────────────────────────────────────────────────────

/// Supported Indian languages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndicLanguage {
    Hindi,
    Bengali,
    Tamil,
    Telugu,
    Gujarati,
    Marathi,
    Kannada,
    Malayalam,
    Punjabi,
    Odia,
    English,
}

impl IndicLanguage {
    pub fn name(&self) -> &'static str {
        match self {
            IndicLanguage::Hindi => "Hindi",
            IndicLanguage::Bengali => "Bengali",
            IndicLanguage::Tamil => "Tamil",
            IndicLanguage::Telugu => "Telugu",
            IndicLanguage::Gujarati => "Gujarati",
            IndicLanguage::Marathi => "Marathi",
            IndicLanguage::Kannada => "Kannada",
            IndicLanguage::Malayalam => "Malayalam",
            IndicLanguage::Punjabi => "Punjabi",
            IndicLanguage::Odia => "Odia",
            IndicLanguage::English => "English",
        }
    }

    pub fn bcp47_code(&self) -> &'static str {
        match self {
            IndicLanguage::Hindi => "hi-IN",
            IndicLanguage::Bengali => "bn-IN",
            IndicLanguage::Tamil => "ta-IN",
            IndicLanguage::Telugu => "te-IN",
            IndicLanguage::Gujarati => "gu-IN",
            IndicLanguage::Marathi => "mr-IN",
            IndicLanguage::Kannada => "kn-IN",
            IndicLanguage::Malayalam => "ml-IN",
            IndicLanguage::Punjabi => "pa-IN",
            IndicLanguage::Odia => "or-IN",
            IndicLanguage::English => "en-IN",
        }
    }
}

/// Indian number formatting (Lakh/Crore system)
pub struct IndianNumberFormatter;

impl IndianNumberFormatter {
    pub fn new() -> Self {
        IndianNumberFormatter
    }

    /// Format number in Indian lakh/crore system
    pub fn format_inr(&self, amount: f64) -> String {
        if amount >= 10_000_000.0 {
            format!("₹{:.2} Cr", amount / 10_000_000.0)
        } else if amount >= 100_000.0 {
            format!("₹{:.2} L", amount / 100_000.0)
        } else if amount >= 1_000.0 {
            format!("₹{:.2} K", amount / 1_000.0)
        } else {
            format!("₹{:.2}", amount)
        }
    }

    /// Format in words (Indian system)
    pub fn amount_in_words(&self, amount: u64) -> String {
        let crore = amount / 10_000_000;
        let lakh = (amount % 10_000_000) / 100_000;
        let thousand = (amount % 100_000) / 1_000;
        let remainder = amount % 1_000;

        let mut parts: Vec<String> = Vec::new();
        if crore > 0 {
            parts.push(format!("{} Crore", crore));
        }
        if lakh > 0 {
            parts.push(format!("{} Lakh", lakh));
        }
        if thousand > 0 {
            parts.push(format!("{} Thousand", thousand));
        }
        if remainder > 0 {
            parts.push(format!("{}", remainder));
        }

        if parts.is_empty() {
            "Zero".to_string()
        } else {
            parts.join(" ")
        }
    }
}

impl Default for IndianNumberFormatter {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Indic Professional Tools ─────────────────────────────────────────────────

/// Indian agricultural tool for Farmers (Krishi)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CropType {
    Wheat,
    Paddy,
    Sugarcane,
    Cotton,
}

pub struct KrishiHelper;

impl KrishiHelper {
    pub fn new() -> Self {
        Self
    }

    /// Estimate fertilizer (NPK) required in kilograms based on crop type and land in Bighas.
    /// Standard recommendation per hectare (1 Hectare = 4 Bighas):
    /// - Wheat: N:120, P:60, K:40 kg
    /// - Paddy: N:100, P:50, K:50 kg
    pub fn estimate_fertilizer(&self, crop: CropType, bighas: f64) -> (f64, f64, f64) {
        let hectares = bighas / 4.0;
        match crop {
            CropType::Wheat => (120.0 * hectares, 60.0 * hectares, 40.0 * hectares),
            CropType::Paddy => (100.0 * hectares, 50.0 * hectares, 50.0 * hectares),
            CropType::Sugarcane => (150.0 * hectares, 80.0 * hectares, 60.0 * hectares),
            CropType::Cotton => (80.0 * hectares, 40.0 * hectares, 40.0 * hectares),
        }
    }

    /// Estimate Minimum Support Price (MSP) in INR based on quintals of harvest (1 Quintal = 100 kg)
    /// Using official MSP rates for FY 2024-25
    pub fn estimate_msp_value(&self, crop: CropType, quintals: f64) -> f64 {
        let rate_per_quintal = match crop {
            CropType::Wheat => 2275.0,     // Wheat MSP
            CropType::Paddy => 2183.0,     // Paddy Common MSP
            CropType::Sugarcane => 315.0,  // Fair and Remunerative Price (FRP)
            CropType::Cotton => 6620.0,    // Cotton Medium Staple MSP
        };
        quintals * rate_per_quintal
    }
}

impl Default for KrishiHelper {
    fn default() -> Self {
        Self::new()
    }
}

/// Indian legal and judicial compliance tool for Advocates (Vakil)
pub struct VakilHelper;

impl VakilHelper {
    pub fn new() -> Self {
        Self
    }

    /// Calculate Court Fee in INR based on claim value as per the Court Fees Act
    /// Slab-based calculation:
    /// - Up to 50,000 INR: 2.5%
    /// - 50,001 to 2,00,000 INR: 1,250 + 5% on excess of 50,000
    /// - Above 2,00,000 INR: 8,750 + 7.5% on excess of 2,00,000 (capped at 1,50,000 INR)
    pub fn calculate_court_fee(&self, claim_value: f64) -> f64 {
        let fee = if claim_value <= 50000.0 {
            claim_value * 0.025
        } else if claim_value <= 200000.0 {
            1250.0 + (claim_value - 50000.0) * 0.05
        } else {
            8750.0 + (claim_value - 200000.0) * 0.075
        };
        fee.min(150000.0)
    }

    /// Look up corresponding BNS (Bharatiya Nyaya Sanhita 2023) section from old IPC (Indian Penal Code) section
    pub fn ipc_to_bns(&self, ipc_section: u32) -> Option<&'static str> {
        match ipc_section {
            302 => Some("Section 101 (Murder)"),
            307 => Some("Section 109 (Attempt to murder)"),
            378 | 379 => Some("Section 303 (Theft)"),
            420 => Some("Section 318 (Cheating)"),
            124 => Some("Section 152 (Acts endangering sovereignty)"),
            498 => Some("Section 85 (Cruelty by husband or relatives)"),
            _ => None,
        }
    }
}

impl Default for VakilHelper {
    fn default() -> Self {
        Self::new()
    }
}

/// Indian retail and merchant billing tool for Traders (Vyapar)
pub struct VyaparHelper;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InvoiceResult {
    pub taxable_value: f64,
    pub cgst_amount: f64,
    pub sgst_amount: f64,
    pub total_amount: f64,
}

impl VyaparHelper {
    pub fn new() -> Self {
        Self
    }

    /// Calculate margin & markup percentages based on cost and selling price
    pub fn margin_markup(&self, cost: f64, selling_price: f64) -> (f64, f64) {
        if cost <= 0.0 || selling_price <= 0.0 || selling_price < cost {
            return (0.0, 0.0);
        }
        let margin = ((selling_price - cost) / selling_price) * 100.0;
        let markup = ((selling_price - cost) / cost) * 100.0;
        (margin, markup)
    }

    /// Calculate CGST and SGST split from gross selling price (GST-inclusive billing)
    pub fn calculate_inclusive_gst(&self, inclusive_price: f64, gst_rate_pct: f64) -> InvoiceResult {
        if inclusive_price <= 0.0 || gst_rate_pct < 0.0 {
            return InvoiceResult {
                taxable_value: 0.0,
                cgst_amount: 0.0,
                sgst_amount: 0.0,
                total_amount: 0.0,
            };
        }
        let divisor = 1.0 + (gst_rate_pct / 100.0);
        let taxable_value = inclusive_price / divisor;
        let total_gst = inclusive_price - taxable_value;
        InvoiceResult {
            taxable_value,
            cgst_amount: total_gst / 2.0,
            sgst_amount: total_gst / 2.0,
            total_amount: inclusive_price,
        }
    }
}

impl Default for VyaparHelper {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gst_intrastate_18() {
        let calc = StandardGstCalculator::new();
        let result = calc.calculate(50000.0, 18.0, GstTransactionType::IntraState);
        assert!((result.cgst_amount - 4500.0).abs() < 0.01);
        assert!((result.sgst_amount - 4500.0).abs() < 0.01);
        assert!((result.grand_total - 59000.0).abs() < 0.01);
    }

    #[test]
    fn test_gst_interstate_18() {
        let calc = StandardGstCalculator::new();
        let result = calc.calculate(100000.0, 18.0, GstTransactionType::InterState);
        assert!((result.igst_amount - 18000.0).abs() < 0.01);
        assert!((result.grand_total - 118000.0).abs() < 0.01);
    }

    #[test]
    fn test_gst_reverse_calculation() {
        let calc = StandardGstCalculator::new();
        let result = calc.reverse_calculate(59000.0, 18.0, GstTransactionType::IntraState);
        assert!((result.base_amount - 50000.0).abs() < 1.0);
        assert!((result.grand_total - 59000.0).abs() < 0.01);
    }

    #[test]
    fn test_gst_export_zero_rated() {
        let calc = StandardGstCalculator::new();
        let result = calc.calculate(75000.0, 18.0, GstTransactionType::Export);
        assert_eq!(result.total_gst, 0.0);
        assert_eq!(result.grand_total, 75000.0);
    }

    #[test]
    fn test_hsn_lookup() {
        let calc = StandardGstCalculator::new();
        assert_eq!(calc.lookup_hsn_rate(8471), Some(18.0));
        assert_eq!(calc.lookup_hsn_rate(4901), Some(0.0));
        assert_eq!(calc.lookup_hsn_rate(99999), None);
    }

    #[test]
    fn test_tds_194j_professional_fees() {
        let calc = IndiaTdsCalculator::new(2024);
        let result =
            calc.calculate_tds(100000.0, TdsSection::Section194J, DeducteeType::Individual);
        assert!((result.tds_amount - 10000.0).abs() < 0.01);
        assert!((result.health_education_cess - 400.0).abs() < 0.01);
        assert!((result.net_payment - 89600.0).abs() < 0.01);
    }

    #[test]
    fn test_tds_194c_individual_vs_company() {
        let calc = IndiaTdsCalculator::new(2024);
        let r_indiv =
            calc.calculate_tds(50000.0, TdsSection::Section194C, DeducteeType::Individual);
        let r_comp = calc.calculate_tds(50000.0, TdsSection::Section194C, DeducteeType::Company);
        assert!((r_indiv.tds_amount - 500.0).abs() < 0.01);
        assert!((r_comp.tds_amount - 1000.0).abs() < 0.01);
    }

    #[test]
    fn test_tds_threshold() {
        let calc = IndiaTdsCalculator::new(2024);
        assert!(!calc.threshold_exceeded(25000.0, TdsSection::Section194J));
        assert!(calc.threshold_exceeded(35000.0, TdsSection::Section194J));
    }

    #[test]
    fn test_income_tax_new_regime_rebate() {
        let calc = IndiaIncomeTaxCalculator::new();
        // Under 7L → full rebate → zero tax
        let result = calc.calculate(600000.0, 0.0, TaxRegime::New);
        assert_eq!(result.total_tax, 0.0);
    }

    #[test]
    fn test_income_tax_new_regime_above_threshold() {
        let calc = IndiaIncomeTaxCalculator::new();
        let result = calc.calculate(1500000.0, 0.0, TaxRegime::New);
        assert!(result.total_tax > 0.0);
        assert!(result.effective_rate > 0.0);
    }

    #[test]
    fn test_income_tax_old_regime() {
        let calc = IndiaIncomeTaxCalculator::new();
        // 10L income, 1.5L deduction (80C), old regime
        let result = calc.calculate(1000000.0, 150000.0, TaxRegime::Old);
        assert!(result.total_tax > 0.0);
    }

    #[test]
    fn test_upi_vpa() {
        let vpa = UpiVpa::new("aaryan", "upi");
        assert_eq!(vpa.to_string(), "aaryan@upi");
    }

    #[test]
    fn test_upi_uri_generation() {
        let gw = NpciUpiGateway::new();
        let payee = UpiVpa::new("merchant", "sbi");
        let payer = UpiVpa::new("customer", "paytm");
        let req = gw.create_payment_request(payee, payer, 599.0, "SigmaOS License");
        let uri = req.to_upi_uri();
        assert!(uri.starts_with("upi://pay?pa=merchant@sbi"));
        assert!(uri.contains("am=599.00"));
    }

    #[test]
    fn test_upi_daily_limit() {
        let req = UpiPaymentRequest {
            payee_vpa: UpiVpa::new("a", "b"),
            payer_vpa: UpiVpa::new("c", "d"),
            amount: 200000.0,
            currency: "INR",
            transaction_note: "Test".to_string(),
            reference_id: "X".to_string(),
            merchant_code: None,
        };
        assert!(!req.within_daily_limit());
    }

    #[test]
    fn test_upi_reference_id_unique() {
        let gw = NpciUpiGateway::new();
        let r1 = gw.generate_reference_id();
        let r2 = gw.generate_reference_id();
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_indian_number_formatter() {
        let fmt = IndianNumberFormatter::new();
        assert!(fmt.format_inr(12500000.0).contains("Cr"));
        assert!(fmt.format_inr(250000.0).contains("L"));
        assert!(fmt.format_inr(1500.0).contains("K"));
    }

    #[test]
    fn test_amount_in_words() {
        let fmt = IndianNumberFormatter::new();
        let words = fmt.amount_in_words(10000000);
        assert!(words.contains("Crore"));
        let words2 = fmt.amount_in_words(250000);
        assert!(words2.contains("Lakh"));
    }

    #[test]
    fn test_indic_language_codes() {
        assert_eq!(IndicLanguage::Hindi.bcp47_code(), "hi-IN");
        assert_eq!(IndicLanguage::Tamil.bcp47_code(), "ta-IN");
        assert_eq!(IndicLanguage::English.bcp47_code(), "en-IN");
    }

    #[test]
    fn test_krishi_helper() {
        let krishi = KrishiHelper::new();
        // 4 bighas = 1 hectare
        let npk = krishi.estimate_fertilizer(CropType::Wheat, 4.0);
        assert_eq!(npk, (120.0, 60.0, 40.0));

        let msp_val = krishi.estimate_msp_value(CropType::Wheat, 10.0);
        assert_eq!(msp_val, 22750.0);
    }

    #[test]
    fn test_vakil_helper() {
        let vakil = VakilHelper::new();
        // 40,000 claim value court fee -> 40,000 * 2.5% = 1,000
        let fee = vakil.calculate_court_fee(40000.0);
        assert_eq!(fee, 1000.0);

        let lookup = vakil.ipc_to_bns(302).unwrap();
        assert!(lookup.contains("Section 101"));
    }

    #[test]
    fn test_vyapar_helper() {
        let vyapar = VyaparHelper::new();
        let (margin, markup) = vyapar.margin_markup(100.0, 125.0);
        assert_eq!(margin, 20.0);
        assert_eq!(markup, 25.0);

        // Inclusive of 18% GST on 118 INR should result in 100 taxable value
        let res = vyapar.calculate_inclusive_gst(118.0, 18.0);
        assert!((res.taxable_value - 100.0).abs() < 1e-5);
        assert!((res.cgst_amount - 9.0).abs() < 1e-5);
    }
}
