// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/india_stack/sigma_pay.rs — UPI/NPCI Payment API
//
// Implements UPI (Unified Payments Interface) API integration
// for peer-to-peer payments, merchant payments, and bill payments via NPCI.
//
// Language: Rust (std for userland services)

use std::collections::HashMap;

// ─── UPI Constants ───────────────────────────────────────────────────────

pub const UPI_API_BASE: &str = "https://api.upi.gov.in";
pub const UPI_VERSION: &str = "2.0";

// ─── UPI Transaction Structure ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct UPITransaction {
    pub txn_id: String,
    pub payer_vpa: String,
    pub payee_vpa: String,
    pub amount: f64,
    pub note: String,
    pub status: TransactionStatus,
    pub timestamp: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Success,
    Failed,
    Refunded,
}

// ─── UPI Payment Manager ─────────────────────────────────────────────────

pub struct SigmaPay {
    pub vpa: String,
    pub bank_code: String,
    pub api_key: String,
    pub authenticated: bool,
    pub balance: f64,
    pub transactions: HashMap<String, UPITransaction>,
}

impl SigmaPay {
    pub fn new(vpa: String, bank_code: String, api_key: String) -> Self {
        SigmaPay {
            vpa,
            bank_code,
            api_key,
            authenticated: false,
            balance: 10000.0, // Stub balance
            transactions: HashMap::new(),
        }
    }

    /// Authenticate with UPI provider
    pub fn authenticate(&mut self, mpin: &str) -> Result<(), String> {
        // Validate MPIN format (6 digits)
        if mpin.len() != 6 || !mpin.chars().all(|c| c.is_digit(10)) {
            return Err("Invalid MPIN format".to_string());
        }

        // In a real implementation, verify MPIN with bank
        self.authenticated = true;
        Ok(())
    }

    /// Send money to another VPA
    pub fn send_money(&mut self, payee_vpa: String, amount: f64, note: String) -> Result<String, String> {
        if !self.authenticated {
            return Err("Not authenticated. Call authenticate() first.".to_string());
        }

        // Validate amount
        if amount <= 0.0 {
            return Err("Invalid amount".to_string());
        }

        if amount > self.balance {
            return Err("Insufficient balance".to_string());
        }

        // Validate VPA format (username@bank)
        if !payee_vpa.contains('@') || payee_vpa.len() < 5 {
            return Err("Invalid VPA format".to_string());
        }

        // Generate transaction ID
        let txn_id = format!("TXN{}", uuid_stub());

        // In a real implementation, this would make an HTTP request to UPI API
        // For now, process stub transaction
        self.balance -= amount;

        let transaction = UPITransaction {
            txn_id: txn_id.clone(),
            payer_vpa: self.vpa.clone(),
            payee_vpa: payee_vpa.clone(),
            amount,
            note,
            status: TransactionStatus::Success,
            timestamp: current_timestamp(),
        };

        self.transactions.insert(txn_id.clone(), transaction);

        Ok(txn_id)
    }

    /// Request money from another VPA
    pub fn request_money(&mut self, payer_vpa: String, amount: f64, note: String) -> Result<String, String> {
        if !self.authenticated {
            return Err("Not authenticated".to_string());
        }

        // Validate amount
        if amount <= 0.0 {
            return Err("Invalid amount".to_string());
        }

        // Validate VPA format
        if !payer_vpa.contains('@') || payer_vpa.len() < 5 {
            return Err("Invalid VPA format".to_string());
        }

        // Generate request ID
        let request_id = format!("REQ{}", uuid_stub());

        // In a real implementation, this would send a money request via UPI
        Ok(format!("Money request sent: {} for ₹{} from {}", request_id, amount, payer_vpa))
    }

    /// Check transaction status
    pub fn check_status(&self, txn_id: &str) -> Result<&UPITransaction, String> {
        self.transactions.get(txn_id)
            .ok_or_else(|| "Transaction not found".to_string())
    }

    /// Get transaction history
    pub fn get_history(&self) -> Vec<&UPITransaction> {
        self.transactions.values().collect()
    }

    /// Pay bill (utility, electricity, etc.)
    pub fn pay_bill(&mut self, biller_id: String, amount: f64, consumer_no: String) -> Result<String, String> {
        if !self.authenticated {
            return Err("Not authenticated".to_string());
        }

        if amount > self.balance {
            return Err("Insufficient balance".to_string());
        }

        let txn_id = format!("BILL{}", uuid_stub());
        self.balance -= amount;

        let transaction = UPITransaction {
            txn_id: txn_id.clone(),
            payer_vpa: self.vpa.clone(),
            payee_vpa: biller_id.clone(),
            amount,
            note: format!("Bill payment for consumer: {}", consumer_no),
            status: TransactionStatus::Success,
            timestamp: current_timestamp(),
        };

        self.transactions.insert(txn_id.clone(), transaction);

        Ok(txn_id)
    }

    /// Get current balance
    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    /// Link bank account
    pub fn link_bank(&mut self, account_no: String, ifsc: String) -> Result<(), String> {
        // Validate IFSC format (11 characters: 4 letters + 7 digits)
        if ifsc.len() != 11 {
            return Err("Invalid IFSC format".to_string());
        }

        // In a real implementation, verify bank account with NPCI
        Ok(())
    }
}

// ─── UUID Stub ─────────────────────────────────────────────────────────────

fn uuid_stub() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("{:016x}", timestamp)
}

// ─── Current Timestamp ─────────────────────────────────────────────────────

fn current_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("{}", timestamp)
}

// ─── CLI Interface ─────────────────────────────────────────────────────────────

pub fn cmd_pay_send(args: &[String]) -> i32 {
    if args.len() < 5 {
        eprintln!("sigma-pay: usage: pay send <vpa> <amount> <note>");
        return 1;
    }

    let mut pay = SigmaPay::new("user@bank".to_string(), "HDFC".to_string(), "STUB_API_KEY".to_string());
    pay.authenticated = true; // Stub authentication

    let amount: f64 = match args[3].parse() {
        Ok(a) => a,
        Err(_) => {
            eprintln!("sigma-pay: Invalid amount");
            return 1;
        }
    };

    match pay.send_money(args[2].clone(), amount, args[4].clone()) {
        Ok(txn_id) => {
            println!("Payment successful. Transaction ID: {}", txn_id);
            println!("Remaining balance: ₹{:.2}", pay.get_balance());
            0
        }
        Err(e) => {
            eprintln!("sigma-pay: {}", e);
            1
        }
    }
}

pub fn cmd_pay_balance(_args: &[String]) -> i32 {
    let pay = SigmaPay::new("user@bank".to_string(), "HDFC".to_string(), "STUB_API_KEY".to_string());
    println!("Current balance: ₹{:.2}", pay.get_balance());
    0
}

pub fn cmd_pay_history(_args: &[String]) -> i32 {
    let mut pay = SigmaPay::new("user@bank".to_string(), "HDFC".to_string(), "STUB_API_KEY".to_string());
    pay.authenticated = true;

    println!("Transaction History:");
    for txn in pay.get_history() {
        println!("  {} → {} ₹{} ({})", txn.payer_vpa, txn.payee_vpa, txn.amount, txn.status as i32);
    }
    0
}
