// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/security/sigma_policy.rs — Sigma Security Policy Advisor
//
// Implements AI-driven security policy advisor that suggests best practices
// for securing student projects, systems, and educational environments.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Policy Types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PolicyDomain {
    Network,
    Application,
    Data,
    Access,
    Physical,
}

#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub id: String,
    pub domain: PolicyDomain,
    pub title: String,
    pub description: String,
    pub severity: String,
    pub recommendations: Vec<String>,
    pub compliance_standards: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SecurityAssessment {
    pub project_id: String,
    pub project_name: String,
    pub domain: PolicyDomain,
    pub score: u32,
    pub findings: Vec<String>,
    pub recommendations: Vec<String>,
}

// ─── Security Policy Advisor ─────────────────────────────────────────────────

pub struct PolicyAdvisor {
    pub policies: HashMap<String, SecurityPolicy>,
    pub assessments: Vec<SecurityAssessment>,
}

impl PolicyAdvisor {
    pub fn new() -> Self {
        let mut advisor = PolicyAdvisor {
            policies: HashMap::new(),
            assessments: Vec::new(),
        };
        
        advisor.init_policies();
        advisor
    }

    /// Initialize security policies
    fn init_policies(&mut self) {
        // Network Security Policies
        self.policies.insert("net_001".to_string(), SecurityPolicy {
            id: "net_001".to_string(),
            domain: PolicyDomain::Network,
            title: "Firewall Configuration".to_string(),
            description: "Proper firewall configuration to protect network perimeter".to_string(),
            severity: "High".to_string(),
            recommendations: vec![
                "Block all inbound traffic by default".to_string(),
                "Only allow necessary outbound traffic".to_string(),
                "Implement stateful packet inspection".to_string(),
                "Regularly review and update firewall rules".to_string(),
                "Log all blocked and allowed traffic".to_string(),
            ],
            compliance_standards: vec!["ISO 27001".to_string(), "NIST CSF".to_string()],
        });

        self.policies.insert("net_002".to_string(), SecurityPolicy {
            id: "net_002".to_string(),
            domain: PolicyDomain::Network,
            title: "Wireless Security".to_string(),
            description: "Secure wireless network configuration".to_string(),
            severity: "Medium".to_string(),
            recommendations: vec![
                "Use WPA3 or WPA2-AES encryption".to_string(),
                "Disable WPS (Wi-Fi Protected Setup)".to_string(),
                "Implement separate guest network".to_string(),
                "Change default SSID and passwords".to_string(),
                "Enable MAC address filtering".to_string(),
            ],
            compliance_standards: vec!["ISO 27001".to_string()],
        });

        // Application Security Policies
        self.policies.insert("app_001".to_string(), SecurityPolicy {
            id: "app_001".to_string(),
            domain: PolicyDomain::Application,
            title: "Input Validation".to_string(),
            description: "Validate all user inputs to prevent injection attacks".to_string(),
            severity: "Critical".to_string(),
            recommendations: vec![
                "Sanitize all user inputs".to_string(),
                "Use parameterized queries for database access".to_string(),
                "Implement output encoding".to_string(),
                "Validate file uploads (type, size, content)".to_string(),
                "Use prepared statements for SQL".to_string(),
            ],
            compliance_standards: vec!["OWASP Top 10".to_string(), "PCI DSS".to_string()],
        });

        self.policies.insert("app_002".to_string(), SecurityPolicy {
            id: "app_002".to_string(),
            domain: PolicyDomain::Application,
            title: "Authentication & Authorization".to_string(),
            description: "Implement strong authentication and proper authorization".to_string(),
            severity: "High".to_string(),
            recommendations: vec![
                "Enforce strong password policies".to_string(),
                "Implement multi-factor authentication".to_string(),
                "Use session timeout".to_string(),
                "Implement role-based access control".to_string(),
                "Log all authentication attempts".to_string(),
            ],
            compliance_standards: vec!["OWASP Top 10".to_string(), "GDPR".to_string()],
        });

        // Data Security Policies
        self.policies.insert("data_001".to_string(), SecurityPolicy {
            id: "data_001".to_string(),
            domain: PolicyDomain::Data,
            title: "Data Encryption".to_string(),
            description: "Encrypt sensitive data at rest and in transit".to_string(),
            severity: "Critical".to_string(),
            recommendations: vec![
                "Use AES-256 for data at rest".to_string(),
                "Use TLS 1.3 for data in transit".to_string(),
                "Encrypt database backups".to_string(),
                "Implement key rotation policy".to_string(),
                "Use hardware security modules (HSM)".to_string(),
            ],
            compliance_standards: vec!["GDPR".to_string(), "PCI DSS".to_string(), "ISO 27001".to_string()],
        });

        self.policies.insert("data_002".to_string(), SecurityPolicy {
            id: "data_002".to_string(),
            domain: PolicyDomain::Data,
            title: "Data Backup & Recovery".to_string(),
            description: "Implement regular backup and disaster recovery procedures".to_string(),
            severity: "High".to_string(),
            recommendations: vec![
                "Perform daily automated backups".to_string(),
                "Store backups in secure offsite location".to_string(),
                "Test backup restoration regularly".to_string(),
                "Implement backup encryption".to_string(),
                "Maintain backup retention policy".to_string(),
            ],
            compliance_standards: vec!["ISO 27001".to_string(), "NIST CSF".to_string()],
        });

        // Access Control Policies
        self.policies.insert("access_001".to_string(), SecurityPolicy {
            id: "access_001".to_string(),
            domain: PolicyDomain::Access,
            title: "Least Privilege Principle".to_string(),
            description: "Grant minimum necessary access to users and systems".to_string(),
            severity: "High".to_string(),
            recommendations: vec![
                "Implement role-based access control (RBAC)".to_string(),
                "Regularly review user access rights".to_string(),
                "Revoke access immediately upon termination".to_string(),
                "Use temporary elevated privileges when needed".to_string(),
                "Document all access grants and revocations".to_string(),
            ],
            compliance_standards: vec!["ISO 27001".to_string(), "NIST CSF".to_string()],
        });

        // Physical Security Policies
        self.policies.insert("phys_001".to_string(), SecurityPolicy {
            id: "phys_001".to_string(),
            domain: PolicyDomain::Physical,
            title: "Server Room Security".to_string(),
            description: "Secure physical access to server rooms and equipment".to_string(),
            severity: "Medium".to_string(),
            recommendations: vec![
                "Implement access control systems".to_string(),
                "Use CCTV monitoring".to_string(),
                "Maintain visitor logs".to_string(),
                "Implement environmental controls (temperature, humidity)".to_string(),
                "Use UPS and backup power".to_string(),
            ],
            compliance_standards: vec!["ISO 27001".to_string()],
        });
    }

    /// Get policy by ID
    pub fn get_policy(&self, id: &str) -> Option<&SecurityPolicy> {
        self.policies.get(id)
    }

    /// Get policies by domain
    pub fn get_policies_by_domain(&self, domain: PolicyDomain) -> Vec<&SecurityPolicy> {
        self.policies.values()
            .filter(|p| p.domain == domain)
            .collect()
    }

    /// Get all policies
    pub fn get_all_policies(&self) -> Vec<&SecurityPolicy> {
        self.policies.values().collect()
    }

    /// Assess project security
    pub fn assess_project(&mut self, project_id: String, project_name: String, domain: PolicyDomain, answers: Vec<bool>) -> SecurityAssessment {
        let policies = self.get_policies_by_domain(domain);
        let mut findings = Vec::new();
        let mut recommendations = Vec::new();
        
        let mut score = 0u32;
        let total = policies.len() as u32;
        
        for (i, policy) in policies.iter().enumerate() {
            if i < answers.len() {
                if answers[i] {
                    score += 1;
                } else {
                    findings.push(format!("{} - Not compliant", policy.title));
                    recommendations.extend(policy.recommendations.clone());
                }
            } else {
                findings.push(format!("{} - Not assessed", policy.title));
                recommendations.extend(policy.recommendations.clone());
            }
        }
        
        let final_score = if total > 0 { (score * 100) / total } else { 0 };
        
        let assessment = SecurityAssessment {
            project_id: project_id.clone(),
            project_name,
            domain,
            score: final_score,
            findings,
            recommendations,
        };
        
        self.assessments.push(assessment.clone());
        assessment
    }

    /// Get AI suggestions for project security
    pub fn get_ai_suggestions(&self, domain: PolicyDomain) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        match domain {
            PolicyDomain::Network => {
                suggestions.push("Implement network segmentation to isolate critical systems".to_string());
                suggestions.push("Use intrusion detection/prevention systems (IDS/IPS)".to_string());
                suggestions.push("Implement VPN for remote access".to_string());
                suggestions.push("Regularly scan for vulnerabilities".to_string());
            }
            PolicyDomain::Application => {
                suggestions.push("Implement secure coding practices (OWASP guidelines)".to_string());
                suggestions.push("Perform regular security testing (SAST/DAST)".to_string());
                suggestions.push("Use dependency scanning for third-party libraries".to_string());
                suggestions.push("Implement security logging and monitoring".to_string());
            }
            PolicyDomain::Data => {
                suggestions.push("Implement data classification and labeling".to_string());
                suggestions.push("Use data loss prevention (DLP) solutions".to_string());
                suggestions.push("Implement privacy by design principles".to_string());
                suggestions.push("Regularly audit data access logs".to_string());
            }
            PolicyDomain::Access => {
                suggestions.push("Implement single sign-on (SSO) where appropriate".to_string());
                suggestions.push("Use privileged access management (PAM)".to_string());
                suggestions.push("Implement just-in-time access".to_string());
                suggestions.push("Regular security awareness training".to_string());
            }
            PolicyDomain::Physical => {
                suggestions.push("Implement badge-based access control".to_string());
                suggestions.push("Use biometric authentication for sensitive areas".to_string());
                suggestions.push("Regular security patrols and monitoring".to_string());
                suggestions.push("Emergency response procedures".to_string());
            }
        }
        
        suggestions
    }

    /// Get assessment history
    pub fn get_assessment_history(&self) -> &[SecurityAssessment] {
        &self.assessments
    }

    /// Get compliance checklist
    pub fn get_compliance_checklist(&self, standard: &str) -> Vec<&SecurityPolicy> {
        self.policies.values()
            .filter(|p| p.compliance_standards.iter().any(|s| s == standard))
            .collect()
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut advisor = PolicyAdvisor::new();
    
    println!("Sigma Security Policy Advisor v0.1 - AI-Driven Security Best Practices");
    
    loop {
        println!("\nCommands: list, domain <type>, policy <id>, assess <id> <name> <domain>, suggest <domain>, compliance <standard>, history, quit");
        println!("Domains: network, application, data, access, physical");
        println!("Standards: ISO 27001, NIST CSF, OWASP Top 10, PCI DSS, GDPR");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "list" => {
                println!("--- All Security Policies ---");
                for policy in advisor.get_all_policies() {
                    let domain_str = match policy.domain {
                        PolicyDomain::Network => "NET",
                        PolicyDomain::Application => "APP",
                        PolicyDomain::Data => "DATA",
                        PolicyDomain::Access => "ACCESS",
                        PolicyDomain::Physical => "PHYS",
                    };
                    println!("[{}] {} - {} ({})", domain_str, policy.id, policy.title, policy.severity);
                }
            }
            "domain" => {
                if let Some(arg) = parts.get(1) {
                    let domain = match *arg {
                        "network" => PolicyDomain::Network,
                        "application" => PolicyDomain::Application,
                        "data" => PolicyDomain::Data,
                        "access" => PolicyDomain::Access,
                        "physical" => PolicyDomain::Physical,
                        _ => {
                            println!("Unknown domain");
                            continue;
                        }
                    };
                    println!("--- {} Policies ---", arg);
                    for policy in advisor.get_policies_by_domain(domain) {
                        println!("{} - {} ({})", policy.id, policy.title, policy.severity);
                    }
                }
            }
            "policy" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(policy) = advisor.get_policy(arg) {
                        println!("--- Policy Details ---");
                        println!("Title: {}", policy.title);
                        println!("Description: {}", policy.description);
                        println!("Severity: {}", policy.severity);
                        println!("Compliance: {}", policy.compliance_standards.join(", "));
                        println!("\nRecommendations:");
                        for (i, rec) in policy.recommendations.iter().enumerate() {
                            println!("{}. {}", i + 1, rec);
                        }
                    }
                }
            }
            "assess" => {
                if parts.len() >= 4 {
                    let project_id = parts[1].to_string();
                    let project_name = parts[2].to_string();
                    let domain = match parts[3] {
                        "network" => PolicyDomain::Network,
                        "application" => PolicyDomain::Application,
                        "data" => PolicyDomain::Data,
                        "access" => PolicyDomain::Access,
                        "physical" => PolicyDomain::Physical,
                        _ => {
                            println!("Unknown domain");
                            continue;
                        }
                    };
                    
                    let policies = advisor.get_policies_by_domain(domain);
                    let mut answers = Vec::new();
                    
                    println!("Answer yes (1) or no (0) for each policy:");
                    for policy in policies {
                        print!("{} compliant? ", policy.title);
                        std::io::stdout().flush().unwrap();
                        let mut answer = String::new();
                        std::io::stdin().read_line(&mut answer).unwrap();
                        answers.push(answer.trim() == "1");
                    }
                    
                    let assessment = advisor.assess_project(project_id, project_name, domain, answers);
                    println!("\n--- Assessment Results ---");
                    println!("Security Score: {}/100", assessment.score);
                    println!("\nFindings:");
                    for finding in &assessment.findings {
                        println!("- {}", finding);
                    }
                    println!("\nRecommendations:");
                    for rec in &assessment.recommendations {
                        println!("- {}", rec);
                    }
                }
            }
            "suggest" => {
                if let Some(arg) = parts.get(1) {
                    let domain = match *arg {
                        "network" => PolicyDomain::Network,
                        "application" => PolicyDomain::Application,
                        "data" => PolicyDomain::Data,
                        "access" => PolicyDomain::Access,
                        "physical" => PolicyDomain::Physical,
                        _ => {
                            println!("Unknown domain");
                            continue;
                        }
                    };
                    println!("--- AI Suggestions ---");
                    for suggestion in advisor.get_ai_suggestions(domain) {
                        println!("- {}", suggestion);
                    }
                }
            }
            "compliance" => {
                if let Some(arg) = parts.get(1) {
                    println!("--- {} Compliance Checklist ---", arg);
                    for policy in advisor.get_compliance_checklist(arg) {
                        println!("- {} ({})", policy.title, policy.severity);
                    }
                }
            }
            "history" => {
                println!("--- Assessment History ---");
                for assessment in advisor.get_assessment_history() {
                    println!("{} - {}: {}/100", assessment.project_id, assessment.project_name, assessment.score);
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
