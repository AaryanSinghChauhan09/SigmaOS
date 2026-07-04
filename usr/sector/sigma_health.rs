// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/sector/sigma_health.rs — Sigma Healthcare Records (OpenMRS)
//
// Implements OpenMRS-style healthcare records with patient management,
// medical records, appointments, prescriptions, and lab results.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Healthcare Types ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Patient {
    pub id: String,
    pub name: String,
    pub date_of_birth: String,
    pub gender: String,
    pub blood_type: String,
    pub phone: String,
    pub email: String,
    pub address: String,
    pub emergency_contact: String,
    pub allergies: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Encounter {
    pub id: String,
    pub patient_id: String,
    pub encounter_type: String,  // outpatient, inpatient, emergency
    pub date: String,
    pub provider: String,
    pub diagnosis: String,
    pub notes: String,
}

#[derive(Debug, Clone)]
pub struct Prescription {
    pub id: String,
    pub patient_id: String,
    pub medication: String,
    pub dosage: String,
    pub frequency: String,
    pub duration: String,
    pub prescribed_by: String,
    pub date: String,
}

#[derive(Debug, Clone)]
pub struct LabResult {
    pub id: String,
    pub patient_id: String,
    pub test_name: String,
    pub test_type: String,
    pub result: String,
    pub reference_range: String,
    pub date: String,
    pub performed_by: String,
}

#[derive(Debug, Clone)]
pub struct Appointment {
    pub id: String,
    pub patient_id: String,
    pub provider: String,
    pub date: String,
    pub time: String,
    pub reason: String,
    pub status: String,
}

// ─── Healthcare Manager ────────────────────────────────────────────────────

pub struct HealthcareManager {
    pub patients: HashMap<String, Patient>,
    pub encounters: Vec<Encounter>,
    pub prescriptions: HashMap<String, Prescription>,
    pub lab_results: Vec<LabResult>,
    pub appointments: Vec<Appointment>,
}

impl HealthcareManager {
    pub fn new() -> Self {
        let mut manager = HealthcareManager {
            patients: HashMap::new(),
            encounters: Vec::new(),
            prescriptions: HashMap::new(),
            lab_results: Vec::new(),
            appointments: Vec::new(),
        };
        
        manager.init_sample_patients();
        manager
    }

    /// Initialize sample patients
    fn init_sample_patients(&mut self) {
        self.patients.insert("patient_001".to_string(), Patient {
            id: "patient_001".to_string(),
            name: "Rajesh Kumar".to_string(),
            date_of_birth: "1985-05-15".to_string(),
            gender: "Male".to_string(),
            blood_type: "O+".to_string(),
            phone: "+91-9876543210".to_string(),
            email: "rajesh.kumar@example.com".to_string(),
            address: "New Delhi, India".to_string(),
            emergency_contact: "Sunita Kumar - +91-9876543211".to_string(),
            allergies: vec!["Penicillin".to_string()],
        });
    }

    /// Register patient
    pub fn register_patient(&mut self, patient: Patient) {
        self.patients.insert(patient.id.clone(), patient);
    }

    /// Create encounter
    pub fn create_encounter(&mut self, patient_id: &str, encounter_type: String, provider: String, diagnosis: String, notes: String) -> Result<Encounter, String> {
        if self.patients.contains_key(patient_id) {
            let encounter = Encounter {
                id: format!("encounter_{}", self.encounters.len()),
                patient_id: patient_id.to_string(),
                encounter_type,
                date: "now".to_string(),
                provider,
                diagnosis,
                notes,
            };
            
            self.encounters.push(encounter.clone());
            Ok(encounter)
        } else {
            Err("Patient not found".to_string())
        }
    }

    /// Create prescription
    pub fn create_prescription(&mut self, patient_id: &str, medication: String, dosage: String, frequency: String, duration: String, prescribed_by: String) -> Result<Prescription, String> {
        if self.patients.contains_key(patient_id) {
            let prescription = Prescription {
                id: format!("rx_{}", self.prescriptions.len()),
                patient_id: patient_id.to_string(),
                medication,
                dosage,
                frequency,
                duration,
                prescribed_by,
                date: "now".to_string(),
            };
            
            self.prescriptions.insert(prescription.id.clone(), prescription.clone());
            Ok(prescription)
        } else {
            Err("Patient not found".to_string())
        }
    }

    /// Add lab result
    pub fn add_lab_result(&mut self, patient_id: &str, test_name: String, test_type: String, result: String, reference_range: String, performed_by: String) -> Result<LabResult, String> {
        if self.patients.contains_key(patient_id) {
            let lab_result = LabResult {
                id: format!("lab_{}", self.lab_results.len()),
                patient_id: patient_id.to_string(),
                test_name,
                test_type,
                result,
                reference_range,
                date: "now".to_string(),
                performed_by,
            };
            
            self.lab_results.push(lab_result);
            Ok(lab_result)
        } else {
            Err("Patient not found".to_string())
        }
    }

    /// Schedule appointment
    pub fn schedule_appointment(&mut self, patient_id: &str, provider: String, date: String, time: String, reason: String) -> Result<Appointment, String> {
        if self.patients.contains_key(patient_id) {
            let appointment = Appointment {
                id: format!("appt_{}", self.appointments.len()),
                patient_id: patient_id.to_string(),
                provider,
                date,
                time,
                reason,
                status: "scheduled".to_string(),
            };
            
            self.appointments.push(appointment);
            Ok(appointment)
        } else {
            Err("Patient not found".to_string())
        }
    }

    /// Cancel appointment
    pub fn cancel_appointment(&mut self, appointment_id: &str) -> Result<(), String> {
        if let Some(appointment) = self.appointments.iter_mut().find(|a| a.id == appointment_id) {
            appointment.status = "cancelled".to_string();
            Ok(())
        } else {
            Err("Appointment not found".to_string())
        }
    }

    /// Get patient by ID
    pub fn get_patient(&self, id: &str) -> Option<&Patient> {
        self.patients.get(id)
    }

    /// Get all patients
    pub fn get_all_patients(&self) -> Vec<&Patient> {
        self.patients.values().collect()
    }

    /// Get encounters for patient
    pub fn get_patient_encounters(&self, patient_id: &str) -> Vec<&Encounter> {
        self.encounters.iter().filter(|e| e.patient_id == patient_id).collect()
    }

    /// Get prescriptions for patient
    pub fn get_patient_prescriptions(&self, patient_id: &str) -> Vec<&Prescription> {
        self.prescriptions.values().filter(|p| p.patient_id == patient_id).collect()
    }

    /// Get lab results for patient
    pub fn get_patient_lab_results(&self, patient_id: &str) -> Vec<&LabResult> {
        self.lab_results.iter().filter(|l| l.patient_id == patient_id).collect()
    }

    /// Get appointments for patient
    pub fn get_patient_appointments(&self, patient_id: &str) -> Vec<&Appointment> {
        self.appointments.iter().filter(|a| a.patient_id == patient_id).collect()
    }

    /// Search patients
    pub fn search_patients(&self, query: &str) -> Vec<&Patient> {
        self.patients.values()
            .filter(|p| {
                p.name.to_lowercase().contains(&query.to_lowercase()) ||
                p.phone.contains(query) ||
                p.email.to_lowercase().contains(&query.to_lowercase())
            })
            .collect()
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut manager = HealthcareManager::new();
    
    println!("Sigma Healthcare Records v0.1 - OpenMRS Style");
    
    loop {
        println!("\n--- Healthcare Status ---");
        println!("Patients: {}", manager.patients.len());
        println!("Encounters: {}", manager.encounters.len());
        println!("Prescriptions: {}", manager.prescriptions.len());
        println!("Lab Results: {}", manager.lab_results.len());
        println!("Appointments: {}", manager.appointments.len());
        
        println!("\nCommands: register_patient <name> <dob> <gender> <blood_type> <phone> <email>, encounter <patient_id> <type> <provider> <diagnosis> <notes>, prescribe <patient_id> <medication> <dosage> <frequency> <duration> <provider>, lab_result <patient_id> <test_name> <test_type> <result> <ref_range> <performed_by>, schedule <patient_id> <provider> <date> <time> <reason>, cancel <appt_id>, search <query>, patients, patient <id>, patient_encounters <id>, patient_prescriptions <id>, patient_labs <id>, patient_appointments <id>, quit");
        println!("Encounter types: outpatient, inpatient, emergency");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "register_patient" => {
                if parts.len() >= 7 {
                    let name = parts[1].to_string();
                    let dob = parts[2].to_string();
                    let gender = parts[3].to_string();
                    let blood_type = parts[4].to_string();
                    let phone = parts[5].to_string();
                    let email = parts[6].to_string();
                    let patient = Patient {
                        id: format!("patient_{}", manager.patients.len()),
                        name,
                        date_of_birth: dob,
                        gender,
                        blood_type,
                        phone,
                        email,
                        address: "".to_string(),
                        emergency_contact: "".to_string(),
                        allergies: Vec::new(),
                    };
                    manager.register_patient(patient);
                    println!("Patient registered");
                }
            }
            "encounter" => {
                if parts.len() >= 6 {
                    let patient_id = parts[1].to_string();
                    let encounter_type = parts[2].to_string();
                    let provider = parts[3].to_string();
                    let diagnosis = parts[4].to_string();
                    let notes = parts[5..].join(" ");
                    match manager.create_encounter(&patient_id, encounter_type, provider, diagnosis, notes) {
                        Ok(_) => println!("Encounter created"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "prescribe" => {
                if parts.len() >= 7 {
                    let patient_id = parts[1].to_string();
                    let medication = parts[2].to_string();
                    let dosage = parts[3].to_string();
                    let frequency = parts[4].to_string();
                    let duration = parts[5].to_string();
                    let provider = parts[6].to_string();
                    match manager.create_prescription(&patient_id, medication, dosage, frequency, duration, provider) {
                        Ok(_) => println!("Prescription created"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "lab_result" => {
                if parts.len() >= 7 {
                    let patient_id = parts[1].to_string();
                    let test_name = parts[2].to_string();
                    let test_type = parts[3].to_string();
                    let result = parts[4].to_string();
                    let ref_range = parts[5].to_string();
                    let performed_by = parts[6].to_string();
                    match manager.add_lab_result(&patient_id, test_name, test_type, result, ref_range, performed_by) {
                        Ok(_) => println!("Lab result added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "schedule" => {
                if parts.len() >= 6 {
                    let patient_id = parts[1].to_string();
                    let provider = parts[2].to_string();
                    let date = parts[3].to_string();
                    let time = parts[4].to_string();
                    let reason = parts[5..].join(" ");
                    match manager.schedule_appointment(&patient_id, provider, date, time, reason) {
                        Ok(_) => println!("Appointment scheduled"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "cancel" => {
                if let Some(arg) = parts.get(1) {
                    match manager.cancel_appointment(arg) {
                        Ok(_) => println!("Appointment cancelled"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "search" => {
                if let Some(arg) = parts.get(1) {
                    println!("--- Search Results ---");
                    for patient in manager.search_patients(arg) {
                        println!("{} - {} ({})", patient.name, patient.phone, patient.gender);
                    }
                }
            }
            "patients" => {
                println!("--- All Patients ---");
                for patient in manager.get_all_patients() {
                    println!("{} - {} ({})", patient.name, patient.date_of_birth, patient.gender);
                }
            }
            "patient" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(patient) = manager.get_patient(arg) {
                        println!("--- Patient Details ---");
                        println!("Name: {}", patient.name);
                        println!("DOB: {}", patient.date_of_birth);
                        println!("Gender: {}", patient.gender);
                        println!("Blood Type: {}", patient.blood_type);
                        println!("Phone: {}", patient.phone);
                        println!("Email: {}", patient.email);
                        println!("Allergies: {}", patient.allergies.join(", "));
                    }
                }
            }
            "patient_encounters" => {
                if let Some(arg) = parts.get(1) {
                    println!("--- Patient Encounters ---");
                    for encounter in manager.get_patient_encounters(arg) {
                        println!("{} - {} ({}) - {}", encounter.date, encounter.encounter_type, encounter.provider, encounter.diagnosis);
                    }
                }
            }
            "patient_prescriptions" => {
                if let Some(arg) = parts.get(1) {
                    println!("--- Patient Prescriptions ---");
                    for rx in manager.get_patient_prescriptions(arg) {
                        println!("{} - {} ({} {} {}) by {}", rx.medication, rx.dosage, rx.frequency, rx.duration, rx.prescribed_by);
                    }
                }
            }
            "patient_labs" => {
                if let Some(arg) = parts.get(1) {
                    println!("--- Patient Lab Results ---");
                    for lab in manager.get_patient_lab_results(arg) {
                        println!("{} - {} ({}): {} (Ref: {})", lab.date, lab.test_name, lab.test_type, lab.result, lab.reference_range);
                    }
                }
            }
            "patient_appointments" => {
                if let Some(arg) = parts.get(1) {
                    println!("--- Patient Appointments ---");
                    for appt in manager.get_patient_appointments(arg) {
                        println!("{} {} - {} with {} ({})", appt.date, appt.time, appt.reason, appt.provider, appt.status);
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
