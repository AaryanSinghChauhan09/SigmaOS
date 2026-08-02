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

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// India Stack Integration Suite for SigmaOS
/// Provides sovereign Indian technology stack features including Mock UPI Payments,
/// GST Tax calculation engine, and Multilingual support for major Indic languages.
use core::sync::atomic::{AtomicUsize, Ordering};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndiaStackError {
    Success = 0,
    InvalidUpiId = 1,
    InsufficientFunds = 2,
    InvalidGstRate = 3,
    TranslationNotFound = 4,
}

/// Represents a Mock Unified Payments Interface (UPI) payment engine
pub struct MockUPIService {
    pub balance: AtomicUsize,
}

impl MockUPIService {
    pub fn new(initial_balance: usize) -> Self {
        MockUPIService {
            balance: AtomicUsize::new(initial_balance),
        }
    }

    /// Sign and execute a secure UPI transaction via Virtual Payment Address (VPA)
    pub fn execute_transaction(
        &self,
        from_vpa: &[u8],
        to_vpa: &[u8],
        amount: usize,
    ) -> Result<[u8; 16], IndiaStackError> {
        // Validate VPA format containing '@'
        if !from_vpa.contains(&b'@') || !to_vpa.contains(&b'@') {
            return Err(IndiaStackError::InvalidUpiId);
        }

        let current_bal = self.balance.load(Ordering::SeqCst);
        if current_bal < amount {
            return Err(IndiaStackError::InsufficientFunds);
        }

        self.balance.store(current_bal - amount, Ordering::SeqCst);

        // Generate a mock secure transaction signature block
        let mut sig = [0u8; 16];
        for i in 0..16 {
            sig[i] = ((amount ^ i) & 0xFF) as u8;
        }

        Ok(sig)
    }

    /// Generate a raw UPI QR string (e.g., upi://pay?pa=recipient@upi&am=amount)
    pub fn generate_upi_qr(
        &self,
        recipient_vpa: &[u8],
        amount: usize,
        qr_buffer: &mut [u8],
    ) -> Result<usize, IndiaStackError> {
        if !recipient_vpa.contains(&b'@') {
            return Err(IndiaStackError::InvalidUpiId);
        }

        let prefix = b"upi://pay?pa=";
        let mid = b"&am=";

        let mut idx = 0;

        // Copy prefix
        for &b in prefix {
            if idx < qr_buffer.len() {
                qr_buffer[idx] = b;
                idx += 1;
            }
        }

        // Copy VPA
        for &b in recipient_vpa {
            if idx < qr_buffer.len() {
                qr_buffer[idx] = b;
                idx += 1;
            }
        }

        // Copy middle delimiter
        for &b in mid {
            if idx < qr_buffer.len() {
                qr_buffer[idx] = b;
                idx += 1;
            }
        }

        // Copy amount as string
        let mut temp = amount;
        let mut digits = [0u8; 10];
        let mut d_idx = 0;
        if temp == 0 {
            digits[0] = b'0';
            d_idx = 1;
        } else {
            while temp > 0 {
                digits[d_idx] = (b'0' + (temp % 10) as u8);
                temp /= 10;
                d_idx += 1;
            }
        }

        for i in (0..d_idx).rev() {
            if idx < qr_buffer.len() {
                qr_buffer[idx] = digits[i];
                idx += 1;
            }
        }

        Ok(idx)
    }
}

/// Robust GST Tax calculation engine separating CGST, SGST, and IGST parts
pub struct GstCalculator;

impl GstCalculator {
    /// Calculate GST details based on basic cost and rate slab (5%, 12%, 18%, 28%)
    pub fn calculate_gst(
        basic_cost: f64,
        rate_percentage: u32,
        is_interstate: bool,
    ) -> Result<(f64, f64, f64), IndiaStackError> {
        match rate_percentage {
            5 | 12 | 18 | 28 => {}
            _ => return Err(IndiaStackError::InvalidGstRate),
        }

        let total_gst = basic_cost * (rate_percentage as f64 / 100.0);

        if is_interstate {
            // IGST applies completely
            Ok((0.0f64, 0.0f64, total_gst))
        } else {
            // Split equally between CGST and SGST
            let cgst = total_gst / 2.0;
            let sgst = total_gst / 2.0;
            Ok((cgst, sgst, 0.0f64))
        }
    }
}

/// Sovereign Indic Multilingual translations support
pub struct MultilingualSupport;

impl MultilingualSupport {
    /// Translate a key like "welcome" or "login" into Indic languages
    pub fn translate(lang_code: &[u8], key: &[u8]) -> Result<&'static [u8], IndiaStackError> {
        match lang_code {
            b"hi" => {
                // Hindi
                match key {
                    b"welcome" => Ok(b"\xe0\xa4\xb8\xe0\xa5\x8d\xe0\xa4\xb5\xe0\xa4\xbe\xe0\xa4\x97\xe0\xa4\xa4 \xe0\xa4\xb9\xe0\xa5\x8d\xe0\xa4\xae"), // स्वागत है
                    b"login" => Ok(b"\xe0\xa4\xb2\xe0\xa5\x89\xe0\xa4\x97\xe0\xa4\x87\xe0\xa4\xa8"), // लॉगिन
                    _ => Err(IndiaStackError::TranslationNotFound),
                }
            }
            b"ta" => {
                // Tamil
                match key {
                    b"welcome" => Ok(b"\xe0\xae\xb5\xe0\xae\xb0\xe0\xae\xb5\xe0\xae\xb1\xe0\xaf\x8d\xe0\xae\xaa\xe0\xaf\x81"), // வரவேற்பு
                    b"login" => Ok(b"\xe0\xae\xaa\xe0\xae\xaa\xe0\xae\xbf\xe0\xae\xb5\xe0\xae\xbf\xe0\xae\xb1\xe0\xae\x95\xe0\xaf\x8d\xe0\xae\x95\xe0\xae\xae\xe0\xaf\x8d"), // பதிவிறக்கம்/நுழைவு
                    _ => Err(IndiaStackError::TranslationNotFound),
                }
            }
            b"sa" => {
                // Sanskrit
                match key {
                    b"welcome" => Ok(b"\xe0\xa4\xb8\xe0\xa5\x8d\xe0\xa4\xb5\xe0\xa4\xbe\xe0\xa4\x97\xe0\xa4\xa4\xe0\xa4\xae\xe0\xa5\x8d"), // स्वागतम्
                    b"login" => Ok(b"\xe0\xa4\xaa\xe0\xa5\x8d\xe0\xa4\xb0\xe0\xa4\xb5\xe0\xa5\x87\xe0\xa4\xb6\xe0\xa4\x83"), // प्रवेशः
                    _ => Err(IndiaStackError::TranslationNotFound),
                }
            }
            _ => Err(IndiaStackError::TranslationNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upi_payments_and_qr() {
        let upi = MockUPIService::new(5000);
        let sig = upi
            .execute_transaction(b"sender@okhdfcbank", b"receiver@okaxis", 1500)
            .unwrap();
        assert_eq!(sig.len(), 16);
        assert_eq!(upi.balance.load(Ordering::SeqCst), 3500);

        let mut qr_buf = [0u8; 128];
        let bytes_written = upi
            .generate_upi_qr(b"recipient@upi", 1500, &mut qr_buf)
            .unwrap();
        assert_eq!(&qr_buf[..26], b"upi://pay?pa=recipient@upi");
    }

    #[test]
    fn test_gst_calculations() {
        let (cgst, sgst, igst) = GstCalculator::calculate_gst(1000.0, 18, false).unwrap();
        assert_eq!(cgst, 90.0);
        assert_eq!(sgst, 90.0);
        assert_eq!(igst, 0.0);

        let (cgst_in, sgst_in, igst_in) = GstCalculator::calculate_gst(1000.0, 18, true).unwrap();
        assert_eq!(cgst_in, 0.0);
        assert_eq!(sgst_in, 0.0);
        assert_eq!(igst_in, 180.0);
    }

    #[test]
    fn test_multilingual_indic() {
        let hi_welcome = MultilingualSupport::translate(b"hi", b"welcome").unwrap();
        assert!(hi_welcome.len() > 0);

        let ta_welcome = MultilingualSupport::translate(b"ta", b"welcome").unwrap();
        assert!(ta_welcome.len() > 0);
    }
}
