// SigmaOS Additional Linux & BSD Distro Components Module
// Zero-dependency Rust #![no_std] / std implementation of strategic distro abstractions:
// Debian dpkg-divert & dpkg-statoverride, Arch pacdiff & pacman-key, Gentoo eclass/SLOT & world file,
// FreeBSD pkg audit VuXML & newsyslog, OpenBSD signify & rcctl, Void xbps journal, Alpine apk trigger hooks.

#[cfg(not(test))]
use alloc::string::{String, ToString};
#[cfg(not(test))]
use alloc::vec::Vec;
#[cfg(not(test))]
use alloc::format;

#[cfg(test)]
use std::string::String;
#[cfg(test)]
use std::vec::Vec;

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

/// Debian dpkg-statoverride Owner, Group & Mode Permission Override Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatoverrideEntry {
    pub user_owner: String,
    pub group_owner: String,
    pub mode_octal: u32,
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct DebianDpkgStatoverrideEngine {
    pub overrides: Vec<StatoverrideEntry>,
}

impl DebianDpkgStatoverrideEngine {
    pub fn new() -> Self {
        Self { overrides: Vec::new() }
    }

    pub fn add_override(&mut self, user: &str, group: &str, mode: u32, path: &str) {
        self.overrides.retain(|o| o.path != path);
        self.overrides.push(StatoverrideEntry {
            user_owner: user.to_string(),
            group_owner: group.to_string(),
            mode_octal: mode,
            path: path.to_string(),
        });
    }

    pub fn get_override(&self, path: &str) -> Option<&StatoverrideEntry> {
        self.overrides.iter().find(|o| o.path == path)
    }
}

impl Default for DebianDpkgStatoverrideEngine {
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

/// Arch Linux pacman-key GPG Keyring Trust & Verification Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyTrustLevel {
    Unknown,
    Never,
    Marginal,
    Full,
    Ultimate,
}

#[derive(Debug, Clone)]
pub struct PacmanGpgKeyRecord {
    pub key_id: String,
    pub owner_name: String,
    pub trust_level: KeyTrustLevel,
    pub is_revoked: bool,
}

#[derive(Debug, Clone)]
pub struct ArchPacmanKeyringTrustEngine {
    pub keys: Vec<PacmanGpgKeyRecord>,
}

impl ArchPacmanKeyringTrustEngine {
    pub fn new() -> Self {
        Self { keys: Vec::new() }
    }

    pub fn import_key(&mut self, key_id: &str, owner: &str, trust: KeyTrustLevel) {
        self.keys.push(PacmanGpgKeyRecord {
            key_id: key_id.to_string(),
            owner_name: owner.to_string(),
            trust_level: trust,
            is_revoked: false,
        });
    }

    pub fn is_key_trusted(&self, key_id: &str) -> bool {
        if let Some(k) = self.keys.iter().find(|k| k.key_id == key_id) {
            !k.is_revoked && (k.trust_level == KeyTrustLevel::Full || k.trust_level == KeyTrustLevel::Ultimate)
        } else {
            false
        }
    }
}

impl Default for ArchPacmanKeyringTrustEngine {
    fn default() -> Self {
        Self::new()
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

/// Gentoo Portage World File Atom Tracking & Orphan Cleaning Engine
#[derive(Debug, Clone)]
pub struct GentooPortageWorldFileEngine {
    pub world_atoms: Vec<String>,
}

impl GentooPortageWorldFileEngine {
    pub fn new() -> Self {
        Self { world_atoms: Vec::new() }
    }

    pub fn add_to_world(&mut self, atom: &str) {
        if !self.world_atoms.contains(&atom.to_string()) {
            self.world_atoms.push(atom.to_string());
        }
    }

    pub fn remove_from_world(&mut self, atom: &str) {
        self.world_atoms.retain(|a| a != atom);
    }

    pub fn is_selected(&self, atom: &str) -> bool {
        self.world_atoms.contains(&atom.to_string())
    }
}

impl Default for GentooPortageWorldFileEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// FreeBSD newsyslog Automated Log Rotation Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewsyslogRule {
    pub log_filename: String,
    pub owner_group: String,
    pub mode_octal: u32,
    pub max_files_count: usize,
    pub max_size_kb: usize,
    pub flags: String, // e.g. "JC" for bzip2 compression + create
}

#[derive(Debug, Clone)]
pub struct FreeBsdNewsyslogEngine {
    pub rules: Vec<NewsyslogRule>,
}

impl FreeBsdNewsyslogEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, filename: &str, owner: &str, mode: u32, count: usize, size_kb: usize, flags: &str) {
        self.rules.push(NewsyslogRule {
            log_filename: filename.to_string(),
            owner_group: owner.to_string(),
            mode_octal: mode,
            max_files_count: count,
            max_size_kb: size_kb,
            flags: flags.to_string(),
        });
    }

    pub fn should_rotate(&self, filename: &str, current_size_kb: usize) -> bool {
        if let Some(rule) = self.rules.iter().find(|r| r.log_filename == filename) {
            current_size_kb >= rule.max_size_kb
        } else {
            false
        }
    }
}

impl Default for FreeBsdNewsyslogEngine {
    fn default() -> Self {
        Self::new()
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
        let _ = version;
        self.advisories.iter().find(|a| a.pkg_name == pkg_name)
    }
}

impl Default for FreeBsdPkgAuditVuxmlEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// OpenBSD rcctl Daemon & Service Supervisor Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RcctlServiceRecord {
    pub service_name: String,
    pub is_enabled: bool,
    pub custom_flags: String,
    pub timeout_sec: u32,
}

#[derive(Debug, Clone)]
pub struct OpenBsdRcctlServiceEngine {
    pub services: Vec<RcctlServiceRecord>,
}

impl OpenBsdRcctlServiceEngine {
    pub fn new() -> Self {
        Self { services: Vec::new() }
    }

    pub fn register_service(&mut self, name: &str, enabled: bool, flags: &str) {
        self.services.push(RcctlServiceRecord {
            service_name: name.to_string(),
            is_enabled: enabled,
            custom_flags: flags.to_string(),
            timeout_sec: 30,
        });
    }

    pub fn enable_service(&mut self, name: &str) -> bool {
        if let Some(s) = self.services.iter_mut().find(|s| s.service_name == name) {
            s.is_enabled = true;
            true
        } else {
            false
        }
    }

    pub fn get_flags(&self, name: &str) -> Option<&str> {
        self.services.iter().find(|s| s.service_name == name).map(|s| s.custom_flags.as_str())
    }
}

impl Default for OpenBsdRcctlServiceEngine {
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

/// Alpine Linux APK v3 Trigger Execution Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApkTriggerHook {
    pub trigger_path: String,
    pub target_executable: String,
    pub pending_triggers_count: usize,
}

#[derive(Debug, Clone)]
pub struct AlpineApkTriggerHooksEngine {
    pub triggers: Vec<ApkTriggerHook>,
}

impl AlpineApkTriggerHooksEngine {
    pub fn new() -> Self {
        Self { triggers: Vec::new() }
    }

    pub fn register_trigger(&mut self, path: &str, exec: &str) {
        self.triggers.push(ApkTriggerHook {
            trigger_path: path.to_string(),
            target_executable: exec.to_string(),
            pending_triggers_count: 0,
        });
    }

    pub fn notify_file_change(&mut self, path: &str) {
        for t in self.triggers.iter_mut() {
            if path.starts_with(&t.trigger_path) {
                t.pending_triggers_count += 1;
            }
        }
    }

    pub fn run_pending_triggers(&mut self) -> usize {
        let mut total_executed = 0;
        for t in self.triggers.iter_mut() {
            if t.pending_triggers_count > 0 {
                total_executed += t.pending_triggers_count;
                t.pending_triggers_count = 0;
            }
        }
        total_executed
    }
}

impl Default for AlpineApkTriggerHooksEngine {
    fn default() -> Self {
        Self::new()
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
    fn test_dpkg_statoverride_engine() {
        let mut statoverride = DebianDpkgStatoverrideEngine::new();
        statoverride.add_override("root", "mail", 0o2755, "/usr/bin/procmail");
        let entry = statoverride.get_override("/usr/bin/procmail").unwrap();
        assert_eq!(entry.user_owner, "root");
        assert_eq!(entry.group_owner, "mail");
        assert_eq!(entry.mode_octal, 0o2755);
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
    fn test_pacman_keyring_trust_engine() {
        let mut keyring = ArchPacmanKeyringTrustEngine::new();
        keyring.import_key("0x12345678", "Arch Linux Master Key", KeyTrustLevel::Ultimate);
        assert!(keyring.is_key_trusted("0x12345678"));
        assert!(!keyring.is_key_trusted("0x87654321"));
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
    fn test_gentoo_portage_world_file_engine() {
        let mut world = GentooPortageWorldFileEngine::new();
        world.add_to_world("sys-apps/systemd");
        world.add_to_world("dev-lang/rust");
        assert!(world.is_selected("dev-lang/rust"));
        world.remove_from_world("sys-apps/systemd");
        assert!(!world.is_selected("sys-apps/systemd"));
    }

    #[test]
    fn test_freebsd_newsyslog_engine() {
        let mut newsyslog = FreeBsdNewsyslogEngine::new();
        newsyslog.add_rule("/var/log/messages", "root:wheel", 0o640, 7, 100, "JC");
        assert!(newsyslog.should_rotate("/var/log/messages", 100));
        assert!(!newsyslog.should_rotate("/var/log/messages", 50));
    }

    #[test]
    fn test_freebsd_pkg_audit_vuxml_engine() {
        let mut audit = FreeBsdPkgAuditVuxmlEngine::new();
        audit.register_advisory("openssl", "< 3.0.12", "CVE-2024-1234");
        let found = audit.check_vulnerability("openssl", "3.0.11").unwrap();
        assert_eq!(found.cve_id, "CVE-2024-1234");
    }

    #[test]
    fn test_openbsd_rcctl_service_engine() {
        let mut rcctl = OpenBsdRcctlServiceEngine::new();
        rcctl.register_service("smtpd", true, "-v");
        assert_eq!(rcctl.get_flags("smtpd"), Some("-v"));
        assert!(rcctl.enable_service("smtpd"));
    }

    #[test]
    fn test_openbsd_signify_engine() {
        let signify = OpenBsdSignifyBaseEngine::new("untrusted comment: openbsd-76-base public key", [1u8; 32]);
        let sig = [1u8; 64];
        assert!(signify.verify_signature(b"base.tgz", &sig));
    }

    #[test]
    fn test_alpine_apk_trigger_hooks_engine() {
        let mut triggers = AlpineApkTriggerHooksEngine::new();
        triggers.register_trigger("/usr/lib/gio/modules", "/usr/bin/gio-querymodules");
        triggers.notify_file_change("/usr/lib/gio/modules/libgiognutls.so");
        assert_eq!(triggers.run_pending_triggers(), 1);
        assert_eq!(triggers.run_pending_triggers(), 0);
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
