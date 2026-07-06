// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/professional/sigma_finance.rs — Sigma Finance Tools
//
// Implements GST calculator, TDS/TCS compliance tools, and AI-driven
// accounting assistant for Indian businesses and professionals.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Finance Types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaxType {
    GST,
    TDS,
    TCS,
    IncomeTax,
}

#[derive(Debug, Clone)]
pub struct GSTInvoice {
    pub invoice_number: String,
    pub date: String,
    pub customer_name: String,
    pub customer_gst: String,
    pub items: Vec<InvoiceItem>,
    pub subtotal: f64,
    pub cgst: f64,
    pub sgst: f64,
    pub igst: f64,
    pub total: f64,
}

#[derive(Debug, Clone)]
pub struct InvoiceItem {
    pub description: String,
    pub hsn_code: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub gst_rate: f64,
    pub amount: f64,
}

#[derive(Debug, Clone)]
pub struct TDSEntry {
    pub section: String,
    pub nature_of_payment: String,
    pub rate: f64,
    pub threshold: f64,
    pub amount: f64,
    pub tds_deducted: f64,
}

// ─── Finance Compliance Manager ───────────────────────────────────────────────

pub struct FinanceManager {
    pub gst_rates: HashMap<String, f64>,
    pub tds_sections: HashMap<String, TDSEntry>,
    pub invoices: Vec<GSTInvoice>,
}

impl FinanceManager {
    pub fn new() -> Self {
        let mut manager = FinanceManager {
            gst_rates: HashMap::new(),
            tds_sections: HashMap::new(),
            invoices: Vec::new(),
        };
        
        manager.init_gst_rates();
        manager.init_tds_sections();
        manager
    }

    /// Initialize GST rates for common goods/services
    fn init_gst_rates(&mut self) {
        self.gst_rates.insert("nil".to_string(), 0.0);
        self.gst_rates.insert("0.25".to_string(), 0.25);
        self.gst_rates.insert("5".to_string(), 5.0);
        self.gst_rates.insert("12".to_string(), 12.0);
        self.gst_rates.insert("18".to_string(), 18.0);
        self.gst_rates.insert("28".to_string(), 28.0);
    }

    /// Initialize TDS sections
    fn init_tds_sections(&mut self) {
        // Section 194C - Contract
        self.tds_sections.insert("194C".to_string(), TDSEntry {
            section: "194C".to_string(),
            nature_of_payment: "Contract".to_string(),
            rate: 1.0,
            threshold: 30000.0,
            amount: 0.0,
            tds_deducted: 0.0,
        });

        // Section 194J - Professional Fees
        self.tds_sections.insert("194J".to_string(), TDSEntry {
            section: "194J".to_string(),
            nature_of_payment: "Professional Fees".to_string(),
            rate: 10.0,
            threshold: 30000.0,
            amount: 0.0,
            tds_deducted: 0.0,
        });

        // Section 194H - Commission
        self.tds_sections.insert("194H".to_string(), TDSEntry {
            section: "194H".to_string(),
            nature_of_payment: "Commission/Brokerage".to_string(),
            rate: 5.0,
            threshold: 15000.0,
            amount: 0.0,
            tds_deducted: 0.0,
        });

        // Section 194A - Interest
        self.tds_sections.insert("194A".to_string(), TDSEntry {
            section: "194A".to_string(),
            nature_of_payment: "Interest (other than securities)".to_string(),
            rate: 10.0,
            threshold: 10000.0,
            amount: 0.0,
            tds_deducted: 0.0,
        });
    }

    /// Calculate GST for intrastate transaction
    pub fn calculate_gst_intrastate(&self, amount: f64, rate: f64) -> (f64, f64, f64) {
        let total_gst = amount * (rate / 100.0);
        let cgst = total_gst / 2.0;
        let sgst = total_gst / 2.0;
        (cgst, sgst, total_gst)
    }

    /// Calculate GST for interstate transaction
    pub fn calculate_gst_interstate(&self, amount: f64, rate: f64) -> f64 {
        amount * (rate / 100.0)
    }

    /// Create GST invoice
    pub fn create_invoice(&mut self, invoice: GSTInvoice) {
        self.invoices.push(invoice);
    }

    /// Calculate TDS
    pub fn calculate_tds(&self, section: &str, amount: f64) -> Result<(f64, f64), String> {
        if let Some(entry) = self.tds_sections.get(section) {
            if amount < entry.threshold {
                Ok((0.0, amount))
            } else {
                let tds = amount * (entry.rate / 100.0);
                let net_amount = amount - tds;
                Ok((tds, net_amount))
            }
        } else {
            Err("Section not found".to_string())
        }
    }

    /// Calculate TCS (Tax Collected at Source)
    pub fn calculate_tcs(&self, amount: f64, rate: f64) -> (f64, f64) {
        let tcs = amount * (rate / 100.0);
        let net_amount = amount + tcs;
        (tcs, net_amount)
    }

    /// Get TDS section details
    pub fn get_tds_section(&self, section: &str) -> Option<&TDSEntry> {
        self.tds_sections.get(section)
    }

    /// Get all TDS sections
    pub fn get_all_tds_sections(&self) -> Vec<&TDSEntry> {
        self.tds_sections.values().collect()
    }

    /// Generate GST report
    pub fn generate_gst_report(&self) -> HashMap<String, f64> {
        let mut report = HashMap::new();
        
        let mut total_subtotal = 0.0;
        let mut total_cgst = 0.0;
        let mut total_sgst = 0.0;
        let mut total_igst = 0.0;
        let mut total_gst = 0.0;
        
        for invoice in &self.invoices {
            total_subtotal += invoice.subtotal;
            total_cgst += invoice.cgst;
            total_sgst += invoice.sgst;
            total_igst += invoice.igst;
            total_gst += invoice.cgst + invoice.sgst + invoice.igst;
        }
        
        report.insert("total_subtotal".to_string(), total_subtotal);
        report.insert("total_cgst".to_string(), total_cgst);
        report.insert("total_sgst".to_string(), total_sgst);
        report.insert("total_igst".to_string(), total_igst);
        report.insert("total_gst".to_string(), total_gst);
        report.insert("total_invoice_amount".to_string(), total_subtotal + total_gst);
        
        report
    }

    /// Get GST rate by HSN code (simplified)
    pub fn get_gst_rate(&self, hsn_code: &str) -> Option<&f64> {
        // Simplified GST rate lookup based on HSN code patterns
        let first_digit = hsn_code.chars().next()?;
        
        let rate = match first_digit {
            '0' => Some(&self.gst_rates["nil"]),
            '1' | '2' => Some(&self.gst_rates["5"]),
            '3' | '4' => Some(&self.gst_rates["12"]),
            '5' | '6' | '7' => Some(&self.gst_rates["18"]),
            '8' | '9' => Some(&self.gst_rates["28"]),
            _ => Some(&self.gst_rates["18"]),
        };
        
        rate
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut manager = FinanceManager::new();
    
    println!("Sigma Finance Tools v0.1 - GST/TDS/TCS Compliance");
    
    loop {
        println!("\nCommands: gst_intrastate <amount> <rate>, gst_interstate <amount> <rate>, tds <section> <amount>, tcs <amount> <rate>, sections, report, quit");
        println!("GST Rates: nil, 0.25, 5, 12, 18, 28");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "gst_intrastate" => {
                if parts.len() >= 3 {
                    if let (Ok(amount), Ok(rate)) = (parts[1].parse::<f64>(), parts[2].parse::<f64>()) {
                        let (cgst, sgst, total_gst) = manager.calculate_gst_intrastate(amount, rate);
                        println!("--- GST Calculation (Intrastate) ---");
                        println!("Amount: ₹{:.2}", amount);
                        println!("GST Rate: {}%", rate);
                        println!("CGST: ₹{:.2}", cgst);
                        println!("SGST: ₹{:.2}", sgst);
                        println!("Total GST: ₹{:.2}", total_gst);
                        println!("Total Amount: ₹{:.2}", amount + total_gst);
                    }
                }
            }
            "gst_interstate" => {
                if parts.len() >= 3 {
                    if let (Ok(amount), Ok(rate)) = (parts[1].parse::<f64>(), parts[2].parse::<f64>()) {
                        let igst = manager.calculate_gst_interstate(amount, rate);
                        println!("--- GST Calculation (Interstate) ---");
                        println!("Amount: ₹{:.2}", amount);
                        println!("GST Rate: {}%", rate);
                        println!("IGST: ₹{:.2}", igst);
                        println!("Total Amount: ₹{:.2}", amount + igst);
                    }
                }
            }
            "tds" => {
                if parts.len() >= 3 {
                    let section = parts[1];
                    if let Ok(amount) = parts[2].parse::<f64>() {
                        match manager.calculate_tds(section, amount) {
                            Ok((tds, net_amount)) => {
                                println!("--- TDS Calculation ---");
                                println!("Section: {}", section);
                                if let Some(entry) = manager.get_tds_section(section) {
                                    println!("Nature of Payment: {}", entry.nature_of_payment);
                                    println!("Rate: {}%", entry.rate);
                                    println!("Threshold: ₹{:.2}", entry.threshold);
                                }
                                println!("Amount: ₹{:.2}", amount);
                                println!("TDS Deducted: ₹{:.2}", tds);
                                println!("Net Amount: ₹{:.2}", net_amount);
                            }
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "tcs" => {
                if parts.len() >= 3 {
                    if let (Ok(amount), Ok(rate)) = (parts[1].parse::<f64>(), parts[2].parse::<f64>()) {
                        let (tcs, net_amount) = manager.calculate_tcs(amount, rate);
                        println!("--- TCS Calculation ---");
                        println!("Amount: ₹{:.2}", amount);
                        println!("TCS Rate: {}%", rate);
                        println!("TCS Collected: ₹{:.2}", tcs);
                        println!("Total Amount (including TCS): ₹{:.2}", net_amount);
                    }
                }
            }
            "sections" => {
                println!("--- TDS Sections ---");
                for section in manager.get_all_tds_sections() {
                    println!("{} - {} ({}%, Threshold: ₹{:.2})", 
                        section.section, section.nature_of_payment, section.rate, section.threshold);
                }
            }
            "report" => {
                let report = manager.generate_gst_report();
                println!("--- GST Report ---");
                for (key, value) in &report {
                    if key.contains("total") {
                        println!("{}: ₹{:.2}", key, value);
                    }
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
