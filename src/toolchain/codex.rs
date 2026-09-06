#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS Ancient Build Replay Codex (BuildCodex)
// Formulates compiler build codex logs for legacy reproducible tooling

use crate::klib::collections::HashMap;
use std::string::String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodexCategory {
    LegacyC,
    LegacyCpp,
    LegacyAsm,
}

pub struct CodexEntry {
    pub file_name: String,
    pub compiler_used: String,
    pub binary_hash: String,
}

pub struct BuildCodex {
    pub category: CodexCategory,
    pub codex_map: HashMap<String, CodexEntry>,
}

impl BuildCodex {
    pub fn new(cat: CodexCategory) -> Self {
        BuildCodex {
            category: cat,
            codex_map: HashMap::new(),
        }
    }

    pub fn register_build_log(&mut self, file: String, cc: String, hash: String) {
        self.codex_map.insert(
            file.clone(),
            CodexEntry {
                file_name: file,
                compiler_used: cc,
                binary_hash: hash,
            },
        );
    }

    pub fn verify_build_integrity(&self, file: &str, expected_hash: &str) -> bool {
        if let Some(entry) = self.codex_map.get(file) {
            entry.binary_hash == expected_hash
        } else {
            false
        }
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_build_codex_registration() {
        let mut codex = BuildCodex::new(CodexCategory::LegacyC);
        codex.register_build_log(
            "init.c".to_string(),
            "gcc-2.7.2".to_string(),
            "hash999".to_string(),
        );

        assert!(codex.verify_build_integrity("init.c", "hash999"));
        assert!(!codex.verify_build_integrity("init.c", "badhash"));
    }
}
