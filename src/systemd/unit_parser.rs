// SigmaOS Systemd Unit File (.service / .target / .socket) INI Parser
// Reference: systemd unit file specification (systemd.unit(5), systemd.service(5))

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use super::{SystemdServiceUnit, SystemdUnitType};

/// Parsed systemd unit file AST
#[derive(Debug, Clone, Default)]
pub struct SystemdUnitFile {
    pub unit_section: BTreeMap<String, String>,
    pub service_section: BTreeMap<String, String>,
    pub install_section: BTreeMap<String, String>,
}

impl SystemdUnitFile {
    /// Parse unit file content string into structured sections
    pub fn parse(content: &str) -> Result<Self, &'static str> {
        let mut unit_file = Self::default();
        let mut current_section: Option<String> = None;

        for raw_line in content.lines() {
            let line = raw_line.trim();
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                let section_name = &line[1..line.len() - 1];
                current_section = Some(String::from(section_name));
                continue;
            }

            if let Some(ref section) = current_section {
                if let Some(eq_pos) = line.find('=') {
                    let key = line[..eq_pos].trim();
                    let val = line[eq_pos + 1..].trim();

                    match section.as_str() {
                        "Unit" => {
                            unit_file.unit_section.insert(String::from(key), String::from(val));
                        }
                        "Service" => {
                            unit_file.service_section.insert(String::from(key), String::from(val));
                        }
                        "Install" => {
                            unit_file.install_section.insert(String::from(key), String::from(val));
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(unit_file)
    }

    /// Converts parsed INI unit file into a SystemdServiceUnit runtime instance
    pub fn to_service_unit(&self, unit_name: &str) -> SystemdServiceUnit {
        let mut unit = SystemdServiceUnit::new(String::from(unit_name));

        if let Some(desc) = self.unit_section.get("Description") {
            unit.description = desc.clone();
        }

        if let Some(after) = self.unit_section.get("After") {
            unit.after = after.split_whitespace().map(String::from).collect();
        }

        if let Some(before) = self.unit_section.get("Before") {
            unit.before = before.split_whitespace().map(String::from).collect();
        }

        if let Some(requires) = self.unit_section.get("Requires") {
            unit.requires = requires.split_whitespace().map(String::from).collect();
        }

        if let Some(wants) = self.unit_section.get("Wants") {
            for want in wants.split_whitespace() {
                unit.requires.push(String::from(want));
            }
        }

        if let Some(exec_start) = self.service_section.get("ExecStart") {
            unit.exec_start = exec_start.clone();
        }

        if let Some(exec_stop) = self.service_section.get("ExecStop") {
            unit.exec_stop = exec_stop.clone();
        }

        if let Some(exec_reload) = self.service_section.get("ExecReload") {
            unit.exec_reload = exec_reload.clone();
        }

        if let Some(restart) = self.service_section.get("Restart") {
            unit.restart = restart.clone();
        }

        if let Some(restart_sec) = self.service_section.get("RestartSec") {
            if let Ok(sec) = restart_sec.parse::<u32>() {
                unit.restart_sec = sec;
            }
        }

        if let Some(user) = self.service_section.get("User") {
            unit.user = user.clone();
        }

        if let Some(group) = self.service_section.get("Group") {
            unit.group = group.clone();
        }

        if let Some(workdir) = self.service_section.get("WorkingDirectory") {
            unit.working_directory = workdir.clone();
        }

        if let Some(env_str) = self.service_section.get("Environment") {
            for pair in env_str.split_whitespace() {
                if let Some(eq) = pair.find('=') {
                    let k = &pair[..eq];
                    let v = &pair[eq + 1..];
                    unit.environment.insert(String::from(k), String::from(v));
                }
            }
        }

        if let Some(wanted_by) = self.install_section.get("WantedBy") {
            unit.wanted_by = wanted_by.split_whitespace().map(String::from).collect();
        }

        unit
    }
}
