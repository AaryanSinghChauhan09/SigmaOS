// SigmaOS — gentoo_inspirations.rs
// Implements Gentoo-inspired features:
//   • USE flags system
//   • Portage-style source compilation (emerge equivalent)
//   • ACCEPT_KEYWORDS / FEATURES mask mechanism
//   • ebuilds metadata model
//   • World set and dependency graph

extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// ── USE flags ─────────────────────────────────────────────────────────────────

/// A single USE flag: either enabled (`+`) or disabled (`-`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseFlag {
    pub name: String,
    pub enabled: bool,
    /// Human-readable description of what enabling this flag does.
    pub description: String,
}

impl UseFlag {
    pub fn enabled(name: &str, desc: &str) -> Self {
        UseFlag {
            name: name.to_string(),
            enabled: true,
            description: desc.to_string(),
        }
    }

    pub fn disabled(name: &str, desc: &str) -> Self {
        UseFlag {
            name: name.to_string(),
            enabled: false,
            description: desc.to_string(),
        }
    }

    /// Returns `"+name"` or `"-name"`.
    pub fn as_flag_str(&self) -> String {
        if self.enabled {
            format!("+{}", self.name)
        } else {
            format!("-{}", self.name)
        }
    }
}

/// The full set of USE flags active for a build.
///
/// Analogous to the `USE` variable in Gentoo's `/etc/portage/make.conf`.
#[derive(Debug, Clone, Default)]
pub struct UseFlags {
    flags: Vec<UseFlag>,
}

impl UseFlags {
    pub fn new() -> Self {
        UseFlags { flags: Vec::new() }
    }

    /// Enable a USE flag (adding it if not already present).
    pub fn enable(&mut self, name: &str, desc: &str) {
        for f in self.flags.iter_mut() {
            if f.name == name {
                f.enabled = true;
                return;
            }
        }
        self.flags.push(UseFlag::enabled(name, desc));
    }

    /// Disable a USE flag.
    pub fn disable(&mut self, name: &str, desc: &str) {
        for f in self.flags.iter_mut() {
            if f.name == name {
                f.enabled = false;
                return;
            }
        }
        self.flags.push(UseFlag::disabled(name, desc));
    }

    /// Returns `true` if the named flag is enabled.
    pub fn is_enabled(&self, name: &str) -> bool {
        self.flags.iter().any(|f| f.name == name && f.enabled)
    }

    /// Toggle a flag.
    pub fn toggle(&mut self, name: &str) {
        for f in self.flags.iter_mut() {
            if f.name == name {
                f.enabled = !f.enabled;
                return;
            }
        }
    }

    /// Return all enabled flag names.
    pub fn enabled_flags(&self) -> Vec<&str> {
        self.flags
            .iter()
            .filter(|f| f.enabled)
            .map(|f| f.name.as_str())
            .collect()
    }

    /// Return all disabled flag names.
    pub fn disabled_flags(&self) -> Vec<&str> {
        self.flags
            .iter()
            .filter(|f| !f.enabled)
            .map(|f| f.name.as_str())
            .collect()
    }

    /// Parse a USE string such as `"openssl -gnutls ipv6 -doc"`.
    pub fn parse_use_string(&mut self, s: &str) {
        for token in s.split_whitespace() {
            let (enabled, name) = if let Some(stripped) = token.strip_prefix('-') {
                (false, stripped)
            } else {
                let name = token.strip_prefix('+').unwrap_or(token);
                (true, name)
            };
            if enabled {
                self.enable(name, "");
            } else {
                self.disable(name, "");
            }
        }
    }
}

// ── Ebuild metadata ───────────────────────────────────────────────────────────

/// Stability keyword for a package on a given architecture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    /// Stable on this arch.
    Stable(String),
    /// Testing (`~arch`) on this arch.
    Testing(String),
    /// Broken / unavailable on this arch.
    Broken(String),
}

impl Keyword {
    pub fn arch(&self) -> &str {
        match self {
            Keyword::Stable(a) | Keyword::Testing(a) | Keyword::Broken(a) => a.as_str(),
        }
    }

    /// Parse a keyword token such as `"amd64"`, `"~arm64"`, `"-riscv"`.
    pub fn parse(token: &str) -> Keyword {
        if let Some(a) = token.strip_prefix('-') {
            Keyword::Broken(a.to_string())
        } else if let Some(a) = token.strip_prefix('~') {
            Keyword::Testing(a.to_string())
        } else {
            Keyword::Stable(token.to_string())
        }
    }
}

/// A USE-conditional dependency specification.
#[derive(Debug, Clone)]
pub struct UseDep {
    /// If `None`, the dependency is unconditional.
    pub use_flag: Option<String>,
    /// `true` → dep is active when the flag is enabled.
    /// `false` → dep is active when the flag is disabled.
    pub when_enabled: bool,
    pub atom: String,
}

impl UseDep {
    pub fn unconditional(atom: &str) -> Self {
        UseDep {
            use_flag: None,
            when_enabled: true,
            atom: atom.to_string(),
        }
    }

    pub fn conditional(flag: &str, when_enabled: bool, atom: &str) -> Self {
        UseDep {
            use_flag: Some(flag.to_string()),
            when_enabled,
            atom: atom.to_string(),
        }
    }

    /// Returns `true` if the dependency is active given the current USE flags.
    pub fn is_active(&self, use_flags: &UseFlags) -> bool {
        match &self.use_flag {
            None => true,
            Some(flag) => {
                let flag_on = use_flags.is_enabled(flag);
                if self.when_enabled {
                    flag_on
                } else {
                    !flag_on
                }
            }
        }
    }
}

/// An ebuild metadata record — the Gentoo equivalent of a package recipe.
#[derive(Debug, Clone, Default)]
pub struct Ebuild {
    pub category: String,
    pub name: String,
    pub version: String,
    pub revision: u32,
    pub description: String,
    pub homepage: String,
    pub license: String,
    pub slot: String,
    pub keywords: Vec<Keyword>,
    pub iuse: Vec<String>,    // declared USE flags this ebuild recognises
    pub depend: Vec<UseDep>,  // build-time dependencies
    pub rdepend: Vec<UseDep>, // run-time dependencies
    pub bdepend: Vec<UseDep>, // build-host dependencies
    pub src_uri: Vec<String>,
    pub eapi: u32,
}

impl Ebuild {
    /// Full atom: `category/name-version[-rN]`.
    pub fn atom(&self) -> String {
        if self.revision > 0 {
            format!(
                "{}/{}-{}-r{}",
                self.category, self.name, self.version, self.revision
            )
        } else {
            format!("{}/{}-{}", self.category, self.name, self.version)
        }
    }

    /// Return build-time deps that are active for the given USE flags.
    pub fn active_depends<'a>(&'a self, use_flags: &UseFlags) -> Vec<&'a str> {
        self.depend
            .iter()
            .filter(|d| d.is_active(use_flags))
            .map(|d| d.atom.as_str())
            .collect()
    }

    /// Return run-time deps that are active for the given USE flags.
    pub fn active_rdepends<'a>(&'a self, use_flags: &UseFlags) -> Vec<&'a str> {
        self.rdepend
            .iter()
            .filter(|d| d.is_active(use_flags))
            .map(|d| d.atom.as_str())
            .collect()
    }

    /// Check whether this ebuild is stable/testing for the given architecture.
    pub fn keyword_for_arch(&self, arch: &str) -> Option<&Keyword> {
        self.keywords.iter().find(|k| k.arch() == arch)
    }
}

// ── Portage-style package tree ─────────────────────────────────────────────────

/// A simple in-memory Portage tree.
pub struct PortageTree {
    ebuilds: Vec<Ebuild>,
}

impl PortageTree {
    pub fn new() -> Self {
        PortageTree {
            ebuilds: Vec::new(),
        }
    }

    pub fn add(&mut self, eb: Ebuild) {
        self.ebuilds.push(eb);
    }

    /// Find the latest stable or testing ebuild for `category/name` on `arch`.
    pub fn best_version<'a>(
        &'a self,
        category: &str,
        name: &str,
        arch: &str,
    ) -> Option<&'a Ebuild> {
        let candidates: Vec<&Ebuild> = self
            .ebuilds
            .iter()
            .filter(|e| e.category == category && e.name == name)
            .filter(|e| {
                e.keyword_for_arch(arch)
                    .map(|k| !matches!(k, Keyword::Broken(_)))
                    .unwrap_or(false)
            })
            .collect();
        // Return the last one (simple version ordering — a real impl would
        // parse version segments).
        candidates.into_iter().last()
    }

    /// Return all ebuilds that provide `name` (via PROVIDES or direct name match).
    pub fn find_by_name(&self, name: &str) -> Vec<&Ebuild> {
        self.ebuilds.iter().filter(|e| e.name == name).collect()
    }
}

// ── Emerge-equivalent build pipeline ─────────────────────────────────────────

/// Outcome of a single emerge phase.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmergePhase {
    Fetch,
    Unpack,
    Prepare,
    Configure,
    Compile,
    Test,
    Install,
    QaCheck,
    Merge,
}

impl EmergePhase {
    pub fn as_str(&self) -> &'static str {
        match self {
            EmergePhase::Fetch => "fetch",
            EmergePhase::Unpack => "unpack",
            EmergePhase::Prepare => "prepare",
            EmergePhase::Configure => "configure",
            EmergePhase::Compile => "compile",
            EmergePhase::Test => "test",
            EmergePhase::Install => "install",
            EmergePhase::QaCheck => "qa-check",
            EmergePhase::Merge => "merge",
        }
    }
}

/// Result of an emerge run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmergeResult {
    Success,
    Failed { phase: String, reason: String },
    Blocked(String),
}

/// Configuration for an emerge run (analogous to `/etc/portage/make.conf`).
#[derive(Debug, Clone)]
pub struct PortageConfig {
    pub chost: String,
    pub cflags: String,
    pub cxxflags: String,
    pub makeopts: String,
    /// Accepted keyword tokens, e.g. `["amd64", "~amd64"]`.
    pub accept_keywords: Vec<String>,
    pub features: Vec<String>,
    pub use_flags: UseFlags,
}

impl Default for PortageConfig {
    fn default() -> Self {
        let mut use_flags = UseFlags::new();
        use_flags.enable("ipv6", "IPv6 support");
        use_flags.enable("openssl", "OpenSSL crypto backend");
        use_flags.disable("doc", "Build documentation");
        use_flags.disable("test", "Run test suite during build");

        PortageConfig {
            chost: "x86_64-pc-linux-gnu".to_string(),
            cflags: "-O2 -pipe -march=native".to_string(),
            cxxflags: "-O2 -pipe -march=native".to_string(),
            makeopts: "-j4".to_string(),
            accept_keywords: vec!["amd64".to_string()],
            features: vec![
                "parallel-fetch".to_string(),
                "compress-build-logs".to_string(),
                "split-elog".to_string(),
            ],
            use_flags,
        }
    }
}

/// Simulates an `emerge` invocation for a single ebuild.
pub struct EmergeRunner<'a> {
    pub config: &'a PortageConfig,
}

impl<'a> EmergeRunner<'a> {
    pub fn new(config: &'a PortageConfig) -> Self {
        EmergeRunner { config }
    }

    /// Run all phases for `ebuild` and return the outcome.
    pub fn emerge(&self, ebuild: &Ebuild) -> EmergeResult {
        // Validate keyword acceptance.
        let arch_ok = ebuild.keywords.iter().any(|k| match k {
            Keyword::Stable(a) => self.config.accept_keywords.contains(a),
            Keyword::Testing(a) => {
                let tilde = format!("~{}", a);
                self.config.accept_keywords.contains(&tilde)
                    || self.config.accept_keywords.contains(&"**".to_string())
            }
            Keyword::Broken(_) => false,
        });
        if !arch_ok {
            return EmergeResult::Blocked(format!(
                "{} has no accepted keyword for {:?}",
                ebuild.atom(),
                self.config.accept_keywords
            ));
        }

        // Run phases in order.
        let phases = [
            EmergePhase::Fetch,
            EmergePhase::Unpack,
            EmergePhase::Prepare,
            EmergePhase::Configure,
            EmergePhase::Compile,
            EmergePhase::Install,
            EmergePhase::Merge,
        ];
        for phase in &phases {
            let ok = self.run_phase(ebuild, phase);
            if !ok {
                return EmergeResult::Failed {
                    phase: phase.as_str().to_string(),
                    reason: format!("phase {} failed for {}", phase.as_str(), ebuild.atom()),
                };
            }
        }
        EmergeResult::Success
    }

    fn run_phase(&self, ebuild: &Ebuild, phase: &EmergePhase) -> bool {
        // In a real implementation each phase would fork a subprocess running
        // the ebuild functions with the correct environment.  Here we always
        // succeed unless the ebuild name contains "BROKEN".
        match phase {
            EmergePhase::Fetch => !ebuild.src_uri.iter().any(|u| u.contains("BROKEN")),
            _ => !ebuild.name.contains("BROKEN"),
        }
    }
}

// ── World set ─────────────────────────────────────────────────────────────────

/// The "world" set: packages explicitly installed by the user.
/// Analogous to Gentoo's `/var/lib/portage/world`.
pub struct WorldSet {
    atoms: Vec<String>,
}

impl WorldSet {
    pub fn new() -> Self {
        WorldSet { atoms: Vec::new() }
    }

    pub fn add(&mut self, atom: &str) {
        if !self.atoms.iter().any(|a| a == atom) {
            self.atoms.push(atom.to_string());
        }
    }

    pub fn remove(&mut self, atom: &str) {
        self.atoms.retain(|a| a != atom);
    }

    pub fn contains(&self, atom: &str) -> bool {
        self.atoms.iter().any(|a| a == atom)
    }

    pub fn all(&self) -> &[String] {
        &self.atoms
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_ebuild() -> Ebuild {
        Ebuild {
            category: "app-editors".to_string(),
            name: "vim".to_string(),
            version: "9.1.0".to_string(),
            revision: 0,
            description: "Vim text editor".to_string(),
            homepage: "https://www.vim.org".to_string(),
            license: "vim".to_string(),
            slot: "0".to_string(),
            keywords: vec![
                Keyword::Stable("amd64".to_string()),
                Keyword::Testing("arm64".to_string()),
            ],
            iuse: vec!["perl".to_string(), "python".to_string(), "lua".to_string()],
            depend: vec![
                UseDep::unconditional(">=dev-libs/glibc-2.34"),
                UseDep::conditional("python", true, ">=dev-lang/python-3.11"),
            ],
            rdepend: vec![UseDep::unconditional(">=dev-libs/glibc-2.34")],
            bdepend: vec![],
            src_uri: vec!["https://ftp.vim.org/vim-9.1.0.tar.gz".to_string()],
            eapi: 8,
        }
    }

    #[test]
    fn test_use_flags() {
        let mut uf = UseFlags::new();
        uf.parse_use_string("openssl -gnutls ipv6 -doc");
        assert!(uf.is_enabled("openssl"));
        assert!(!uf.is_enabled("gnutls"));
        assert!(uf.is_enabled("ipv6"));
        assert!(!uf.is_enabled("doc"));
    }

    #[test]
    fn test_use_conditional_deps() {
        let eb = sample_ebuild();
        let mut uf = UseFlags::new();
        uf.enable("python", "");
        let deps = eb.active_depends(&uf);
        assert!(deps.contains(&">=dev-lang/python-3.11"));

        uf.disable("python", "");
        let deps_no_py = eb.active_depends(&uf);
        assert!(!deps_no_py.contains(&">=dev-lang/python-3.11"));
    }

    #[test]
    fn test_emerge_stable() {
        let eb = sample_ebuild();
        let cfg = PortageConfig::default();
        let runner = EmergeRunner::new(&cfg);
        assert_eq!(runner.emerge(&eb), EmergeResult::Success);
    }

    #[test]
    fn test_emerge_blocked_unknown_arch() {
        let mut eb = sample_ebuild();
        eb.keywords = vec![Keyword::Testing("riscv".to_string())];
        let cfg = PortageConfig::default(); // accepts "amd64"
        let runner = EmergeRunner::new(&cfg);
        assert!(matches!(runner.emerge(&eb), EmergeResult::Blocked(_)));
    }

    #[test]
    fn test_portage_tree_best_version() {
        let mut tree = PortageTree::new();
        let mut eb_old = sample_ebuild();
        eb_old.version = "8.0.0".to_string();
        tree.add(eb_old);
        tree.add(sample_ebuild());
        let best = tree.best_version("app-editors", "vim", "amd64");
        assert!(best.is_some());
        // The last added (9.1.0) should win since we iterate in insertion order.
        assert_eq!(best.unwrap().version, "9.1.0");
    }

    #[test]
    fn test_world_set() {
        let mut world = WorldSet::new();
        world.add("app-editors/vim");
        world.add("dev-vcs/git");
        world.add("app-editors/vim"); // duplicate
        assert_eq!(world.all().len(), 2);
        assert!(world.contains("dev-vcs/git"));
        world.remove("dev-vcs/git");
        assert!(!world.contains("dev-vcs/git"));
    }

    #[test]
    fn test_keyword_parse() {
        assert!(matches!(Keyword::parse("amd64"), Keyword::Stable(_)));
        assert!(matches!(Keyword::parse("~arm64"), Keyword::Testing(_)));
        assert!(matches!(Keyword::parse("-riscv"), Keyword::Broken(_)));
    }
}
