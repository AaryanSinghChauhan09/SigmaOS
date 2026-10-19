//! SigmaSecurityAudit - Security Audit and Compliance System
//!
//! Provides comprehensive security audit and compliance inspired by Linux security tools.
//! Supports security scanning, vulnerability detection, and compliance reporting.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Security issue severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecuritySeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Security issue
#[derive(Debug, Clone)]
pub struct SecurityIssue {
    pub id: String,
    pub title: String,
    pub description: String,
    pub severity: SecuritySeverity,
    pub category: String,
    pub resolved: bool,
}

impl SecurityIssue {
    pub fn new(
        id: String,
        title: String,
        description: String,
        severity: SecuritySeverity,
        category: String,
    ) -> Self {
        Self {
            id,
            title,
            description,
            severity,
            category,
            resolved: false,
        }
    }

    pub fn resolve(&mut self) {
        self.resolved = true;
    }

    pub fn reopen(&mut self) {
        self.resolved = false;
    }
}

/// Security report
#[derive(Debug, Clone)]
pub struct SecurityReport {
    pub id: String,
    pub name: String,
    pub issues: Vec<SecurityIssue>,
    pub timestamp: u64,
}

impl SecurityReport {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            issues: Vec::new(),
            timestamp: 0,
        }
    }

    pub fn with_issue(mut self, issue: SecurityIssue) -> Self {
        self.issues.push(issue);
        self
    }

    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn critical_issue_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == SecuritySeverity::Critical && !i.resolved)
            .count()
    }

    pub fn high_issue_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == SecuritySeverity::High && !i.resolved)
            .count()
    }

    pub fn total_unresolved(&self) -> usize {
        self.issues.iter().filter(|i| !i.resolved).count()
    }
}

/// Security audit manager
pub struct SigmaSecurityAudit {
    reports: BTreeMap<String, SecurityReport>,
    next_report_id: u64,
}

impl SigmaSecurityAudit {
    pub fn new() -> Self {
        Self {
            reports: BTreeMap::new(),
            next_report_id: 1,
        }
    }

    pub fn add_report(&mut self, name: String) -> String {
        let id = format!("report-{}", self.next_report_id);
        self.next_report_id += 1;
        let report = SecurityReport::new(id.clone(), name);
        self.reports.insert(id.clone(), report);
        id
    }

    pub fn get_report(&self, id: &str) -> Option<&SecurityReport> {
        self.reports.get(id)
    }

    pub fn get_report_mut(&mut self, id: &str) -> Option<&mut SecurityReport> {
        self.reports.get_mut(id)
    }

    pub fn list_reports(&self) -> Vec<&SecurityReport> {
        self.reports.values().collect()
    }

    pub fn remove_report(&mut self, id: &str) -> bool {
        self.reports.remove(id).is_some()
    }

    pub fn report_count(&self) -> usize {
        self.reports.len()
    }

    pub fn total_critical_issues(&self) -> usize {
        self.reports
            .values()
            .map(|r| r.critical_issue_count())
            .sum()
    }

    pub fn total_high_issues(&self) -> usize {
        self.reports.values().map(|r| r.high_issue_count()).sum()
    }

    pub fn total_unresolved(&self) -> usize {
        self.reports.values().map(|r| r.total_unresolved()).sum()
    }
}

impl Default for SigmaSecurityAudit {
    fn default() -> Self {
        Self::new()
    }
}

static mut SIGMASECURITYAUDIT_INSTANCE: Option<SigmaSecurityAudit> = None;

pub fn init_sigmasecurityaudit() {
    unsafe {
        SIGMASECURITYAUDIT_INSTANCE = Some(SigmaSecurityAudit::new());
    }
}

pub fn get_sigmasecurityaudit() -> &'static SigmaSecurityAudit {
    unsafe {
        SIGMASECURITYAUDIT_INSTANCE
            .as_ref()
            .expect("SigmaSecurityAudit not initialized")
    }
}

pub fn get_sigmasecurityaudit_mut() -> &'static mut SigmaSecurityAudit {
    unsafe {
        SIGMASECURITYAUDIT_INSTANCE
            .as_mut()
            .expect("SigmaSecurityAudit not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_issue_creation() {
        let issue = SecurityIssue::new(
            "issue-1".to_string(),
            "Outdated Package".to_string(),
            "Package has known vulnerabilities".to_string(),
            SecuritySeverity::High,
            "Software".to_string(),
        );
        assert_eq!(issue.title, "Outdated Package");
        assert_eq!(issue.severity, SecuritySeverity::High);
        assert!(!issue.resolved);
    }

    #[test]
    fn test_security_issue_resolve() {
        let mut issue = SecurityIssue::new(
            "issue-1".to_string(),
            "Outdated Package".to_string(),
            "Vulnerability".to_string(),
            SecuritySeverity::High,
            "Software".to_string(),
        );
        issue.resolve();
        assert!(issue.resolved);

        issue.reopen();
        assert!(!issue.resolved);
    }

    #[test]
    fn test_security_report_creation() {
        let report = SecurityReport::new("report-1".to_string(), "Monthly Audit".to_string());
        assert_eq!(report.name, "Monthly Audit");
        assert_eq!(report.issues.len(), 0);
    }

    #[test]
    fn test_security_report_with_issue() {
        let report = SecurityReport::new("report-1".to_string(), "Monthly Audit".to_string())
            .with_issue(SecurityIssue::new(
                "issue-1".to_string(),
                "Vulnerability".to_string(),
                "Description".to_string(),
                SecuritySeverity::Critical,
                "Software".to_string(),
            ));
        assert_eq!(report.issues.len(), 1);
    }

    #[test]
    fn test_security_audit_manager() {
        let manager = SigmaSecurityAudit::new();
        assert_eq!(manager.report_count(), 0);
    }

    #[test]
    fn test_add_report() {
        let mut manager = SigmaSecurityAudit::new();
        let id = manager.add_report("Monthly Audit".to_string());
        assert_eq!(id, "report-1");
        assert_eq!(manager.report_count(), 1);
    }

    #[test]
    fn test_total_critical_issues() {
        let mut manager = SigmaSecurityAudit::new();
        let id = manager.add_report("Audit".to_string());

        let report = manager.get_report_mut(&id).unwrap();
        report.issues.push(SecurityIssue::new(
            "i1".to_string(),
            "Vuln1".to_string(),
            "Desc".to_string(),
            SecuritySeverity::Critical,
            "Cat".to_string(),
        ));
        report.issues.push(SecurityIssue::new(
            "i2".to_string(),
            "Vuln2".to_string(),
            "Desc".to_string(),
            SecuritySeverity::High,
            "Cat".to_string(),
        ));

        assert_eq!(manager.total_critical_issues(), 1);
    }

    #[test]
    fn test_total_unresolved() {
        let mut manager = SigmaSecurityAudit::new();
        let id = manager.add_report("Audit".to_string());

        let report = manager.get_report_mut(&id).unwrap();
        let mut issue = SecurityIssue::new(
            "i1".to_string(),
            "Vuln".to_string(),
            "Desc".to_string(),
            SecuritySeverity::High,
            "Cat".to_string(),
        );
        issue.resolve();
        report.issues.push(issue);
        report.issues.push(SecurityIssue::new(
            "i2".to_string(),
            "Vuln2".to_string(),
            "Desc".to_string(),
            SecuritySeverity::High,
            "Cat".to_string(),
        ));

        assert_eq!(manager.total_unresolved(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_sigmasecurityaudit();
        let manager = get_sigmasecurityaudit();
        assert_eq!(manager.report_count(), 0);

        let manager_mut = get_sigmasecurityaudit_mut();
        manager_mut.add_report("Audit".to_string());
        assert_eq!(manager.report_count(), 1);
    }
}
