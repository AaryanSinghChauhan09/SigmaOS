// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/professional/sigma_healthcare.rs — Sigma Healthcare Tools
//
// Implements AI-assisted medical data analysis and drug interaction
// simulators for healthcare professionals and medical students.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Healthcare Types ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MedicalDataType {
    PatientRecords,
    LabResults,
    PrescriptionData,
    VitalSigns,
    ImagingData,
}

#[derive(Debug, Clone)]
pub struct PatientRecord {
    pub id: String,
    pub name: String,
    pub age: u32,
    pub gender: String,
    pub blood_type: String,
    pub allergies: Vec<String>,
    pub chronic_conditions: Vec<String>,
    pub current_medications: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LabResult {
    pub test_name: String,
    pub value: String,
    pub unit: String,
    pub reference_range: String,
    pub status: String,  // Normal, Abnormal, Critical
}

#[derive(Debug, Clone)]
pub struct DrugInteraction {
    pub drug1: String,
    pub drug2: String,
    pub severity: String,
    pub description: String,
    pub recommendation: String,
}

// ─── Healthcare Analytics Engine ─────────────────────────────────────────────

pub struct HealthcareAnalytics {
    pub patient_records: HashMap<String, PatientRecord>,
    pub drug_interactions: Vec<DrugInteraction>,
    pub lab_results: HashMap<String, Vec<LabResult>>,
}

impl HealthcareAnalytics {
    pub fn new() -> Self {
        let mut analytics = HealthcareAnalytics {
            patient_records: HashMap::new(),
            drug_interactions: Vec::new(),
            lab_results: HashMap::new(),
        };
        
        analytics.init_drug_interactions();
        analytics
    }

    /// Initialize common drug interactions
    fn init_drug_interactions(&mut self) {
        self.drug_interactions.push(DrugInteraction {
            drug1: "Warfarin".to_string(),
            drug2: "Aspirin".to_string(),
            severity: "High".to_string(),
            description: "Increased risk of bleeding due to additive antiplatelet effects".to_string(),
            recommendation: "Monitor INR closely, consider alternative antiplatelet therapy".to_string(),
        });

        self.drug_interactions.push(DrugInteraction {
            drug1: "ACE Inhibitors".to_string(),
            drug2: "Potassium Supplements".to_string(),
            severity: "High".to_string(),
            description: "Risk of hyperkalemia (elevated potassium levels)".to_string(),
            recommendation: "Monitor potassium levels regularly, avoid potassium supplements".to_string(),
        });

        self.drug_interactions.push(DrugInteraction {
            drug1: "Statins".to_string(),
            drug2: "Macrolide Antibiotics".to_string(),
            severity: "Medium".to_string(),
            description: "Increased risk of myopathy and rhabdomyolysis".to_string(),
            recommendation: "Monitor CK levels, consider temporary statin discontinuation".to_string(),
        });

        self.drug_interactions.push(DrugInteraction {
            drug1: "SSRIs".to_string(),
            drug2: "MAO Inhibitors".to_string(),
            severity: "Critical".to_string(),
            description: "Serotonin syndrome - potentially life-threatening".to_string(),
            recommendation: "Contraindicated - allow 2-week washout period between medications".to_string(),
        });
    }

    /// Add patient record
    pub fn add_patient(&mut self, patient: PatientRecord) {
        self.patient_records.insert(patient.id.clone(), patient);
    }

    /// Get patient by ID
    pub fn get_patient(&self, id: &str) -> Option<&PatientRecord> {
        self.patient_records.get(id)
    }

    /// Check for drug interactions
    pub fn check_drug_interactions(&self, medications: &[String]) -> Vec<&DrugInteraction> {
        let mut interactions = Vec::new();
        
        for interaction in &self.drug_interactions {
            let has_drug1 = medications.iter().any(|m| m.to_lowercase().contains(&interaction.drug1.to_lowercase()));
            let has_drug2 = medications.iter().any(|m| m.to_lowercase().contains(&interaction.drug2.to_lowercase()));
            
            if has_drug1 && has_drug2 {
                interactions.push(interaction);
            }
        }
        
        interactions
    }

    /// Add lab results for patient
    pub fn add_lab_results(&mut self, patient_id: String, results: Vec<LabResult>) {
        self.lab_results.insert(patient_id, results);
    }

    /// Get lab results for patient
    pub fn get_lab_results(&self, patient_id: &str) -> Option<&[LabResult]> {
        self.lab_results.get(patient_id).map(|v| v.as_slice())
    }

    /// Analyze lab results for abnormalities
    pub fn analyze_lab_results(&self, patient_id: &str) -> Vec<String> {
        let mut findings = Vec::new();
        
        if let Some(results) = self.get_lab_results(patient_id) {
            for result in results {
                if result.status != "Normal" {
                    findings.push(format!("{}: {} ({})", result.test_name, result.value, result.status));
                }
            }
        }
        
        findings
    }

    /// Calculate BMI
    pub fn calculate_bmi(&self, weight_kg: f64, height_cm: f64) -> (f64, String) {
        let height_m = height_cm / 100.0;
        let bmi = weight_kg / (height_m * height_m);
        
        let category = if bmi < 18.5 {
            "Underweight".to_string()
        } else if bmi < 25.0 {
            "Normal weight".to_string()
        } else if bmi < 30.0 {
            "Overweight".to_string()
        } else {
            "Obese".to_string()
        };
        
        (bmi, category)
    }

    /// Generate patient summary
    pub fn generate_patient_summary(&self, patient_id: &str) -> Option<String> {
        if let Some(patient) = self.get_patient(patient_id) {
            let summary = format!(
                "Patient Summary\n\
                Name: {}\n\
                Age: {}\n\
                Gender: {}\n\
                Blood Type: {}\n\
                Allergies: {}\n\
                Chronic Conditions: {}\n\
                Current Medications: {}",
                patient.name,
                patient.age,
                patient.gender,
                patient.blood_type,
                patient.allergies.join(", "),
                patient.chronic_conditions.join(", "),
                patient.current_medications.join(", ")
            );
            Some(summary)
        } else {
            None
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut analytics = HealthcareAnalytics::new();
    
    println!("Sigma Healthcare Analytics v0.1 - Medical Data Analysis");
    
    loop {
        println!("\nCommands: add_patient, patient <id>, interactions <drug1,drug2,...>, labs <id>, analyze <id>, bmi <weight> <height>, summary <id>, quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "add_patient" => {
                println!("Enter patient details:");
                print!("ID: ");
                std::io::stdout().flush().unwrap();
                let mut id = String::new();
                std::io::stdin().read_line(&mut id).unwrap();
                
                print!("Name: ");
                std::io::stdout().flush().unwrap();
                let mut name = String::new();
                std::io::stdin().read_line(&mut name).unwrap();
                
                print!("Age: ");
                std::io::stdout().flush().unwrap();
                let mut age_str = String::new();
                std::io::stdin().read_line(&mut age_str).unwrap();
                let age = age_str.trim().parse().unwrap_or(0);
                
                print!("Gender: ");
                std::io::stdout().flush().unwrap();
                let mut gender = String::new();
                std::io::stdin().read_line(&mut gender).unwrap();
                
                print!("Blood Type: ");
                std::io::stdout().flush().unwrap();
                let mut blood_type = String::new();
                std::io::stdin().read_line(&mut blood_type).unwrap();
                
                let patient = PatientRecord {
                    id: id.trim().to_string(),
                    name: name.trim().to_string(),
                    age,
                    gender: gender.trim().to_string(),
                    blood_type: blood_type.trim().to_string(),
                    allergies: Vec::new(),
                    chronic_conditions: Vec::new(),
                    current_medications: Vec::new(),
                };
                
                analytics.add_patient(patient);
                println!("Patient added");
            }
            "patient" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(patient) = analytics.get_patient(arg) {
                        println!("--- Patient Record ---");
                        println!("Name: {}", patient.name);
                        println!("Age: {}", patient.age);
                        println!("Gender: {}", patient.gender);
                        println!("Blood Type: {}", patient.blood_type);
                        println!("Allergies: {}", patient.allergies.join(", "));
                        println!("Chronic Conditions: {}", patient.chronic_conditions.join(", "));
                        println!("Current Medications: {}", patient.current_medications.join(", "));
                    }
                }
            }
            "interactions" => {
                if parts.len() >= 2 {
                    let medications: Vec<String> = parts[1].split(',').map(|s| s.trim().to_string()).collect();
                    let interactions = analytics.check_drug_interactions(&medications);
                    println!("--- Drug Interactions ---");
                    if interactions.is_empty() {
                        println!("No known interactions found");
                    } else {
                        for interaction in interactions {
                            println!("\n{} + {}", interaction.drug1, interaction.drug2);
                            println!("Severity: {}", interaction.severity);
                            println!("Description: {}", interaction.description);
                            println!("Recommendation: {}", interaction.recommendation);
                        }
                    }
                }
            }
            "labs" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(results) = analytics.get_lab_results(arg) {
                        println!("--- Lab Results ---");
                        for result in results {
                            println!("{}: {} {} ({}) - {}", result.test_name, result.value, result.unit, result.reference_range, result.status);
                        }
                    } else {
                        println!("No lab results found");
                    }
                }
            }
            "analyze" => {
                if let Some(arg) = parts.get(1) {
                    let findings = analytics.analyze_lab_results(arg);
                    println!("--- Lab Analysis ---");
                    if findings.is_empty() {
                        println!("All results are within normal range");
                    } else {
                        for finding in findings {
                            println!("- {}", finding);
                        }
                    }
                }
            }
            "bmi" => {
                if parts.len() >= 3 {
                    if let (Ok(weight), Ok(height)) = (parts[1].parse::<f64>(), parts[2].parse::<f64>()) {
                        let (bmi, category) = analytics.calculate_bmi(weight, height);
                        println!("BMI: {:.1}", bmi);
                        println!("Category: {}", category);
                    }
                }
            }
            "summary" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(summary) = analytics.generate_patient_summary(arg) {
                        println!("--- Patient Summary ---");
                        println!("{}", summary);
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
