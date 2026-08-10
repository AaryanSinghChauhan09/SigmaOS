// SigmaOS Legal & Compliance Module
pub mod compliance;
pub mod licensing;
pub mod compliance;

pub use compliance::{
    ComplianceStatus, GlobalStandard, InternationalComplianceTracker, LabourLawCompliance,
    LabourLawConfig, RegulatoryControl, StatutoryFiling, StatutoryFilingDashboard,
    StatutoryPayrollBreakdown,
};
pub use licensing::{
    ComplianceCert, ComponentLicense, LegalComplianceRegistry, LicenseType, PatentRecord,
};
pub use compliance::{
    GlobalStandard, ComplianceStatus, RegulatoryControl, InternationalComplianceTracker,
    LabourLawConfig, StatutoryPayrollBreakdown, LabourLawCompliance, StatutoryFiling,
    StatutoryFilingDashboard,
};
