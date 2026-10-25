// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - Slackware BSD-style init and .txz package format
// Principles: UNIX PHILOSOPHY, SIMPLICITY, KEEP IT SIMPLE STUPID
use alloc::{collections::BTreeMap, string::String, vec::Vec};
#[derive(Debug, Clone)]
pub struct SlackRcScript {
    pub name: String,
    pub runlevel: u8,
    pub start_cmd: String,
    pub stop_cmd: String,
    pub enabled: bool,
}
impl SlackRcScript {
    pub fn new(name: &str, runlevel: u8, start: &str, stop: &str) -> Self {
        Self {
            name: name.into(),
            runlevel,
            start_cmd: start.into(),
            stop_cmd: stop.into(),
            enabled: true,
        }
    }
}
pub struct SlackInit {
    pub scripts: Vec<SlackRcScript>,
    pub current_runlevel: u8,
}
impl SlackInit {
    pub fn new() -> Self {
        Self {
            scripts: Vec::new(),
            current_runlevel: 3,
        }
    }
    pub fn register_script(&mut self, s: SlackRcScript) {
        self.scripts.push(s);
    }
    pub fn run_rc_scripts(&self, rl: u8) -> Vec<&str> {
        self.scripts
            .iter()
            .filter(|s| s.runlevel == rl && s.enabled)
            .map(|s| s.name.as_str())
            .collect()
    }
    pub fn determine_runlevel(&self) -> u8 {
        self.current_runlevel
    }
}
#[derive(Debug, Clone)]
pub struct TxzPackage {
    pub name: String,
    pub version: String,
    pub arch: String,
    pub description: String,
    pub files: Vec<String>,
}
impl TxzPackage {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            arch: String::from("x86_64"),
            description: String::new(),
            files: Vec::new(),
        }
    }
    pub fn add_file(&mut self, f: &str) {
        self.files.push(f.into());
    }
    pub fn package_name(&self) -> String {
        alloc::format!("{}-{}-{}.txz", self.name, self.version, self.arch)
    }
}
pub struct PkgtoolInterface {
    pub installed: BTreeMap<String, TxzPackage>,
}
impl PkgtoolInterface {
    pub fn new() -> Self {
        Self {
            installed: BTreeMap::new(),
        }
    }
    pub fn install_pkg(&mut self, pkg: TxzPackage) {
        self.installed.insert(pkg.name.clone(), pkg);
    }
    pub fn remove_pkg(&mut self, name: &str) -> Result<(), &'static str> {
        self.installed
            .remove(name)
            .ok_or("not installed")
            .map(|_| ())
    }
    pub fn list_contents(&self, name: &str) -> Vec<&str> {
        self.installed
            .get(name)
            .map(|p| p.files.iter().map(|f| f.as_str()).collect())
            .unwrap_or_default()
    }
    pub fn upgrade_pkg(&mut self, pkg: TxzPackage) {
        self.installed.insert(pkg.name.clone(), pkg);
    }
    pub fn is_installed(&self, name: &str) -> bool {
        self.installed.contains_key(name)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rc_scripts() {
        let mut init = SlackInit::new();
        init.register_script(SlackRcScript::new(
            "rc.httpd",
            3,
            "httpd start",
            "httpd stop",
        ));
        let scripts = init.run_rc_scripts(3);
        assert_eq!(scripts, &["rc.httpd"]);
    }
    #[test]
    fn test_pkgtool() {
        let mut pt = PkgtoolInterface::new();
        let mut pkg = TxzPackage::new("bash", "5.2.0");
        pkg.add_file("/bin/bash");
        pt.install_pkg(pkg);
        assert!(pt.is_installed("bash"));
        let files = pt.list_contents("bash");
        assert_eq!(files, &["/bin/bash"]);
        pt.remove_pkg("bash").unwrap();
        assert!(!pt.is_installed("bash"));
    }
}
