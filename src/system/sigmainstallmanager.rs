use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallStatus {
    Pending,
    Installing,
    Completed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallType {
    Binary,
    Source,
    Package,
}

#[derive(Debug, Clone)]
pub struct SoftwareItem {
    pub name: String,
    pub version: String,
}

pub struct SigmaInstallManager;
