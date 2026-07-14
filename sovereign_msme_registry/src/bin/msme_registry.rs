// SovereignMSMERegistry CLI
// Command-line interface for MSME registration system

use sovereign_msme_registry::{BusinessDetails, MSMERegistry, OwnerDetails};
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }
    
    let mut registry = MSMERegistry::new();
    
    match args[1].as_str() {
        "register" => handle_register(&mut registry, &args),
        "get" => handle_get(&registry, &args),
        "list" => handle_list(&registry),
        "verify" => handle_verify(&registry, &args),
        _ => {
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!("SovereignMSMERegistry CLI");
    println!();
    println!("Usage:");
    println!("  msme_registry register <udyam_aadhar> <pan> <business_name> <address> <investment> <turnover> <owner_name> <aadhaar> <email> <phone>");
    println!("  msme_registry get <registration_id>");
    println!("  msme_registry list");
    println!("  msme_registry verify <registration_id>");
    println!();
    println!("Example:");
    println!("  msme_registry register UDYAM12345 ABCDE1234F \"My Business\" \"123 Main St\" 5000000 2000000 \"John Doe\" 123456789012 john@example.com 9876543210");
}

fn handle_register(registry: &mut MSMERegistry, args: &[String]) {
    if args.len() < 12 {
        eprintln!("Error: Insufficient arguments for register command");
        print_usage();
        std::process::exit(1);
    }
    
    let udyam_aadhar = args[2].clone();
    let pan = args[3].clone();
    let business_name = args[4].clone();
    let address = args[5].clone();
    let investment: u64 = args[6].parse().expect("Invalid investment amount");
    let turnover: u64 = args[7].parse().expect("Invalid turnover amount");
    let owner_name = args[8].clone();
    let aadhaar = args[9].clone();
    let email = args[10].clone();
    let phone = args[11].clone();
    
    let business_details = BusinessDetails::new(business_name, pan, address, investment, turnover);
    let owner_details = OwnerDetails::new(owner_name, aadhaar, email, phone);
    
    match registry.register_enterprise(udyam_aadhar, business_details, owner_details) {
        Ok(registration_id) => {
            println!("Registration successful!");
            println!("Registration ID: {}", registration_id);
            
            if let Some(registration) = registry.get_registration(&registration_id) {
                println!();
                println!("{}", registration);
            }
        }
        Err(e) => {
            eprintln!("Registration failed: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_get(registry: &MSMERegistry, args: &[String]) {
    if args.len() < 3 {
        eprintln!("Error: Registration ID required");
        print_usage();
        std::process::exit(1);
    }
    
    let registration_id = &args[2];
    
    match registry.get_registration(registration_id) {
        Some(registration) => {
            println!("{}", registration);
        }
        None => {
            eprintln!("Registration not found: {}", registration_id);
            std::process::exit(1);
        }
    }
}

fn handle_list(registry: &MSMERegistry) {
    let registrations = registry.list_registrations();
    
    if registrations.is_empty() {
        println!("No registrations found.");
        return;
    }
    
    println!("Registrations ({}):", registrations.len());
    println!();
    
    for registration in registrations {
        println!("ID: {}", registration.get_registration_id());
        println!("Business: {}", registration.business_details.name);
        println!("Type: {}", registration.enterprise_type.as_str());
        println!("Owner: {}", registration.owner_details.name);
        println!();
    }
}

fn handle_verify(registry: &MSMERegistry, args: &[String]) {
    if args.len() < 3 {
        eprintln!("Error: Registration ID required");
        print_usage();
        std::process::exit(1);
    }
    
    let registration_id = &args[2];
    
    match registry.verify_certificate(registration_id) {
        Ok(valid) => {
            if valid {
                println!("Certificate is valid");
            } else {
                println!("Certificate is invalid or expired");
            }
        }
        Err(e) => {
            eprintln!("Verification failed: {}", e);
            std::process::exit(1);
        }
    }
}
