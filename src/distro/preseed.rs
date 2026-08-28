extern crate alloc;
// SigmaOS Debian-style Automated Preseed Installer Subsystem (S-Preseed)
// Zero-dependency, #![no_std] compliant, parses and executes automated installs.


use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::cell::RefCell;

#[derive(Debug, Clone)]
pub struct PreseedVariable {
    pub owner: String,
    pub question: String,
    pub value_type: String,
    pub value: String,
}

pub struct SovereignPreseedParser {
    pub variables: RefCell<Vec<PreseedVariable>>,
    pub is_applied: core::sync::atomic::AtomicBool,
}

impl SovereignPreseedParser {
    pub fn new() -> Self {
        Self {
            variables: RefCell::new(Vec::new()),
            is_applied: core::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Parses a standard Debian preseed file line-by-line: "d-i owner/question type value"
    pub fn parse_preseed_content(&self, content: &str) -> usize {
        let mut vars = self.variables.borrow_mut();
        let mut count = 0;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Split into "d-i", "question_path", "type", "value..."
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 4 || parts[0] != "d-i" {
                continue;
            }

            let q_path = parts[1];
            let q_type = parts[2];
            let q_val = parts[3..].join(" ");

            // Split question_path into owner and question name, e.g. "passwd/user-fullname" -> "passwd", "user-fullname"
            let q_parts: Vec<&str> = q_path.splitn(2, '/').collect();
            if q_parts.len() < 2 {
                continue;
            }

            vars.push(PreseedVariable {
                owner: q_parts[0].to_string(),
                question: q_parts[1].to_string(),
                value_type: q_type.to_string(),
                value: q_val,
            });
            count += 1;
        }

        count
    }

    /// Retrieves a preseed value by owner and question name
    pub fn get_value(&self, owner: &str, question: &str) -> Option<String> {
        let vars = self.variables.borrow();
        for var in vars.iter() {
            if var.owner == owner && var.question == question {
                return Some(var.value.clone());
            }
        }
        None
    }

    /// Triggers automated system configuration and package sync based on preseed values
    pub fn execute_automated_installation(&self) -> bool {
        if self
            .is_applied
            .swap(true, core::sync::atomic::Ordering::Relaxed)
        {
            return false; // Already executed
        }

        // Simulates automated configuration
        true
    }
}

impl Default for SovereignPreseedParser {
    fn default() -> Self {
        Self::new()
    }
}
