// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/sector/sigma_accounting.rs — Sigma Accounting (GNUCash)
//
// Implements GNUCash-style accounting with double-entry bookkeeping,
// account management, transaction recording, and reporting.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Accounting Types ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub account_type: String,  // asset, liability, equity, income, expense
    pub parent_id: Option<String>,
    pub code: String,
    pub description: String,
    pub balance: f64,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub date: String,
    pub description: String,
    pub splits: Vec<Split>,
    pub reference: String,
}

#[derive(Debug, Clone)]
pub struct Split {
    pub account_id: String,
    pub amount: f64,
    pub memo: String,
}

#[derive(Debug, Clone)]
pub struct Budget {
    pub id: String,
    pub name: String,
    pub period: String,  // monthly, yearly
    pub allocations: HashMap<String, f64>,  // account_id -> amount
}

// ─── Accounting Manager ────────────────────────────────────────────────────

pub struct AccountingManager {
    pub accounts: HashMap<String, Account>,
    pub transactions: Vec<Transaction>,
    pub budgets: HashMap<String, Budget>,
    pub currency: String,
}

impl AccountingManager {
    pub fn new() -> Self {
        let mut manager = AccountingManager {
            accounts: HashMap::new(),
            transactions: Vec::new(),
            budgets: HashMap::new(),
            currency: "INR".to_string(),
        };
        
        manager.init_sample_accounts();
        manager
    }

    /// Initialize sample accounts
    fn init_sample_accounts(&mut self) {
        self.accounts.insert("acc_001".to_string(), Account {
            id: "acc_001".to_string(),
            name: "Cash".to_string(),
            account_type: "asset".to_string(),
            parent_id: None,
            code: "1000".to_string(),
            description: "Cash on hand".to_string(),
            balance: 50000.0,
        });

        self.accounts.insert("acc_002".to_string(), Account {
            id: "acc_002".to_string(),
            name: "Bank Account".to_string(),
            account_type: "asset".to_string(),
            parent_id: None,
            code: "1100".to_string(),
            description: "Main bank account".to_string(),
            balance: 250000.0,
        });

        self.accounts.insert("acc_003".to_string(), Account {
            id: "acc_003".to_string(),
            name: "Salary Income".to_string(),
            account_type: "income".to_string(),
            parent_id: None,
            code: "4000".to_string(),
            description: "Salary income".to_string(),
            balance: 0.0,
        });

        self.accounts.insert("acc_004".to_string(), Account {
            id: "acc_004".to_string(),
            name: "Rent Expense".to_string(),
            account_type: "expense".to_string(),
            parent_id: None,
            code: "6000".to_string(),
            description: "Monthly rent".to_string(),
            balance: 0.0,
        });
    }

    /// Create account
    pub fn create_account(&mut self, name: String, account_type: String, code: String, description: String) -> Account {
        let account = Account {
            id: format!("acc_{}", self.accounts.len()),
            name,
            account_type,
            parent_id: None,
            code,
            description,
            balance: 0.0,
        };
        
        self.accounts.insert(account.id.clone(), account.clone());
        account
    }

    /// Record transaction (double-entry)
    pub fn record_transaction(&mut self, date: String, description: String, splits: Vec<Split>, reference: String) -> Result<Transaction, String> {
        // Validate double-entry: debits must equal credits
        let debits: f64 = splits.iter().filter(|s| s.amount > 0.0).map(|s| s.amount).sum();
        let credits: f64 = splits.iter().filter(|s| s.amount < 0.0).map(|s| s.amount.abs()).sum();
        
        if (debits - credits).abs() > 0.01 {
            return Err("Transaction must balance (debits = credits)".to_string());
        }
        
        // Update account balances
        for split in &splits {
            if let Some(account) = self.accounts.get_mut(&split.account_id) {
                match account.account_type.as_str() {
                    "asset" | "expense" => account.balance += split.amount,
                    "liability" | "equity" | "income" => account.balance -= split.amount,
                    _ => {}
                }
            }
        }
        
        let transaction = Transaction {
            id: format!("txn_{}", self.transactions.len()),
            date,
            description,
            splits,
            reference,
        };
        
        self.transactions.push(transaction.clone());
        Ok(transaction)
    }

    /// Create budget
    pub fn create_budget(&mut self, name: String, period: String, allocations: HashMap<String, f64>) -> Budget {
        let budget = Budget {
            id: format!("budget_{}", self.budget.len()),
            name,
            period,
            allocations,
        };
        
        self.budgets.insert(budget.id.clone(), budget.clone());
        budget
    }

    /// Get account balance
    pub fn get_account_balance(&self, account_id: &str) -> Option<f64> {
        self.accounts.get(account_id).map(|a| a.balance)
    }

    /// Get trial balance
    pub fn get_trial_balance(&self) -> HashMap<String, f64> {
        let mut balance: HashMap<String, f64> = HashMap::new();
        
        for account in self.accounts.values() {
            if account.account_type == "asset" || account.account_type == "expense" {
                *balance.entry(account.name.clone()).or_insert(0.0) += account.balance;
            } else {
                *balance.entry(account.name.clone()).or_insert(0.0) -= account.balance;
            }
        }
        
        balance
    }

    /// Get income statement
    pub fn get_income_statement(&self) -> (f64, f64, f64) {
        let mut income = 0.0;
        let mut expenses = 0.0;
        
        for account in self.accounts.values() {
            match account.account_type.as_str() {
                "income" => income += account.balance,
                "expense" => expenses += account.balance,
                _ => {}
            }
        }
        
        (income, expenses, income - expenses)
    }

    /// Get balance sheet
    pub fn get_balance_sheet(&self) -> (f64, f64, f64) {
        let mut assets = 0.0;
        let mut liabilities = 0.0;
        let mut equity = 0.0;
        
        for account in self.accounts.values() {
            match account.account_type.as_str() {
                "asset" => assets += account.balance,
                "liability" => liabilities += account.balance,
                "equity" => equity += account.balance,
                _ => {}
            }
        }
        
        (assets, liabilities, equity)
    }

    /// Get account by ID
    pub fn get_account(&self, id: &str) -> Option<&Account> {
        self.accounts.get(id)
    }

    /// Get all accounts
    pub fn get_all_accounts(&self) -> Vec<&Account> {
        self.accounts.values().collect()
    }

    /// Get accounts by type
    pub fn get_accounts_by_type(&self, account_type: &str) -> Vec<&Account> {
        self.accounts.values().filter(|a| a.account_type == account_type).collect()
    }

    /// Get all transactions
    pub fn get_all_transactions(&self) -> Vec<&Transaction> {
        self.transactions.iter().collect()
    }

    /// Get budget by ID
    pub fn get_budget(&self, id: &str) -> Option<&Budget> {
        self.budgets.get(id)
    }

    /// Get all budgets
    pub fn get_all_budgets(&self) -> Vec<&Budget> {
        self.budgets.values().collect()
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut manager = AccountingManager::new();
    
    println!("Sigma Accounting v0.1 - GNUCash Style");
    
    loop {
        println!("\n--- Accounting Status ---");
        println!("Currency: {}", manager.currency);
        println!("Accounts: {}", manager.accounts.len());
        println!("Transactions: {}", manager.transactions.len());
        println!("Budgets: {}", manager.budgets.len());
        
        let (income, expenses, net_income) = manager.get_income_statement();
        println!("Net Income: ₹{:.2} (Income: ₹{:.2}, Expenses: ₹{:.2})", net_income, income, expenses);
        
        println!("\nCommands: create_account <name> <type> <code> <description>, transaction <date> <description> <reference>, split <account_id> <amount>, create_budget <name> <period>, trial_balance, income_statement, balance_sheet, accounts <type>, transactions, budgets, quit");
        println!("Account types: asset, liability, equity, income, expense");
        println!("Budget periods: monthly, yearly");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "create_account" => {
                if parts.len() >= 5 {
                    let name = parts[1].to_string();
                    let account_type = parts[2].to_string();
                    let code = parts[3].to_string();
                    let description = parts[4..].join(" ");
                    let account = manager.create_account(name, account_type, code, description);
                    println!("Account created: {}", account.name);
                }
            }
            "transaction" => {
                if parts.len() >= 4 {
                    let date = parts[1].to_string();
                    let description = parts[2].to_string();
                    let reference = parts[3].to_string();
                    println!("Enter splits (account_id amount, one per line, empty to finish):");
                    let mut splits = Vec::new();
                    loop {
                        let mut line = String::new();
                        std::io::stdin().read_line(&mut line).unwrap();
                        let line = line.trim();
                        if line.is_empty() {
                            break;
                        }
                        let split_parts: Vec<&str> = line.split_whitespace().collect();
                        if split_parts.len() >= 2 {
                            if let Ok(amount) = split_parts[1].parse::<f64>() {
                                splits.push(Split {
                                    account_id: split_parts[0].to_string(),
                                    amount,
                                    memo: "".to_string(),
                                });
                            }
                        }
                    }
                    match manager.record_transaction(date, description, splits, reference) {
                        Ok(_) => println!("Transaction recorded"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "split" => {
                // This is a helper for the transaction command
                println!("Use 'transaction' command to record transactions with splits");
            }
            "create_budget" => {
                if parts.len() >= 3 {
                    let name = parts[1].to_string();
                    let period = parts[2].to_string();
                    println!("Enter budget allocations (account_id amount, one per line, empty to finish):");
                    let mut allocations = HashMap::new();
                    loop {
                        let mut line = String::new();
                        std::io::stdin().read_line(&mut line).unwrap();
                        let line = line.trim();
                        if line.is_empty() {
                            break;
                        }
                        let alloc_parts: Vec<&str> = line.split_whitespace().collect();
                        if alloc_parts.len() >= 2 {
                            if let Ok(amount) = alloc_parts[1].parse::<f64>() {
                                allocations.insert(alloc_parts[0].to_string(), amount);
                            }
                        }
                    }
                    let budget = manager.create_budget(name, period, allocations);
                    println!("Budget created: {}", budget.name);
                }
            }
            "trial_balance" => {
                println!("--- Trial Balance ---");
                for (account, balance) in manager.get_trial_balance() {
                    println!("{}: ₹{:.2}", account, balance);
                }
            }
            "income_statement" => {
                println!("--- Income Statement ---");
                let (income, expenses, net_income) = manager.get_income_statement();
                println!("Income: ₹{:.2}", income);
                println!("Expenses: ₹{:.2}", expenses);
                println!("Net Income: ₹{:.2}", net_income);
            }
            "balance_sheet" => {
                println!("--- Balance Sheet ---");
                let (assets, liabilities, equity) = manager.get_balance_sheet();
                println!("Assets: ₹{:.2}", assets);
                println!("Liabilities: ₹{:.2}", liabilities);
                println!("Equity: ₹{:.2}", equity);
                println!("Total: ₹{:.2}", assets);
            }
            "accounts" => {
                if let Some(arg) = parts.get(1) {
                    println!("--- Accounts: {} ---", arg);
                    for account in manager.get_accounts_by_type(arg) {
                        println!("{} - {} ({}) - ₹{:.2}", account.code, account.name, account.description, account.balance);
                    }
                } else {
                    println!("--- All Accounts ---");
                    for account in manager.get_all_accounts() {
                        println!("{} - {} ({}) - ₹{:.2}", account.code, account.name, account.account_type, account.balance);
                    }
                }
            }
            "transactions" => {
                println!("--- All Transactions ---");
                for txn in manager.get_all_transactions() {
                    println!("{} - {} ({})", txn.date, txn.description, txn.reference);
                    for split in &txn.splits {
                        println!("  {} ₹{:.2}", split.account_id, split.amount);
                    }
                }
            }
            "budgets" => {
                println!("--- All Budgets ---");
                for budget in manager.get_all_budgets() {
                    println!("{} - ({})", budget.name, budget.period);
                    for (account_id, amount) in &budget.allocations {
                        println!("  {} ₹{:.2}", account_id, amount);
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
