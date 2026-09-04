// Toolchain Bootstrap Engine
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootstrapStage {
    Stage0,
    Stage1,
    Stage2,
}

pub struct LfsBootstrapEngine {
    pub stage: BootstrapStage,
    pub environment: HashMap<String, String>,
}

impl LfsBootstrapEngine {
    pub fn new() -> Self {
        LfsBootstrapEngine {
            stage: BootstrapStage::Stage0,
            environment: HashMap::new(),
        }
    }
}
