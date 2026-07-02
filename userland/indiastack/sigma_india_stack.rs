// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/indiastack/sigma_india_stack.rs — India Stack Integration
// ABDM FHIR client, UPI/NPCI, GST/IRN, Aadhaar APIs
// Language: Rust (std)

use std::collections::HashMap;

// ── ABDM FHIR ─────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct FhirPatient {
    pub health_id:   String,
    pub name:        String,
    pub dob:         String,
    pub gender:      String,
    pub mobile:      String,
    pub abha_address: String,
}

#[derive(Debug, Clone)]
pub struct FhirObservation {
    pub patient_id:  String,
    pub code:        String,   // LOINC/SNOMED code
    pub value:       String,
    pub unit:        String,
    pub timestamp:   String,
}

pub struct AbdmClient {
    pub base_url:    String,
    pub client_id:   String,
    pub access_token: Option<String>,
}

impl AbdmClient {
    pub fn new(client_id: &str) -> Self {
        Self {
            base_url:    "https://dev.abdm.gov.in/gateway".to_owned(),
            client_id:   client_id.to_owned(),
            access_token: None,
        }
    }

    pub fn sandbox() -> Self {
        let mut c = Self::new("sigma-os-sandbox");
        c.base_url = "https://dev.abdm.gov.in/gateway".to_owned();
        c
    }

    /// OAuth2 token request
    pub fn authenticate(&mut self, client_secret: &str) -> bool {
        let body = format!(
            r#"{{"clientId":"{}","clientSecret":"{}","grantType":"client_credentials"}}"#,
            self.client_id, client_secret
        );
        let out = std::process::Command::new("curl")
            .args(["-sf","-X","POST",
                   &format!("{}/v0.5/sessions", self.base_url),
                   "-H","Content-Type: application/json",
                   "-d",&body,"--max-time","10"])
            .output();
        match out {
            Ok(o) if o.status.success() => {
                let resp = String::from_utf8_lossy(&o.stdout);
                if let Some(pos) = resp.find("\"accessToken\":\"") {
                    let rest = &resp[pos+15..];
                    if let Some(end) = rest.find('"') {
                        self.access_token = Some(rest[..end].to_owned());
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }

    fn auth_header(&self) -> String {
        self.access_token.as_ref().map(|t| format!("Bearer {}", t))
            .unwrap_or_default()
    }

    /// Search for a patient by ABHA (Ayushman Bharat Health Account) ID
    pub fn search_patient(&self, abha_id: &str) -> Option<FhirPatient> {
        let token = self.auth_header();
        if token.is_empty() { return None; }
        let url = format!("{}/v0.5/patients/find", self.base_url);
        let body = format!(r#"{{"query":{{"id":"{}","purpose":"KYC"}}}}"#, abha_id);
        let out = std::process::Command::new("curl")
            .args(["-sf","-X","POST",&url,
                   "-H","Content-Type: application/json",
                   "-H",&format!("Authorization: {}", token),
                   "-d",&body,"--max-time","10"])
            .output().ok()?;
        if !out.status.success() { return None; }
        let resp = String::from_utf8_lossy(&out.stdout);
        // Parse minimal FHIR Patient resource
        Some(FhirPatient {
            health_id:    abha_id.to_owned(),
            name:         extract_json(&resp, "name").unwrap_or_default(),
            dob:          extract_json(&resp, "dob").unwrap_or_default(),
            gender:       extract_json(&resp, "gender").unwrap_or_default(),
            mobile:       extract_json(&resp, "mobile").unwrap_or_default(),
            abha_address: extract_json(&resp, "healthId").unwrap_or_default(),
        })
    }

    /// Link health records to ABHA ID
    pub fn link_records(&self, abha_id: &str, care_context_reference: &str) -> bool {
        let token = self.auth_header();
        if token.is_empty() { return false; }
        let body = format!(r#"{{"requestId":"sigma-{}","patient":{{"id":"{}","referenceNumber":"{}"}}}}"#,
                           std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
                               .unwrap_or_default().as_secs(),
                           abha_id, care_context_reference);
        let out = std::process::Command::new("curl")
            .args(["-sf","-X","POST",
                   &format!("{}/v0.5/links/link/init", self.base_url),
                   "-H","Content-Type: application/json",
                   "-H",&format!("Authorization: {}", token),
                   "-d",&body,"--max-time","10"])
            .output();
        matches!(out, Ok(o) if o.status.success())
    }
}

// ── UPI / NPCI ────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct UpiTransaction {
    pub txn_id:     String,
    pub payer_vpa:  String,   // Virtual Payment Address e.g. user@okicici
    pub payee_vpa:  String,
    pub amount_paise: u64,    // Amount in paise (1 INR = 100 paise)
    pub note:       String,
    pub ref_id:     String,
}

#[derive(Debug, Clone)]
pub enum UpiStatus { Pending, Success, Failed, Reversed }

pub struct UpiClient {
    pub merchant_id: String,
    pub vpa:         String,   // Merchant VPA
    pub upi_url:     String,   // NPCI production: https://api.npci.org.in/upiip/v1
}

impl UpiClient {
    pub fn new(merchant_id: &str, vpa: &str) -> Self {
        Self {
            merchant_id: merchant_id.to_owned(),
            vpa:         vpa.to_owned(),
            upi_url:     "https://api.npci.org.in/upiip/v1".to_owned(),
        }
    }

    /// Generate a UPI payment URI (for QR code)
    pub fn payment_uri(&self, amount_paise: u64, note: &str, txn_ref: &str) -> String {
        format!("upi://pay?pa={}&pn=SigmaOS&am={:.2}&tn={}&tr={}&cu=INR",
                self.vpa,
                amount_paise as f64 / 100.0,
                urlencoded(note),
                txn_ref)
    }

    /// Initiate a collect request (pull payment)
    pub fn collect_request(&self, payer_vpa: &str, amount_paise: u64, note: &str) -> Option<String> {
        let txn = UpiTransaction {
            txn_id:       format!("sigma-{}", timestamp_millis()),
            payer_vpa:    payer_vpa.to_owned(),
            payee_vpa:    self.vpa.clone(),
            amount_paise,
            note:         note.to_owned(),
            ref_id:       format!("ref{}", timestamp_millis()),
        };
        let body = format!(
            r#"{{"txnId":"{}","payerVpa":"{}","payeeVpa":"{}","amount":{},"note":"{}","refId":"{}"}}"#,
            txn.txn_id, txn.payer_vpa, txn.payee_vpa,
            txn.amount_paise, txn.note, txn.ref_id
        );
        let out = std::process::Command::new("curl")
            .args(["-sf","-X","POST",&format!("{}/collect", self.upi_url),
                   "-H","Content-Type: application/json",
                   "-d",&body,"--max-time","15"])
            .output().ok()?;
        if out.status.success() { Some(txn.txn_id) } else { None }
    }

    /// Check transaction status
    pub fn check_status(&self, txn_id: &str) -> UpiStatus {
        let out = std::process::Command::new("curl")
            .args(["-sf",&format!("{}/status/{}", self.upi_url, txn_id),"--max-time","10"])
            .output();
        match out {
            Ok(o) if o.status.success() => {
                let resp = String::from_utf8_lossy(&o.stdout);
                if resp.contains("SUCCESS") { UpiStatus::Success }
                else if resp.contains("FAILURE") || resp.contains("FAILED") { UpiStatus::Failed }
                else if resp.contains("REVERSED") { UpiStatus::Reversed }
                else { UpiStatus::Pending }
            }
            _ => UpiStatus::Pending,
        }
    }
}

// ── GST / IRN (Invoice Reference Number) ──────────────────────────────────
#[derive(Debug, Clone)]
pub struct GstInvoice {
    pub seller_gstin: String,
    pub buyer_gstin:  String,
    pub invoice_no:   String,
    pub invoice_date: String,
    pub total_value:  f64,
    pub items:        Vec<GstLineItem>,
    pub irn:          Option<String>,
    pub qr_code:      Option<String>,
}

#[derive(Debug, Clone)]
pub struct GstLineItem {
    pub hsn_code:   String,
    pub description: String,
    pub qty:        f64,
    pub rate:       f64,
    pub taxable_val: f64,
    pub cgst_rate:  f64,
    pub sgst_rate:  f64,
    pub igst_rate:  f64,
}

pub struct GstClient {
    pub gstin:       String,
    pub client_id:   String,
    pub irn_url:     String,   // NIC e-Invoice: https://api.einvoice1.gst.gov.in
    pub access_token: Option<String>,
}

impl GstClient {
    pub fn new(gstin: &str, client_id: &str) -> Self {
        Self {
            gstin:       gstin.to_owned(),
            client_id:   client_id.to_owned(),
            irn_url:     "https://api.einvoice1.gst.gov.in".to_owned(),
            access_token: None,
        }
    }

    pub fn sandbox() -> Self {
        let mut c = Self::new("29AAFCD5862R1ZR", "sigma-sandbox");
        c.irn_url = "https://einv-apisandbox.nic.in".to_owned();
        c
    }

    pub fn authenticate(&mut self, username: &str, password: &str) -> bool {
        let body = format!(r#"{{"UserName":"{}","Password":"{}","AppKey":"{}","ForceRefreshAccessToken":"Y"}}"#,
                           username, password, self.client_id);
        let out = std::process::Command::new("curl")
            .args(["-sf","-X","POST",&format!("{}/eivital/v1.04/Auth", self.irn_url),
                   "-H","Content-Type: application/json","-d",&body,"--max-time","10"])
            .output();
        match out {
            Ok(o) if o.status.success() => {
                let resp = String::from_utf8_lossy(&o.stdout);
                if let Some(tok) = extract_json(&resp, "AuthToken") {
                    self.access_token = Some(tok); return true;
                }
                false
            }
            _ => false,
        }
    }

    /// Generate IRN for an invoice
    pub fn generate_irn(&self, invoice: &mut GstInvoice) -> bool {
        let token = match &self.access_token {
            Some(t) => t.clone(),
            None => return false,
        };
        let items_json: Vec<String> = invoice.items.iter().map(|item| {
            format!(r#"{{"HsnCd":"{}","Desc":"{}","Qty":{},"UnitPrice":{},"TotAmt":{},"AssAmt":{},"GstRt":{},"CgstAmt":{},"SgstAmt":{},"IgstAmt":{}}}"#,
                    item.hsn_code, item.description, item.qty, item.rate,
                    item.qty * item.rate, item.taxable_val,
                    item.cgst_rate + item.sgst_rate + item.igst_rate,
                    item.taxable_val * item.cgst_rate / 100.0,
                    item.taxable_val * item.sgst_rate / 100.0,
                    item.taxable_val * item.igst_rate / 100.0)
        }).collect();
        let body = format!(
            r#"{{"Version":"1.1","TranDtls":{{"TaxSch":"GST","SupTyp":"B2B"}},"DocDtls":{{"Typ":"INV","No":"{}","Dt":"{}"}},"SellerDtls":{{"Gstin":"{}","TrdNm":"SigmaOS"}},"BuyerDtls":{{"Gstin":"{}","TrdNm":"Buyer"}},"ItemList":[{}],"ValDtls":{{"TotInvVal":{}}}}}"#,
            invoice.invoice_no, invoice.invoice_date,
            invoice.seller_gstin, invoice.buyer_gstin,
            items_json.join(","), invoice.total_value
        );
        let out = std::process::Command::new("curl")
            .args(["-sf","-X","POST",&format!("{}/eicore/v1.03/Invoice", self.irn_url),
                   "-H","Content-Type: application/json",
                   "-H",&format!("AuthToken: {}", token),
                   "-d",&body,"--max-time","15"])
            .output();
        match out {
            Ok(o) if o.status.success() => {
                let resp = String::from_utf8_lossy(&o.stdout);
                if let Some(irn) = extract_json(&resp, "Irn") {
                    invoice.irn = Some(irn);
                    invoice.qr_code = extract_json(&resp, "SignedQRCode");
                    return true;
                }
                false
            }
            _ => false,
        }
    }
}

// ── e-RUPI (one-time voucher) ─────────────────────────────────────────────
pub struct ERupiVoucher {
    pub voucher_id:  String,
    pub amount_paise: u64,
    pub beneficiary: String,
    pub purpose:     String,
    pub expiry:      String,
    pub status:      String,
}

impl ERupiVoucher {
    pub fn create(amount_paise: u64, beneficiary: &str, purpose: &str) -> Self {
        Self {
            voucher_id:   format!("erupi-sigma-{}", timestamp_millis()),
            amount_paise,
            beneficiary:  beneficiary.to_owned(),
            purpose:      purpose.to_owned(),
            expiry:       "2026-12-31".to_owned(),
            status:       "ACTIVE".to_owned(),
        }
    }
}

// ── Helpers ────────────────────────────────────────────────────────────────
fn extract_json(json: &str, key: &str) -> Option<String> {
    let search = format!("\"{}\":\"", key);
    let pos = json.find(&search)?;
    let rest = &json[pos + search.len()..];
    let end = rest.find('"')?;
    Some(rest[..end].to_owned())
}

fn urlencoded(s: &str) -> String {
    s.chars().map(|c| match c {
        'A'..='Z'|'a'..='z'|'0'..='9'|'-'|'_'|'.'|'~' => c.to_string(),
        ' ' => "+".to_owned(),
        c => format!("%{:02X}", c as u32),
    }).collect()
}

fn timestamp_millis() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

// ── CLI ────────────────────────────────────────────────────────────────────
pub fn india_stack_cmd(args: &[String]) {
    if args.is_empty() {
        eprintln!("sigma-india — India Stack Integration\n\
            Usage:\n\
            sigma-india abdm search <abha-id>\n\
            sigma-india upi pay <vpa> <amount-inr> <note>\n\
            sigma-india upi status <txn-id>\n\
            sigma-india gst irn <invoice-json>\n\
            sigma-india erupi create <amount-inr> <beneficiary> <purpose>\
        ");
        return;
    }
    match args[0].as_str() {
        "abdm" if args.len() > 2 && args[1] == "search" => {
            let mut client = AbdmClient::sandbox();
            if let Some(p) = client.search_patient(&args[2]) {
                println!("ABHA: {}\nName: {}\nDOB: {}\nGender: {}",
                         p.health_id, p.name, p.dob, p.gender);
            } else {
                println!("Patient not found or sandbox auth required.");
            }
        }
        "upi" if args.len() > 2 => {
            let client = UpiClient::new("sigma-merchant", "sigmaos@ybl");
            match args[1].as_str() {
                "pay" if args.len() > 4 => {
                    let amount_paise = (args[3].parse::<f64>().unwrap_or(0.0) * 100.0) as u64;
                    let uri = client.payment_uri(amount_paise, &args[4], &format!("ref{}", timestamp_millis()));
                    println!("UPI Payment URI:\n{}", uri);
                }
                "status" => {
                    println!("{:?}", client.check_status(&args[2]));
                }
                _ => {}
            }
        }
        "gst" => {
            let client = GstClient::sandbox();
            println!("GST client ready: {}", client.irn_url);
            println!("Authenticate with: sigma-india gst auth <username> <password>");
        }
        "erupi" if args.len() > 4 && args[1] == "create" => {
            let amount_paise = (args[2].parse::<f64>().unwrap_or(0.0) * 100.0) as u64;
            let v = ERupiVoucher::create(amount_paise, &args[3], &args[4]);
            println!("e-RUPI Voucher: {}\nAmount: ₹{:.2}\nBeneficiary: {}\nPurpose: {}",
                     v.voucher_id, amount_paise as f64/100.0, v.beneficiary, v.purpose);
        }
        _ => println!("Unknown india command. Run: sigma-india"),
    }
}
