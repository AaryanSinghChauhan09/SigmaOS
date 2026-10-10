//! Pop!_OS Repolib & Linux Mint SoftwareSources APT Repository Engine
//!
//! Inspired by Pop!_OS Repolib (patched APT sources handling library) and Linux Mint SoftwareSources.
//! Implements safe parsing, management, PPA handling, DEB822 file format support, and mirror speed testing
//! for APT repository sources (`/etc/apt/sources.list`, `/etc/apt/sources.list.d/*.list`, and `.sources` DEB822 files).

use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// APT Source File Format Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AptSourceFormat {
    ClassicOneLine, // Traditional deb http://... suite component
    Deb822Format,   // Modern DEB822 key-value stanzas (.sources)
}

/// APT Repository Type (Binary or Source Code)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AptRepoType {
    Deb,    // Binary packages
    DebSrc, // Source packages
}

impl AptRepoType {
    pub fn as_str(&self) -> &str {
        match self {
            AptRepoType::Deb => "deb",
            AptRepoType::DebSrc => "deb-src",
        }
    }
}

/// APT Repository Source Entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AptRepositorySource {
    pub id: String,
    pub repo_type: AptRepoType,
    pub uri: String,
    pub suite: String, // e.g. "noble", "virginiamint", "stable"
    pub components: Vec<String>, // e.g. ["main", "restricted", "universe", "multiverse"]
    pub architectures: Vec<String>, // e.g. ["amd64", "arm64"]
    pub signed_by_keyring: Option<String>, // e.g. "/usr/share/keyrings/linuxmint-archive-keyring.gpg"
    pub format: AptSourceFormat,
    pub enabled: bool,
    pub comment: Option<String>,
}

impl AptRepositorySource {
    pub fn new(id: &str, uri: &str, suite: &str, components: &[&str]) -> Self {
        Self {
            id: id.to_string(),
            repo_type: AptRepoType::Deb,
            uri: uri.to_string(),
            suite: suite.to_string(),
            components: components.iter().map(|s| s.to_string()).collect(),
            architectures: vec!["amd64".to_string()],
            signed_by_keyring: None,
            format: AptSourceFormat::ClassicOneLine,
            enabled: true,
            comment: None,
        }
    }

    /// Renders traditional one-line APT source string (`deb [signed-by=...] http://... suite main`)
    pub fn to_one_line_spec(&self) -> String {
        let prefix = if self.enabled { "" } else { "# " };
        let repo_type_str = self.repo_type.as_str();

        let mut options = Vec::new();
        if !self.architectures.is_empty() {
            options.push(format!("arch={}", self.architectures.join(",")));
        }
        if let Some(ref key) = self.signed_by_keyring {
            options.push(format!("signed-by={}", key));
        }

        let options_str = if options.is_empty() {
            String::new()
        } else {
            format!("[{}] ", options.join(" "))
        };

        format!(
            "{}{} {}{} {} {}",
            prefix,
            repo_type_str,
            options_str,
            self.uri,
            self.suite,
            self.components.join(" ")
        )
    }

    /// Renders DEB822 multi-line stanza specification (.sources)
    pub fn to_deb822_stanza(&self) -> String {
        let mut stanza = String::new();
        stanza.push_str(&format!("Enabled: {}\n", if self.enabled { "yes" } else { "no" }));
        stanza.push_str(&format!("Types: {}\n", self.repo_type.as_str()));
        stanza.push_str(&format!("URIs: {}\n", self.uri));
        stanza.push_str(&format!("Suites: {}\n", self.suite));
        stanza.push_str(&format!("Components: {}\n", self.components.join(" ")));
        if !self.architectures.is_empty() {
            stanza.push_str(&format!("Architectures: {}\n", self.architectures.join(" ")));
        }
        if let Some(ref key) = self.signed_by_keyring {
            stanza.push_str(&format!("Signed-By: {}\n", key));
        }
        stanza
    }
}

/// Pop!_OS Repolib & Linux Mint SoftwareSources Manager
pub struct RepolibSourcesManager {
    pub sources: Vec<AptRepositorySource>,
    pub trusted_keyrings: Vec<String>,
}

impl RepolibSourcesManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            sources: Vec::new(),
            trusted_keyrings: Vec::new(),
        };
        mgr.load_default_mint_pop_repos();
        mgr
    }

    /// Loads default Linux Mint and Pop!_OS repositories
    fn load_default_mint_pop_repos(&mut self) {
        let mut mint_main = AptRepositorySource::new(
            "mint-main",
            "https://packages.linuxmint.com",
            "wilma",
            &["main", "upstream", "import", "backport"],
        );
        mint_main.signed_by_keyring = Some("/usr/share/keyrings/linuxmint-archive-keyring.gpg".to_string());
        self.sources.push(mint_main);

        let mut ubuntu_base = AptRepositorySource::new(
            "ubuntu-base",
            "http://archive.ubuntu.com/ubuntu",
            "noble",
            &["main", "restricted", "universe", "multiverse"],
        );
        ubuntu_base.signed_by_keyring = Some("/usr/share/keyrings/ubuntu-archive-keyring.gpg".to_string());
        self.sources.push(ubuntu_base);
    }

    /// Adds or updates an APT repository source
    pub fn add_source(&mut self, source: AptRepositorySource) {
        if let Some(pos) = self.sources.iter().position(|s| s.id == source.id) {
            self.sources[pos] = source;
        } else {
            self.sources.push(source);
        }
    }

    /// Parses PPA specifier (`ppa:user/repo`) into repository source entry
    pub fn add_ppa_repository(&mut self, ppa_spec: &str) -> Result<String, String> {
        if !ppa_spec.starts_with("ppa:") {
            return Err("Invalid PPA specifier: must start with 'ppa:'".to_string());
        }

        let path = &ppa_spec[4..];
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() != 2 {
            return Err("Invalid PPA specifier: expected 'ppa:owner/name'".to_string());
        }

        let owner = parts[0];
        let ppa_name = parts[1];
        let ppa_id = format!("ppa-{}-{}", owner, ppa_name);
        let uri = format!("https://ppa.launchpadcontent.net/{}/{}/ubuntu", owner, ppa_name);

        let mut source = AptRepositorySource::new(&ppa_id, &uri, "noble", &["main"]);
        source.signed_by_keyring = Some(format!("/etc/apt/trusted.gpg.d/{}_{}.gpg", owner, ppa_name));

        self.add_source(source);
        Ok(format!("Added PPA '{}' successfully", ppa_spec))
    }

    /// Toggles enabled/disabled state of repository by ID
    pub fn toggle_source(&mut self, id: &str, enabled: bool) -> Result<(), String> {
        if let Some(source) = self.sources.iter_mut().find(|s| s.id == id) {
            source.enabled = enabled;
            Ok(())
        } else {
            Err(format!("Repository ID '{}' not found", id))
        }
    }

    /// Benchmark and select fastest mirror
    pub fn benchmark_and_switch_mirror(&mut self, repo_id: &str, mirrors: &[(&str, u32)]) -> Result<String, String> {
        let fastest = mirrors.iter().min_by_key(|m| m.1).ok_or("No mirrors provided for benchmarking")?;
        if let Some(source) = self.sources.iter_mut().find(|s| s.id == repo_id) {
            source.uri = fastest.0.to_string();
            Ok(format!("Switched '{}' to fastest mirror: {} ({}ms)", repo_id, fastest.0, fastest.1))
        } else {
            Err(format!("Repository ID '{}' not found", repo_id))
        }
    }

    /// Parses DEB822 stanzas text into `AptRepositorySource` entries
    pub fn parse_deb822_stanzas(content: &str) -> Vec<AptRepositorySource> {
        let mut parsed = Vec::new();
        let mut current_enabled = true;
        let mut current_types = AptRepoType::Deb;
        let mut current_uri = String::new();
        let mut current_suite = String::new();
        let mut current_components = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                if !current_uri.is_empty() {
                    let mut src = AptRepositorySource::new("deb822-parsed", &current_uri, &current_suite, &[]);
                    src.enabled = current_enabled;
                    src.repo_type = current_types;
                    src.components = current_components.clone();
                    src.format = AptSourceFormat::Deb822Format;
                    parsed.push(src);
                    current_uri.clear();
                    current_suite.clear();
                    current_components.clear();
                }
                continue;
            }

            if let Some(pos) = trimmed.find(':') {
                let key = trimmed[..pos].trim();
                let val = trimmed[pos + 1..].trim();
                match key {
                    "Enabled" => current_enabled = val.eq_ignore_ascii_case("yes") || val.eq_ignore_ascii_case("true"),
                    "Types" => {
                        if val.contains("deb-src") {
                            current_types = AptRepoType::DebSrc;
                        } else {
                            current_types = AptRepoType::Deb;
                        }
                    }
                    "URIs" => current_uri = val.to_string(),
                    "Suites" => current_suite = val.to_string(),
                    "Components" => current_components = val.split_whitespace().map(|s| s.to_string()).collect(),
                    _ => {}
                }
            }
        }

        if !current_uri.is_empty() {
            let mut src = AptRepositorySource::new("deb822-parsed", &current_uri, &current_suite, &[]);
            src.enabled = current_enabled;
            src.repo_type = current_types;
            src.components = current_components;
            src.format = AptSourceFormat::Deb822Format;
            parsed.push(src);
        }

        parsed
    }
}

impl Default for RepolibSourcesManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apt_repository_source_formatting() {
        let mut src = AptRepositorySource::new("mint-main", "https://packages.linuxmint.com", "wilma", &["main", "upstream"]);
        src.signed_by_keyring = Some("/usr/share/keyrings/mint.gpg".to_string());

        let one_line = src.to_one_line_spec();
        assert!(one_line.contains("deb [arch=amd64 signed-by=/usr/share/keyrings/mint.gpg]"));
        assert!(one_line.contains("https://packages.linuxmint.com wilma main upstream"));

        let deb822 = src.to_deb822_stanza();
        assert!(deb822.contains("Enabled: yes"));
        assert!(deb822.contains("URIs: https://packages.linuxmint.com"));
        assert!(deb822.contains("Signed-By: /usr/share/keyrings/mint.gpg"));
    }

    #[test]
    fn test_repolib_ppa_addition_and_mirror_switch() {
        let mut mgr = RepolibSourcesManager::new();
        assert!(mgr.sources.len() >= 2);

        let ppa_res = mgr.add_ppa_repository("ppa:cinnamon/stable");
        assert!(ppa_res.is_ok());
        assert!(mgr.sources.iter().any(|s| s.id == "ppa-cinnamon-stable"));

        let mirrors = vec![("https://mirror.us.kernel.org/ubuntu", 12), ("https://mirror.eu.kernel.org/ubuntu", 45)];
        let switch_res = mgr.benchmark_and_switch_mirror("ubuntu-base", &mirrors);
        assert!(switch_res.is_ok());
        assert_eq!(mgr.sources.iter().find(|s| s.id == "ubuntu-base").unwrap().uri, "https://mirror.us.kernel.org/ubuntu");
    }

    #[test]
    fn test_parse_deb822_stanzas() {
        let stanza_text = "Enabled: yes\nTypes: deb\nURIs: http://archive.ubuntu.com/ubuntu\nSuites: noble\nComponents: main universe\n";
        let parsed = RepolibSourcesManager::parse_deb822_stanzas(stanza_text);
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].uri, "http://archive.ubuntu.com/ubuntu");
        assert_eq!(parsed[0].suite, "noble");
        assert_eq!(parsed[0].components, vec!["main", "universe"]);
    }
}
