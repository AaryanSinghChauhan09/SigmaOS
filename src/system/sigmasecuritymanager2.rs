use alloc::string::String;

#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityPolicyType {
    Mandatory,
    Discretionary,
}

pub struct SigmaSecurityManager2;
