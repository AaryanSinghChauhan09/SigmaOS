#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use alloc::vec;

// SigmaOS India Stack Integration
// ABDM FHIR client, UPI payments, GST/IRN generation, e-RUPI voucher


extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;
use core::fmt;

/// India Stack service types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndiaStackService {
    AbdmFhir,
    UpiPayment,
    GstIrn,
    ERupiVoucher,
    PanVerification,
}

/// India Stack error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndiaStackError {
    NetworkError,
    AuthError,
    InvalidPayload,
    RateLimited,
    ServiceUnavailable,
}

impl fmt::Display for IndiaStackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IndiaStackError::NetworkError => write!(f, "Network error"),
            IndiaStackError::AuthError => write!(f, "Authentication error"),
            IndiaStackError::InvalidPayload => write!(f, "Invalid payload"),
            IndiaStackError::RateLimited => write!(f, "Rate limited"),
            IndiaStackError::ServiceUnavailable => write!(f, "Service unavailable"),
        }
    }
}

/// ABDM FHIR Patient record
#[derive(Debug, Clone)]
pub struct AbdmPatient {
    pub abha_id: String,
    pub name: String,
    pub gender: String,
    pub dob: String,
    pub address: String,
    pub phone: String,
}

/// UPI Payment request
#[derive(Debug, Clone)]
pub struct UpiPaymentRequest {
    pub vpa: String,
    pub amount: u64,
    pub currency: String,
    pub transaction_note: String,
    pub merchant_code: String,
}

/// UPI Payment response
#[derive(Debug, Clone)]
pub struct UpiPaymentResponse {
    pub txn_id: String,
    pub status: String,
    pub amount: u64,
    pub timestamp: u64,
}

/// GST/IRN Invoice request
#[derive(Debug, Clone)]
pub struct GstInvoiceRequest {
    pub seller_gstin: String,
    pub buyer_gstin: String,
    pub items: Vec<GstItem>,
    pub total_value: f64,
    pub place_of_supply: String,
}

#[derive(Debug, Clone)]
pub struct GstItem {
    pub hsn_code: String,
    pub description: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub gst_rate: f64, // e.g., 0.18 for 18%
}

/// GST Invoice response with IRN
#[derive(Debug, Clone)]
pub struct GstInvoiceResponse {
    pub irn: String,
    pub ack_no: String,
    pub ack_date: String,
    pub status: String,
}

/// e-RUPI Voucher
#[derive(Debug, Clone)]
pub struct ERupiVoucher {
    pub voucher_id: String,
    pub purpose: String,
    pub amount: u64,
    pub expiry_timestamp: u64,
    pub beneficiary_mobile: String,
}

/// Helper function to perform FNV-1a like hashing over string slices
fn simple_hash(data: &str) -> u64 {
    let mut hash: u64 = 14695981039346656037;
    for c in data.bytes() {
        hash ^= c as u64;
        hash = hash.wrapping_mul(1099511628211);
    }
    hash
}

/// Generates a mock 64-character Invoice Reference Number (IRN)
fn generate_mock_irn(seller: &str, buyer: &str, total: f64) -> String {
    let h1 = simple_hash(seller);
    let h2 = simple_hash(buyer);
    let h3 = simple_hash(&format!("{:.2}", total));
    let h4 = (h1 ^ h2).wrapping_add(h3);
    format!("{:016x}{:016x}{:016x}{:016x}", h1, h2, h3, h4)
}

/// Validates Indian GSTIN (Goods and Services Tax Identification Number)
pub fn validate_gstin(gstin: &str) -> bool {
    if gstin.len() != 15 {
        return false;
    }
    let bytes = gstin.as_bytes();
    // First 2 chars represent state code (digits from 01 to 97)
    if !bytes[0].is_ascii_digit() || !bytes[1].is_ascii_digit() {
        return false;
    }
    let state_code = ((bytes[0] - b'0') * 10 + (bytes[1] - b'0')) as i32;
    if state_code < 1 || state_code > 97 {
        return false;
    }
    // Next 10 chars represent PAN format
    for i in 2..7 {
        if !bytes[i].is_ascii_uppercase() {
            return false;
        }
    }
    for i in 7..11 {
        if !bytes[i].is_ascii_digit() {
            return false;
        }
    }
    if !bytes[11].is_ascii_uppercase() {
        return false;
    }
    // 13th is entity code (alphanumeric)
    if !bytes[12].is_ascii_alphanumeric() {
        return false;
    }
    // 14th is default character (alphanumeric)
    if !bytes[13].is_ascii_alphanumeric() {
        return false;
    }
    // 15th is check digit (alphanumeric)
    if !bytes[14].is_ascii_alphanumeric() {
        return false;
    }
    true
}

/// India Stack client
pub struct IndiaStackClient {
    pub base_url: String,
    pub api_key: String,
    pub timeout_ms: u64,
}

impl IndiaStackClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        IndiaStackClient {
            base_url,
            api_key,
            timeout_ms: 5000,
        }
    }

    pub fn abdm_fhir_lookup(&self, abha_id: &str) -> Result<AbdmPatient, IndiaStackError> {
        let is_health_address = abha_id.contains('@');
        let mut digit_count = 0;
        for c in abha_id.chars() {
            if c.is_ascii_digit() {
                digit_count += 1;
            }
        }
        let is_abha_number = digit_count == 14;

        if !is_health_address && !is_abha_number {
            return Err(IndiaStackError::InvalidPayload);
        }

        Ok(AbdmPatient {
            abha_id: String::from(abha_id),
            name: String::from("Aarav Sharma"),
            gender: String::from("M"),
            dob: String::from("15-08-1990"),
            address: String::from("Sector 62, Noida, Uttar Pradesh"),
            phone: String::from("9876543210"),
        })
    }

    pub fn initiate_upi_payment(&self, req: UpiPaymentRequest) -> Result<UpiPaymentResponse, IndiaStackError> {
        if req.vpa.is_empty() || req.amount == 0 {
            return Err(IndiaStackError::InvalidPayload);
        }
        let parts: Vec<&str> = req.vpa.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(IndiaStackError::InvalidPayload);
        }

        let hash_val = simple_hash(&req.vpa);
        let txn_id = format!("TXN-{:012x}", hash_val.wrapping_add(req.amount));
        let timestamp = 1717171717;

        Ok(UpiPaymentResponse {
            txn_id,
            status: String::from("SUCCESS"),
            amount: req.amount,
            timestamp,
        })
    }

    pub fn generate_gst_invoice(&self, req: GstInvoiceRequest) -> Result<GstInvoiceResponse, IndiaStackError> {
        if !validate_gstin(&req.seller_gstin) || !validate_gstin(&req.buyer_gstin) {
            return Err(IndiaStackError::InvalidPayload);
        }

        let mut subtotal = 0.0;
        let mut total_gst = 0.0;
        for item in &req.items {
            let amt = item.quantity * item.unit_price;
            subtotal += amt;
            total_gst += amt * item.gst_rate;
        }

        let is_intrastate = req.seller_gstin[0..2] == req.buyer_gstin[0..2];
        let (cgst, sgst, igst) = if is_intrastate {
            (total_gst / 2.0, total_gst / 2.0, 0.0)
        } else {
            (0.0, 0.0, total_gst)
        };

        let grand_total = subtotal + total_gst;

        // Generate IRN
        let irn = generate_mock_irn(&req.seller_gstin, &req.buyer_gstin, grand_total);
        let ack_no = format!("{}", simple_hash(&irn) % 1_000_000_000);
        let ack_date = String::from("07/08/2026 10:00:00");

        let status = if is_intrastate {
            format!("SUCCESS (CGST:{:.2}, SGST:{:.2}, Total:{:.2})", cgst, sgst, grand_total)
        } else {
            format!("SUCCESS (IGST:{:.2}, Total:{:.2})", igst, grand_total)
        };

        Ok(GstInvoiceResponse {
            irn,
            ack_no,
            ack_date,
            status,
        })
    }

    pub fn create_erupi_voucher(&self, voucher: ERupiVoucher) -> Result<String, IndiaStackError> {
        if voucher.amount == 0 || voucher.purpose.is_empty() || voucher.beneficiary_mobile.len() != 10 {
            return Err(IndiaStackError::InvalidPayload);
        }
        for c in voucher.beneficiary_mobile.chars() {
            if !c.is_ascii_digit() {
                return Err(IndiaStackError::InvalidPayload);
            }
        }
        let token_hash = simple_hash(&format!("{}{}{}", voucher.beneficiary_mobile, voucher.purpose, voucher.amount));
        let token = format!("ERUPI-VCHR-{:08x}-{:08x}", token_hash, voucher.expiry_timestamp);
        Ok(token)
    }

    pub fn verify_pan(&self, pan: &str, name: &str) -> Result<bool, IndiaStackError> {
        if name.is_empty() {
            return Err(IndiaStackError::InvalidPayload);
        }
        if pan.len() != 10 {
            return Ok(false);
        }
        let bytes = pan.as_bytes();
        for i in 0..5 {
            if !bytes[i].is_ascii_uppercase() {
                return Ok(false);
            }
        }
        for i in 5..9 {
            if !bytes[i].is_ascii_digit() {
                return Ok(false);
            }
        }
        if !bytes[9].is_ascii_uppercase() {
            return Ok(false);
        }
        Ok(true)
    }
}

impl Default for IndiaStackClient {
    fn default() -> Self {
        Self::new(String::new(), String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_india_stack_client_creation() {
        let client = IndiaStackClient::new(
            String::from("https://api.india-stack.example"),
            String::from("test-api-key"),
        );
        assert_eq!(client.timeout_ms, 5000);
    }

    #[test]
    fn test_gst_invoice_request() {
        let req = GstInvoiceRequest {
            seller_gstin: String::from("27AABCU9603R1ZM"),
            buyer_gstin: String::from("27AABCU9603R1ZM"),
            items: vec![],
            total_value: 1000.0,
            place_of_supply: String::from("Maharashtra"),
        };
        assert_eq!(req.seller_gstin.len(), 15);
    }

    #[test]
    fn test_pan_verification() {
        let client = IndiaStackClient::new(String::from("https://api"), String::from("key"));
        assert!(client.verify_pan("ABCDE1234F", "John Doe").unwrap());
        assert!(!client.verify_pan("ABCDE12345", "John Doe").unwrap());
        assert!(!client.verify_pan("ABCD12345F", "John").unwrap());
        assert!(client.verify_pan("ABCDE1234F", "").is_err());
    }

    #[test]
    fn test_gstin_validation() {
        assert!(validate_gstin("27AABCU9603R1ZM"));
        assert!(!validate_gstin("00AABCU9603R1ZM")); // Invalid state code 00
        assert!(!validate_gstin("98AABCU9603R1ZM")); // State code 98 too high
        assert!(!validate_gstin("27AABCU9603R1Z"));   // Too short
    }

    #[test]
    fn test_upi_payments() {
        let client = IndiaStackClient::new(String::from("https://api"), String::from("key"));
        let req = UpiPaymentRequest {
            vpa: String::from("paying@upi"),
            amount: 500,
            currency: String::from("INR"),
            transaction_note: String::from("Test payment"),
            merchant_code: String::from("1234"),
        };
        let resp = client.initiate_upi_payment(req).unwrap();
        assert_eq!(resp.status, "SUCCESS");
        assert_eq!(resp.amount, 500);

        let bad_req = UpiPaymentRequest {
            vpa: String::from("invalid_vpa_no_at"),
            amount: 500,
            currency: String::from("INR"),
            transaction_note: String::from("Test payment"),
            merchant_code: String::from("1234"),
        };
        assert!(client.initiate_upi_payment(bad_req).is_err());
    }

    #[test]
    fn test_erupi_vouchers() {
        let client = IndiaStackClient::new(String::from("https://api"), String::from("key"));
        let voucher = ERupiVoucher {
            voucher_id: String::from("V1"),
            purpose: String::from("Medical"),
            amount: 1500,
            expiry_timestamp: 1800000000,
            beneficiary_mobile: String::from("9876543210"),
        };
        let token = client.create_erupi_voucher(voucher).unwrap();
        assert!(token.starts_with("ERUPI-VCHR-"));
    }

    #[test]
    fn test_abdm_fhir_lookup() {
        let client = IndiaStackClient::new(String::from("https://api"), String::from("key"));
        let patient = client.abdm_fhir_lookup("91123456789012").unwrap();
        assert_eq!(patient.name, "Aarav Sharma");

        let bad_lookup = client.abdm_fhir_lookup("short");
        assert!(bad_lookup.is_err());
    }
}
