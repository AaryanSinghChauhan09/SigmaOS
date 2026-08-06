// SigmaOS Compliance Module
// Implements comprehensive compliance dashboard and regulatory tracking

pub mod dashboard;

pub use dashboard::{
    ComplianceAlert, ComplianceOverviewDashboard, ComplianceStatus, Deadline,
    BoardMeetingModule, ComplianceReport, RegulatoryComplianceError, GovernanceStatus,
    TaxationStatus, LabourStatus, EnvironmentalStatus, BankingStatus,
    TDSComplianceModule, EPFContributionModule, AlertSeverity,
};
