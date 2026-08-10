// SigmaOS Legal & Compliance Module
pub mod compliance;
pub mod licensing;

pub use compliance::{
    ComplianceStatus, GlobalStandard, InternationalComplianceTracker, LabourLawCompliance,
    LabourLawConfig, RegulatoryControl, StatutoryFiling, StatutoryFilingDashboard,
    StatutoryPayrollBreakdown,
};
pub use licensing::{
    ComplianceCert, ComponentLicense, LegalComplianceRegistry, LicenseType, PatentRecord,
};