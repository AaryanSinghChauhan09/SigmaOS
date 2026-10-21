use alloc::string::String;

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct PackageRepo {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageState {
    Installed,
    Available,
}

pub struct SigmaPackageManager3;
