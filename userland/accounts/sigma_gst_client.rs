// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/accounts/sigma_gst_client.rs — GST IRN API Client
// Implements client for GST Invoice Reference Number (IRN) and e-Way Bill API
//
// Features:
//   - GST authentication (GSP integration)
//   - IRN generation
//   - e-Way Bill generation
//   - Invoice validation
//   - Return filing support
//
// Language: Rust

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── GST Invoice Structure ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GstInvoice {
    pub invoice_no: String,
    pub invoice_date: String,
    pub invoice_type: String,
    pub supply_type: String,
    pub doc_type: String,
    pub customer_gst: String,
    pub customer_name: String,
    pub customer_address: String,
    pub customer_state: String,
    pub customer_code: String,
    pub items: Vec<InvoiceItem>,
    pub total_value: f64,
    pub cgst: f64,
    pub sgst: f64,
    pub igst: f64,
    pub cess: f64,
    pub total_tax: f64,
    pub round_off: f64,
    pub grand_total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub sl_no: u32,
    pub product_name: String,
    pub hsn_code: String,
    pub quantity: f64,
    pub unit: String,
    pub rate: f64,
    pub total: f64,
    pub cgst_rate: f64,
    pub sgst_rate: f64,
    pub igst_rate: f64,
    pub cess_rate: f64,
    pub cgst_amount: f64,
    pub sgst_amount: f64,
    pub igst_amount: f64,
    pub cess_amount: f64,
}

// ── IRN Response ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrnResponse {
    pub success: bool,
    pub irn: String,
    pub signed_invoice: String,
    pub signed_qrcode: String,
    pub ack_no: String,
    pub ack_date: String,
    pub error: Option<IrnError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrnError {
    pub code: String,
    pub message: String,
}

// ── e-Way Bill Structure ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EwayBill {
    pub eway_bill_no: String,
    pub eway_bill_date: String,
    pub generated_by: String,
    pub valid_upto: String,
    pub supplier_gst: String,
    pub supplier_name: String,
    pub recipient_gst: String,
    pub recipient_name: String,
    pub from_state: String,
    pub to_state: String,
    pub vehicle_no: String,
    pub vehicle_type: String,
    pub distance: u32,
    pub doc_type: String,
    pub doc_no: String,
    pub doc_date: String,
    pub transaction_type: String,
    pub sub_supply_type: String,
    pub items: Vec<EwayBillItem>,
    pub total_value: f64,
    pub cgst: f64,
    pub sgst: f64,
    pub igst: f64,
    pub cess: f64,
    pub total_tax: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EwayBillItem {
    pub product_name: String,
    pub hsn_code: String,
    pub quantity: f64,
    pub unit: String,
    pub cgst_rate: f64,
    pub sgst_rate: f64,
    pub igst_rate: f64,
    pub cess_rate: f64,
    pub taxable_value: f64,
    pub total_amount: f64,
}

// ── GST Authentication ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GstAuthRequest {
    pub username: String,
    pub password: String,
    pub gstin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GstAuthResponse {
    pub auth_token: String,
    pub sek: String,
    pub expiry: String,
}

// ── GST Client ─────────────────────────────────────────────────────────

pub struct GstClient {
    base_url: String,
    gsp_id: String,
    auth_token: Option<String>,
    sek: Option<String>,
}

impl GstClient {
    pub fn new(base_url: String, gsp_id: String) -> Self {
        Self {
            base_url,
            gsp_id,
            auth_token: None,
            sek: None,
        }
    }

    /// Authenticate with GST portal via GSP
    pub fn authenticate(&mut self, username: &str, password: &str, gstin: &str) -> Result<GstAuthResponse, String> {
        // In production: Make HTTP request to GST auth endpoint via GSP
        // For now: Return mock response
        let response = GstAuthResponse {
            auth_token: format!("mock_gst_token_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()),
            sek: "mock_encrypted_key".to_string(),
            expiry: "2024-12-31T23:59:59Z".to_string(),
        };
        self.auth_token = Some(response.auth_token.clone());
        self.sek = Some(response.sek.clone());
        Ok(response)
    }

    /// Generate IRN for invoice
    pub fn generate_irn(&self, invoice: &GstInvoice) -> Result<IrnResponse, String> {
        let _token = self.auth_token.as_ref()
            .ok_or("Not authenticated")?;
        
        // In production: Make HTTP POST request to GST IRN endpoint
        // For now: Return mock response
        Ok(IrnResponse {
            success: true,
            irn: format!("IRN{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            signed_invoice: "mock_signed_invoice_base64".to_string(),
            signed_qrcode: "mock_qrcode_base64".to_string(),
            ack_no: format!("ACK{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            ack_date: "2024-01-15T10:00:00Z".to_string(),
            error: None,
        })
    }

    /// Generate e-Way Bill
    pub fn generate_eway_bill(&self, eway_bill: &EwayBill) -> Result<String, String> {
        let _token = self.auth_token.as_ref()
            .ok_or("Not authenticated")?;
        
        // In production: Make HTTP POST request to GST e-Way Bill endpoint
        // For now: Return mock e-Way Bill number
        Ok(format!("31{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Cancel IRN
    pub fn cancel_irn(&self, irn: &str, reason: &str, remark: &str) -> Result<bool, String> {
        let _token = self.auth_token.as_ref()
            .ok_or("Not authenticated")?;
        
        // In production: Make HTTP POST request to cancel IRN
        // For now: Return success
        Ok(true)
    }

    /// Cancel e-Way Bill
    pub fn cancel_eway_bill(&self, eway_bill_no: &str, reason: &str, remark: &str) -> Result<bool, String> {
        let _token = self.auth_token.as_ref()
            .ok_or("Not authenticated")?;
        
        // In production: Make HTTP POST request to cancel e-Way Bill
        // For now: Return success
        Ok(true)
    }

    /// Validate GSTIN
    pub fn validate_gstin(&self, gstin: &str) -> Result<bool, String> {
        // Basic GSTIN validation (15 characters, first 2 are state code)
        if gstin.len() != 15 {
            return Ok(false);
        }
        
        // Check state code (first 2 digits)
        let state_code = &gstin[0..2];
        if state_code.parse::<u32>().is_err() {
            return Ok(false);
        }
        
        // In production: Make HTTP GET request to GST common portal API
        // For now: Return true for valid format
        Ok(true)
    }

    /// Get GSTIN details
    pub fn get_gstin_details(&self, gstin: &str) -> Result<GstinDetails, String> {
        let _token = self.auth_token.as_ref()
            .ok_or("Not authenticated")?;
        
        // In production: Make HTTP GET request to GST common portal API
        // For now: Return mock details
        Ok(GstinDetails {
            gstin: gstin.to_string(),
            legal_name: "SigmaOS Technologies Pvt Ltd".to_string(),
            trade_name: "SigmaOS".to_string(),
            constitution: "Private Limited".to_string(),
            address: "123 Tech Park, Bangalore".to_string(),
            state: "Karnataka".to_string(),
            state_code: "29".to_string(),
            registration_date: "2020-01-01".to_string(),
            status: "Active".to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GstinDetails {
    pub gstin: String,
    pub legal_name: String,
    pub trade_name: String,
    pub constitution: String,
    pub address: String,
    pub state: String,
    pub state_code: String,
    pub registration_date: String,
    pub status: String,
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn gst_client_create(base_url: *const u8, base_url_len: usize,
                                    gsp_id: *const u8, gsp_id_len: usize) -> *mut GstClient {
    unsafe {
        let base_url = String::from_utf8_unchecked(
            std::slice::from_raw_parts(base_url, base_url_len));
        let gsp_id = String::from_utf8_unchecked(
            std::slice::from_raw_parts(gsp_id, gsp_id_len));
        Box::into_raw(Box::new(GstClient::new(base_url, gsp_id)))
    }
}

#[no_mangle]
pub extern "C" fn gst_client_destroy(client: *mut GstClient) {
    unsafe {
        if !client.is_null() {
            let _ = Box::from_raw(client);
        }
    }
}

#[no_mangle]
pub extern "C" fn gst_authenticate(client: *mut GstClient,
                                    username: *const u8, username_len: usize,
                                    password: *const u8, password_len: usize,
                                    gstin: *const u8, gstin_len: usize) -> i32 {
    unsafe {
        if client.is_null() { return -1; }
        let username = String::from_utf8_unchecked(
            std::slice::from_raw_parts(username, username_len));
        let password = String::from_utf8_unchecked(
            std::slice::from_raw_parts(password, password_len));
        let gstin = String::from_utf8_unchecked(
            std::slice::from_raw_parts(gstin, gstin_len));
        match (*client).authenticate(&username, &password, &gstin) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }
}

#[no_mangle]
pub extern "C" fn gst_generate_irn(client: *const GstClient,
                                    invoice_json: *const u8, invoice_len: usize,
                                    out_json: *mut u8, out_len: usize) -> i32 {
    unsafe {
        if client.is_null() || invoice_json.is_null() { return -1; }
        let invoice_str = String::from_utf8_unchecked(
            std::slice::from_raw_parts(invoice_json, invoice_len));
        let invoice: GstInvoice = match serde_json::from_str(&invoice_str) {
            Ok(i) => i,
            Err(_) => return -1,
        };
        match (*client).generate_irn(&invoice) {
            Ok(response) => {
                let json = serde_json::to_string(&response).unwrap_or_default();
                let bytes = json.as_bytes();
                let copy_len = std::cmp::min(bytes.len(), out_len);
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_json, copy_len);
                copy_len as i32
            }
            Err(_) => -1,
        }
    }
}
