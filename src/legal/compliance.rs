use std::string::{String, ToString};
use std::vec::Vec;
use std::format;
// SigmaOS Unified Statutory Compliance & Labour Integration Engine
// Natively tracks global standards (GDPR, ISO-27001, SOC-2) and automates national statutory calculations (EPF, ESI, Payroll Auditing)

use crate::klib::HashMap;

/// Global security and privacy standards tracked by SigmaOS
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlobalStandard {
    Gdpr,
    Iso27001,
    Soc2,
    Hipaa,
    Cppa, // California Consumer Privacy Act
}

/// Compliance status for a particular requirement or control
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplianceStatus {
    Compliant,
    PartiallyCompliant,
    NonCompliant,
    NotApplicable,
}

/// A specific regulatory compliance control
#[derive(Debug, Clone)]
pub struct RegulatoryControl {
    pub id: String,
    pub description: String,
    pub standard: GlobalStandard,
    pub status: ComplianceStatus,
    pub last_audited_timestamp: u64,
}

/// International Compliance Tracker
pub struct InternationalComplianceTracker {
    pub controls: HashMap<String, RegulatoryControl>,
}

impl InternationalComplianceTracker {
    pub fn new() -> Self {
        Self {
            controls: HashMap::new(),
        }
    }

    pub fn register_control(&mut self, id: String, description: String, standard: GlobalStandard) {
        let control = RegulatoryControl {
            id: id.clone(),
            description,
            standard,
            status: ComplianceStatus::NonCompliant,
            last_audited_timestamp: 0,
        };
        self.controls.insert(id, control);
    }

    pub fn audit_control(&mut self, id: &str, status: ComplianceStatus, timestamp: u64) -> bool {
        if let Some(control) = self.controls.get_mut(id) {
            control.status = status;
            control.last_audited_timestamp = timestamp;
            true
        } else {
            false
        }
    }

    pub fn get_compliance_score(&self, standard: GlobalStandard) -> f64 {
        let standard_controls: Vec<&RegulatoryControl> = self
            .controls
            .values()
            .filter(|c| c.standard == standard)
            .collect();

        if standard_controls.is_empty() {
            return 100.0;
        }

        let total = standard_controls.len() as f64;
        let mut compliant_count = 0.0;

        for control in standard_controls {
            match control.status {
                ComplianceStatus::Compliant => compliant_count += 1.0,
                ComplianceStatus::PartiallyCompliant => compliant_count += 0.5,
                _ => {}
            }
        }

        (compliant_count / total) * 100.0
    }
}

// =========================================================================
// LABOUR LAW & SOCIAL SECURITY COMPLIANCE ENGINE (e.g. EPF, ESI calculations)
// =========================================================================

/// Statutory limits and configurations for EPF and ESI under Indian Labour Laws
pub struct LabourLawConfig {
    pub epf_wage_ceiling: f64,  // e.g. ₹15,000 per month
    pub epf_employee_rate: f64, // 12% (0.12)
    pub epf_employer_rate: f64, // 12% (0.12)
    pub esi_wage_ceiling: f64,  // e.g. ₹21,000 per month
    pub esi_employee_rate: f64, // 0.75% (0.0075)
    pub esi_employer_rate: f64, // 3.25% (0.0325)
}

impl Default for LabourLawConfig {
    fn default() -> Self {
        Self {
            epf_wage_ceiling: 15000.0,
            epf_employee_rate: 0.12,
            epf_employer_rate: 0.12,
            esi_wage_ceiling: 21000.0,
            esi_employee_rate: 0.0075,
            esi_employer_rate: 0.0325,
        }
    }
}

/// Calculated statutory payroll breakdown for an individual employee
#[derive(Debug, Clone)]
pub struct StatutoryPayrollBreakdown {
    pub employee_name: String,
    pub gross_salary: f64,
    pub epf_employee_deduction: f64,
    pub epf_employer_contribution: f64,
    pub esi_employee_deduction: f64,
    pub esi_employer_contribution: f64,
    pub net_take_home: f64,
}

pub struct LabourLawCompliance {
    pub config: LabourLawConfig,
}

impl LabourLawCompliance {
    pub fn new(config: LabourLawConfig) -> Self {
        Self { config }
    }

    /// Compute legal statutory contributions based on the employee's gross base salary
    pub fn calculate_payroll(&self, name: String, gross_salary: f64) -> StatutoryPayrollBreakdown {
        // EPF Calculation: computed on basic salary, capped at ₹15,000 (unless voluntarily higher)
        let epf_basis = if gross_salary > self.config.epf_wage_ceiling {
            self.config.epf_wage_ceiling
        } else {
            gross_salary
        };

        let epf_employee_deduction = epf_basis * self.config.epf_employee_rate;
        let epf_employer_contribution = epf_basis * self.config.epf_employer_rate;

        // ESI Calculation: eligibility check. If gross salary <= ₹21,000, ESI is deducted.
        let (esi_employee_deduction, esi_employer_contribution) =
            if gross_salary <= self.config.esi_wage_ceiling {
                (
                    gross_salary * self.config.esi_employee_rate,
                    gross_salary * self.config.esi_employer_rate,
                )
            } else {
                (0.0, 0.0)
            };

        let net_take_home = gross_salary - epf_employee_deduction - esi_employee_deduction;

        StatutoryPayrollBreakdown {
            employee_name: name,
            gross_salary,
            epf_employee_deduction,
            epf_employer_contribution,
            esi_employee_deduction,
            esi_employer_contribution,
            net_take_home,
        }
    }
}

// =========================================================================
// STATUTORY FILING DASHBOARD
// =========================================================================

/// Represents a critical statutory filing task (e.g., MCA Return, TDS Return, EPF ECR)
#[derive(Debug, Clone)]
pub struct StatutoryFiling {
    pub form_name: String,
    pub jurisdiction: String, // e.g., "MCA", "IncomeTax", "EPFO"
    pub due_date: String,
    pub status: String, // e.g., "Pending", "Drafted", "Filed", "Overdue"
    pub priority: u32,  // 1 = Critical, 3 = Low
}

pub struct StatutoryFilingDashboard {
    pub filings: Vec<StatutoryFiling>,
}

impl StatutoryFilingDashboard {
    pub fn new() -> Self {
        Self {
            filings: Vec::new(),
        }
    }

    pub fn register_filing(
        &mut self,
        form: String,
        jurisdiction: String,
        due_date: String,
        priority: u32,
    ) {
        self.filings.push(StatutoryFiling {
            form_name: form,
            jurisdiction,
            due_date,
            status: "Pending".to_string(),
            priority,
        });
    }

    pub fn update_filing_status(&mut self, form_name: &str, status: &str) -> bool {
        if let Some(filing) = self.filings.iter_mut().find(|f| f.form_name == form_name) {
            filing.status = status.to_string();
            true
        } else {
            false
        }
    }

    /// Computes an aggregated filing compliance score (0.0 to 100.0) weighted by priority
    pub fn get_compliance_score(&self) -> f64 {
        if self.filings.is_empty() {
            return 100.0;
        }

        let mut total_weight = 0.0;
        let mut completed_weight = 0.0;

        for filing in &self.filings {
            let weight = match filing.priority {
                1 => 3.0, // Critical
                2 => 2.0, // Medium
                _ => 1.0, // Low
            };

            total_weight += weight;

            if filing.status == "Filed" {
                completed_weight += weight;
            } else if filing.status == "Drafted" {
                completed_weight += weight * 0.5; // partial credit
            }
        }

        (completed_weight / total_weight) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_compliance_tracker() {
        let mut tracker = InternationalComplianceTracker::new();
        tracker.register_control(
            "GDPR-01".to_string(),
            "Right to erasure implementation".to_string(),
            GlobalStandard::Gdpr,
        );
        tracker.register_control(
            "GDPR-02".to_string(),
            "Consent logging".to_string(),
            GlobalStandard::Gdpr,
        );

        assert_eq!(tracker.get_compliance_score(GlobalStandard::Gdpr), 0.0);

        tracker.audit_control("GDPR-01", ComplianceStatus::Compliant, 1700000000);
        assert_eq!(tracker.get_compliance_score(GlobalStandard::Gdpr), 50.0);

        tracker.audit_control("GDPR-02", ComplianceStatus::PartiallyCompliant, 1700000000);
        assert_eq!(tracker.get_compliance_score(GlobalStandard::Gdpr), 75.0);
    }

    #[test]
    fn test_labour_compliance_engine() {
        let engine = LabourLawCompliance::new(LabourLawConfig::default());

        // Employee below ceilings (Pooja earns ₹12,000)
        let pooja = engine.calculate_payroll("Pooja".to_string(), 12000.0);
        assert_eq!(pooja.epf_employee_deduction, 1440.0); // 12% of 12k
        assert_eq!(pooja.esi_employee_deduction, 90.0); // 0.75% of 12k
        assert_eq!(pooja.net_take_home, 12000.0 - 1440.0 - 90.0);

        // Employee above ceilings (Aditya earns ₹30,000)
        let aditya = engine.calculate_payroll("Aditya".to_string(), 30000.0);
        assert_eq!(aditya.epf_employee_deduction, 1800.0); // 12% of 15k limit
        assert_eq!(aditya.esi_employee_deduction, 0.0); // Aditya earns > ₹21,000, exempt from ESI
        assert_eq!(aditya.net_take_home, 30000.0 - 1800.0);
    }

    #[test]
    fn test_statutory_filing_dashboard() {
        let mut dashboard = StatutoryFilingDashboard::new();
        dashboard.register_filing(
            "Form 26Q".to_string(),
            "IncomeTax".to_string(),
            "2026-07-31".to_string(),
            1, // Critical
        );
        dashboard.register_filing(
            "Form GSTR-3B".to_string(),
            "GST".to_string(),
            "2026-07-20".to_string(),
            2, // Medium
        );

        assert_eq!(dashboard.get_compliance_score(), 0.0);

        dashboard.update_filing_status("Form GSTR-3B", "Filed");
        // GSTR-3B (Medium, weight 2.0) is filed. 26Q (Critical, weight 3.0) is pending. Total weight = 5.0. Score = 2/5 * 100 = 40.0
        assert_eq!(dashboard.get_compliance_score(), 40.0);

        dashboard.update_filing_status("Form 26Q", "Filed");
        assert_eq!(dashboard.get_compliance_score(), 100.0);
    }
}
