use crate::security::Permission;
/// Universal Package Format Adapter for SigmaOS (Sovereign Packaging)
/// Natively absorbs, parses, and translates package metadata formats from Apt (.deb),
/// Yum/Rpm (.rpm/.spec), Pacman (PKGBUILD), Snap (snapcraft.yaml), and Flatpak (.json manifests).
/// Translates containerized permissions (Plugs, Plugs/Slots, Finish-args) directly into SigmaOS Capability Gate Permissions.
use crate::sigpkg::{Dependency, Package, Version, VersionConstraint};

#[derive(Debug, Clone)]
pub struct AptDebManifest {
    pub package: String,
    pub version: String,
    pub depends: Vec<String>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct PacmanPkgbuild {
    pub pkgname: String,
    pub pkgver: String,
    pub depends: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SnapcraftManifest {
    pub name: String,
    pub version: String,
    pub summary: String,
    pub confinement: String, // "strict", "classic", "devmode"
    pub plugs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FlatpakManifest {
    pub app_id: String,
    pub command: String,
    pub finish_args: Vec<String>, // Sandboxed permissions like "--share=network", "--share=ipc"
}

pub struct UniversalPackageAdapter;

impl UniversalPackageAdapter {
    pub fn new() -> Self {
        UniversalPackageAdapter
    }

    /// Parses raw Debian control file text (Apt)
    pub fn parse_apt_control(&self, text: &str) -> Result<AptDebManifest, &'static str> {
        let mut package = String::new();
        let mut version = String::new();
        let mut depends = Vec::new();
        let mut description = String::new();

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Some(pos) = line.find(':') {
                let key = line[..pos].trim();
                let val = line[pos + 1..].trim();
                match key {
                    "Package" => package = val.to_string(),
                    "Version" => version = val.to_string(),
                    "Depends" => {
                        for dep in val.split(',') {
                            depends.push(dep.trim().to_string());
                        }
                    }
                    "Description" => description = val.to_string(),
                    _ => {}
                }
            }
        }

        if package.is_empty() || version.is_empty() {
            return Err("Invalid Debian control manifest: missing Package or Version");
        }

        Ok(AptDebManifest {
            package,
            version,
            depends,
            description,
        })
    }

    /// Parses raw PKGBUILD script text (Pacman)
    pub fn parse_pacman_pkgbuild(&self, text: &str) -> Result<PacmanPkgbuild, &'static str> {
        let mut pkgname = String::new();
        let mut pkgver = String::new();
        let mut depends = Vec::new();

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line.starts_with("pkgname=") {
                pkgname = line["pkgname=".len()..]
                    .trim_matches(|c| c == '"' || c == '\'' || c == ' ')
                    .to_string();
            } else if line.starts_with("pkgver=") {
                pkgver = line["pkgver=".len()..]
                    .trim_matches(|c| c == '"' || c == '\'' || c == ' ')
                    .to_string();
            } else if line.starts_with("depends=") {
                let dep_content =
                    line["depends=".len()..].trim_matches(|c| c == '(' || c == ')' || c == ' ');
                for dep in dep_content.split_whitespace() {
                    let cleaned = dep.trim_matches(|c| c == '\'' || c == '"');
                    depends.push(cleaned.to_string());
                }
            }
        }

        if pkgname.is_empty() || pkgver.is_empty() {
            return Err("Invalid PKGBUILD: missing pkgname or pkgver");
        }

        Ok(PacmanPkgbuild {
            pkgname,
            pkgver,
            depends,
        })
    }

    /// Parses raw snapcraft.yaml text (Snap)
    pub fn parse_snapcraft_yaml(&self, text: &str) -> Result<SnapcraftManifest, &'static str> {
        let mut name = String::new();
        let mut version = String::new();
        let mut summary = String::new();
        let mut confinement = String::new();
        let mut plugs = Vec::new();

        let mut in_plugs_block = false;

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(pos) = line.find(':') {
                let key = line[..pos].trim();
                let val = line[pos + 1..].trim();
                in_plugs_block = false;
                match key {
                    "name" => name = val.trim_matches(|c| c == '"' || c == '\'').to_string(),
                    "version" => version = val.trim_matches(|c| c == '"' || c == '\'').to_string(),
                    "summary" => summary = val.trim_matches(|c| c == '"' || c == '\'').to_string(),
                    "confinement" => confinement = val.to_string(),
                    "plugs" => {
                        in_plugs_block = true;
                        if !val.is_empty() {
                            plugs.push(val.to_string());
                        }
                    }
                    _ => {}
                }
            } else if line.starts_with("- ") && in_plugs_block {
                let plug_name = line["- ".len()..].trim();
                plugs.push(plug_name.to_string());
            }
        }

        if name.is_empty() || version.is_empty() {
            return Err("Invalid snapcraft.yaml: missing name or version");
        }

        Ok(SnapcraftManifest {
            name,
            version,
            summary,
            confinement,
            plugs,
        })
    }

    /// Parses raw Flatpak JSON manifest text
    pub fn parse_flatpak_json(&self, text: &str) -> Result<FlatpakManifest, &'static str> {
        let mut app_id = String::new();
        let mut command = String::new();
        let mut finish_args = Vec::new();

        let mut in_finish_args = false;

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            if line.starts_with("\"app-id\"") {
                if let Some(pos) = line.find(':') {
                    app_id = line[pos + 1..]
                        .trim_matches(|c| c == ',' || c == '"' || c == ' ' || c == '\n')
                        .to_string();
                }
            } else if line.starts_with("\"command\"") {
                if let Some(pos) = line.find(':') {
                    command = line[pos + 1..]
                        .trim_matches(|c| c == ',' || c == '"' || c == ' ' || c == '\n')
                        .to_string();
                }
            } else if line.starts_with("\"finish-args\"") {
                in_finish_args = true;
            } else if line.starts_with(']') {
                in_finish_args = false;
            } else if in_finish_args && line.starts_with('"') {
                let arg = line
                    .trim_matches(|c| c == ',' || c == '"' || c == ' ' || c == '\n')
                    .to_string();
                finish_args.push(arg);
            }
        }

        if app_id.is_empty() {
            return Err("Invalid Flatpak JSON: missing app-id");
        }

        Ok(FlatpakManifest {
            app_id,
            command,
            finish_args,
        })
    }

    /// Translates sandboxed containerized permissions (Flatpak/Snap) into SigmaOS native Capability permissions
    pub fn translate_sandbox_permissions(&self, plugs_or_args: &[String]) -> Vec<Permission> {
        let mut permissions = Vec::new();
        for arg in plugs_or_args {
            if arg == "network" || arg == "network-bind" || arg == "--share=network" {
                permissions.push(Permission::NetworkTcp);
                permissions.push(Permission::NetworkUdp);
            } else if arg == "home" || arg == "--filesystem=home" || arg == "--filesystem=host" {
                permissions.push(Permission::FileRead);
                permissions.push(Permission::FileWrite);
            } else if arg == "--share=ipc" {
                permissions.push(Permission::Ipc);
            }
        }
        permissions
    }

    /// Standardizes any foreign parsed manifest into SigmaOS native Package models
    pub fn translate_to_native_package(
        &self,
        name: &str,
        version_str: &str,
        desc: &str,
        raw_deps: &[String],
    ) -> Result<Package, &'static str> {
        let cleaned_ver = if version_str.contains('-') {
            version_str.split('-').next().unwrap()
        } else {
            version_str
        };

        let parsed_ver =
            Version::parse(cleaned_ver).map_err(|_| "Failed to parse semver representation")?;

        let mut dependencies = Vec::new();
        for dep in raw_deps {
            dependencies.push(Dependency {
                name: dep.clone(),
                version_constraint: VersionConstraint::Any,
            });
        }
<<<<<<< HEAD
        Ok(Package::new(
            crate::klib::String::from(name),
            parsed_ver,
            crate::klib::String::from(desc),
||||||| 23ef22a4a

        Ok(Package {
            name: name.to_string(),
            version: parsed_ver,
            description: desc.to_string(),
=======

        Ok(Package::new(
            crate::klib::String::from(name),
            parsed_ver,
            crate::klib::String::from(desc),
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
            dependencies,
<<<<<<< HEAD
            crate::klib::String::from(&format!("SHA256:{}", name)),
        ))
        })
||||||| 23ef22a4a
            checksum: format!("SHA256:{}", name),
        })
=======
            crate::klib::String::from(&format!("SHA256:{}", name)),
        ))
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
    }
}

=======
            format!("SHA256:{}", name),
        ))
    }
}

>>>>>>> origin/jules-12039768019242344345-034693dc
impl Default for UniversalPackageAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apt_control_parsing_and_translation() {
        let adapter = UniversalPackageAdapter::new();
        let manifest_text = r#"
            Package: curl
            Version: 8.2.1
            Depends: libcurl4, libssl3, zlib1g
            Description: Command line tool for transferring data
        "#;

        let parsed = adapter.parse_apt_control(manifest_text).unwrap();
        assert_eq!(parsed.package, "curl");
        assert_eq!(parsed.version, "8.2.1");
        assert_eq!(parsed.depends.len(), 3);

        let native = adapter
            .translate_to_native_package(
                &parsed.package,
                &parsed.version,
                &parsed.description,
                parsed.depends.as_slice(),
            )
            .unwrap();
        assert_eq!(native.name, "curl");
        assert_eq!(native.version, Version::new(8, 2, 1));
    }

    #[test]
    fn test_pacman_pkgbuild_parsing() {
        let adapter = UniversalPackageAdapter::new();
        let pkgbuild_text = r#"
            # Maintainer: Sigma Team
            pkgname=nginx
            pkgver=1.25.1
            depends=('openssl' 'zlib' 'pcre')
        "#;

        let parsed = adapter.parse_pacman_pkgbuild(pkgbuild_text).unwrap();
        assert_eq!(parsed.pkgname, "nginx");
        assert_eq!(parsed.pkgver, "1.25.1");
        assert_eq!(parsed.depends.len(), 3);
        assert_eq!(parsed.depends[0], "openssl");
    }

    #[test]
    fn test_snap_manifest_and_permissions() {
        let adapter = UniversalPackageAdapter::new();
        let yaml_text = r#"
            name: vlc
            version: '3.0.18'
            summary: VLC media player
            confinement: strict
            plugs:
              - network
              - home
        "#;

        let parsed = adapter.parse_snapcraft_yaml(yaml_text).unwrap();
        assert_eq!(parsed.name, "vlc");
        assert_eq!(parsed.version, "3.0.18");
        assert_eq!(parsed.plugs.len(), 2);

        // Verify container permissions map perfectly to SigmaOS capability permissions
        let perms = adapter.translate_sandbox_permissions(parsed.plugs.as_slice());
        assert!(perms.contains(&Permission::NetworkTcp));
        assert!(perms.contains(&Permission::FileRead));
    }

    #[test]
    fn test_flatpak_manifest_and_permissions() {
        let adapter = UniversalPackageAdapter::new();
        let json_text = r#"
            {
                "app-id": "org.mozilla.Firefox",
                "command": "firefox",
                "finish-args": [
                    "--share=ipc",
                    "--share=network",
                    "--filesystem=home"
                ]
            }
        "#;

        let parsed = adapter.parse_flatpak_json(json_text).unwrap();
        assert_eq!(parsed.app_id, "org.mozilla.Firefox");
        assert_eq!(parsed.finish_args.len(), 3);

        let perms = adapter.translate_sandbox_permissions(parsed.finish_args.as_slice());
        assert!(perms.contains(&Permission::Ipc));
        assert!(perms.contains(&Permission::NetworkTcp));
        assert!(perms.contains(&Permission::FileWrite));
    }
}
