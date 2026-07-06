// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/msme/sigma_msme.rs — MSME (Micro, Small & Medium Enterprises) Platform
// Implements integration with Indian MSME government schemes and services
//
// Features:
//   - Udyam Registration portal integration
//   - GeM (Government e-Marketplace) seller management
//   - TReDS invoice discounting
//   - SIDBI loan application (OCEN framework)
//   - PLI (Production-Linked Incentive) scheme tracker
//   - Startup India DPIIT recognition
//   - MSME Sambandh public procurement compliance
//
// Language: Rust

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Udyam Registration ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UdyamRegistration {
    pub udyam_number: String,
    pub enterprise_name: String,
    pub enterprise_type: String,  // Micro/Small/Medium
    pub pan: String,
    pub aadhaar: String,
    pub date_of_incorporation: String,
    pub date_of_commencement: String,
    pub address: Address,
    pub activity: String,
    pub nic_code: String,
    pub investment: f64,
    pub turnover: f64,
    pub women_owned: bool,
    pub social_category: String,
    pub registration_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub line1: String,
    pub line2: String,
    pub city: String,
    pub district: String,
    pub state: String,
    pub pincode: String,
}

// ── GeM (Government e-Marketplace) ───────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GemSellerProfile {
    pub seller_id: String,
    pub company_name: String,
    gstin: String,
    pan: String,
    email: String,
    phone: String,
    address: Address,
    product_categories: Vec<String>,
    rating: f32,
    total_orders: u32,
    total_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GemOrder {
    pub order_id: String,
    pub buyer_name: String,
    pub product_name: String,
    pub quantity: u32,
    pub unit_price: f64,
    pub total_value: f64,
    pub order_date: String,
    pub delivery_date: String,
    pub status: String,
}

// ── TReDS (Trade Receivables Discounting System) ───────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TredsInvoice {
    pub invoice_id: String,
    pub invoice_number: String,
    pub invoice_date: String,
    pub due_date: String,
    pub buyer_name: String,
    pub seller_name: String,
    pub invoice_amount: f64,
    pub discount_rate: f64,
    pub discounted_amount: f64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TredsRequest {
    pub invoice: TredsInvoice,
    pub discount_request: f64,
    pub tenure_days: u32,
}

// ── SIDBI Loan (OCEN Framework) ───────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcenLoanRequest {
    pub msme_id: String,
    pub loan_amount: f64,
    pub purpose: String,
    pub tenure_months: u32,
    pub collateral_type: String,
    pub collateral_value: f64,
    pub financials: Financials,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Financials {
    pub annual_turnover: f64,
    pub net_profit: f64,
    pub total_assets: f64,
    pub total_liabilities: f64,
    pub debt_equity_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcenLoanResponse {
    pub loan_id: String,
    pub approved_amount: f64,
    pub interest_rate: f64,
    pub tenure_months: u32,
    pub emi: f64,
    pub status: String,
}

// ── PLI Scheme Tracker ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PliScheme {
    pub scheme_name: String,
    pub sector: String,
    pub incentive_type: String,
    pub eligibility_criteria: String,
    pub incentive_rate: f64,
    pub tenure_years: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PliApplication {
    pub application_id: String,
    pub scheme_name: String,
    pub company_name: String,
    pub application_date: String,
    pub status: String,
    pub approved_incentive: Option<f64>,
}

// ── Startup India DPIIT Recognition ─────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupRecognition {
    pub dpiit_number: String,
    pub startup_name: String,
    pub incorporation_date: String,
    pub sector: String,
    pub description: String,
    pub founders: Vec<Founder>,
    pub recognition_date: String,
    pub tax_exemption_eligible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Founder {
    pub name: String,
    pub din: String,
    pub pan: String,
    pub percentage_ownership: f64,
}

// ── MSME Client ───────────────────────────────────────────────────────────

pub struct MsmeClient {
    base_url: String,
    api_key: String,
    company_id: Option<String>,
}

impl MsmeClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            base_url,
            api_key,
            company_id: None,
        }
    }

    pub fn set_company_id(&mut self, company_id: String) {
        self.company_id = Some(company_id);
    }

    /// Register with Udyam
    pub fn register_udyam(&self, registration: &UdyamRegistration) -> Result<String, String> {
        // In production: Make HTTP POST request to Udyam API
        // For now: Return mock Udyam number
        Ok(format!("UDYAM-MH-00-{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Get GeM seller profile
    pub fn get_gem_profile(&self, seller_id: &str) -> Result<GemSellerProfile, String> {
        // In production: Make HTTP GET request to GeM API
        // For now: Return mock profile
        Ok(GemSellerProfile {
            seller_id: seller_id.to_string(),
            company_name: "SigmaOS Technologies Pvt Ltd".to_string(),
            gstin: "27AAAC1234F1Z9".to_string(),
            pan: "AAAC1234F".to_string(),
            email: "contact@sigmaos.dev".to_string(),
            phone: "+919876543210".to_string(),
            address: Address {
                line1: "123 Tech Park".to_string(),
                line2: "Electronic City".to_string(),
                city: "Bengaluru".to_string(),
                district: "Bengaluru Urban".to_string(),
                state: "Karnataka".to_string(),
                pincode: "560100".to_string(),
            },
            product_categories: vec![
                "Software".to_string(),
                "IT Services".to_string(),
                "Cloud Computing".to_string(),
            ],
            rating: 4.8,
            total_orders: 150,
            total_value: 25000000.0,
        })
    }

    /// Submit invoice to TReDS for discounting
    pub fn submit_treds_invoice(&self, request: &TredsRequest) -> Result<TredsInvoice, String> {
        // In production: Make HTTP POST request to TReDS API
        // For now: Return mock discounted invoice
        let discounted_amount = request.invoice.invoice_amount * (1.0 - request.discount_rate / 100.0);
        Ok(TredsInvoice {
            invoice_id: format!("TREDS_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            invoice_number: request.invoice.invoice_number.clone(),
            invoice_date: request.invoice.invoice_date.clone(),
            due_date: request.invoice.due_date.clone(),
            buyer_name: request.invoice.buyer_name.clone(),
            seller_name: request.invoice.seller_name.clone(),
            invoice_amount: request.invoice.invoice_amount,
            discount_rate: request.discount_rate,
            discounted_amount,
            status: "Approved".to_string(),
        })
    }

    /// Apply for OCEN loan
    pub fn apply_ocen_loan(&self, request: &OcenLoanRequest) -> Result<OcenLoanResponse, String> {
        // In production: Make HTTP POST request to OCEN API
        // For now: Return mock loan response
        let emi = Self::calculate_emi(request.loan_amount, 12.0, request.tenure_months);
        Ok(OcenLoanResponse {
            loan_id: format!("OCEN_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            approved_amount: request.loan_amount,
            interest_rate: 12.0,
            tenure_months: request.tenure_months,
            emi,
            status: "Approved".to_string(),
        })
    }

    /// Calculate EMI
    fn calculate_emi(principal: f64, annual_rate: f64, months: u32) -> f64 {
        let monthly_rate = annual_rate / 12.0 / 100.0;
        let emi = principal * monthly_rate * (1.0 + monthly_rate).powi(months as i32) 
            / ((1.0 + monthly_rate).powi(months as i32) - 1.0);
        emi
    }

    /// Get available PLI schemes
    pub fn get_pli_schemes(&self, sector: &str) -> Result<Vec<PliScheme>, String> {
        // In production: Make HTTP GET request to PLI API
        // For now: Return mock schemes
        Ok(vec![
            PliScheme {
                scheme_name: "PLI for Large Scale Electronics Manufacturing".to_string(),
                sector: "Electronics".to_string(),
                incentive_type: "Production Linked".to_string(),
                eligibility_criteria: "Minimum investment of ₹1000 crore".to_string(),
                incentive_rate: 6.0,
                tenure_years: 6,
            },
            PliScheme {
                scheme_name: "PLI for Telecom Equipment".to_string(),
                sector: "Telecom".to_string(),
                incentive_type: "Production Linked".to_string(),
                eligibility_criteria: "Minimum investment of ₹500 crore".to_string(),
                incentive_rate: 4.0,
                tenure_years: 5,
            },
        ])
    }

    /// Apply for PLI scheme
    pub fn apply_pli_scheme(&self, scheme_name: &str, company_name: &str) -> Result<PliApplication, String> {
        // In production: Make HTTP POST request to PLI application API
        // For now: Return mock application
        Ok(PliApplication {
            application_id: format!("PLI_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            scheme_name: scheme_name.to_string(),
            company_name: company_name.to_string(),
            application_date: chrono::Utc::now().to_rfc3339(),
            status: "Submitted".to_string(),
            approved_incentive: None,
        })
    }

    /// Get Startup India recognition
    pub fn get_startup_recognition(&self, dpiit_number: &str) -> Result<StartupRecognition, String> {
        // In production: Make HTTP GET request to Startup India API
        // For now: Return mock recognition
        Ok(StartupRecognition {
            dpiit_number: dpiit_number.to_string(),
            startup_name: "SigmaOS Innovations Pvt Ltd".to_string(),
            incorporation_date: "2020-01-15".to_string(),
            sector: "Technology".to_string(),
            description: "Sovereign AI-native operating system for India".to_string(),
            founders: vec![
                Founder {
                    name: "Founder 1".to_string(),
                    din: "00012345".to_string(),
                    pan: "ABCDE1234F".to_string(),
                    percentage_ownership: 60.0,
                },
                Founder {
                    name: "Founder 2".to_string(),
                    din: "00067890".to_string(),
                    pan: "FGHIJ5678K".to_string(),
                    percentage_ownership: 40.0,
                },
            ],
            recognition_date: "2020-02-01".to_string(),
            tax_exemption_eligible: true,
        })
    }

    /// Check MSME Sambandh compliance
    pub fn check_sambandh_compliance(&self, company_id: &str) -> Result<bool, String> {
        // In production: Make HTTP GET request to MSME Sambandh API
        // For now: Return mock compliance status
        Ok(true)
    }
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn msme_client_create(base_url: *const u8, base_url_len: usize,
                                    api_key: *const u8, api_key_len: usize) -> *mut MsmeClient {
    unsafe {
        let base_url = String::from_utf8_unchecked(
            std::slice::from_raw_parts(base_url, base_url_len));
        let api_key = String::from_utf8_unchecked(
            std::slice::from_raw_parts(api_key, api_key_len));
        Box::into_raw(Box::new(MsmeClient::new(base_url, api_key)))
    }
}

#[no_mangle]
pub extern "C" fn msme_client_destroy(client: *mut MsmeClient) {
    unsafe {
        if !client.is_null() {
            let _ = Box::from_raw(client);
        }
    }
}

#[no_mangle]
pub extern "C" fn msme_register_udyam(client: *const MsmeClient,
                                      registration_json: *const u8, reg_len: usize,
                                      out: *mut u8, out_len: usize) -> i32 {
    unsafe {
        if client.is_null() || registration_json.is_null() { return -1; }
        let reg_str = String::from_utf8_unchecked(
            std::slice::from_raw_parts(registration_json, reg_len));
        let registration: UdyamRegistration = match serde_json::from_str(&reg_str) {
            Ok(r) => r,
            Err(_) => return -1,
        };
        match (*client).register_udyam(&registration) {
            Ok(udyam_number) => {
                let bytes = udyam_number.as_bytes();
                let copy_len = std::cmp::min(bytes.len(), out_len);
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out, copy_len);
                copy_len as i32
            }
            Err(_) => -1,
        }
    }
}
