// Security Audit System for SigmaOS
// Implements security-first approach inspired by Alpine Linux
// Provides CVE scanning, security auditing, and vulnerability management

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vulnerability {
    pub cve_id: String,
    pub severity: Severity,
    pub affected_packages: Vec<String>,
    pub description: String,
    pub fix_available: bool,
    pub fix_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuditReport {
    pub timestamp: i64,
    pub total_packages: usize,
    pub vulnerable_packages: usize,
    pub vulnerabilities: Vec<Vulnerability>,
    pub security_score: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecurityPolicy {
    pub allow_critical: bool,
    pub allow_high: bool,
    pub auto_update_critical: bool,
    pub require_signature_verification: bool,
    pub max_cvss_score: f64,
}

pub struct SecurityAuditor {
    vulnerability_db: HashMap<String, Vulnerability>,
    security_policy: SecurityPolicy,
}

impl SecurityAuditor {
    pub fn new() -> Result<Self, std::io::Error> {
        let vulnerability_db = Self::load_vulnerability_db()?;
        let security_policy = Self::load_security_policy()?;
        
        Ok(SecurityAuditor {
            vulnerability_db,
            security_policy,
        })
    }

    /// Perform a full security audit of installed packages
    pub fn audit_system(&self) -> Result<AuditReport, std::io::Error> {
        let installed_packages = self.get_installed_packages()?;
        let mut vulnerabilities = Vec::new();
        
        for package in &installed_packages {
            if let Some(vuln) = self.check_package_vulnerabilities(package) {
                vulnerabilities.extend(vuln);
            }
        }

        let security_score = self.calculate_security_score(&vulnerabilities, installed_packages.len());
        
        Ok(AuditReport {
            timestamp: chrono::Utc::now().timestamp(),
            total_packages: installed_packages.len(),
            vulnerable_packages: vulnerabilities.len(),
            vulnerabilities,
            security_score,
        })
    }

    /// Check if a specific package has vulnerabilities
    pub fn check_package_vulnerabilities(&self, package: &str) -> Option<Vec<Vulnerability>> {
        let mut package_vulns = Vec::new();
        
        for vuln in self.vulnerability_db.values() {
            if vuln.affected_packages.contains(&package.to_string()) {
                package_vulns.push(vuln.clone());
            }
        }
        
        if package_vulns.is_empty() {
            None
        } else {
            Some(package_vulns)
        }
    }

    /// Check if system meets security policy requirements
    pub fn check_policy_compliance(&self, report: &AuditReport) -> Result<bool, std::io::Error> {
        for vuln in &report.vulnerabilities {
            if vuln.severity == Severity::Critical && !self.security_policy.allow_critical {
                return Ok(false);
            }
            if vuln.severity == Severity::High && !self.security_policy.allow_high {
                return Ok(false);
            }
        }
        
        Ok(report.security_score >= self.security_policy.max_cvss_score)
    }

    /// Generate security recommendations
    pub fn generate_recommendations(&self, report: &AuditReport) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if report.security_score < 7.0 {
            recommendations.push("Security score is below threshold. Consider updating vulnerable packages.".to_string());
        }
        
        for vuln in &report.vulnerabilities {
            if vuln.severity == Severity::Critical || vuln.severity == Severity::High {
                if vuln.fix_available {
                    recommendations.push(format!(
                        "Update {} to {} to fix {} ({} severity)",
                        vuln.affected_packages.join(", "),
                        vuln.fix_version.as_deref().unwrap_or("latest"),
                        vuln.cve_id,
                        format!("{:?}", vuln.severity)
                    ));
                } else {
                    recommendations.push(format!(
                        "Monitor {} for fix of {} ({} severity - no fix available)",
                        vuln.affected_packages.join(", "),
                        vuln.cve_id,
                        format!("{:?}", vuln.severity)
                    ));
                }
            }
        }
        
        recommendations
    }

    /// Update vulnerability database from remote source
    pub fn update_vulnerability_db(&mut self) -> Result<(), std::io::Error> {
        // In a real implementation, this would fetch from a CVE database
        // For now, we'll simulate an update
        println!("Updating vulnerability database...");
        self.vulnerability_db = Self::load_vulnerability_db()?;
        Ok(())
    }

    fn calculate_security_score(&self, vulnerabilities: &[Vulnerability], total_packages: usize) -> f64 {
        if total_packages == 0 {
            return 10.0;
        }

        let mut score = 10.0;
        
        for vuln in vulnerabilities {
            match vuln.severity {
                Severity::Critical => score -= 2.5,
                Severity::High => score -= 1.5,
                Severity::Medium => score -= 0.5,
                Severity::Low => score -= 0.1,
                Severity::Info => score -= 0.05,
            }
        }
        
        score.max(0.0)
    }

    fn get_installed_packages(&self) -> Result<Vec<String>, std::io::Error> {
        // In a real implementation, this would query the package manager
        // For now, return a sample list
        Ok(vec![
            "sigma-coreutils".to_string(),
            "sigma-shell".to_string(),
            "sigma-network".to_string(),
            "sigma-storage".to_string(),
            "sigma-security".to_string(),
        ])
    }

    fn load_vulnerability_db() -> Result<HashMap<String, Vulnerability>, std::io::Error> {
        let db_path = Path::new("/var/lib/sigmaos/security/vulnerabilities.db");
        
        if db_path.exists() {
            let content = fs::read_to_string(db_path)?;
            let db: HashMap<String, Vulnerability> = serde_json::from_str(&content)
                .unwrap_or_default();
            Ok(db)
        } else {
            // Return empty database if file doesn't exist
            Ok(HashMap::new())
        }
    }

    fn load_security_policy() -> Result<SecurityPolicy, std::io::Error> {
        let policy_path = Path::new("/etc/sigmaos/security/policy.toml");
        
        if policy_path.exists() {
            let content = fs::read_to_string(policy_path)?;
            let policy: SecurityPolicy = toml::from_str(&content)
                .unwrap_or_else(|_| SecurityPolicy::default());
            Ok(policy)
        } else {
            Ok(SecurityPolicy::default())
        }
    }
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        SecurityPolicy {
            allow_critical: false,
            allow_high: true,
            auto_update_critical: true,
            require_signature_verification: true,
            max_cvss_score: 7.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_score_calculation() {
        let auditor = SecurityAuditor::new().unwrap();
        let vulnerabilities = vec![
            Vulnerability {
                cve_id: "CVE-2024-0001".to_string(),
                severity: Severity::Critical,
                affected_packages: vec!["test-package".to_string()],
                description: "Test vulnerability".to_string(),
                fix_available: true,
                fix_version: Some("1.0.1".to_string()),
            }
        ];
        
        let score = auditor.calculate_security_score(&vulnerabilities, 10);
        assert!(score < 10.0);
        assert!(score >= 0.0);
    }

    #[test]
    fn test_policy_compliance() {
        let auditor = SecurityAuditor::new().unwrap();
        let report = AuditReport {
            timestamp: 0,
            total_packages: 10,
            vulnerable_packages: 0,
            vulnerabilities: vec![],
            security_score: 10.0,
        };
        
        assert!(auditor.check_policy_compliance(&report).unwrap());
    }
}
