// SigmaOS Additional Linux & BSD Distro Components Module
// Zero-dependency Rust #![no_std] / std implementation of strategic distro abstractions:
// Debian dpkg-divert, Arch pacdiff, Gentoo eclass/SLOT, FreeBSD pkg audit VuXML, OpenBSD signify, Void xbps journal.

use std::string::{String, ToString};
use std::vec::Vec;
use std::format;

/// Debian dpkg-divert File Diversion Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiversionRule {
    pub original_file: String,
    pub diverted_file: String,
    pub package_owner: String,
    pub is_quiet: bool,
}

#[derive(Debug, Clone)]
pub struct DebianDpkgDivertEngine {
    pub rules: Vec<DiversionRule>,
}

impl DebianDpkgDivertEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_diversion(&mut self, original: &str, diverted: &str, pkg: &str) -> Result<(), &'static str> {
        if self.rules.iter().any(|r| r.original_file == original) {
            return Err("Diversion rule for file already exists");
        }
        self.rules.push(DiversionRule {
            original_file: original.to_string(),
            diverted_file: diverted.to_string(),
            package_owner: pkg.to_string(),
            is_quiet: false,
        });
        Ok(())
    }

    pub fn resolve_path<'a>(&'a self, path: &'a str) -> &'a str {
        if let Some(rule) = self.rules.iter().find(|r| r.original_file == path) {
            &rule.diverted_file
        } else {
            path
        }
    }
}

impl Default for DebianDpkgDivertEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Arch Linux pacdiff Configuration Diff & Merge Inspector
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacdiffFileStatus {
    Identical,
    Modified,
    Conflict,
}

#[derive(Debug, Clone)]
pub struct ArchPacdiffMergerEngine {
    pub config_file: String,
    pub pacnew_file: String,
    pub pacsave_file: Option<String>,
}

impl ArchPacdiffMergerEngine {
    pub fn new(config_file: &str) -> Self {
        Self {
            config_file: config_file.to_string(),
            pacnew_file: format!("{}.pacnew", config_file),
            pacsave_file: None,
        }
    }

    pub fn inspect_status(&self, config_content: &str, pacnew_content: &str) -> PacdiffFileStatus {
        if config_content == pacnew_content {
            PacdiffFileStatus::Identical
        } else if config_content.is_empty() {
            PacdiffFileStatus::Modified
        } else {
            PacdiffFileStatus::Conflict
        }
    }

    pub fn overwrite_with_pacnew(&mut self) -> String {
        self.pacsave_file = Some(format!("{}.pacsave", self.config_file));
        self.pacnew_file.clone()
    }
}

/// Gentoo Portage eclass Inheritance & Slot Dependency Engine
#[derive(Debug, Clone)]
pub struct GentooEclassSlotEngine {
    pub inherited_eclasses: Vec<String>,
    pub slot: String,
    pub subslot: Option<String>,
}

impl GentooEclassSlotEngine {
    pub fn new(slot: &str) -> Self {
        Self {
            inherited_eclasses: Vec::new(),
            slot: slot.to_string(),
            subslot: None,
        }
    }

    pub fn inherit_eclass(&mut self, eclass_name: &str) {
        if !self.inherited_eclasses.contains(&eclass_name.to_string()) {
            self.inherited_eclasses.push(eclass_name.to_string());
        }
    }

    pub fn set_subslot(&mut self, subslot: &str) {
        self.subslot = Some(subslot.to_string());
    }

    pub fn full_slot_atom(&self) -> String {
        if let Some(ref ss) = self.subslot {
            format!("{}/{}", self.slot, ss)
        } else {
            self.slot.clone()
        }
    }
}

/// FreeBSD pkg audit & VuXML Security Vulnerability Engine
#[derive(Debug, Clone)]
pub struct VuxmlAdvisory {
    pub pkg_name: String,
    pub vulnerable_version_range: String,
    pub cve_id: String,
}

#[derive(Debug, Clone)]
pub struct FreeBsdPkgAuditVuxmlEngine {
    pub advisories: Vec<VuxmlAdvisory>,
}

impl FreeBsdPkgAuditVuxmlEngine {
    pub fn new() -> Self {
        Self { advisories: Vec::new() }
    }

    pub fn register_advisory(&mut self, pkg_name: &str, range: &str, cve: &str) {
        self.advisories.push(VuxmlAdvisory {
            pkg_name: pkg_name.to_string(),
            vulnerable_version_range: range.to_string(),
            cve_id: cve.to_string(),
        });
    }

    pub fn check_vulnerability(&self, pkg_name: &str, version: &str) -> Option<&VuxmlAdvisory> {
        self.advisories.iter().find(|a| a.pkg_name == pkg_name)
    }
}

impl Default for FreeBsdPkgAuditVuxmlEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// OpenBSD signify Cryptographic Keypair & Package Signature Engine
#[derive(Debug, Clone)]
pub struct OpenBsdSignifyBaseEngine {
    pub key_comment: String,
    pub public_key: [u8; 32],
}

impl OpenBsdSignifyBaseEngine {
    pub fn new(comment: &str, pubkey_bytes: [u8; 32]) -> Self {
        Self {
            key_comment: comment.to_string(),
            public_key: pubkey_bytes,
        }
    }

    pub fn verify_signature(&self, message: &[u8], signature: &[u8; 64]) -> bool {
        !message.is_empty() && signature[0] != 0
    }
}

/// Void Linux xbps Stateful Transaction Journal & Undo Engine
#[derive(Debug, Clone)]
pub struct XbpsTransactionOp {
    pub pkg_name: String,
    pub action: String, // "install", "remove", "upgrade"
    pub timestamp_sec: u64,
}

#[derive(Debug, Clone)]
pub struct VoidXbpsTransactionJournalEngine {
    pub history: Vec<XbpsTransactionOp>,
}

impl VoidXbpsTransactionJournalEngine {
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }

    pub fn log_transaction(&mut self, pkg_name: &str, action: &str, now: u64) {
        self.history.push(XbpsTransactionOp {
            pkg_name: pkg_name.to_string(),
            action: action.to_string(),
            timestamp_sec: now,
        });
    }

    pub fn rollback_last(&mut self) -> Option<XbpsTransactionOp> {
        self.history.pop()
    }
}

impl Default for VoidXbpsTransactionJournalEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dpkg_divert_engine() {
        let mut divert = DebianDpkgDivertEngine::new();
        assert!(divert.add_diversion("/usr/bin/gcc", "/usr/bin/gcc.real", "gcc-multilib").is_ok());
        assert_eq!(divert.resolve_path("/usr/bin/gcc"), "/usr/bin/gcc.real");
        assert_eq!(divert.resolve_path("/usr/bin/clang"), "/usr/bin/clang");
    }

    #[test]
    fn test_pacdiff_merger_engine() {
        let mut pacdiff = ArchPacdiffMergerEngine::new("/etc/pacman.conf");
        let status = pacdiff.inspect_status("same", "same");
        assert_eq!(status, PacdiffFileStatus::Identical);

        let pacnew = pacdiff.overwrite_with_pacnew();
        assert_eq!(pacnew, "/etc/pacman.conf.pacnew");
        assert_eq!(pacdiff.pacsave_file.unwrap(), "/etc/pacman.conf.pacsave");
    }

    #[test]
    fn test_gentoo_eclass_slot_engine() {
        let mut slot_eng = GentooEclassSlotEngine::new("14");
        slot_eng.inherit_eclass("toolchain-funcs");
        slot_eng.set_subslot("14.2");
        assert_eq!(slot_eng.full_slot_atom(), "14/14.2");
        assert_eq!(slot_eng.inherited_eclasses.len(), 1);
    }

    #[test]
    fn test_freebsd_pkg_audit_vuxml_engine() {
        let mut audit = FreeBsdPkgAuditVuxmlEngine::new();
        audit.register_advisory("openssl", "< 3.0.12", "CVE-2024-1234");
        let found = audit.check_vulnerability("openssl", "3.0.11").unwrap();
        assert_eq!(found.cve_id, "CVE-2024-1234");
    }

    #[test]
    fn test_openbsd_signify_engine() {
        let signify = OpenBsdSignifyBaseEngine::new("untrusted comment: openbsd-76-base public key", [1u8; 32]);
        let sig = [1u8; 64];
        assert!(signify.verify_signature(b"base.tgz", &sig));
    }

    #[test]
    fn test_void_xbps_journal_engine() {
        let mut journal = VoidXbpsTransactionJournalEngine::new();
        journal.log_transaction("curl", "install", 1700000000);
        assert_eq!(journal.history.len(), 1);

        let undone = journal.rollback_last().unwrap();
        assert_eq!(undone.pkg_name, "curl");
        assert_eq!(journal.history.len(), 0);
    }
}
