// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/net/sigma_cups.rs — Sigma CUPS Printing System
//
// Implements CUPS-style printing system with printer management,
// job queue management, printer drivers, and network printing.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── CUPS Types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JobState {
    Pending,
    Held,
    Processing,
    Completed,
    Aborted,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrinterState {
    Idle,
    Printing,
    Stopped,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MediaType {
    Plain,
    Photo,
    Glossy,
    Matte,
    Transparencies,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrintQuality {
    Draft,
    Normal,
    High,
    Photo,
}

#[derive(Debug, Clone)]
pub struct Printer {
    pub id: String,
    pub name: String,
    pub device_uri: String,
    pub make_and_model: String,
    pub location: String,
    pub state: PrinterState,
    pub accepting_jobs: bool,
    pub default_media: MediaType,
    pub default_quality: PrintQuality,
    pub max_resolution: (u32, u32),  // DPI (x, y)
    pub color_supported: bool,
    pub duplex_supported: bool,
}

#[derive(Debug, Clone)]
pub struct PrintJob {
    pub id: String,
    pub title: String,
    pub user: String,
    pub printer_id: String,
    pub file_path: String,
    pub pages: u32,
    pub copies: u32,
    pub state: JobState,
    pub created: String,
    pub started: Option<String>,
    pub completed: Option<String>,
    pub media: MediaType,
    pub quality: PrintQuality,
    pub duplex: bool,
    pub color: bool,
}

#[derive(Debug, Clone)]
pub struct PrintQueue {
    pub printer_id: String,
    pub jobs: Vec<String>,  // Job IDs
    pub max_jobs: u32,
    pub enabled: bool,
}

// ─── CUPS Manager ────────────────────────────────────────────────────────

pub struct CUPSManager {
    pub printers: HashMap<String, Printer>,
    pub jobs: HashMap<String, PrintJob>,
    pub queues: HashMap<String, PrintQueue>,
    pub default_printer: Option<String>,
    pub server_running: bool,
    pub server_port: u16,
}

impl CUPSManager {
    pub fn new() -> Self {
        let mut manager = CUPSManager {
            printers: HashMap::new(),
            jobs: HashMap::new(),
            queues: HashMap::new(),
            default_printer: None,
            server_running: true,
            server_port: 631,
        };

        manager.init_default_printers();
        manager
    }

    /// Initialize default printers
    fn init_default_printers(&mut self) {
        let printer = Printer {
            id: "printer_1".to_string(),
            name: "HP_LaserJet".to_string(),
            device_uri: "ipp://192.168.1.50:631/ipp/print".to_string(),
            make_and_model: "HP LaserJet Pro M404n".to_string(),
            location: "Office".to_string(),
            state: PrinterState::Idle,
            accepting_jobs: true,
            default_media: MediaType::Plain,
            default_quality: PrintQuality::Normal,
            max_resolution: (1200, 1200),
            color_supported: false,
            duplex_supported: true,
        };

        let printer_id = printer.id.clone();
        self.printers.insert(printer_id.clone(), printer.clone());
        self.queues.insert(printer_id.clone(), PrintQueue {
            printer_id: printer_id.clone(),
            jobs: vec![],
            max_jobs: 100,
            enabled: true,
        });
        self.default_printer = Some(printer_id);
    }

    /// Add a printer
    pub fn add_printer(&mut self, printer: Printer) -> Result<Printer, String> {
        if self.printers.contains_key(&printer.id) {
            return Err("Printer already exists".to_string());
        }

        let printer_id = printer.id.clone();
        self.queues.insert(printer_id.clone(), PrintQueue {
            printer_id: printer_id.clone(),
            jobs: vec![],
            max_jobs: 100,
            enabled: true,
        });

        self.printers.insert(printer_id.clone(), printer.clone());
        Ok(printer)
    }

    /// Remove a printer
    pub fn remove_printer(&mut self, printer_id: &str) -> Result<(), String> {
        if self.printers.remove(printer_id).is_some() {
            self.queues.remove(printer_id);
            if self.default_printer.as_ref() == Some(&printer_id.to_string()) {
                self.default_printer = None;
            }
            Ok(())
        } else {
            Err("Printer not found".to_string())
        }
    }

    /// Set default printer
    pub fn set_default_printer(&mut self, printer_id: &str) -> Result<(), String> {
        if self.printers.contains_key(printer_id) {
            self.default_printer = Some(printer_id.to_string());
            Ok(())
        } else {
            Err("Printer not found".to_string())
        }
    }

    /// Enable printer
    pub fn enable_printer(&mut self, printer_id: &str) -> Result<(), String> {
        if let Some(printer) = self.printers.get_mut(printer_id) {
            printer.accepting_jobs = true;
            if let Some(queue) = self.queues.get_mut(printer_id) {
                queue.enabled = true;
            }
            Ok(())
        } else {
            Err("Printer not found".to_string())
        }
    }

    /// Disable printer
    pub fn disable_printer(&mut self, printer_id: &str) -> Result<(), String> {
        if let Some(printer) = self.printers.get_mut(printer_id) {
            printer.accepting_jobs = false;
            if let Some(queue) = self.queues.get_mut(printer_id) {
                queue.enabled = false;
            }
            Ok(())
        } else {
            Err("Printer not found".to_string())
        }
    }

    /// Submit a print job
    pub fn submit_job(&mut self, printer_id: &str, title: String, user: String, file_path: String, pages: u32, copies: u32) -> Result<PrintJob, String> {
        if !self.printers.contains_key(printer_id) {
            return Err("Printer not found".to_string());
        }

        let printer = self.printers.get(printer_id).unwrap();
        if !printer.accepting_jobs {
            return Err("Printer not accepting jobs".to_string());
        }

        let job_id = format!("job_{}", self.jobs.len());
        let job = PrintJob {
            id: job_id.clone(),
            title,
            user,
            printer_id: printer_id.to_string(),
            file_path,
            pages,
            copies,
            state: JobState::Pending,
            created: "now".to_string(),
            started: None,
            completed: None,
            media: printer.default_media,
            quality: printer.default_quality,
            duplex: printer.duplex_supported,
            color: printer.color_supported,
        };

        if let Some(queue) = self.queues.get_mut(printer_id) {
            queue.jobs.push(job_id.clone());
        }

        self.jobs.insert(job_id.clone(), job.clone());
        Ok(job)
    }

    /// Cancel a job
    pub fn cancel_job(&mut self, job_id: &str) -> Result<(), String> {
        if let Some(job) = self.jobs.get_mut(job_id) {
            if job.state == JobState::Completed {
                return Err("Job already completed".to_string());
            }
            job.state = JobState::Cancelled;
            
            // Remove from queue
            if let Some(queue) = self.queues.get_mut(&job.printer_id) {
                queue.jobs.retain(|id| id != job_id);
            }
            
            Ok(())
        } else {
            Err("Job not found".to_string())
        }
    }

    /// Hold a job
    pub fn hold_job(&mut self, job_id: &str) -> Result<(), String> {
        if let Some(job) = self.jobs.get_mut(job_id) {
            if job.state == JobState::Processing {
                return Err("Cannot hold processing job".to_string());
            }
            job.state = JobState::Held;
            Ok(())
        } else {
            Err("Job not found".to_string())
        }
    }

    /// Release a held job
    pub fn release_job(&mut self, job_id: &str) -> Result<(), String> {
        if let Some(job) = self.jobs.get_mut(job_id) {
            if job.state == JobState::Held {
                job.state = JobState::Pending;
            }
            Ok(())
        } else {
            Err("Job not found".to_string())
        }
    }

    /// Process next job in queue
    pub fn process_next_job(&mut self, printer_id: &str) -> Result<(), String> {
        if let Some(queue) = self.queues.get_mut(printer_id) {
            if let Some(job_id) = queue.jobs.first() {
                if let Some(job) = self.jobs.get_mut(job_id) {
                    job.state = JobState::Processing;
                    job.started = Some("now".to_string());
                    
                    // Simulate processing
                    job.state = JobState::Completed;
                    job.completed = Some("now".to_string());
                    
                    queue.jobs.remove(0);
                }
            }
            Ok(())
        } else {
            Err("Queue not found".to_string())
        }
    }

    /// Get jobs for printer
    pub fn get_printer_jobs(&self, printer_id: &str) -> Vec<&PrintJob> {
        self.jobs.values()
            .filter(|j| j.printer_id == printer_id)
            .collect()
    }

    /// Get all jobs
    pub fn get_all_jobs(&self) -> Vec<&PrintJob> {
        self.jobs.values().collect()
    }

    /// Get job by ID
    pub fn get_job(&self, job_id: &str) -> Option<&PrintJob> {
        self.jobs.get(job_id)
    }

    /// List all printers
    pub fn list_printers(&self) -> Vec<&Printer> {
        self.printers.values().collect()
    }

    /// Get printer by ID
    pub fn get_printer(&self, printer_id: &str) -> Option<&Printer> {
        self.printers.get(printer_id)
    }

    /// Get queue status
    pub fn get_queue_status(&self, printer_id: &str) -> Option<&PrintQueue> {
        self.queues.get(printer_id)
    }

    /// Set server port
    pub fn set_server_port(&mut self, port: u16) {
        self.server_port = port;
    }

    /// Start server
    pub fn start_server(&mut self) {
        self.server_running = true;
    }

    /// Stop server
    pub fn stop_server(&mut self) {
        self.server_running = false;
    }

    /// Get statistics
    pub fn get_statistics(&self) -> HashMap<String, u32> {
        let mut stats = HashMap::new();
        stats.insert("total_printers".to_string(), self.printers.len() as u32);
        stats.insert("total_jobs".to_string(), self.jobs.len() as u32);
        stats.insert("pending_jobs".to_string(), self.jobs.values().filter(|j| j.state == JobState::Pending).count() as u32);
        stats.insert("processing_jobs".to_string(), self.jobs.values().filter(|j| j.state == JobState::Processing).count() as u32);
        stats.insert("completed_jobs".to_string(), self.jobs.values().filter(|j| j.state == JobState::Completed).count() as u32);
        stats.insert("active_printers".to_string(), self.printers.values().filter(|p| p.state == PrinterState::Printing).count() as u32);
        stats
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut cups = CUPSManager::new();
    
    println!("Sigma CUPS v0.1 - Printing System");
    
    loop {
        println!("\n--- CUPS Commands ---");
        println!("printers          - List all printers");
        println!("add_printer <id> <name> <uri> <model> <location> - Add printer");
        println!("remove_printer <id> - Remove printer");
        println!("set_default <id>   - Set default printer");
        println!("enable <id>        - Enable printer");
        println!("disable <id>       - Disable printer");
        println!("printer_info <id>  - Show printer details");
        println!("queue <id>         - Show queue status");
        println!("submit <printer> <title> <user> <file> <pages> <copies> - Submit job");
        println!("jobs [printer]     - List jobs");
        println!("job_info <id>     - Show job details");
        println!("cancel <job_id>    - Cancel job");
        println!("hold <job_id>      - Hold job");
        println!("release <job_id>   - Release job");
        println!("process <printer>  - Process next job");
        println!("stats              - Show statistics");
        println!("server_port <port> - Set server port");
        println!("start_server       - Start server");
        println!("stop_server        - Stop server");
        println!("quit               - Exit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "printers" => {
                println!("--- Printers ---");
                for printer in cups.list_printers() {
                    println!("{} - {} - {} - {:?}", 
                        printer.id, printer.name, printer.location, printer.state);
                }
            }
            "add_printer" => {
                if parts.len() >= 6 {
                    let id = parts[1].to_string();
                    let name = parts[2].to_string();
                    let uri = parts[3].to_string();
                    let model = parts[4].to_string();
                    let location = parts[5].to_string();
                    let printer = Printer {
                        id: id.clone(),
                        name,
                        device_uri: uri,
                        make_and_model: model,
                        location,
                        state: PrinterState::Idle,
                        accepting_jobs: true,
                        default_media: MediaType::Plain,
                        default_quality: PrintQuality::Normal,
                        max_resolution: (600, 600),
                        color_supported: true,
                        duplex_supported: false,
                    };
                    match cups.add_printer(printer) {
                        Ok(_) => println!("Printer added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "remove_printer" => {
                if let Some(id) = parts.get(1) {
                    match cups.remove_printer(id) {
                        Ok(_) => println!("Printer removed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "set_default" => {
                if let Some(id) = parts.get(1) {
                    match cups.set_default_printer(id) {
                        Ok(_) => println!("Default printer set"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "enable" => {
                if let Some(id) = parts.get(1) {
                    match cups.enable_printer(id) {
                        Ok(_) => println!("Printer enabled"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "disable" => {
                if let Some(id) = parts.get(1) {
                    match cups.disable_printer(id) {
                        Ok(_) => println!("Printer disabled"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "printer_info" => {
                if let Some(id) = parts.get(1) {
                    if let Some(printer) = cups.get_printer(id) {
                        println!("--- Printer Info ---");
                        println!("ID: {}", printer.id);
                        println!("Name: {}", printer.name);
                        println!("Model: {}", printer.make_and_model);
                        println!("Location: {}", printer.location);
                        println!("URI: {}", printer.device_uri);
                        println!("State: {:?}", printer.state);
                        println!("Accepting Jobs: {}", printer.accepting_jobs);
                        println!("Max Resolution: {}x{} DPI", printer.max_resolution.0, printer.max_resolution.1);
                        println!("Color: {}", printer.color_supported);
                        println!("Duplex: {}", printer.duplex_supported);
                    }
                }
            }
            "queue" => {
                if let Some(id) = parts.get(1) {
                    if let Some(queue) = cups.get_queue_status(id) {
                        println!("--- Queue Status ---");
                        println!("Printer: {}", queue.printer_id);
                        println!("Jobs in queue: {}", queue.jobs.len());
                        println!("Max jobs: {}", queue.max_jobs);
                        println!("Enabled: {}", queue.enabled);
                    }
                }
            }
            "submit" => {
                if parts.len() >= 7 {
                    let printer = parts[1];
                    let title = parts[2].to_string();
                    let user = parts[3].to_string();
                    let file = parts[4].to_string();
                    let pages = parts[5].parse::<u32>().unwrap_or(1);
                    let copies = parts[6].parse::<u32>().unwrap_or(1);
                    match cups.submit_job(printer, title, user, file, pages, copies) {
                        Ok(job) => println!("Job submitted: {}", job.id),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "jobs" => {
                let printer = parts.get(1).copied();
                println!("--- Jobs ---");
                let jobs = if let Some(p) = printer {
                    cups.get_printer_jobs(p)
                } else {
                    cups.get_all_jobs()
                };
                for job in jobs {
                    println!("{} - {} - {} - {:?}", job.id, job.title, job.user, job.state);
                }
            }
            "job_info" => {
                if let Some(id) = parts.get(1) {
                    if let Some(job) = cups.get_job(id) {
                        println!("--- Job Info ---");
                        println!("ID: {}", job.id);
                        println!("Title: {}", job.title);
                        println!("User: {}", job.user);
                        println!("Printer: {}", job.printer_id);
                        println!("File: {}", job.file_path);
                        println!("Pages: {}", job.pages);
                        println!("Copies: {}", job.copies);
                        println!("State: {:?}", job.state);
                        println!("Created: {}", job.created);
                        println!("Media: {:?}", job.media);
                        println!("Quality: {:?}", job.quality);
                    }
                }
            }
            "cancel" => {
                if let Some(id) = parts.get(1) {
                    match cups.cancel_job(id) {
                        Ok(_) => println!("Job cancelled"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "hold" => {
                if let Some(id) = parts.get(1) {
                    match cups.hold_job(id) {
                        Ok(_) => println!("Job held"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "release" => {
                if let Some(id) = parts.get(1) {
                    match cups.release_job(id) {
                        Ok(_) => println!("Job released"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "process" => {
                if let Some(id) = parts.get(1) {
                    match cups.process_next_job(id) {
                        Ok(_) => println!("Job processed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "stats" => {
                println!("--- Statistics ---");
                for (key, value) in cups.get_statistics() {
                    println!("{}: {}", key, value);
                }
            }
            "server_port" => {
                if let Some(port) = parts.get(1).and_then(|p| p.parse::<u16>().ok()) {
                    cups.set_server_port(port);
                    println!("Server port set to {}", port);
                }
            }
            "start_server" => {
                cups.start_server();
                println!("Server started");
            }
            "stop_server" => {
                cups.stop_server();
                println!("Server stopped");
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
