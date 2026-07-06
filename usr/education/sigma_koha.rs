// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/education/sigma_koha.rs — Sigma Library Management (Koha)
//
// Implements Koha-style library management with catalog management,
// patron management, circulation, acquisitions, and reporting.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Library Types ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Book {
    pub id: String,
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub publisher: String,
    pub publication_year: u32,
    pub category: String,
    pub location: String,
    pub status: String,  // available, checked_out, lost, damaged
    pub barcode: String,
}

#[derive(Debug, Clone)]
pub struct Patron {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub card_number: String,
    pub category: String,  // student, faculty, staff
    pub borrowed_books: Vec<String>,
    pub fines: f64,
}

#[derive(Debug, Clone)]
pub struct Checkout {
    pub id: String,
    pub book_id: String,
    pub patron_id: String,
    pub checkout_date: String,
    pub due_date: String,
    pub return_date: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct Acquisition {
    pub id: String,
    pub title: String,
    pub isbn: String,
    pub quantity: u32,
    pub cost: f64,
    pub vendor: String,
    pub order_date: String,
    pub received_date: Option<String>,
}

// ─── Library Manager ────────────────────────────────────────────────────

pub struct LibraryManager {
    pub books: HashMap<String, Book>,
    pub patrons: HashMap<String, Patron>,
    pub checkouts: Vec<Checkout>,
    pub acquisitions: Vec<Acquisition>,
    pub categories: Vec<String>,
}

impl LibraryManager {
    pub fn new() -> Self {
        let mut manager = LibraryManager {
            books: HashMap::new(),
            patrons: HashMap::new(),
            checkouts: Vec::new(),
            acquisitions: Vec::new(),
            categories: vec![
                "Fiction".to_string(),
                "Non-Fiction".to_string(),
                "Science".to_string(),
                "Technology".to_string(),
                "History".to_string(),
                "Reference".to_string(),
            ],
        };
        
        manager.init_sample_books();
        manager.init_sample_patrons();
        manager
    }

    /// Initialize sample books
    fn init_sample_books(&mut self) {
        self.books.insert("book_001".to_string(), Book {
            id: "book_001".to_string(),
            isbn: "978-0-13-468599-1".to_string(),
            title: "The Rust Programming Language".to_string(),
            author: "Steve Klabnik, Carol Nichols".to_string(),
            publisher: "No Starch Press".to_string(),
            publication_year: 2019,
            category: "Technology".to_string(),
            location: "Stack A, Shelf 1".to_string(),
            status: "available".to_string(),
            barcode: "1234567890".to_string(),
        });

        self.books.insert("book_002".to_string(), Book {
            id: "book_002".to_string(),
            isbn: "978-0-262-03293-3".to_string(),
            title: "Introduction to Algorithms".to_string(),
            author: "Thomas H. Cormen et al.".to_string(),
            publisher: "MIT Press".to_string(),
            publication_year: 2009,
            category: "Science".to_string(),
            location: "Stack B, Shelf 2".to_string(),
            status: "available".to_string(),
            barcode: "1234567891".to_string(),
        });
    }

    /// Initialize sample patrons
    fn init_sample_patrons(&mut self) {
        self.patrons.insert("patron_001".to_string(), Patron {
            id: "patron_001".to_string(),
            name: "Rahul Sharma".to_string(),
            email: "rahul.sharma@sigmaos.edu".to_string(),
            phone: "+91-9876543210".to_string(),
            address: "New Delhi, India".to_string(),
            card_number: "LIB001".to_string(),
            category: "student".to_string(),
            borrowed_books: Vec::new(),
            fines: 0.0,
        });
    }

    /// Add book
    pub fn add_book(&mut self, book: Book) {
        self.books.insert(book.id.clone(), book);
    }

    /// Add patron
    pub fn add_patron(&mut self, patron: Patron) {
        self.patrons.insert(patron.id.clone(), patron);
    }

    /// Checkout book
    pub fn checkout_book(&mut self, book_id: &str, patron_id: &str) -> Result<Checkout, String> {
        if let Some(book) = self.books.get_mut(book_id) {
            if book.status != "available" {
                return Err("Book is not available".to_string());
            }
            
            if let Some(patron) = self.patrons.get_mut(patron_id) {
                book.status = "checked_out".to_string();
                patron.borrowed_books.push(book_id.to_string());
                
                let checkout = Checkout {
                    id: format!("checkout_{}", self.checkouts.len()),
                    book_id: book_id.to_string(),
                    patron_id: patron_id.to_string(),
                    checkout_date: "now".to_string(),
                    due_date: "2024-02-15".to_string(),
                    return_date: None,
                    status: "active".to_string(),
                };
                
                self.checkouts.push(checkout.clone());
                Ok(checkout)
            } else {
                Err("Patron not found".to_string())
            }
        } else {
            Err("Book not found".to_string())
        }
    }

    /// Return book
    pub fn return_book(&mut self, checkout_id: &str) -> Result<(), String> {
        if let Some(checkout) = self.checkouts.iter_mut().find(|c| c.id == checkout_id) {
            if let Some(book) = self.books.get_mut(&checkout.book_id) {
                book.status = "available".to_string();
            }
            
            if let Some(patron) = self.patrons.get_mut(&checkout.patron_id) {
                patron.borrowed_books.retain(|b| b != &checkout.book_id);
            }
            
            checkout.return_date = Some("now".to_string());
            checkout.status = "returned".to_string();
            
            Ok(())
        } else {
            Err("Checkout not found".to_string())
        }
    }

    /// Add acquisition
    pub fn add_acquisition(&mut self, acquisition: Acquisition) {
        self.acquisitions.push(acquisition);
    }

    /// Calculate fine
    pub fn calculate_fine(&self, checkout_id: &str) -> f64 {
        if let Some(checkout) = self.checkouts.iter().find(|c| c.id == checkout_id) {
            if checkout.return_date.is_none() {
                // Simulate overdue calculation
                5.0  // ₹5 per day overdue
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    /// Search books
    pub fn search_books(&self, query: &str) -> Vec<&Book> {
        self.books.values()
            .filter(|b| {
                b.title.to_lowercase().contains(&query.to_lowercase()) ||
                b.author.to_lowercase().contains(&query.to_lowercase()) ||
                b.isbn.contains(query)
            })
            .collect()
    }

    /// Get books by category
    pub fn get_books_by_category(&self, category: &str) -> Vec<&Book> {
        self.books.values().filter(|b| b.category == category).collect()
    }

    /// Get available books
    pub fn get_available_books(&self) -> Vec<&Book> {
        self.books.values().filter(|b| b.status == "available").collect()
    }

    /// Get overdue checkouts
    pub fn get_overdue_checkouts(&self) -> Vec<&Checkout> {
        self.checkouts.iter().filter(|c| c.return_date.is_none() && c.status == "active").collect()
    }

    /// Get book by ID
    pub fn get_book(&self, id: &str) -> Option<&Book> {
        self.books.get(id)
    }

    /// Get patron by ID
    pub fn get_patron(&self, id: &str) -> Option<&Patron> {
        self.patrons.get(id)
    }

    /// Get all books
    pub fn get_all_books(&self) -> Vec<&Book> {
        self.books.values().collect()
    }

    /// Get all patrons
    pub fn get_all_patrons(&self) -> Vec<&Patron> {
        self.patrons.values().collect()
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut manager = LibraryManager::new();
    
    println!("Sigma Library Management v0.1 - Koha Style");
    
    loop {
        println!("\n--- Library Status ---");
        println!("Books: {}", manager.books.len());
        println!("Patrons: {}", manager.patrons.len());
        println!("Checkouts: {}", manager.checkouts.len());
        println!("Acquisitions: {}", manager.acquisitions.len());
        println!("Overdue: {}", manager.get_overdue_checkouts().len());
        
        println!("\nCommands: add_book <isbn> <title> <author> <category>, add_patron <name> <email> <category>, checkout <book_id> <patron_id>, return <checkout_id>, search <query>, books_category <category>, available, overdue, patrons, books, quit");
        println!("Patron categories: student, faculty, staff");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "add_book" => {
                if parts.len() >= 5 {
                    let isbn = parts[1].to_string();
                    let title = parts[2].to_string();
                    let author = parts[3].to_string();
                    let category = parts[4].to_string();
                    let book = Book {
                        id: format!("book_{}", manager.books.len()),
                        isbn,
                        title,
                        author,
                        publisher: "Unknown".to_string(),
                        publication_year: 2024,
                        category,
                        location: "Stack A".to_string(),
                        status: "available".to_string(),
                        barcode: format!("{}", rand_id()),
                    };
                    manager.add_book(book);
                    println!("Book added");
                }
            }
            "add_patron" => {
                if parts.len() >= 4 {
                    let name = parts[1].to_string();
                    let email = parts[2].to_string();
                    let category = parts[3].to_string();
                    let patron = Patron {
                        id: format!("patron_{}", manager.patrons.len()),
                        name,
                        email,
                        phone: "".to_string(),
                        address: "".to_string(),
                        card_number: format!("LIB{:03}", manager.patrons.len()),
                        category,
                        borrowed_books: Vec::new(),
                        fines: 0.0,
                    };
                    manager.add_patron(patron);
                    println!("Patron added");
                }
            }
            "checkout" => {
                if parts.len() >= 3 {
                    match manager.checkout_book(parts[1], parts[2]) {
                        Ok(checkout) => println!("Checkout created: {}", checkout.id),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "return" => {
                if let Some(arg) = parts.get(1) {
                    match manager.return_book(arg) {
                        Ok(_) => println!("Book returned"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "search" => {
                if let Some(arg) = parts.get(1) {
                    println!("--- Search Results ---");
                    for book in manager.search_books(arg) {
                        println!("{} - {} ({})", book.title, book.author, book.status);
                    }
                }
            }
            "books_category" => {
                if let Some(arg) = parts.get(1) {
                    println!("--- Books in {} ---", arg);
                    for book in manager.get_books_by_category(arg) {
                        println!("{} - {} ({})", book.title, book.author, book.status);
                    }
                }
            }
            "available" => {
                println!("--- Available Books ---");
                for book in manager.get_available_books() {
                    println!("{} - {} ({})", book.title, book.author, book.location);
                }
            }
            "overdue" => {
                println!("--- Overdue Checkouts ---");
                for checkout in manager.get_overdue_checkouts() {
                    println!("{} - Due: {}", checkout.id, checkout.due_date);
                }
            }
            "patrons" => {
                println!("--- All Patrons ---");
                for patron in manager.get_all_patrons() {
                    println!("{} - {} ({})", patron.name, patron.card_number, patron.category);
                }
            }
            "books" => {
                println!("--- All Books ---");
                for book in manager.get_all_books() {
                    println!("{} - {} ({})", book.title, book.isbn, book.status);
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}

fn rand_id() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_nanos() as u32
}
