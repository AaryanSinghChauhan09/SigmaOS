// SovereignDosageCalc CLI
// Command-line interface for medical dosage calculation system

use sovereign_dosage_calc::{DosageCalculator, DosageUnit, Duration, DurationUnit, DrugInfo, DrugSchedule, Dosage, Frequency, Medication, PatientParams, RenalFunction, LiverFunction, Prescription};
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }
    
    let calculator = DosageCalculator::new();
    
    match args[1].as_str() {
        "weight_dose" => handle_weight_dose(&calculator, &args),
        "age_dose" => handle_age_dose(&calculator, &args),
        "adjusted_dose" => handle_adjusted_dose(&calculator, &args),
        "prescribe" => handle_prescribe(&mut calculator, &args),
        "verify" => handle_verify(&calculator, &args),
        "interactions" => handle_interactions(&calculator, &args),
        "list" => handle_list(&calculator),
        _ => {
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!("SovereignDosageCalc CLI");
    println!();
    println!("Usage:");
    println!("  dosage_calc weight_dose <standard_dose> <weight> <dose_per_kg>");
    println!("  dosage_calc age_dose <adult_dose> <age>");
    println!("  dosage_calc adjusted_dose <base_dose> <weight> <age> <renal> <liver>");
    println!("  dosage_calc prescribe <patient_id> <doctor_id> <diagnosis> <drug_name> <generic> <brand> <amount> <unit> <frequency> <duration_value> <duration_unit> <schedule>");
    println!("  dosage_calc verify <drug_name> <amount> <unit> <frequency> <duration_value> <duration_unit> <schedule>");
    println!("  dosage_calc interactions <drug1> <drug2>");
    println!("  dosage_calc list");
    println!();
    println!("Example:");
    println!("  dosage_calc weight_dose 100 70 1.0");
    println!("  dosage_calc age_dose 100 10");
    println!("  dosage_calc adjusted_dose 100 70 25 normal normal");
    println!("  dosage_calc prescribe patient001 doctor001 \"Hypertension\" Paracetamol Acetaminophen Tylenol 500 mg twice 7 days None");
}

fn handle_weight_dose(calculator: &DosageCalculator, args: &[String]) {
    if args.len() < 5 {
        eprintln!("Error: Insufficient arguments for weight_dose command");
        print_usage();
        std::process::exit(1);
    }
    
    let standard_dose: f64 = args[2].parse().expect("Invalid standard dose");
    let weight: f64 = args[3].parse().expect("Invalid weight");
    let dose_per_kg: f64 = args[4].parse().expect("Invalid dose per kg");
    
    let dose = calculator.calculate_weight_based_dosage(standard_dose, weight, dose_per_kg);
    
    println!("Weight-based dosage: {:.2} mg", dose);
}

fn handle_age_dose(calculator: &DosageCalculator, args: &[String]) {
    if args.len() < 4 {
        eprintln!("Error: Insufficient arguments for age_dose command");
        print_usage();
        std::process::exit(1);
    }
    
    let adult_dose: f64 = args[2].parse().expect("Invalid adult dose");
    let age: u64 = args[3].parse().expect("Invalid age");
    
    let dose = calculator.calculate_age_based_dosage(adult_dose, age);
    
    println!("Age-based dosage: {:.2} mg", dose);
}

fn handle_adjusted_dose(calculator: &DosageCalculator, args: &[String]) {
    if args.len() < 7 {
        eprintln!("Error: Insufficient arguments for adjusted_dose command");
        print_usage();
        std::process::exit(1);
    }
    
    let base_dose: f64 = args[2].parse().expect("Invalid base dose");
    let weight: f64 = args[3].parse().expect("Invalid weight");
    let age: u64 = args[4].parse().expect("Invalid age");
    let renal_str = &args[5];
    let liver_str = &args[6];
    
    let renal_function = match renal_str.to_lowercase().as_str() {
        "normal" => RenalFunction::Normal,
        "mild" => RenalFunction::MildImpairment,
        "moderate" => RenalFunction::ModerateImpairment,
        "severe" => RenalFunction::SevereImpairment,
        _ => {
            eprintln!("Error: Invalid renal function. Use: normal, mild, moderate, severe");
            std::process::exit(1);
        }
    };
    
    let liver_function = match liver_str.to_lowercase().as_str() {
        "normal" => LiverFunction::Normal,
        "mild" => LiverFunction::MildImpairment,
        "moderate" => LiverFunction::ModerateImpairment,
        "severe" => LiverFunction::SevereImpairment,
        _ => {
            eprintln!("Error: Invalid liver function. Use: normal, mild, moderate, severe");
            std::process::exit(1);
        }
    };
    
    let patient_params = PatientParams::new(weight, age)
        .with_renal_function(renal_function)
        .with_liver_function(liver_function);
    
    let adjusted_dose = calculator.calculate_adjusted_dosage(base_dose, &patient_params);
    
    println!("Adjusted dosage: {:.2} mg", adjusted_dose);
    println!("Adjustment factor: {:.2}", patient_params.get_dose_adjustment_factor());
}

fn handle_prescribe(calculator: &mut DosageCalculator, args: &[String]) {
    if args.len() < 13 {
        eprintln!("Error: Insufficient arguments for prescribe command");
        print_usage();
        std::process::exit(1);
    }
    
    let patient_id_str = args[2].clone();
    let doctor_id_str = args[3].clone();
    let diagnosis = args[4].clone();
    let drug_name = args[5].clone();
    let generic = args[6].clone();
    let brand = args[7].clone();
    let amount: f64 = args[8].parse().expect("Invalid amount");
    let unit_str = &args[9];
    let frequency_str = &args[10];
    let duration_value: u64 = args[11].parse().expect("Invalid duration value");
    let duration_unit_str = &args[12];
    
    let unit = match unit_str.to_lowercase().as_str() {
        "mg" => DosageUnit::Mg,
        "g" => DosageUnit::G,
        "ml" => DosageUnit::Ml,
        "units" => DosageUnit::Units,
        "drops" => DosageUnit::Drops,
        _ => {
            eprintln!("Error: Invalid unit. Use: mg, g, ml, units, drops");
            std::process::exit(1);
        }
    };
    
    let frequency = match frequency_str.to_lowercase().as_str() {
        "once" => Frequency::Once,
        "twice" => Frequency::TwiceDaily,
        "thrice" => Frequency::ThreeTimesDaily,
        "four" => Frequency::FourTimesDaily,
        "6h" => Frequency::Every6Hours,
        "8h" => Frequency::Every8Hours,
        "12h" => Frequency::Every12Hours,
        "prn" => Frequency::AsNeeded,
        _ => {
            eprintln!("Error: Invalid frequency. Use: once, twice, thrice, four, 6h, 8h, 12h, prn");
            std::process::exit(1);
        }
    };
    
    let duration_unit = match duration_unit_str.to_lowercase().as_str() {
        "days" => DurationUnit::Days,
        "weeks" => DurationUnit::Weeks,
        "months" => DurationUnit::Months,
        _ => {
            eprintln!("Error: Invalid duration unit. Use: days, weeks, months");
            std::process::exit(1);
        }
    };
    
    let schedule = DrugSchedule::None; // Default to non-scheduled for CLI
    
    let dosage = Dosage::new(amount, unit);
    let duration = Duration::new(duration_value, duration_unit);
    
    let medication = Medication::new(
        "drug001".to_string(),
        brand,
        generic,
        dosage,
        frequency,
        duration,
        schedule,
    );
    
    let patient_id = hash_string(&patient_id_str);
    let doctor_id = hash_string(&doctor_id_str);
    
    let mut prescription = Prescription::new(patient_id, doctor_id, diagnosis);
    prescription.add_medication(medication);
    
    println!("Prescription created successfully!");
    println!("Prescription ID: {}", prescription.get_rx_id());
    println!();
    println!("{}", prescription);
}

fn handle_verify(calculator: &DosageCalculator, args: &[String]) {
    if args.len() < 8 {
        eprintln!("Error: Insufficient arguments for verify command");
        print_usage();
        std::process::exit(1);
    }
    
    let drug_name = args[2].clone();
    let amount: f64 = args[3].parse().expect("Invalid amount");
    let unit_str = &args[4];
    let frequency_str = &args[5];
    let duration_value: u64 = args[6].parse().expect("Invalid duration value");
    let duration_unit_str = &args[7];
    
    let unit = match unit_str.to_lowercase().as_str() {
        "mg" => DosageUnit::Mg,
        "g" => DosageUnit::G,
        "ml" => DosageUnit::Ml,
        "units" => DosageUnit::Units,
        "drops" => DosageUnit::Drops,
        _ => {
            eprintln!("Error: Invalid unit");
            std::process::exit(1);
        }
    };
    
    let frequency = match frequency_str.to_lowercase().as_str() {
        "once" => Frequency::Once,
        "twice" => Frequency::TwiceDaily,
        "thrice" => Frequency::ThreeTimesDaily,
        "four" => Frequency::FourTimesDaily,
        "6h" => Frequency::Every6Hours,
        "8h" => Frequency::Every8Hours,
        "12h" => Frequency::Every12Hours,
        "prn" => Frequency::AsNeeded,
        _ => {
            eprintln!("Error: Invalid frequency");
            std::process::exit(1);
        }
    };
    
    let duration_unit = match duration_unit_str.to_lowercase().as_str() {
        "days" => DurationUnit::Days,
        "weeks" => DurationUnit::Weeks,
        "months" => DurationUnit::Months,
        _ => {
            eprintln!("Error: Invalid duration unit");
            std::process::exit(1);
        }
    };
    
    let schedule = DrugSchedule::None;
    
    let dosage = Dosage::new(amount, unit);
    let duration = Duration::new(duration_value, duration_unit);
    
    let medication = Medication::new(
        "drug001".to_string(),
        drug_name,
        drug_name,
        dosage,
        frequency,
        duration,
        schedule,
    );
    
    match calculator.verify_dosage(&medication) {
        Ok(_) => println!("Dosage is within safe limits"),
        Err(e) => {
            eprintln!("Dosage verification failed: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_interactions(calculator: &DosageCalculator, args: &[String]) {
    if args.len() < 4 {
        eprintln!("Error: Two drug names required");
        print_usage();
        std::process::exit(1);
    }
    
    let drug1 = args[2].clone();
    let drug2 = args[3].clone();
    
    let med1 = Medication::new(
        "drug001".to_string(),
        drug1.clone(),
        drug1.clone(),
        Dosage::new(100.0, DosageUnit::Mg),
        Frequency::TwiceDaily,
        Duration::new(7, DurationUnit::Days),
        DrugSchedule::None,
    );
    
    let med2 = Medication::new(
        "drug002".to_string(),
        drug2.clone(),
        drug2.clone(),
        Dosage::new(100.0, DosageUnit::Mg),
        Frequency::TwiceDaily,
        Duration::new(7, DurationUnit::Days),
        DrugSchedule::None,
    );
    
    let interactions = calculator.check_interactions(&[med1, med2]);
    
    if interactions.is_empty() {
        println!("No known interactions detected");
    } else {
        println!("Potential interactions detected:");
        for interaction in interactions {
            println!("- {}: {}", interaction.severity.as_str(), interaction.description);
            println!("  Recommendation: {}", interaction.recommendation);
        }
    }
}

fn handle_list(calculator: &DosageCalculator) {
    let drugs = calculator.drug_database;
    
    if drugs.is_empty() {
        println!("No drugs in database");
        return;
    }
    
    println!("Drug Database ({}):", drugs.len());
    println!();
    
    for drug in drugs {
        println!("ID: {}", drug.drug_id);
        println!("Generic: {}", drug.generic_name);
        println!("Brands: {}", drug.brand_names.join(", "));
        println!("Schedule: {}", drug.schedule.as_str());
        println!();
    }
}

fn hash_string(s: &str) -> [u8; 32] {
    let mut hash = [0u8; 32];
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        hash[i % 32] = hash[i % 32].wrapping_add(byte);
    }
    hash
}
