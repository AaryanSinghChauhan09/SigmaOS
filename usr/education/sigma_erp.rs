// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/education/sigma_erp.rs — Sigma ERP System (ERPNext)
//
// Implements ERPNext-style ERP system with modules for accounting,
// inventory, HR, manufacturing, sales, and reporting.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── ERP Types ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Company {
    pub name: String,
    pub address: String,
    pub gstin: String,
    pub pan: String,
}

#[derive(Debug, Clone)]
pub struct Employee {
    pub id: String,
    pub name: String,
    pub email: String,
    pub department: String,
    pub designation: String,
    pub salary: f64,
    pub joining_date: String,
}

#[derive(Debug, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub sku: String,
    pub category: String,
    pub price: f64,
    pub stock: u32,
    pub unit: String,
}

#[derive(Debug, Clone)]
pub struct Invoice {
    pub id: String,
    pub invoice_number: String,
    pub customer: String,
    pub date: String,
    pub items: Vec<InvoiceItem>,
    pub subtotal: f64,
    pub tax: f64,
    pub total: f64,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct InvoiceItem {
    pub product_id: String,
    pub product_name: String,
    pub quantity: u32,
    pub rate: f64,
    pub amount: f64,
}

#[derive(Debug, Clone)]
pub struct LedgerEntry {
    pub id: String,
    pub account: String,
    pub debit: f64,
    pub credit: f64,
    pub date: String,
    pub description: String,
}

// ─── ERP Manager ────────────────────────────────────────────────────

pub struct ERPManager {
    pub company: Company,
    pub employees: HashMap<String, Employee>,
    pub products: HashMap<String, Product>,
    pub invoices: HashMap<String, Invoice>,
    pub ledger: Vec<LedgerEntry>,
}

impl ERPManager {
    pub fn new() -> Self {
        let mut manager = ERPManager {
            company: Company {
                name: "Sigma Education Pvt Ltd".to_string(),
                address: "New Delhi, India".to_string(),
                gstin: "27ABCDE1234F1Z5".to_string(),
                pan: "ABCDE1234F".to_string(),
            },
            employees: HashMap::new(),
            products: HashMap::new(),
            invoices: HashMap::new(),
            ledger: Vec::new(),
        };
        
        manager.init_sample_employees();
        manager.init_sample_products();
        manager
    }

    /// Initialize sample employees
    fn init_sample_employees(&mut self) {
        self.employees.insert("emp_001".to_string(), Employee {
            id: "emp_001".to_string(),
            name: "Amit Kumar".to_string(),
            email: "amit.kumar@sigmaos.edu".to_string(),
            department: "IT".to_string(),
            designation: "Software Engineer".to_string(),
            salary: 750000.0,
            joining_date: "2023-01-15".to_string(),
        });

        self.employees.insert("emp_002".to_string(), Employee {
            id: "emp_002".to_string(),
            name: "Priya Singh".to_string(),
            email: "priya.singh@sigmaos.edu".to_string(),
            department: "HR".to_string(),
            designation: "HR Manager".to_string(),
            salary: 600000.0,
            joining_date: "2022-06-01".to_string(),
        });
    }

    /// Initialize sample products
    fn init_sample_products(&mut self) {
        self.products.insert("prod_001".to_string(), Product {
            id: "prod_001".to_string(),
            name: "Laptop".to_string(),
            sku: "LAP-001".to_string(),
            category: "Electronics".to_string(),
            price: 45000.0,
            stock: 50,
            unit: "Piece".to_string(),
        });

        self.products.insert("prod_002".to_string(), Product {
            id: "prod_002".to_string(),
            name: "Textbook - Mathematics".to_string(),
            sku: "TBK-MATH-001".to_string(),
            category: "Books".to_string(),
            price: 850.0,
            stock: 200,
            unit: "Piece".to_string(),
        });
    }

    /// Add employee
    pub fn add_employee(&mut self, employee: Employee) {
        self.employees.insert(employee.id.clone(), employee);
    }

    /// Add product
    pub fn add_product(&mut self, product: Product) {
        self.products.insert(product.id.clone(), product);
    }

    /// Create invoice
    pub fn create_invoice(&mut self, customer: String, items: Vec<InvoiceItem>) -> Invoice {
        let subtotal: f64 = items.iter().map(|i| i.amount).sum();
        let tax = subtotal * 0.18;  // 18% GST
        let total = subtotal + tax;
        
        let invoice = Invoice {
            id: format!("inv_{}", self.invoices.len()),
            invoice_number: format!("INV-{:04}", self.invoices.len() + 1),
            customer,
            date: "now".to_string(),
            items,
            subtotal,
            tax,
            total,
            status: "draft".to_string(),
        };
        
        self.invoices.insert(invoice.id.clone(), invoice.clone());
        invoice
    }

    /// Post invoice
    pub fn post_invoice(&mut self, invoice_id: &str) -> Result<(), String> {
        if let Some(invoice) = self.invoices.get_mut(invoice_id) {
            invoice.status = "posted".to_string();
            
            // Create ledger entries
            self.ledger.push(LedgerEntry {
                id: format!("ledger_{}", self.ledger.len()),
                account: "Accounts Receivable".to_string(),
                debit: invoice.total,
                credit: 0.0,
                date: invoice.date.clone(),
                description: format!("Invoice {}", invoice.invoice_number),
            });
            
            self.ledger.push(LedgerEntry {
                id: format!("ledger_{}", self.ledger.len()),
                account: "Sales".to_string(),
                debit: 0.0,
                credit: invoice.subtotal,
                date: invoice.date.clone(),
                description: format!("Invoice {}", invoice.invoice_number),
            });
            
            self.ledger.push(LedgerEntry {
                id: format!("ledger_{}", self.ledger.len()),
                account: "Output GST".to_string(),
                debit: 0.0,
                credit: invoice.tax,
                date: invoice.date.clone(),
                description: format!("Invoice {}", invoice.invoice_number),
            });
            
            Ok(())
        } else {
            Err("Invoice not found".to_string())
        }
    }

    /// Update stock
    pub fn update_stock(&mut self, product_id: &str, quantity: i32) -> Result<(), String> {
        if let Some(product) = self.products.get_mut(product_id) {
            let new_stock = product.stock as i32 + quantity;
            if new_stock >= 0 {
                product.stock = new_stock as u32;
                Ok(())
            } else {
                Err("Insufficient stock".to_string())
            }
        } else {
            Err("Product not found".to_string())
        }
    }

    /// Get trial balance
    pub fn get_trial_balance(&self) -> HashMap<String, f64> {
        let mut balance: HashMap<String, f64> = HashMap::new();
        
        for entry in &self.ledger {
            *balance.entry(entry.account.clone()).or_insert(0.0) += entry.debit - entry.credit;
        }
        
        balance
    }

    /// Get employee by ID
    pub fn get_employee(&self, id: &str) -> Option<&Employee> {
        self.employees.get(id)
    }

    /// Get all employees
    pub fn get_all_employees(&self) -> Vec<&Employee> {
        self.employees.values().collect()
    }

    /// Get product by ID
    pub fn get_product(&self, id: &str) -> Option<&Product> {
        self.products.get(id)
    }

    /// Get all products
    pub fn get_all_products(&self) -> Vec<&Product> {
        self.products.values().collect()
    }

    /// Get all invoices
    pub fn get_all_invoices(&self) -> Vec<&Invoice> {
        self.invoices.values().collect()
    }

    /// Get ledger entries
    pub fn get_ledger_entries(&self) -> Vec<&LedgerEntry> {
        self.ledger.iter().collect()
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut manager = ERPManager::new();
    
    println!("Sigma ERP System v0.1 - ERPNext Style");
    
    loop {
        println!("\n--- ERP Status ---");
        println!("Company: {}", manager.company.name);
        println!("Employees: {}", manager.employees.len());
        println!("Products: {}", manager.products.len());
        println!("Invoices: {}", manager.invoices.len());
        println!("Ledger Entries: {}", manager.ledger.len());
        
        println!("\nCommands: add_employee <name> <email> <dept> <designation> <salary>, add_product <name> <sku> <category> <price> <stock>, create_invoice <customer>, post_invoice <id>, update_stock <product_id> <quantity>, trial_balance, employees, products, invoices, ledger, quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "add_employee" => {
                if parts.len() >= 6 {
                    let name = parts[1].to_string();
                    let email = parts[2].to_string();
                    let department = parts[3].to_string();
                    let designation = parts[4].to_string();
                    if let Ok(salary) = parts[5].parse::<f64>() {
                        let employee = Employee {
                            id: format!("emp_{}", manager.employees.len()),
                            name,
                            email,
                            department,
                            designation,
                            salary,
                            joining_date: "now".to_string(),
                        };
                        manager.add_employee(employee);
                        println!("Employee added");
                    }
                }
            }
            "add_product" => {
                if parts.len() >= 6 {
                    let name = parts[1].to_string();
                    let sku = parts[2].to_string();
                    let category = parts[3].to_string();
                    if let (Ok(price), Ok(stock)) = (parts[4].parse::<f64>(), parts[5].parse::<u32>()) {
                        let product = Product {
                            id: format!("prod_{}", manager.products.len()),
                            name,
                            sku,
                            category,
                            price,
                            stock,
                            unit: "Piece".to_string(),
                        };
                        manager.add_product(product);
                        println!("Product added");
                    }
                }
            }
            "create_invoice" => {
                if let Some(customer) = parts.get(1) {
                    println!("Enter product ID, quantity, rate (one per line, empty to finish):");
                    let mut items = Vec::new();
                    loop {
                        let mut line = String::new();
                        std::io::stdin().read_line(&mut line).unwrap();
                        let line = line.trim();
                        if line.is_empty() {
                            break;
                        }
                        let item_parts: Vec<&str> = line.split_whitespace().collect();
                        if item_parts.len() >= 3 {
                            if let (Ok(quantity), Ok(rate)) = (item_parts[1].parse::<u32>(), item_parts[2].parse::<f64>()) {
                                items.push(InvoiceItem {
                                    product_id: item_parts[0].to_string(),
                                    product_name: item_parts[0].to_string(),
                                    quantity,
                                    rate,
                                    amount: quantity as f64 * rate,
                                });
                            }
                        }
                    }
                    let invoice = manager.create_invoice(customer.to_string(), items);
                    println!("Invoice created: {}", invoice.invoice_number);
                }
            }
            "post_invoice" => {
                if let Some(arg) = parts.get(1) {
                    match manager.post_invoice(arg) {
                        Ok(_) => println!("Invoice posted"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "update_stock" => {
                if parts.len() >= 3 {
                    if let Ok(quantity) = parts[2].parse::<i32>() {
                        match manager.update_stock(parts[1], quantity) {
                            Ok(_) => println!("Stock updated"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "trial_balance" => {
                println!("--- Trial Balance ---");
                for (account, balance) in manager.get_trial_balance() {
                    println!("{}: ₹{:.2}", account, balance);
                }
            }
            "employees" => {
                println!("--- All Employees ---");
                for emp in manager.get_all_employees() {
                    println!("{} - {} ({}) - ₹{:.2}", emp.name, emp.designation, emp.department, emp.salary);
                }
            }
            "products" => {
                println!("--- All Products ---");
                for prod in manager.get_all_products() {
                    println!("{} - {} ({}) - ₹{:.2} - Stock: {}", prod.name, prod.sku, prod.category, prod.price, prod.stock);
                }
            }
            "invoices" => {
                println!("--- All Invoices ---");
                for inv in manager.get_all_invoices() {
                    println!("{} - {} - {} - ₹{:.2} ({})", inv.invoice_number, inv.customer, inv.date, inv.total, inv.status);
                }
            }
            "ledger" => {
                println!("--- Ledger Entries ---");
                for entry in manager.get_ledger_entries() {
                    println!("{} - {} - Dr: ₹{:.2}, Cr: ₹{:.2} - {}", entry.date, entry.account, entry.debit, entry.credit, entry.description);
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
