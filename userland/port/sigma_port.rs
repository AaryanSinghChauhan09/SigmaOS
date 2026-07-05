// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/port/sigma_port.rs — Customs & Logistics
// Implements integration with Indian customs and logistics systems
//
// Features:
//   - ICEGATE customs EDI integration (import/export declarations)
//   - PCS1x Port Community System
//   - SWIFT Bill of Lading digital handling
//   - FASTag for logistics fleet (automatic toll + weigh bridge)
//   - EXIM bank loan application workflow
//   - RODTEP scheme claim (export duty remission)
//
// Language: Rust

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── ICEGATE Customs EDI ───────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomsDeclaration {
    pub declaration_id: String,
    pub declaration_type: String,  // Import, Export
    pub bill_of_entry_number: String,
    pub bill_of_entry_date: String,
    pub importer_exporter: Party,
    pub customs_house: String,
    pub port: String,
    pub goods: Vec<CustomsGoods>,
    pub total_value_inr: f64,
    pub total_duty_inr: f64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Party {
    pub name: String,
    pub gstin: String,
    pub iec: String,  // Importer Exporter Code
    pub pan: String,
    pub address: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub line1: String,
    pub line2: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub pincode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomsGoods {
    pub item_number: u32,
    pub description: String,
    pub hsn_code: String,
    pub quantity: f64,
    pub unit: String,
    pub assessable_value_inr: f64,
    pub duty_rate_percent: f64,
    pub duty_amount_inr: f64,
}

// ── PCS1x Port Community System ───────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortTransaction {
    pub transaction_id: String,
    pub port: String,
    pub terminal: String,
    pub vessel_name: String,
    pub voyage_number: String,
    pub eta: String,
    pub etd: String,
    pub cargo: Vec<CargoItem>,
    pub consignee: Party,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoItem {
    pub container_number: String,
    pub seal_number: String,
    pub cargo_type: String,  // FCL, LCL, Breakbulk
    pub weight_tonnes: f64,
    pub description: String,
    pub hazardous: bool,
}

// ── Bill of Lading ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillOfLading {
    pub bl_number: String,
    pub carrier: String,
    pub vessel: String,
    pub voyage: String,
    pub port_of_loading: String,
    pub port_of_discharge: String,
    pub shipper: Party,
    pub consignee: Party,
    pub notify_party: Party,
    pub cargo: Vec<CargoItem>,
    pub freight_terms: String,
    pub issue_date: String,
    pub status: String,
}

// ── FASTag Logistics ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FastagVehicle {
    pub vehicle_id: String,
    pub vehicle_number: String,
    pub vehicle_type: String,  // Truck, Bus, Car
    pub fastag_id: String,
    pub wallet_balance_inr: f64,
    pub operator: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TollTransaction {
    pub transaction_id: String,
    pub vehicle_id: String,
    pub toll_plaza: String,
    pub toll_amount_inr: f64,
    pub transaction_date: String,
    pub distance_km: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeighBridgeReading {
    pub reading_id: String,
    pub vehicle_id: String,
    pub location: String,
    pub gross_weight_tonnes: f64,
    pub tare_weight_tonnes: f64,
    pub net_weight_tonnes: f64,
    pub reading_date: String,
}

// ── EXIM Bank Loan ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EximLoanApplication {
    pub application_id: String,
    pub applicant: Party,
    pub loan_type: String,  // Pre-shipment, Post-shipment, Buyer's Credit
    pub loan_amount_inr: f64,
    pub purpose: String,
    pub export_order: ExportOrder,
    pub collateral: Vec<Collateral>,
    pub application_date: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOrder {
    pub order_number: String,
    pub buyer: Party,
    pub country: String,
    pub value_inr: f64,
    pub shipment_date: String,
    pub payment_terms: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collateral {
    pub collateral_type: String,
    pub description: String,
    pub value_inr: f64,
}

// ── RODTEP Scheme ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RodtepClaim {
    pub claim_id: String,
    pub exporter: Party,
    pub shipping_bill_number: String,
    pub shipping_bill_date: String,
    pub export_value_inr: f64,
    pub fob_value_inr: f64,
    pub claimed_amount_inr: f64,
    pub status: String,
    pub sanction_date: Option<String>,
}

// ── Port Client ─────────────────────────────────────────────────────

pub struct PortClient {
    base_url: String,
    api_key: String,
}

impl PortClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            base_url,
            api_key,
        }
    }

    /// Submit customs declaration to ICEGATE
    pub fn submit_customs_declaration(&self, declaration: &CustomsDeclaration) -> Result<String, String> {
        // In production: Make HTTP POST request to ICEGATE API
        // For now: Return mock declaration ID
        Ok(format!("CUST_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Get customs declaration status
    pub fn get_customs_status(&self, declaration_id: &str) -> Result<CustomsDeclaration, String> {
        // In production: Make HTTP GET request to ICEGATE API
        // For now: Return mock status
        Ok(CustomsDeclaration {
            declaration_id: declaration_id.to_string(),
            declaration_type: "Import".to_string(),
            bill_of_entry_number: "BE/2024/12345".to_string(),
            bill_of_entry_date: "2024-01-15".to_string(),
            importer_exporter: Party {
                name: "SigmaOS Imports Ltd".to_string(),
                gstin: "27AAAC1234F1Z9".to_string(),
                iec: "0908012345".to_string(),
                pan: "AAAC1234F".to_string(),
                address: Address {
                    line1: "123 Import Street".to_string(),
                    line2: "SEEPZ".to_string(),
                    city: "Mumbai".to_string(),
                    state: "Maharashtra".to_string(),
                    country: "India".to_string(),
                    pincode: "400096".to_string(),
                },
            },
            customs_house: "JNCH".to_string(),
            port: "Nhava Sheva".to_string(),
            goods: vec![
                CustomsGoods {
                    item_number: 1,
                    description: "Electronic Components".to_string(),
                    hsn_code: "85423100".to_string(),
                    quantity: 1000.0,
                    unit: "PCS".to_string(),
                    assessable_value_inr: 500000.0,
                    duty_rate_percent: 10.0,
                    duty_amount_inr: 50000.0,
                },
            ],
            total_value_inr: 500000.0,
            total_duty_inr: 50000.0,
            status: "Assessed".to_string(),
        })
    }

    /// Get port transaction from PCS1x
    pub fn get_port_transaction(&self, transaction_id: &str) -> Result<PortTransaction, String> {
        // In production: Make HTTP GET request to PCS1x API
        // For now: Return mock transaction
        Ok(PortTransaction {
            transaction_id: transaction_id.to_string(),
            port: "Nhava Sheva".to_string(),
            terminal: "JNPCT".to_string(),
            vessel_name: "MV SigmaOS Express".to_string(),
            voyage_number: "V001".to_string(),
            eta: "2024-01-20T08:00:00Z".to_string(),
            etd: "2024-01-25T18:00:00Z".to_string(),
            cargo: vec![
                CargoItem {
                    container_number: "CONT0012345".to_string(),
                    seal_number: "SEAL123456".to_string(),
                    cargo_type: "FCL".to_string(),
                    weight_tonnes: 25.0,
                    description: "General Cargo".to_string(),
                    hazardous: false,
                },
            ],
            consignee: Party {
                name: "SigmaOS Logistics".to_string(),
                gstin: "27AAAC1234F1Z9".to_string(),
                iec: "0908012345".to_string(),
                pan: "AAAC1234F".to_string(),
                address: Address {
                    line1: "123 Logistics Park".to_string(),
                    line2: "".to_string(),
                    city: "Mumbai".to_string(),
                    state: "Maharashtra".to_string(),
                    country: "India".to_string(),
                    pincode: "400096".to_string(),
                },
            },
            status: "Berthed".to_string(),
        })
    }

    /// Create Bill of Lading
    pub fn create_bill_of_lading(&self, bl: &BillOfLading) -> Result<String, String> {
        // In production: Make HTTP POST request to shipping line API
        // For now: Return mock BL number
        Ok(format!("BL_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Get FASTag vehicle balance
    pub fn get_fastag_balance(&self, vehicle_id: &str) -> Result<FastagVehicle, String> {
        // In production: Make HTTP GET request to FASTag API
        // For now: Return mock vehicle data
        Ok(FastagVehicle {
            vehicle_id: vehicle_id.to_string(),
            vehicle_number: "MH01AB1234".to_string(),
            vehicle_type: "Truck".to_string(),
            fastag_id: "FASTAG12345".to_string(),
            wallet_balance_inr: 5000.0,
            operator: "ICICI Bank".to_string(),
            status: "Active".to_string(),
        })
    }

    /// Get weigh bridge reading
    pub fn get_weighbridge_reading(&self, vehicle_id: &str) -> Result<WeighBridgeReading, String> {
        // In production: Make HTTP GET request to weigh bridge API
        // For now: Return mock reading
        Ok(WeighBridgeReading {
            reading_id: format!("WB_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            vehicle_id: vehicle_id.to_string(),
            location: "JNPT Weigh Bridge 1".to_string(),
            gross_weight_tonnes: 30.0,
            tare_weight_tonnes: 10.0,
            net_weight_tonnes: 20.0,
            reading_date: chrono::Utc::now().to_rfc3339(),
        })
    }

    /// Apply for EXIM bank loan
    pub fn apply_exim_loan(&self, application: &EximLoanApplication) -> Result<String, String> {
        // In production: Make HTTP POST request to EXIM Bank API
        // For now: Return mock application ID
        Ok(format!("EXIM_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Submit RODTEP claim
    pub fn submit_rodtep_claim(&self, claim: &RodtepClaim) -> Result<String, String> {
        // In production: Make HTTP POST request to RODTEP API
        // For now: Return mock claim ID
        Ok(format!("RODTEP_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Get RODTEP claim status
    pub fn get_rodtep_status(&self, claim_id: &str) -> Result<RodtepClaim, String> {
        // In production: Make HTTP GET request to RODTEP API
        // For now: Return mock status
        Ok(RodtepClaim {
            claim_id: claim_id.to_string(),
            exporter: Party {
                name: "SigmaOS Exports Ltd".to_string(),
                gstin: "27AAAC1234F1Z9".to_string(),
                iec: "0908012345".to_string(),
                pan: "AAAC1234F".to_string(),
                address: Address {
                    line1: "123 Export Street".to_string(),
                    line2: "SEEPZ".to_string(),
                    city: "Mumbai".to_string(),
                    state: "Maharashtra".to_string(),
                    country: "India".to_string(),
                    pincode: "400096".to_string(),
                },
            },
            shipping_bill_number: "SB/2024/12345".to_string(),
            shipping_bill_date: "2024-01-15".to_string(),
            export_value_inr: 1000000.0,
            fob_value_inr: 950000.0,
            claimed_amount_inr: 47500.0,
            status: "Sanctioned".to_string(),
            sanction_date: Some("2024-02-15".to_string()),
        })
    }
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn port_client_create(base_url: *const u8, base_url_len: usize,
                                   api_key: *const u8, api_key_len: usize) -> *mut PortClient {
    unsafe {
        let base_url = String::from_utf8_unchecked(
            std::slice::from_raw_parts(base_url, base_url_len));
        let api_key = String::from_utf8_unchecked(
            std::slice::from_raw_parts(api_key, api_key_len));
        Box::into_raw(Box::new(PortClient::new(base_url, api_key)))
    }
}

#[no_mangle]
pub extern "C" fn port_client_destroy(client: *mut PortClient) {
    unsafe {
        if !client.is_null() {
            let _ = Box::from_raw(client);
        }
    }
}

#[no_mangle]
pub extern "C" fn port_get_customs_status(client: *const PortClient,
                                        decl_id: *const u8, decl_len: usize,
                                        out_json: *mut u8, out_len: usize) -> i32 {
    unsafe {
        if client.is_null() || decl_id.is_null() { return -1; }
        let decl_id = String::from_utf8_unchecked(
            std::slice::from_raw_parts(decl_id, decl_len));
        match (*client).get_customs_status(&decl_id) {
            Ok(declaration) => {
                let json = serde_json::to_string(&declaration).unwrap_or_default();
                let bytes = json.as_bytes();
                let copy_len = std::cmp::min(bytes.len(), out_len);
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_json, copy_len);
                copy_len as i32
            }
            Err(_) => -1,
        }
    }
}
