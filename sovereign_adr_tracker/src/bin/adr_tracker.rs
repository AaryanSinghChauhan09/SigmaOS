// SovereignADRTracker CLI
// Command-line interface for ADR tracking system

use sovereign_adr_tracker::{ADRCase, ADRMechanism, ADRTracker, CaseStatus, ContactInfo, Document, Hearing, Party, PartyType, Settlement};
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }
    
    let mut tracker = ADRTracker::new();
    
    match args[1].as_str() {
        "create" => handle_create(&mut tracker, &args),
        "get" => handle_get(&tracker, &args),
        "list" => handle_list(&tracker),
        "status" => handle_status(&mut tracker, &args),
        "party" => handle_party(&mut tracker, &args),
        "document" => handle_document(&mut tracker, &args),
        "hearing" => handle_hearing(&mut tracker, &args),
        "settle" => handle_settle(&mut tracker, &args),
        "compliance" => handle_compliance(&tracker, &args),
        _ => {
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!("SovereignADRTracker CLI");
    println!();
    println!("Usage:");
    println!("  adr_tracker create <case_number> <mechanism> <subject> <description> <amount>");
    println!("  adr_tracker get <case_id>");
    println!("  adr_tracker list");
    println!("  adr_tracker status <case_id> <status>");
    println!("  adr_tracker party <case_id> <name> <party_type> <email> <phone> <address>");
    println!("  adr_tracker document <case_id> <doc_type> <file_name> <hash>");
    println!("  adr_tracker hearing <case_id> <scheduled_at> <location> <notes>");
    println!("  adr_tracker settle <case_id> <amount> <terms>");
    println!("  adr_tracker compliance <case_id>");
    println!();
    println!("Example:");
    println!("  adr_tracker create ADR001 arbitration \"Contract Dispute\" \"Test description\" 100000");
    println!("  adr_tracker party case001 \"John Doe\" claimant john@example.com 9876543210 \"123 Main St\"");
    println!("  adr_tracker status case001 in_progress");
    println!("  adr_tracker settle case001 50000 \"Full settlement\"");
}

fn handle_create(tracker: &mut ADRTracker, args: &[String]) {
    if args.len() < 7 {
        eprintln!("Error: Insufficient arguments for create command");
        print_usage();
        std::process::exit(1);
    }
    
    let case_number = args[2].clone();
    let mechanism_str = &args[3];
    let subject = args[4].clone();
    let description = args[5].clone();
    let amount: u64 = args[6].parse().expect("Invalid amount");
    
    let mechanism = match mechanism_str.to_lowercase().as_str() {
        "arbitration" => ADRMechanism::Arbitration,
        "mediation" => ADRMechanism::Mediation,
        "conciliation" => ADRMechanism::Conciliation,
        "negotiation" => ADRMechanism::Negotiation,
        _ => {
            eprintln!("Error: Invalid mechanism. Use: arbitration, mediation, conciliation, negotiation");
            std::process::exit(1);
        }
    };
    
    let case_id = tracker.create_case(case_number, mechanism, subject, description, amount);
    
    println!("Case created successfully!");
    println!("Case ID: {}", case_id);
    println!();
    
    if let Some(case) = tracker.get_case(&case_id) {
        println!("{}", case);
    }
}

fn handle_get(tracker: &ADRTracker, args: &[String]) {
    if args.len() < 3 {
        eprintln!("Error: Case ID required");
        print_usage();
        std::process::exit(1);
    }
    
    let case_id = &args[2];
    
    match tracker.get_case(case_id) {
        Some(case) => {
            println!("{}", case);
        }
        None => {
            eprintln!("Case not found: {}", case_id);
            std::process::exit(1);
        }
    }
}

fn handle_list(tracker: &ADRTracker) {
    let cases = tracker.list_cases();
    
    if cases.is_empty() {
        println!("No cases found.");
        return;
    }
    
    println!("ADR Cases ({}):", cases.len());
    println!();
    
    for case in cases {
        println!("Number: {}", case.case_number);
        println!("ID: {}", case.get_case_id());
        println!("Mechanism: {}", case.mechanism.as_str());
        println!("Status: {}", case.status.as_str());
        println!("Subject: {}", case.subject);
        println!("Amount: ₹{}", case.amount_in_dispute);
        println!();
    }
}

fn handle_status(tracker: &mut ADRTracker, args: &[String]) {
    if args.len() < 4 {
        eprintln!("Error: Case ID and status required");
        print_usage();
        std::process::exit(1);
    }
    
    let case_id = &args[2];
    let status_str = &args[3];
    
    let status = match status_str.to_lowercase().as_str() {
        "filed" => CaseStatus::Filed,
        "in_progress" => CaseStatus::InProgress,
        "pending" => CaseStatus::Pending,
        "resolved" => CaseStatus::Resolved,
        "withdrawn" => CaseStatus::Withdrawn,
        "escalated" => CaseStatus::Escalated,
        _ => {
            eprintln!("Error: Invalid status. Use: filed, in_progress, pending, resolved, withdrawn, escalated");
            std::process::exit(1);
        }
    };
    
    match tracker.update_case_status(case_id, status) {
        Ok(_) => {
            println!("Case status updated successfully");
            if let Some(case) = tracker.get_case(case_id) {
                println!("New status: {}", case.status.as_str());
            }
        }
        Err(e) => {
            eprintln!("Failed to update status: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_party(tracker: &mut ADRTracker, args: &[String]) {
    if args.len() < 8 {
        eprintln!("Error: Insufficient arguments for party command");
        print_usage();
        std::process::exit(1);
    }
    
    let case_id = &args[2];
    let name = args[3].clone();
    let party_type_str = &args[4];
    let email = args[5].clone();
    let phone = args[6].clone();
    let address = args[7].clone();
    
    let party_type = match party_type_str.to_lowercase().as_str() {
        "claimant" => PartyType::Claimant,
        "respondent" => PartyType::Respondent,
        _ => {
            eprintln!("Error: Invalid party type. Use: claimant, respondent");
            std::process::exit(1);
        }
    };
    
    let contact = ContactInfo::new(email, phone, address);
    let party = Party::new(name, party_type, contact);
    
    match tracker.add_party_to_case(case_id, party) {
        Ok(_) => {
            println!("Party added successfully");
            if let Some(case) = tracker.get_case(case_id) {
                println!("Total parties: {}", case.parties.len());
            }
        }
        Err(e) => {
            eprintln!("Failed to add party: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_document(tracker: &mut ADRTracker, args: &[String]) {
    if args.len() < 6 {
        eprintln!("Error: Insufficient arguments for document command");
        print_usage();
        std::process::exit(1);
    }
    
    let case_id = &args[2];
    let doc_type = args[3].clone();
    let file_name = args[4].clone();
    let hash_str = &args[5];
    
    let mut hash = [0u8; 32];
    let hash_bytes: Vec<u8> = hash_str.as_bytes().chunks(2).map(|b| u8::from_str_radix(b, 16).unwrap_or(0)).collect();
    for (i, &byte) in hash_bytes.iter().enumerate() {
        if i < 32 {
            hash[i] = byte;
        }
    }
    
    let document = Document::new(doc_type, file_name, hash);
    
    match tracker.add_document_to_case(case_id, document) {
        Ok(_) => {
            println!("Document added successfully");
            if let Some(case) = tracker.get_case(case_id) {
                println!("Total documents: {}", case.documents.len());
            }
        }
        Err(e) => {
            eprintln!("Failed to add document: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_hearing(tracker: &mut ADRTracker, args: &[String]) {
    if args.len() < 6 {
        eprintln!("Error: Insufficient arguments for hearing command");
        print_usage();
        std::process::exit(1);
    }
    
    let case_id = &args[2];
    let scheduled_at: u64 = args[3].parse().expect("Invalid scheduled time");
    let location = args[4].clone();
    let notes = args[5].clone();
    
    let hearing = Hearing::new(scheduled_at, location, notes);
    
    match tracker.schedule_hearing(case_id, hearing) {
        Ok(_) => {
            println!("Hearing scheduled successfully");
            if let Some(case) = tracker.get_case(case_id) {
                println!("Total hearings: {}", case.hearings.len());
                println!("Case status: {}", case.status.as_str());
            }
        }
        Err(e) => {
            eprintln!("Failed to schedule hearing: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_settle(tracker: &mut ADRTracker, args: &[String]) {
    if args.len() < 5 {
        eprintln!("Error: Insufficient arguments for settle command");
        print_usage();
        std::process::exit(1);
    }
    
    let case_id = &args[2];
    let amount: u64 = args[3].parse().expect("Invalid amount");
    let terms = args[4].clone();
    
    let settlement = Settlement::new(amount, terms);
    
    match tracker.set_settlement(case_id, settlement) {
        Ok(_) => {
            println!("Settlement recorded successfully");
            if let Some(case) = tracker.get_case(case_id) {
                println!("Case status: {}", case.status.as_str());
                if let Some(settlement) = &case.settlement {
                    println!("Settlement amount: ₹{}", settlement.amount);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to record settlement: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_compliance(tracker: &ADRTracker, args: &[String]) {
    if args.len() < 3 {
        eprintln!("Error: Case ID required");
        print_usage();
        std::process::exit(1);
    }
    
    let case_id = &args[2];
    
    let report = tracker.get_compliance_report(case_id);
    
    println!("{}", report);
}
