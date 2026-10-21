use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WizardPageType {
    Welcome,
    Configuration,
    Confirmation,
}

#[derive(Debug, Clone)]
pub struct WizardPage {
    pub title: String,
    pub page_type: WizardPageType,
}

pub struct SigmaWizardManager;
