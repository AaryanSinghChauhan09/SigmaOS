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

    #[test]
    fn test_banking_cheque_types_and_clearing() {
        let valid_cheque = ChequeValidationRecord {
            cheque_number: 100201,
            micr_code: *b"400240012",
            ifsc_code: *b"SBIN0001234",
            issue_timestamp_secs: 1000000,
            presentation_timestamp_secs: 2000000,
            amount_in_paisa: 500000, // 5000 INR
            cheque_type: ChequeType::AccountPayeeCheque,
            is_mutilated: false,
            is_signature_valid: true,
            is_account_payee_only: true,
        };

        // Valid cheque clearing
        let status = SovereignChequeProcessingEngine::validate_cheque(&valid_cheque);
        assert_eq!(status, ChequeStatus::ValidForClearing);

        // Stale cheque (> 90 days)
        let stale_cheque = ChequeValidationRecord {
            presentation_timestamp_secs: 1000000
                + SovereignChequeProcessingEngine::STALE_PERIOD_SECS
                + 1,
            ..valid_cheque.clone()
        };
        assert_eq!(
            SovereignChequeProcessingEngine::validate_cheque(&stale_cheque),
            ChequeStatus::StaleExpired
        );

        // Post-dated cheque (future issue date)
        let post_dated_cheque = ChequeValidationRecord {
            issue_timestamp_secs: 3000000,
            presentation_timestamp_secs: 2000000,
            ..valid_cheque.clone()
        };
        assert_eq!(
            SovereignChequeProcessingEngine::validate_cheque(&post_dated_cheque),
            ChequeStatus::PostDatedFuture
        );

        // Mutilated cheque
        let mutilated_cheque = ChequeValidationRecord {
            is_mutilated: true,
            ..valid_cheque.clone()
        };
        assert_eq!(
            SovereignChequeProcessingEngine::validate_cheque(&mutilated_cheque),
            ChequeStatus::MutilatedDamaged
        );

        // Test classification of all 9 cheque types
        assert_eq!(
            SovereignChequeProcessingEngine::classify_cheque(
                false, false, true, false, false, false, 100, 100
            ),
            ChequeType::BearerCheque
        );
        assert_eq!(
            SovereignChequeProcessingEngine::classify_cheque(
                false, false, false, false, false, false, 100, 100
            ),
            ChequeType::OrderCheque
        );
        assert_eq!(
            SovereignChequeProcessingEngine::classify_cheque(
                true, false, false, false, false, false, 100, 100
            ),
            ChequeType::CrossedCheque
        );
        assert_eq!(
            SovereignChequeProcessingEngine::classify_cheque(
                true, true, false, false, false, false, 100, 100
            ),
            ChequeType::AccountPayeeCheque
        );
        assert_eq!(
            SovereignChequeProcessingEngine::classify_cheque(
                false,
                false,
                false,
                false,
                false,
                false,
                100,
                100 + SovereignChequeProcessingEngine::STALE_PERIOD_SECS + 10
            ),
            ChequeType::StaleCheque
        );
        assert_eq!(
            SovereignChequeProcessingEngine::classify_cheque(
                false, false, false, false, false, false, 200, 100
            ),
            ChequeType::PostDatedCheque
        );
        assert_eq!(
            SovereignChequeProcessingEngine::classify_cheque(
                false, false, false, true, false, false, 100, 100
            ),
            ChequeType::BankersCheque
        );
        assert_eq!(
            SovereignChequeProcessingEngine::classify_cheque(
                false, false, false, false, true, false, 100, 100
            ),
            ChequeType::SelfCheque
        );
        assert_eq!(
            SovereignChequeProcessingEngine::classify_cheque(
                false, false, false, false, false, true, 100, 100
            ),
            ChequeType::MutilatedCheque
        );
    }
}

/// Types of Banking Cheques supported by Sovereign Banking Clearing Engine
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChequeType {
    BearerCheque = 1,
    OrderCheque = 2,
    CrossedCheque = 3,
    AccountPayeeCheque = 4,
    StaleCheque = 5,
    PostDatedCheque = 6,
    BankersCheque = 7,
    SelfCheque = 8,
    MutilatedCheque = 9,
}

/// Cheque clearing and validation status
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChequeStatus {
    ValidForClearing = 0,
    StaleExpired = 1,
    PostDatedFuture = 2,
    MutilatedDamaged = 3,
    DrawerSignatureMismatch = 4,
    AccountPayeeRestricted = 5,
    InvalidMicrCode = 6,
}

/// Record representing a Banking Cheque presented for clearing
#[derive(Debug, Clone)]
pub struct ChequeValidationRecord {
    pub cheque_number: u32,
    pub micr_code: [u8; 9],
    pub ifsc_code: [u8; 11],
    pub issue_timestamp_secs: u64,
    pub presentation_timestamp_secs: u64,
    pub amount_in_paisa: u64,
    pub cheque_type: ChequeType,
    pub is_mutilated: bool,
    pub is_signature_valid: bool,
    pub is_account_payee_only: bool,
}

/// Sovereign Banking Cheque Processing Engine
pub struct SovereignChequeProcessingEngine;

impl SovereignChequeProcessingEngine {
    /// Three months validity period in seconds (90 days = 90 * 24 * 3600 = 7,776,000s)
    pub const STALE_PERIOD_SECS: u64 = 7_776_000;

    /// Validate and clear a presented cheque
    pub fn validate_cheque(record: &ChequeValidationRecord) -> ChequeStatus {
        // Verify MICR code (9 digits)
        if record.micr_code.iter().any(|&b| !b.is_ascii_digit()) {
            return ChequeStatus::InvalidMicrCode;
        }

        // Verify signature validity
        if !record.is_signature_valid {
            return ChequeStatus::DrawerSignatureMismatch;
        }

        // Verify physical damage / mutilation
        if record.is_mutilated || record.cheque_type == ChequeType::MutilatedCheque {
            return ChequeStatus::MutilatedDamaged;
        }

        // Verify post-dated cheque (issue date in the future)
        if record.issue_timestamp_secs > record.presentation_timestamp_secs {
            return ChequeStatus::PostDatedFuture;
        }

        // Verify stale cheque (issue date older than 90 days / 3 months)
        if record
            .presentation_timestamp_secs
            .saturating_sub(record.issue_timestamp_secs)
            > Self::STALE_PERIOD_SECS
            || record.cheque_type == ChequeType::StaleCheque
        {
            return ChequeStatus::StaleExpired;
        }

        // Verify account payee restriction
        if record.is_account_payee_only && record.cheque_type == ChequeType::BearerCheque {
            return ChequeStatus::AccountPayeeRestricted;
        }

        ChequeStatus::ValidForClearing
    }

    /// Identify cheque classification from attributes
    pub fn classify_cheque(
        is_crossed: bool,
        is_account_payee: bool,
        is_bearer: bool,
        is_banker_issued: bool,
        is_self: bool,
        is_damaged: bool,
        issue_time: u64,
        present_time: u64,
    ) -> ChequeType {
        if is_damaged {
            return ChequeType::MutilatedCheque;
        }
        if issue_time > present_time {
            return ChequeType::PostDatedCheque;
        }
        if present_time.saturating_sub(issue_time) > Self::STALE_PERIOD_SECS {
            return ChequeType::StaleCheque;
        }
        if is_banker_issued {
            return ChequeType::BankersCheque;
        }
        if is_self {
            return ChequeType::SelfCheque;
        }
        if is_account_payee {
            return ChequeType::AccountPayeeCheque;
        }
        if is_crossed {
            return ChequeType::CrossedCheque;
        }
        if is_bearer {
            return ChequeType::BearerCheque;
        }
        ChequeType::OrderCheque
    }
}
