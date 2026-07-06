// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/law/sigma_labour.rs — Sigma Labour Code Explorer
//
// Implements interactive modules for Indian Labour Law, OSH Code,
// and Social Security Code for law students and professionals.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Labour Code Types ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LabourCode {
    LabourCode2020,
    OSHCode2020,
    SocialSecurityCode2020,
    IndustrialRelationsCode2020,
    OccupationalSafetyCode2020,
}

#[derive(Debug, Clone)]
pub struct LabourSection {
    pub id: String,
    pub code: LabourCode,
    pub section_number: String,
    pub title: String,
    pub description: String,
    pub key_provisions: Vec<String>,
    pub penalties: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ComplianceChecklist {
    pub code: LabourCode,
    pub items: Vec<ComplianceItem>,
}

#[derive(Debug, Clone)]
pub struct ComplianceItem {
    pub id: String,
    pub requirement: String,
    pub applicable_to: Vec<String>,
    pub deadline: String,
}

// ─── Labour Code Explorer ─────────────────────────────────────────────────────

pub struct LabourCodeExplorer {
    pub sections: HashMap<String, LabourSection>,
    pub checklists: HashMap<String, ComplianceChecklist>,
    pub current_code: Option<LabourCode>,
}

impl LabourCodeExplorer {
    pub fn new() -> Self {
        let mut explorer = LabourCodeExplorer {
            sections: HashMap::new(),
            checklists: HashMap::new(),
            current_code: None,
        };
        
        explorer.init_labour_code_2020();
        explorer.init_osh_code_2020();
        explorer.init_social_security_code_2020();
        explorer
    }

    /// Initialize Labour Code 2020 sections
    fn init_labour_code_2020(&mut self) {
        self.sections.insert("lc_001".to_string(), LabourSection {
            id: "lc_001".to_string(),
            code: LabourCode::LabourCode2020,
            section_number: "Section 2".to_string(),
            title: "Definition of 'Wages'".to_string(),
            description: "Comprehensive definition of wages including all remuneration payable to an employed person".to_string(),
            key_provisions: vec![
                "Basic wages + dearness allowance + retaining allowance".to_string(),
                "Includes overtime, bonus, commission".to_string(),
                "Excludes house rent allowance, travel allowance".to_string(),
                "Minimum wage floor wage to be set by central government".to_string(),
            ],
            penalties: vec![
                "Fine up to ₹10,000 for violation".to_string(),
                "Imprisonment up to 3 months for repeated violations".to_string(),
            ],
        });

        self.sections.insert("lc_002".to_string(), LabourSection {
            id: "lc_002".to_string(),
            code: LabourCode::LabourCode2020,
            section_number: "Section 14".to_string(),
            title: "Hours of Work".to_string(),
            description: "Regulation of working hours and overtime provisions".to_string(),
            key_provisions: vec![
                "Maximum 8 hours per day, 48 hours per week".to_string(),
                "Overtime not to exceed 3 hours per day".to_string(),
                "Overtime wages at twice the ordinary rate".to_string(),
                "Weekly rest day mandatory".to_string(),
            ],
            penalties: vec![
                "Fine up to ₹10,000 per violation".to_string(),
            ],
        });

        self.sections.insert("lc_003".to_string(), LabourSection {
            id: "lc_003".to_string(),
            code: LabourCode::LabourCode2020,
            section_number: "Section 25".to_string(),
            title: "Leave Entitlement".to_string(),
            description: "Provisions for various types of leave for employees".to_string(),
            key_provisions: vec![
                "Earned leave: 1 day for every 20 days worked".to_string(),
                "Casual leave: 12 days per year".to_string(),
                "Sick leave: 12 days per year".to_string(),
                "Maternity benefit: 26 weeks for first two children".to_string(),
            ],
            penalties: vec![
                "Fine up to ₹20,000 for denying leave".to_string(),
            ],
        });
    }

    /// Initialize OSH Code 2020 sections
    fn init_osh_code_2020(&mut self) {
        self.sections.insert("osh_001".to_string(), LabourSection {
            id: "osh_001".to_string(),
            code: LabourCode::OSHCode2020,
            section_number: "Section 3".to_string(),
            title: "Duties of Employer".to_string(),
            description: "Mandatory duties of employers to ensure workplace safety".to_string(),
            key_provisions: vec![
                "Provide safe working environment".to_string(),
                "Maintain safety equipment and machinery".to_string(),
                "Provide training on safety procedures".to_string(),
                "Display safety information at workplace".to_string(),
            ],
            penalties: vec![
                "Fine up to ₹5,00,000 for first violation".to_string(),
                "Fine up to ₹10,00,000 for subsequent violations".to_string(),
                "Imprisonment up to 2 years".to_string(),
            ],
        });

        self.sections.insert("osh_002".to_string(), LabourSection {
            id: "osh_002".to_string(),
            code: LabourCode::OSHCode2020,
            section_number: "Section 21".to_string(),
            title: "Health and Safety Standards".to_string(),
            description: "Standards for workplace health, safety, and welfare".to_string(),
            key_provisions: vec![
                "Adequate ventilation and lighting".to_string(),
                "Safe drinking water and sanitation facilities".to_string(),
                "First aid facilities at workplace".to_string(),
                "Regular health checkups for workers".to_string(),
            ],
            penalties: vec![
                "Fine up to ₹3,00,000 for non-compliance".to_string(),
            ],
        });
    }

    /// Initialize Social Security Code 2020 sections
    fn init_social_security_code_2020(&mut self) {
        self.sections.insert("ss_001".to_string(), LabourSection {
            id: "ss_001".to_string(),
            code: LabourCode::SocialSecurityCode2020,
            section_number: "Section 2".to_string(),
            title: "Definition of 'Employee'".to_string(),
            description: "Definition of employee for social security coverage".to_string(),
            key_provisions: vec![
                "Includes all workers in organized sector".to_string(),
                "Covers gig workers and platform workers".to_string(),
                "Excludes government employees".to_string(),
                "Minimum 10 workers required for coverage".to_string(),
            ],
            penalties: vec![
                "Penalty for misclassification: 50% of contribution".to_string(),
            ],
        });

        self.sections.insert("ss_002".to_string(), LabourSection {
            id: "ss_002".to_string(),
            code: LabourCode::SocialSecurityCode2020,
            section_number: "Section 42".to_string(),
            title: "Provident Fund Contributions".to_string(),
            description: "Employer and employee contribution rates for PF".to_string(),
            key_provisions: vec![
                "Employee contribution: 12% of wages".to_string(),
                "Employer contribution: 12% of wages".to_string(),
                "Interest on accumulated balance".to_string(),
                "Withdrawal rules for various purposes".to_string(),
            ],
            penalties: vec![
                "Interest penalty for delayed payment".to_string(),
                "Imprisonment up to 3 years for willful default".to_string(),
            ],
        });

        self.sections.insert("ss_003".to_string(), LabourSection {
            id: "ss_003".to_string(),
            code: LabourCode::SocialSecurityCode2020,
            section_number: "Section 106".to_string(),
            title: "Gratuity".to_string(),
            description: "Gratuity payment provisions for employees".to_string(),
            key_provisions: vec![
                "Minimum 5 years of service required".to_string(),
                "15 days wages for each completed year".to_string(),
                "Maximum gratuity: ₹20 lakhs".to_string(),
                "Payment within 30 days of exit".to_string(),
            ],
            penalties: vec![
                "Interest for delayed payment".to_string(),
                "Fine up to ₹10,000 for non-payment".to_string(),
            ],
        });
    }

    /// Get section by ID
    pub fn get_section(&self, id: &str) -> Option<&LabourSection> {
        self.sections.get(id)
    }

    /// Get sections by code
    pub fn get_sections_by_code(&self, code: LabourCode) -> Vec<&LabourSection> {
        self.sections.values()
            .filter(|s| s.code == code)
            .collect()
    }

    /// Get all sections
    pub fn get_all_sections(&self) -> Vec<&LabourSection> {
        self.sections.values().collect()
    }

    /// Set current code
    pub fn set_current_code(&mut self, code: LabourCode) {
        self.current_code = Some(code);
    }

    /// Get current code
    pub fn get_current_code(&self) -> Option<LabourCode> {
        self.current_code
    }

    /// Search sections by keyword
    pub fn search(&self, keyword: &str) -> Vec<&LabourSection> {
        self.sections.values()
            .filter(|s| {
                s.title.to_lowercase().contains(&keyword.to_lowercase()) ||
                s.description.to_lowercase().contains(&keyword.to_lowercase()) ||
                s.key_provisions.iter().any(|p| p.to_lowercase().contains(&keyword.to_lowercase()))
            })
            .collect()
    }

    /// Get code name
    pub fn get_code_name(&self, code: LabourCode) -> &str {
        match code {
            LabourCode::LabourCode2020 => "Labour Code 2020",
            LabourCode::OSHCode2020 => "OSH Code 2020",
            LabourCode::SocialSecurityCode2020 => "Social Security Code 2020",
            LabourCode::IndustrialRelationsCode2020 => "Industrial Relations Code 2020",
            LabourCode::OccupationalSafetyCode2020 => "Occupational Safety Code 2020",
        }
    }

    /// Get compliance checklist for code
    pub fn get_compliance_checklist(&self, code: LabourCode) -> Vec<ComplianceItem> {
        // Simplified compliance checklist generation
        match code {
            LabourCode::LabourCode2020 => vec![
                ComplianceItem {
                    id: "lc_c1".to_string(),
                    requirement: "Display wage rates at workplace".to_string(),
                    applicable_to: vec!["All employers".to_string()],
                    deadline: "Immediate".to_string(),
                },
                ComplianceItem {
                    id: "lc_c2".to_string(),
                    requirement: "Maintain wage register".to_string(),
                    applicable_to: vec!["All employers".to_string()],
                    deadline: "Ongoing".to_string(),
                },
            ],
            LabourCode::OSHCode2020 => vec![
                ComplianceItem {
                    id: "osh_c1".to_string(),
                    requirement: "Conduct safety audit".to_string(),
                    applicable_to: vec!["Factories".to_string(), "Mines".to_string()],
                    deadline: "Annually".to_string(),
                },
                ComplianceItem {
                    id: "osh_c2".to_string(),
                    requirement: "Provide safety training".to_string(),
                    applicable_to: vec!["All employers".to_string()],
                    deadline: "Every 6 months".to_string(),
                },
            ],
            LabourCode::SocialSecurityCode2020 => vec![
                ComplianceItem {
                    id: "ss_c1".to_string(),
                    requirement: "Register with EPFO".to_string(),
                    applicable_to: vec!["Employers with 20+ workers".to_string()],
                    deadline: "Within 30 days of reaching threshold".to_string(),
                },
                ComplianceItem {
                    id: "ss_c2".to_string(),
                    requirement: "Deposit PF contributions".to_string(),
                    applicable_to: vec!["Registered employers".to_string()],
                    deadline: "By 15th of following month".to_string(),
                },
            ],
            _ => Vec::new(),
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut explorer = LabourCodeExplorer::new();
    
    println!("Sigma Labour Code Explorer v0.1 - Indian Labour Law");
    
    loop {
        println!("\n--- Available Codes ---");
        println!("1. Labour Code 2020");
        println!("2. OSH Code 2020");
        println!("3. Social Security Code 2020");
        println!("4. Industrial Relations Code 2020");
        println!("5. Occupational Safety Code 2020");
        
        if let Some(code) = explorer.get_current_code() {
            println!("\nCurrent: {}", explorer.get_code_name(code));
        }
        
        println!("\nCommands: code <number>, sections, section <id>, search <keyword>, checklist, quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "code" => {
                if let Some(arg) = parts.get(1) {
                    let code = match *arg {
                        "1" => LabourCode::LabourCode2020,
                        "2" => LabourCode::OSHCode2020,
                        "3" => LabourCode::SocialSecurityCode2020,
                        "4" => LabourCode::IndustrialRelationsCode2020,
                        "5" => LabourCode::OccupationalSafetyCode2020,
                        _ => {
                            println!("Invalid code number");
                            continue;
                        }
                    };
                    explorer.set_current_code(code);
                    println!("Switched to {}", explorer.get_code_name(code));
                }
            }
            "sections" => {
                if let Some(code) = explorer.get_current_code() {
                    println!("--- Sections in {} ---", explorer.get_code_name(code));
                    for section in explorer.get_sections_by_code(code) {
                        println!("{}: {}", section.section_number, section.title);
                    }
                } else {
                    println!("Select a code first");
                }
            }
            "section" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(section) = explorer.get_section(arg) {
                        println!("--- Section Details ---");
                        println!("{}: {}", section.section_number, section.title);
                        println!("Description: {}", section.description);
                        println!("\nKey Provisions:");
                        for (i, provision) in section.key_provisions.iter().enumerate() {
                            println!("{}. {}", i + 1, provision);
                        }
                        println!("\nPenalties:");
                        for penalty in &section.penalties {
                            println!("- {}", penalty);
                        }
                    }
                }
            }
            "search" => {
                if parts.len() >= 2 {
                    let keyword = parts[1..].join(" ");
                    let results = explorer.search(&keyword);
                    println!("--- Search Results for '{}' ---", keyword);
                    for section in results {
                        println!("{}: {} ({})", section.section_number, section.title, explorer.get_code_name(section.code));
                    }
                }
            }
            "checklist" => {
                if let Some(code) = explorer.get_current_code() {
                    println!("--- Compliance Checklist for {} ---", explorer.get_code_name(code));
                    for item in explorer.get_compliance_checklist(code) {
                        println!("\n[{}]", item.id);
                        println!("Requirement: {}", item.requirement);
                        println!("Applicable to: {}", item.applicable_to.join(", "));
                        println!("Deadline: {}", item.deadline);
                    }
                } else {
                    println!("Select a code first");
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
