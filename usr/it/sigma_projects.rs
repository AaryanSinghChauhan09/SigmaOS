// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/it/sigma_projects.rs — Sigma Curriculum-Linked Projects
//
// Implements templates for CBSE IT practicals including databases,
// web design, networking, and other curriculum-aligned projects.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Project Types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProjectCategory {
    Database,
    WebDesign,
    Networking,
    Programming,
    DataStructures,
}

#[derive(Debug, Clone)]
pub struct ProjectTemplate {
    pub id: String,
    pub name: String,
    pub category: ProjectCategory,
    pub chapter: String,
    pub description: String,
    pub difficulty: String,
    pub files: Vec<ProjectFile>,
    pub instructions: Vec<String>,
    pub evaluation_criteria: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ProjectFile {
    pub name: String,
    pub content: String,
    pub language: String,
}

// ─── Project Manager ─────────────────────────────────────────────────────────

pub struct ProjectManager {
    pub templates: HashMap<String, ProjectTemplate>,
    pub current_project: Option<String>,
}

impl ProjectManager {
    pub fn new() -> Self {
        let mut manager = ProjectManager {
            templates: HashMap::new(),
            current_project: None,
        };
        
        manager.init_cbse_projects();
        manager
    }

    /// Initialize CBSE IT project templates
    fn init_cbse_projects(&mut self) {
        // Database Projects
        self.add_template(ProjectTemplate {
            id: "db_library".to_string(),
            name: "Library Management System".to_string(),
            category: ProjectCategory::Database,
            chapter: "Chapter 10: Database Management".to_string(),
            description: "Create a database system to manage books, members, and issue/return operations".to_string(),
            difficulty: "Medium".to_string(),
            files: vec![
                ProjectFile {
                    name: "schema.sql".to_string(),
                    content: "-- Library Management System Schema
CREATE TABLE books (
    book_id INT PRIMARY KEY,
    title VARCHAR(100),
    author VARCHAR(50),
    category VARCHAR(30),
    price DECIMAL(10,2),
    quantity INT
);

CREATE TABLE members (
    member_id INT PRIMARY KEY,
    name VARCHAR(50),
    address VARCHAR(100),
    phone VARCHAR(15),
    join_date DATE
);

CREATE TABLE transactions (
    transaction_id INT PRIMARY KEY,
    book_id INT,
    member_id INT,
    issue_date DATE,
    return_date DATE,
    FOREIGN KEY (book_id) REFERENCES books(book_id),
    FOREIGN KEY (member_id) REFERENCES members(member_id)
);".to_string(),
                    language: "SQL".to_string(),
                },
            ],
            instructions: vec![
                "Create tables as per the schema".to_string(),
                "Insert sample data for books and members".to_string(),
                "Write queries to: List all books, Search by author, Track issued books".to_string(),
                "Create a form for issuing and returning books".to_string(),
            ],
            evaluation_criteria: vec![
                "Correct table structure with proper data types".to_string(),
                "Implementation of primary and foreign keys".to_string(),
                "Functional queries for all required operations".to_string(),
                "User-friendly interface".to_string(),
            ],
        });

        // Web Design Projects
        self.add_template(ProjectTemplate {
            id: "web_portfolio".to_string(),
            name: "Personal Portfolio Website".to_string(),
            category: ProjectCategory::WebDesign,
            chapter: "Chapter 7: Web Technologies".to_string(),
            description: "Design a responsive personal portfolio website with HTML, CSS, and JavaScript".to_string(),
            difficulty: "Easy".to_string(),
            files: vec![
                ProjectFile {
                    name: "index.html".to_string(),
                    content: "<!DOCTYPE html>
<html>
<head>
    <title>My Portfolio</title>
    <link rel=\"stylesheet\" href=\"style.css\">
</head>
<body>
    <header>
        <h1>Welcome to My Portfolio</h1>
        <nav>
            <a href=\"#about\">About</a>
            <a href=\"#projects\">Projects</a>
            <a href=\"#contact\">Contact</a>
        </nav>
    </header>
    <section id=\"about\">
        <h2>About Me</h2>
        <p>Add your introduction here</p>
    </section>
    <section id=\"projects\">
        <h2>My Projects</h2>
        <div class=\"project\">
            <h3>Project 1</h3>
            <p>Description of your project</p>
        </div>
    </section>
    <section id=\"contact\">
        <h2>Contact</h2>
        <p>Email: your.email@example.com</p>
    </section>
</body>
</html>".to_string(),
                    language: "HTML".to_string(),
                },
                ProjectFile {
                    name: "style.css".to_string(),
                    content: "body {
    font-family: Arial, sans-serif;
    margin: 0;
    padding: 0;
    background-color: #f4f4f4;
}

header {
    background-color: #333;
    color: white;
    padding: 20px;
    text-align: center;
}

nav a {
    color: white;
    margin: 0 15px;
    text-decoration: none;
}

section {
    padding: 40px;
    margin: 20px;
    background-color: white;
    border-radius: 5px;
}

.project {
    border: 1px solid #ddd;
    padding: 15px;
    margin: 10px 0;
}".to_string(),
                    language: "CSS".to_string(),
                },
            ],
            instructions: vec![
                "Customize the HTML with your personal information".to_string(),
                "Style the website using CSS for better appearance".to_string(),
                "Add JavaScript for interactivity (optional)".to_string(),
                "Ensure responsive design for mobile devices".to_string(),
            ],
            evaluation_criteria: vec![
                "Clean and semantic HTML structure".to_string(),
                "Effective use of CSS for styling".to_string(),
                "Responsive design implementation".to_string(),
                "Visual appeal and user experience".to_string(),
            ],
        });

        // Networking Projects
        self.add_template(ProjectTemplate {
            id: "net_config".to_string(),
            name: "Network Configuration Lab".to_string(),
            category: ProjectCategory::Networking,
            chapter: "Chapter 8: Networking".to_string(),
            description: "Configure a small network with IP addressing, subnetting, and basic services".to_string(),
            difficulty: "Medium".to_string(),
            files: vec![
                ProjectFile {
                    name: "network_diagram.txt".to_string(),
                    content: "Network Topology:
[Router] -- [Switch] -- [PC1]
                     |-- [PC2]
                     |-- [PC3]
                     |-- [Server]

IP Addressing Scheme:
Network: 192.168.1.0/24
Router: 192.168.1.1
PC1: 192.168.1.10
PC2: 192.168.1.11
PC3: 192.168.1.12
Server: 192.168.1.100".to_string(),
                    language: "Text".to_string(),
                },
            ],
            instructions: vec![
                "Configure IP addresses on all devices".to_string(),
                "Set up DHCP server on the router".to_string(),
                "Configure basic firewall rules".to_string(),
                "Test connectivity between all devices".to_string(),
            ],
            evaluation_criteria: vec![
                "Correct IP address configuration".to_string(),
                "Successful DHCP implementation".to_string(),
                "Proper firewall rule configuration".to_string(),
                "Full network connectivity verified".to_string(),
            ],
        });

        // Programming Projects
        self.add_template(ProjectTemplate {
            id: "prog_banking".to_string(),
            name: "Bank Account Management".to_string(),
            category: ProjectCategory::Programming,
            chapter: "Chapter 5: Object-Oriented Programming".to_string(),
            description: "Implement a banking system with account creation, deposit, withdrawal, and balance inquiry".to_string(),
            difficulty: "Medium".to_string(),
            files: vec![
                ProjectFile {
                    name: "BankAccount.java".to_string(),
                    content: "class BankAccount {
    private String accountNumber;
    private String accountHolder;
    private double balance;
    
    public BankAccount(String accNum, String holder, double initialBalance) {
        this.accountNumber = accNum;
        this.accountHolder = holder;
        this.balance = initialBalance;
    }
    
    public void deposit(double amount) {
        if (amount > 0) {
            balance += amount;
            System.out.println(\"Deposited: \" + amount);
        }
    }
    
    public void withdraw(double amount) {
        if (amount > 0 && balance >= amount) {
            balance -= amount;
            System.out.println(\"Withdrawn: \" + amount);
        } else {
            System.out.println(\"Insufficient balance\");
        }
    }
    
    public double getBalance() {
        return balance;
    }
    
    public void displayAccountDetails() {
        System.out.println(\"Account: \" + accountNumber);
        System.out.println(\"Holder: \" + accountHolder);
        System.out.println(\"Balance: \" + balance);
    }
}".to_string(),
                    language: "Java".to_string(),
                },
            ],
            instructions: vec![
                "Implement the BankAccount class with all methods".to_string(),
                "Create a main class to test the functionality".to_string(),
                "Add error handling for invalid operations".to_string(),
                "Implement account number validation".to_string(),
            ],
            evaluation_criteria: vec![
                "Correct implementation of OOP concepts".to_string(),
                "Proper encapsulation with private fields".to_string(),
                "Effective error handling".to_string(),
                "Clean and readable code".to_string(),
            ],
        });
    }

    /// Add template to manager
    fn add_template(&mut self, template: ProjectTemplate) {
        self.templates.insert(template.id.clone(), template);
    }

    /// Get template by ID
    pub fn get_template(&self, id: &str) -> Option<&ProjectTemplate> {
        self.templates.get(id)
    }

    /// Get templates by category
    pub fn get_templates_by_category(&self, category: ProjectCategory) -> Vec<&ProjectTemplate> {
        self.templates.values()
            .filter(|t| t.category == category)
            .collect()
    }

    /// Get all templates
    pub fn get_all_templates(&self) -> Vec<&ProjectTemplate> {
        self.templates.values().collect()
    }

    /// Start project from template
    pub fn start_project(&mut self, template_id: &str) -> Result<(), String> {
        if self.templates.contains_key(template_id) {
            self.current_project = Some(template_id.to_string());
            Ok(())
        } else {
            Err("Template not found".to_string())
        }
    }

    /// Get current project
    pub fn get_current_project(&self) -> Option<&ProjectTemplate> {
        self.current_project.as_ref()
            .and_then(|id| self.templates.get(id))
    }

    /// Evaluate project (simplified)
    pub fn evaluate_project(&self, template_id: &str, completed_tasks: Vec<String>) -> (u32, u32) {
        if let Some(template) = self.templates.get(template_id) {
            let total = template.evaluation_criteria.len() as u32;
            let completed = completed_tasks.len() as u32;
            (completed, total)
        } else {
            (0, 0)
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut manager = ProjectManager::new();
    
    println!("Sigma Curriculum Projects v0.1 - CBSE IT Practicals");
    
    loop {
        println!("\n--- Available Projects ---");
        for template in manager.get_all_templates() {
            let cat_str = match template.category {
                ProjectCategory::Database => "DB",
                ProjectCategory::WebDesign => "WEB",
                ProjectCategory::Networking => "NET",
                ProjectCategory::Programming => "PROG",
                ProjectCategory::DataStructures => "DS",
            };
            println!("[{}] {} - {} ({})", cat_str, template.name, template.chapter, template.difficulty);
        }
        
        println!("\nCommands: list <category>, start <id>, view, files, instructions, evaluate <id> <tasks>, quit");
        println!("Categories: database, web, networking, programming, ds");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "list" => {
                if let Some(arg) = parts.get(1) {
                    let category = match *arg {
                        "database" => ProjectCategory::Database,
                        "web" => ProjectCategory::WebDesign,
                        "networking" => ProjectCategory::Networking,
                        "programming" => ProjectCategory::Programming,
                        "ds" => ProjectCategory::DataStructures,
                        _ => {
                            println!("Unknown category");
                            continue;
                        }
                    };
                    println!("--- {} Projects ---", arg);
                    for template in manager.get_templates_by_category(category) {
                        println!("{}: {} - {}", template.id, template.name, template.description);
                    }
                }
            }
            "start" => {
                if let Some(arg) = parts.get(1) {
                    match manager.start_project(arg) {
                        Ok(_) => println!("Started project: {}", arg),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "view" => {
                if let Some(project) = manager.get_current_project() {
                    println!("--- Project Details ---");
                    println!("Name: {}", project.name);
                    println!("Chapter: {}", project.chapter);
                    println!("Description: {}", project.description);
                    println!("Difficulty: {}", project.difficulty);
                } else {
                    println!("No project selected");
                }
            }
            "files" => {
                if let Some(project) = manager.get_current_project() {
                    println!("--- Project Files ---");
                    for file in &project.files {
                        println!("{} ({})", file.name, file.language);
                    }
                } else {
                    println!("No project selected");
                }
            }
            "instructions" => {
                if let Some(project) = manager.get_current_project() {
                    println!("--- Instructions ---");
                    for (i, instr) in project.instructions.iter().enumerate() {
                        println!("{}. {}", i + 1, instr);
                    }
                } else {
                    println!("No project selected");
                }
            }
            "evaluate" => {
                if parts.len() >= 3 {
                    let template_id = parts[1];
                    let tasks: Vec<String> = parts[2..].to_vec();
                    let (completed, total) = manager.evaluate_project(template_id, tasks);
                    println!("Evaluation: {}/{} criteria met", completed, total);
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
