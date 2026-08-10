// SigmaOS Legal & Compliance Module
pub mod licensing;
pub mod compliance;

pub use licensing::{
    ComplianceCert, ComponentLicense, LegalComplianceRegistry, LicenseType, PatentRecord,
};
pub use compliance::{
    GlobalStandard, ComplianceStatus, RegulatoryControl, InternationalComplianceTracker,
    LabourLawConfig, StatutoryPayrollBreakdown, LabourLawCompliance, StatutoryFiling,
    StatutoryFilingDashboard,
};
