use alloc::format;
extern crate alloc;
// Sovereign Debian Package (.deb) Translation and Validation Engine for SigmaOS
// Inspired by Debian dpkg and apt system, allowing native absorption of Debian control files and maintainer scripts.

use crate::package::universal::{PackageFormat, PackageSource, UnifiedPackage};
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebianTranslatorError {
    Success = 0,
    InvalidControlHeader = 1,
    MissingRequiredField = 2,
    UnsupportedArchitecture = 3,
    DependencyParsingFailed = 4,
}

/// Debian Maintainer Script Trigger types (postinst, prerm, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebianTriggerType {
    PreInst,
    PostInst,
    PreRm,
    PostRm,
}

/// Represents a translated Debian maintainer script action
#[derive(Debug, Clone)]
pub struct DebianScriptTrigger {
    pub trigger_type: DebianTriggerType,
    pub action_command: String,
}

/// Fully parsed and translated Debian Package Metadata
#[derive(Debug, Clone)]
pub struct DebianPackageMetadata {
    pub package_name: String,
    pub version: String,
    pub architecture: String,
    pub maintainer: String,
    pub dependencies: Vec<String>,
    pub conflicts: Vec<String>,
    pub script_triggers: Vec<DebianScriptTrigger>,
}

impl DebianPackageMetadata {
    pub fn new() -> Self {
        DebianPackageMetadata {
            package_name: String::new(),
            version: String::new(),
            architecture: String::new(),
            maintainer: String::new(),
            dependencies: Vec::new(),
            conflicts: Vec::new(),
            script_triggers: Vec::new(),
        }
    }
}

/// Sovereign Debian Package Translator
pub struct DebianPackageTranslator;

impl DebianPackageTranslator {
    pub fn new() -> Self {
        DebianPackageTranslator
    }

    /// Parses and translates a raw Debian control file string (e.g. from DEBIAN/control)
    pub fn parse_control_file(
        &self,
        control_content: &str,
    ) -> Result<DebianPackageMetadata, DebianTranslatorError> {
        let mut metadata = DebianPackageMetadata::new();

        for line in control_content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() != 2 {
                return Err(DebianTranslatorError::InvalidControlHeader);
            }

            let field = parts[0].trim();
            let value = parts[1].trim();

            match field {
                "Package" => metadata.package_name = value.to_string(),
                "Version" => metadata.version = value.to_string(),
                "Architecture" => {
                    metadata.architecture = value.to_string();
                    if value != "amd64" && value != "all" && value != "any" {
                        return Err(DebianTranslatorError::UnsupportedArchitecture);
                    }
                }
                "Maintainer" => metadata.maintainer = value.to_string(),
                "Depends" => {
                    metadata.dependencies = self.parse_dependencies_list(value);
                }
                "Conflicts" => {
                    metadata.conflicts = self.parse_dependencies_list(value);
                }
                _ => {} // Ignore custom or secondary fields (KISS)
            }
        }

        // Validate required Debian fields
        if metadata.package_name.is_empty()
            || metadata.version.is_empty()
            || metadata.architecture.is_empty()
        {
            return Err(DebianTranslatorError::MissingRequiredField);
        }

        Ok(metadata)
    }

    /// Maps a standard Debian maintainer shell script (like preinst, postinst) into native SigmaOS triggers
    pub fn translate_maintainer_script(
        &self,
        trigger_type: DebianTriggerType,
        script_content: &str,
    ) -> DebianScriptTrigger {
        // Simple translator: strips legacy bash-isms, maps standard directories to SigmaOS equivalents
        let mut clean_script = String::new();
        for line in script_content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with("set -e") {
                continue;
            }
            // Map legacy directories
            let translated_line = line
                .replace("/usr/bin/", "/bin/")
                .replace("/etc/", "/cfg/")
                .replace("/var/log/", "/log/");
            clean_script.push_str(&translated_line);
            clean_script.push('\n');
        }

        DebianScriptTrigger {
            trigger_type,
            action_command: clean_script.trim().to_string(),
        }
    }

    /// Converts a parsed Debian package metadata directly into a native SigmaOS UnifiedPackage
    pub fn translate_to_unified_package(&self, deb_meta: &DebianPackageMetadata) -> UnifiedPackage {
        let mut pkg = UnifiedPackage::new(deb_meta.package_name.clone(), deb_meta.version.clone())
            .with_format(PackageFormat::Deb);

        for dep in &deb_meta.dependencies {
            pkg = pkg.with_dependency(dep.clone());
        }

        for conflict in &deb_meta.conflicts {
            pkg = pkg.with_conflict(conflict.clone());
        }

        pkg.source = PackageSource::Local {
            path: alloc::format!(
                "/tmp/deb_absorb/{}_{}.deb",
                deb_meta.package_name,
                deb_meta.version
            ),
        };
        pkg
    }

    /// Parse standard Debian dependency syntax: e.g. "libc6 (>= 2.31), libssl-dev" -> ["libc6", "libssl-dev"]
    fn parse_dependencies_list(&self, list: &str) -> Vec<String> {
        let mut deps = Vec::new();
        for item in list.split(',') {
            let item = item.trim();
            if item.is_empty() {
                continue;
            }
            // Strip version bounds: e.g. "libc6 (>= 2.31)" -> "libc6"
            let dep_name = match item.find('(') {
                Some(idx) => item[..idx].trim(),
                None => item,
            };
            deps.push(dep_name.to_string());
        }
        deps
    }
}

impl Default for DebianPackageTranslator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debian_control_file_parsing() {
        let raw_control = "Package: curl\n\
                           Version: 7.81.0-1\n\
                           Section: web\n\
                           Priority: optional\n\
                           Architecture: amd64\n\
                           Depends: libc6 (>= 2.31), libcurl4 (= 7.81.0-1), zlib1g\n\
                           Conflicts: curl-ssl\n\
                           Maintainer: Debian Curl Maintainers <pkg-curl-maintainers@lists.alioth.debian.org>\n\
                           Description: command line tool for transferring data with URLs\n";

        let translator = DebianPackageTranslator::new();
        let meta_res = translator.parse_control_file(raw_control);
        assert!(meta_res.is_ok());

        let meta = meta_res.unwrap();
        assert_eq!(meta.package_name, "curl");
        assert_eq!(meta.version, "7.81.0-1");
        assert_eq!(meta.architecture, "amd64");
        assert_eq!(meta.dependencies.len(), 3);
        assert_eq!(meta.dependencies[0], "libc6");
        assert_eq!(meta.dependencies[1], "libcurl4");
        assert_eq!(meta.dependencies[2], "zlib1g");
        assert_eq!(meta.conflicts[0], "curl-ssl");
    }

    #[test]
    fn test_debian_maintainer_script_translation() {
        let translator = DebianPackageTranslator::new();
        let raw_postinst = "#!/bin/sh\n\
                            set -e\n\
                            # Update system alternatives\n\
                            /usr/bin/update-alternatives --install /usr/bin/write write /usr/bin/curl-write 50\n\
                            echo \"curl configured successfully\" > /var/log/curl_install.log\n";

        let trigger =
            translator.translate_maintainer_script(DebianTriggerType::PostInst, raw_postinst);
        assert_eq!(trigger.trigger_type, DebianTriggerType::PostInst);
        assert!(trigger.action_command.contains("/bin/update-alternatives"));
        assert!(trigger.action_command.contains("/log/curl_install.log"));
        assert!(!trigger.action_command.contains("set -e"));
    }

    #[test]
    fn test_debian_to_unified_package_mapping() {
        let translator = DebianPackageTranslator::new();
        let mut meta = DebianPackageMetadata::new();
        meta.package_name = String::from("htop");
        meta.version = String::from("3.1.2");
        meta.architecture = String::from("amd64");
        meta.dependencies.push(String::from("ncurses"));

        let unified = translator.translate_to_unified_package(&meta);
        assert_eq!(unified.name, "htop");
        assert_eq!(unified.version, "3.1.2");
        assert!(unified.formats.contains(&PackageFormat::Deb));
        assert_eq!(unified.dependencies[0], "ncurses");
    }
}
