// Network Analyzer and Packet Inspection Engine
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NixDeclarativeFilter {
    pub name: String,
    pub expression: String,
}

pub trait AnalysisStrategy {
    fn analyze(&self, packet_data: &[u8]) -> bool;
}

pub struct KaliSnoopAnalysis {
    pub signature_patterns: Vec<Vec<u8>>,
}

impl KaliSnoopAnalysis {
    pub fn new() -> Self {
        KaliSnoopAnalysis {
            signature_patterns: Vec::new(),
        }
    }
}

impl AnalysisStrategy for KaliSnoopAnalysis {
    fn analyze(&self, packet_data: &[u8]) -> bool {
        for pat in &self.signature_patterns {
            if packet_data.windows(pat.len()).any(|w| w == pat.as_slice()) {
                return true;
            }
        }
        false
    }
}
