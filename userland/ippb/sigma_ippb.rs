// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/ippb/sigma_ippb.rs — India Post Banking (IPPB)
// Implements integration with India Post Payments Bank
//
// Features:
//   - 650 million rural Indians — closest bank is the post office.
//   - IPPB API in sigma-ultra
//   - DOP savings schemes: NSC, PPF, SSY, KVP
//   - AePS (Aadhaar-enabled Payment System) for cash withdrawal
//   - Grameen Dak Sewak doorstep banking integration
//
// Language: Rust

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── IPPB Account ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IppbAccount {
    pub account_number: String,
    pub customer_id: String,
    pub name: String,
    pub aadhaar: String,
    pub mobile: String,
    pub account_type: String,  // Savings, Current
    pub balance: f64,
    pub account_status: String,
    pub opening_date: String,
    pub branch: String,
}

// ── Savings Scheme ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavingsScheme {
    pub scheme_id: String,
    pub account_number: String,
    pub scheme_type: SchemeType,
    pub principal_amount: f64,
    pub interest_rate: f64,
    pub maturity_date: String,
    pub current_value: f64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchemeType {
    NSC,  // National Savings Certificate
    PPF,  // Public Provident Fund
    SSY,  // Sukanya Samriddhi Yojana
    KVP,  // Kisan Vikas Patra
    RD,   // Recurring Deposit
    TD,   // Term Deposit
}

// ── AePS Transaction ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AepsTransaction {
    pub transaction_id: String,
    pub aadhaar: String,
    pub bank_name: String,
    pub transaction_type: String,  // Balance Inquiry, Cash Withdrawal, Mini Statement
    pub amount: f64,
    pub balance: f64,
    pub timestamp: String,
    pub status: String,
}

// ── Doorstep Banking (Grameen Dak Sewak) ───────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoorstepService {
    pub service_id: String,
    pub customer_id: String,
    pub service_type: String,  // Cash Deposit, Cash Withdrawal, Bill Payment
    pub amount: f64,
    pub location: String,
    pub scheduled_date: String,
    pub gds_id: String,
    pub status: String,
}

// ── Bill Payment ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillPayment {
    pub payment_id: String,
    pub account_number: String,
    pub biller: String,
    pub biller_id: String,
    pub consumer_number: String,
    pub amount: f64,
    pub due_date: String,
    pub payment_date: String,
    pub status: String,
}

// ── IPPB Client ─────────────────────────────────────────────────────

pub struct IppbClient {
    base_url: String,
    api_key: String,
    customer_id: Option<String>,
}

impl IppbClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            base_url,
            api_key,
            customer_id: None,
        }
    }

    pub fn set_customer_id(&mut self, customer_id: String) {
        self.customer_id = Some(customer_id);
    }

    /// Get account balance
    pub fn get_balance(&self, account_number: &str) -> Result<f64, String> {
        // In production: Make HTTP GET request to IPPB API
        // For now: Return mock balance
        Ok(25000.0)
    }

    /// Get account details
    pub fn get_account(&self, account_number: &str) -> Result<IppbAccount, String> {
        // In production: Make HTTP GET request to IPPB API
        // For now: Return mock account
        Ok(IppbAccount {
            account_number: account_number.to_string(),
            customer_id: "CUST001".to_string(),
            name: "Customer Name".to_string(),
            aadhaar: "1234-5678-9012".to_string(),
            mobile: "+919876543210".to_string(),
            account_type: "Savings".to_string(),
            balance: 25000.0,
            account_status: "Active".to_string(),
            opening_date: "2020-01-15".to_string(),
            branch: "Post Office 123456".to_string(),
        })
    }

    /// Open savings scheme
    pub fn open_scheme(&self, scheme_type: SchemeType, principal_amount: f64, tenure_years: u32) -> Result<String, String> {
        // In production: Make HTTP POST request to IPPB API
        // For now: Return mock scheme ID
        Ok(format!("SCHEME_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Get savings schemes
    pub fn get_schemes(&self, customer_id: &str) -> Result<Vec<SavingsScheme>, String> {
        // In production: Make HTTP GET request to IPPB API
        // For now: Return mock schemes
        Ok(vec![
            SavingsScheme {
                scheme_id: "SCHEME001".to_string(),
                account_number: "NSC123456".to_string(),
                scheme_type: SchemeType::NSC,
                principal_amount: 100000.0,
                interest_rate: 7.7,
                maturity_date: "2029-01-15".to_string(),
                current_value: 145000.0,
                status: "Active".to_string(),
            },
            SavingsScheme {
                scheme_id: "SCHEME002".to_string(),
                account_number: "PPF123456".to_string(),
                scheme_type: SchemeType::PPF,
                principal_amount: 150000.0,
                interest_rate: 7.1,
                maturity_date: "2035-01-15".to_string(),
                current_value: 250000.0,
                status: "Active".to_string(),
            },
        ])
    }

    /// Perform AePS transaction
    pub fn perform_aeps(&self, aadhaar: &str, transaction_type: &str, amount: f64) -> Result<AepsTransaction, String> {
        // In production: Make HTTP POST request to AePS API
        // For now: Return mock transaction
        Ok(AepsTransaction {
            transaction_id: format!("AEP_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            aadhaar: aadhaar.to_string(),
            bank_name: "India Post Payments Bank".to_string(),
            transaction_type: transaction_type.to_string(),
            amount,
            balance: 25000.0 - amount,
            timestamp: chrono::Utc::now().to_rfc3339(),
            status: "Success".to_string(),
        })
    }

    /// Request doorstep banking service
    pub fn request_doorstep_service(&self, customer_id: &str, service_type: &str, amount: f64, location: &str) -> Result<String, String> {
        // In production: Make HTTP POST request to GDS API
        // For now: Return mock service ID
        Ok(format!("GDS_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Pay bill
    pub fn pay_bill(&self, account_number: &str, biller: &str, consumer_number: &str, amount: f64) -> Result<String, String> {
        // In production: Make HTTP POST request to bill payment API
        // For now: Return mock payment ID
        Ok(format!("BILL_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Get bill payment history
    pub fn get_bill_history(&self, account_number: &str) -> Result<Vec<BillPayment>, String> {
        // In production: Make HTTP GET request to IPPB API
        // For now: Return mock history
        Ok(vec![
            BillPayment {
                payment_id: "BILL001".to_string(),
                account_number: account_number.to_string(),
                biller: "Electricity Board".to_string(),
                biller_id: "ELEC001".to_string(),
                consumer_number: "1234567890".to_string(),
                amount: 1500.0,
                due_date: "2024-07-10".to_string(),
                payment_date: "2024-07-05".to_string(),
                status: "Paid".to_string(),
            },
        ])
    }

    /// Transfer funds
    pub fn transfer_funds(&self, from_account: &str, to_account: &str, amount: f64, remark: &str) -> Result<String, String> {
        // In production: Make HTTP POST request to IPPB API
        // For now: Return mock transaction ID
        Ok(format!("TXN_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn ippb_client_create(base_url: *const u8, base_url_len: usize,
                                    api_key: *const u8, api_key_len: usize) -> *mut IppbClient {
    unsafe {
        let base_url = String::from_utf8_unchecked(
            std::slice::from_raw_parts(base_url, base_url_len));
        let api_key = String::from_utf8_unchecked(
            std::slice::from_raw_parts(api_key, api_key_len));
        Box::into_raw(Box::new(IppbClient::new(base_url, api_key)))
    }
}

#[no_mangle]
pub extern "C" fn ippb_client_destroy(client: *mut IppbClient) {
    unsafe {
        if !client.is_null() {
            let _ = Box::from_raw(client);
        }
    }
}

#[no_mangle]
pub extern "C" fn ippb_get_balance(client: *const IppbClient,
                                 account_no: *const u8, acc_len: usize) -> f64 {
    unsafe {
        if client.is_null() || account_no.is_null() { return -1.0; }
        let account_no = String::from_utf8_unchecked(
            std::slice::from_raw_parts(account_no, acc_len));
        match (*client).get_balance(&account_no) {
            Ok(balance) => balance,
            Err(_) => -1.0,
        }
    }
}
